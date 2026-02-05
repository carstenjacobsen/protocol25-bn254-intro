#![cfg(test)]

use super::*;
use soroban_sdk::{Env, crypto::bn254::{Bn254G1Affine}};

#[test]
fn test_add_points() {
  let env = Env::default();
  let contract_id = env.register(Contract, ());
  let client = ContractClient::new(&env, &contract_id);

  // Call the contract add_points()
  let result = client.add_points();
  // Create a (0,0) point on the curve
  let infinity = Bn254G1Affine::from_array(&env, &[0u8; 64]);
  // Check if the add_points() function returns the (0,0) point
  assert_eq!(result, infinity);
}
  

