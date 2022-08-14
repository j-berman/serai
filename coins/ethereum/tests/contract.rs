use ethereum_serai::contract::{
  call_router_execute, call_router_execute_no_abi_encode, call_verify, deploy_router_contract,
  deploy_schnorr_verifier_contract, router_mod,
};
use ethers::{prelude::*, utils::Anvil, abi};
use std::{convert::TryFrom, sync::Arc, time::Duration};

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

#[tokio::test]
async fn test_call_router_execute() {
  use ethereum_serai::crypto;
  use ethers::utils::keccak256;
  use frost::{
    algorithm::Schnorr,
    curve::Secp256k1,
    tests::{algorithm_machines, key_gen, sign},
  };
  use k256::elliptic_curve::bigint::ArrayEncoding;
  use k256::{Scalar, U256};
  use rand_core::OsRng;

  let anvil = Anvil::new().spawn();
  let wallet: LocalWallet = anvil.keys()[0].clone().into();
  let provider =
    Provider::<Http>::try_from(anvil.endpoint()).unwrap().interval(Duration::from_millis(10u64));
  let chain_id = provider.get_chainid().await.unwrap();
  let client = Arc::new(SignerMiddleware::new(provider, wallet));

  let keys = key_gen::<_, Secp256k1>(&mut OsRng);
  let group_key = keys[&1].group_key();

  let to = H160([0u8; 20]);
  let value = U256([0u64; 4]);
  let data = Bytes::from([0]);
  let tx = router_mod::Transaction { to: to.clone(), value: value.clone(), data: data.clone() };
  let txs = vec![tx];

  const MESSAGE: &'static [u8] = b"Hello, World!";
  let hashed_message = keccak256(MESSAGE);
  let chain_id = U256::from(Scalar::from(chain_id.as_u32()));

  let full_message = &[chain_id.to_be_byte_array().as_slice(), &hashed_message].concat();

  let sig = sign(
    &mut OsRng,
    algorithm_machines(&mut OsRng, Schnorr::<Secp256k1, crypto::EthereumHram>::new(), &keys),
    full_message,
  );
  let processed_sig =
    crypto::process_signature_for_contract(hashed_message, &sig.R, sig.s, &group_key, chain_id);

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
  let hashed_message = keccak256(encoded_calldata);

  let full_message = &[chain_id.to_be_byte_array().as_slice(), &hashed_message].concat();

  let sig = sign(
    &mut OsRng,
    algorithm_machines(&mut OsRng, Schnorr::<Secp256k1, crypto::EthereumHram>::new(), &keys),
    full_message,
  );
  let processed_sig =
    crypto::process_signature_for_contract(hashed_message, &sig.R, sig.s, &group_key, chain_id);

  let contract = deploy_router_contract(client).await.unwrap();
  call_router_execute(&contract, txs.clone(), &processed_sig).await.unwrap();
  call_router_execute_no_abi_encode(&contract, txs, &processed_sig).await.unwrap();
}

#[tokio::test]
async fn test_ecrecover_hack() {
  use ethereum_serai::crypto;
  use ethers::utils::keccak256;
  use frost::{
    algorithm::Schnorr,
    curve::Secp256k1,
    tests::{algorithm_machines, key_gen, sign},
  };
  use k256::elliptic_curve::bigint::ArrayEncoding;
  use k256::{Scalar, U256};
  use rand_core::OsRng;

  let anvil = Anvil::new().spawn();
  let wallet: LocalWallet = anvil.keys()[0].clone().into();
  let provider =
    Provider::<Http>::try_from(anvil.endpoint()).unwrap().interval(Duration::from_millis(10u64));
  let chain_id = provider.get_chainid().await.unwrap();
  let client = Arc::new(SignerMiddleware::new(provider, wallet));

  let keys = key_gen::<_, Secp256k1>(&mut OsRng);
  let group_key = keys[&1].group_key();

  const MESSAGE: &'static [u8] = b"Hello, World!";
  let hashed_message = keccak256(MESSAGE);
  let chain_id = U256::from(Scalar::from(chain_id.as_u32()));

  let full_message = &[chain_id.to_be_byte_array().as_slice(), &hashed_message].concat();

  let sig = sign(
    &mut OsRng,
    algorithm_machines(&mut OsRng, Schnorr::<Secp256k1, crypto::EthereumHram>::new(), &keys),
    full_message,
  );
  let mut processed_sig =
    crypto::process_signature_for_contract(hashed_message, &sig.R, sig.s, &group_key, chain_id);

  let contract = deploy_schnorr_verifier_contract(client).await.unwrap();
  call_verify(&contract, &processed_sig).await.unwrap();

  // test invalid signature fails
  processed_sig.message[0] = 0;
  let res = call_verify(&contract, &processed_sig).await;
  assert!(res.is_err());
}
