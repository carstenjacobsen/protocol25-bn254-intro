#![cfg(test)]

use super::*;
use soroban_sdk::{Env, U256, crypto::bn254::{Bn254G1Affine}};

#[test]
fn test_multiply_point() {
  let env = Env::default();
  let contract_id = env.register(Contract, ());
  let client = ContractClient::new(&env, &contract_id);
  
  let point_bytes: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,  // X
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,  // Y
  ];
  
  let result = client.multiply_point(
    &Bn254G1Affine::from_array(&env, &point_bytes),
    &U256::from_u32(&env, 1)
  );

  let point = Bn254G1Affine::from_array(&env, &point_bytes);
  
  assert_eq!(result, point);
}
