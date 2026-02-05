#![cfg(test)]

use super::*;
use soroban_sdk::{Env, U256, crypto::bn254::{Bn254G1Affine}};

#[test]
fn test_multiply_point() {
  let env = Env::default();
  let contract_id = env.register(Contract, ());
  let client = ContractClient::new(&env, &contract_id);

  // Create a byte array for a point (1,2)
  let point_bytes: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,  // X
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,  // Y
  ];

  // Call the multiply_point() contract function with the 
  // (1,2) point and the value `1` as the multiplier. By
  // Using `1` as the multiplier mean we can expect the 
  // result to be the same as the point provided
  let result = client.multiply_point(
    &Bn254G1Affine::from_array(&env, &point_bytes),
    &U256::from_u32(&env, 1)
  );

  // Convert the byte array to a Bn254G1Affine point
  let point = Bn254G1Affine::from_array(&env, &point_bytes);

  // Check if the multiply_point() function returns the (1,2) 
  // point after the multiplication
  assert_eq!(result, point);
}
