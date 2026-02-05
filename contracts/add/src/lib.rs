#![no_std]
use soroban_sdk::{contract, contractimpl, Env, BytesN, U256, String, Vec};
use soroban_sdk::crypto::bn254::{Bn254G1Affine};

#[contract]
pub struct Contract;

[contractimpl]
impl Contract {
  pub fn add_points(env: Env) -> Bn254G1Affine {
    // Create a byte array for a point (1,2)
    let point_bytes: [u8; 64] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,  // X
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2,  // Y
    ];

    // Create a point G on the curve from the byte array
    let point = Bn254G1Affine::from_array(&env, &point_bytes);

    // Negate the point: -G has same x but negated y
    let neg_point = -point.clone();

    // Add the two points (G+(-G)) and the expected returned  
    // result is a (0,0) point
    env.crypto().bn254().g1_add(&point, &neg_point)
  }
}
