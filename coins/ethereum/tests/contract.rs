use ethereum_serai::{
  crypto,
  schnorr_contract::{call_verify, deploy_schnorr_verifier_contract},
  router_contract::{call_router_execute, deploy_router_contract, router_mod},
};
use frost::{curve::Secp256k1, FrostKeys};
use k256::ProjectivePoint;
use ethers::{
  prelude::*,
  utils::{Anvil, keccak256},
  abi,
};
use std::{convert::TryFrom, collections::HashMap, sync::Arc, time::Duration};

#[tokio::test]
async fn test_deploy_schnorr_contract() {
  let anvil = Anvil::new().spawn();
  let wallet: LocalWallet = anvil.keys()[0].clone().into();
  let provider =
    Provider::<Http>::try_from(anvil.endpoint()).unwrap().interval(Duration::from_millis(10u64));
  let client = Arc::new(SignerMiddleware::new(provider, wallet));

  let _contract = deploy_schnorr_verifier_contract(client).await.unwrap();
}

#[tokio::test]
async fn test_deploy_router_contract() {
  let anvil = Anvil::new().spawn();
  let wallet: LocalWallet = anvil.keys()[0].clone().into();
  let provider =
    Provider::<Http>::try_from(anvil.endpoint()).unwrap().interval(Duration::from_millis(10u64));
  let client = Arc::new(SignerMiddleware::new(provider, wallet));

  let _contract = deploy_router_contract(client).await.unwrap();
}

async fn generate_keys() -> (HashMap<u16, FrostKeys<Secp256k1>>, ProjectivePoint) {
  use frost::{tests::key_gen};
  use rand_core::OsRng;

  let keys = key_gen::<_, Secp256k1>(&mut OsRng);
  let group_key = keys[&1].group_key();
  (keys, group_key)
}

async fn hash_and_sign(
  message: &[u8],
  keys: &HashMap<u16, FrostKeys<Secp256k1>>,
  group_key: &ProjectivePoint,
  chain_id: ethers::prelude::U256,
) -> crypto::ProcessedSignature {
  use frost::{
    algorithm::Schnorr,
    tests::{algorithm_machines, sign},
  };
  use k256::{elliptic_curve::bigint::ArrayEncoding, Scalar, U256};
  use rand_core::OsRng;

  let hashed_message = keccak256(message);
  let chain_id = U256::from(Scalar::from(chain_id.as_u32()));

  let full_message = &[chain_id.to_be_byte_array().as_slice(), &hashed_message].concat();

  let sig = sign(
    &mut OsRng,
    algorithm_machines(&mut OsRng, Schnorr::<Secp256k1, crypto::EthereumHram>::new(), &keys),
    full_message,
  );
  crypto::process_signature_for_contract(hashed_message, &sig.R, sig.s, &group_key, chain_id)
}

#[tokio::test]
async fn test_call_router_execute() {
  let (keys, group_key): (HashMap<u16, FrostKeys<Secp256k1>>, ProjectivePoint) =
    generate_keys().await;

  let anvil = Anvil::new().spawn();
  let wallet: LocalWallet = anvil.keys()[0].clone().into();
  let provider =
    Provider::<Http>::try_from(anvil.endpoint()).unwrap().interval(Duration::from_millis(10u64));
  let chain_id = provider.get_chainid().await.unwrap();
  let client = Arc::new(SignerMiddleware::new(provider, wallet));

  let to = H160([0u8; 20]);
  let value = U256([0u64; 4]);
  let data = Bytes::from([0]);
  let tx = router_mod::Transaction { to: to.clone(), value: value.clone(), data: data.clone() };
  let txs = vec![tx];

  // try with wrong message
  const MESSAGE: &'static [u8] = b"Hello, World!";
  let processed_sig = hash_and_sign(MESSAGE, &keys, &group_key, chain_id).await;

  let contract = deploy_router_contract(client.clone()).await.unwrap();
  let res = call_router_execute(&contract, txs.clone(), &processed_sig).await;
  assert!(res.is_err()); // should revert as signature is for incorrect message

  // try w actual data
  let tokens = vec![abi::Token::Array(vec![abi::Token::Tuple(vec![
    abi::Token::Address(to),
    abi::Token::Uint(value),
    abi::Token::Bytes(data.to_vec()),
  ])])];
  let encoded_calldata = abi::encode(&tokens);
  let processed_sig = hash_and_sign(&encoded_calldata, &keys, &group_key, chain_id).await;
  let contract = deploy_router_contract(client).await.unwrap();
  let receipt = call_router_execute(&contract, txs.clone(), &processed_sig).await.unwrap().unwrap();
  println!("gas used: {:?}", receipt.cumulative_gas_used);
}

#[tokio::test]
async fn test_ecrecover_hack() {
  let (keys, group_key): (HashMap<u16, FrostKeys<Secp256k1>>, ProjectivePoint) =
    generate_keys().await;

  let anvil = Anvil::new().spawn();
  let wallet: LocalWallet = anvil.keys()[0].clone().into();
  let provider =
    Provider::<Http>::try_from(anvil.endpoint()).unwrap().interval(Duration::from_millis(10u64));
  let chain_id = provider.get_chainid().await.unwrap();
  let client = Arc::new(SignerMiddleware::new(provider, wallet));

  const MESSAGE: &'static [u8] = b"Hello, World!";
  let mut processed_sig = hash_and_sign(MESSAGE, &keys, &group_key, chain_id).await;

  let contract = deploy_schnorr_verifier_contract(client).await.unwrap();
  call_verify(&contract, &processed_sig).await.unwrap();

  // test invalid signature fails
  processed_sig.message[0] = 0;
  let res = call_verify(&contract, &processed_sig).await;
  assert!(res.is_err());
}
