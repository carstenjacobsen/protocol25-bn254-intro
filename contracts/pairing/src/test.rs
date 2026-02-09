#![cfg(test)]
use super::*;
use soroban_sdk::{Env, crypto::bn254::{Bn254G1Affine, Bn254G2Affine}};

#[test]
fn test_simple_pairing_check() {
  // This test checks 

  
  let env = Env::default();
  let contract_id = env.register(Contract, ());
  let client = ContractClient::new(&env, &contract_id);

  // Create a byte array for a G1 point (1,2)
  let p1_point_bytes: [u8; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,  // x
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,  // y
  ];
  
  // Create a point p1 on the G1 curve from the byte array
  let p1 = Bn254G1Affine::from_array(&env, &p1_point_bytes);

  // Create a point p2 by negating the point `p1`
  let p2 = -p1.clone();

  // Create a byte array for a G2 point
  let q1_point_bytes: [u8; 128] = [
    25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37,
    241, 170, 73, 51, 53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194,     // x_1
    24, 0, 222, 239, 18, 31, 30, 118, 66, 106, 0, 102, 94, 92, 68, 121,
    103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92, 217, 146, 246, 237,     // x_0
    9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
    188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91,      // y_1
    18, 200, 94, 165, 219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143,
    227, 209, 231, 105, 12, 67, 211, 123, 76, 230, 204, 1, 102, 250, 125, 170,     // y_0
  ];
  
  // Create a point q1 on the G2 curve from the byte array
  let q1 = Bn254G2Affine::from_array(&env, &q1_point_bytes);

  // Create a point q2 on the G2 curve from the q1 byte array
  let q2 = Bn254G2Affine::from_array(&env, &q1_point_bytes);

  // Call the function and get the result of the pairing check
  let result = client.simple_pairing_check(&p1, &q1, &p2, &q2);

  // Check if the simple_pairing_check() function returns true
  assert_eq!(result, true);
}
