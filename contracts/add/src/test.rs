#![cfg(test)]

use super::*;
use soroban_sdk::{Env, crypto::bn254::{Bn254G1Affine}};

#[test]
fn test_add_points() {
  let env = Env::default();
  let contract_id = env.register(Contract, ());
  let client = ContractClient::new(&env, &contract_id);

  // Create a byte array for a point (1,2)
  let point_bytes: [u8; 64] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,  // x
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,  // y
  ];

  // Create a point on the curve from the byte array
  let point = Bn254G1Affine::from_array(&env, &point_bytes);

  // Negate the point: -G has same x but negated y
  let neg_point = -point.clone();

  // Call the function and get the result of the addition
  let result = client.add_points(&point, &neg_point);

  // Create a (0,0) point on the curve
  let zero_point = Bn254G1Affine::from_array(&env, &[0u8; 64]);
  
  // Check if the add_points() function returns a (0,0) point
  assert_eq!(result, zero_point);
}
  

