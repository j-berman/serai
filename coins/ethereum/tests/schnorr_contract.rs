use rand_core::OsRng;

use k256::{elliptic_curve::bigint::ArrayEncoding, ProjectivePoint, U256};

use ethers::{
  prelude::*,
  utils::{keccak256, Anvil, AnvilInstance},
};

use frost::{
  curve::Secp256k1,
  algorithm::Schnorr as Algo,
  FrostKeys,
  tests::{algorithm_machines, sign},
};

use std::{convert::TryFrom, collections::HashMap, sync::Arc, time::Duration};

use ethereum_serai::{
  crypto,
  schnorr_contract::{Schnorr, call_verify, deploy_schnorr_verifier_contract},
};

mod utils;
use crate::utils::{generate_keys};

async fn deploy_test_contract(
) -> (u32, AnvilInstance, Schnorr<SignerMiddleware<Provider<Http>, LocalWallet>>) {
  let anvil = Anvil::new().spawn();

  let wallet: LocalWallet = anvil.keys()[0].clone().into();
  let provider =
    Provider::<Http>::try_from(anvil.endpoint()).unwrap().interval(Duration::from_millis(10u64));
  let chain_id = provider.get_chainid().await.unwrap().as_u32();
  let client = Arc::new(SignerMiddleware::new_with_provider_chain(provider, wallet).await.unwrap());

  (chain_id, anvil, deploy_schnorr_verifier_contract(client).await.unwrap())
}

#[tokio::test]
async fn test_deploy_contract() {
  deploy_test_contract().await;
}

#[tokio::test]
async fn test_ecrecover_hack() {
  let (keys, group_key): (HashMap<u16, FrostKeys<Secp256k1>>, ProjectivePoint) =
    generate_keys().await;

  let (chain_id, _anvil, contract) = deploy_test_contract().await;
  let chain_id = U256::from(chain_id);

  let keys = key_gen::<_, Secp256k1>(&mut OsRng);
  let group_key = keys[&1].group_key();

  const MESSAGE: &'static [u8] = b"Hello, World!";
  let hashed_message = keccak256(MESSAGE);

  let full_message = &[chain_id.to_be_byte_array().as_slice(), &hashed_message].concat();

  let algo = Algo::<Secp256k1, crypto::EthereumHram>::new();
  let sig = sign(
    &mut OsRng,
    algo.clone(),
    keys.clone(),
    algorithm_machines(&mut OsRng, algo, &keys),
    full_message,
  );
  let mut processed_sig =
    crypto::process_signature_for_contract(hashed_message, &sig.R, sig.s, &group_key, chain_id);

  call_verify(&contract, &processed_sig).await.unwrap();

  // test invalid signature fails
  processed_sig.message[0] = 0;
  assert!(call_verify(&contract, &processed_sig).await.is_err());
}
