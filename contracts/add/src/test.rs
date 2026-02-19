#![cfg(test)]

use super::*;
use soroban_sdk::{Env, crypto::bn254::{Bn254G1Affine}};

#[test]
fn test_add_points() {
  let env = Env::default();
  let contract_id = env.register(Contract, ());
  let client = ContractClient::new(&env, &contract_id);

  // Create a byte array for a generator point (1,2)
  let point_bytes: [u8; 64] = [
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,  // x
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,  // y
  ];

  // Create a generator point on the curve from the byte array
  let point_gen = Bn254G1Affine::from_array(&env, &point_bytes);

  // Create the 2G point
  let scalar_2g = Fr::from(U256::from_u32(&env, 2));
  let point_2g = env.crypto().bn254().g1_mul(&point_p, &scalar_2g);

  // Call the function and get the result of the addition
  let result = client.add_points(&point_gen, &point_2g);

  // Create the 3G point
  let scalar_3g = Fr::from(U256::from_u32(&env, 3));
  let point_3g = env.crypto().bn254().g1_mul(&point_p, &scalar_3g);

  // Check if the add_points() function returns a 3G point
  assert_eq!(result, point_3g);
}
  

