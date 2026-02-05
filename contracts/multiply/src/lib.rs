#![no_std]
use soroban_sdk::{contract, contractimpl, Env, U256};
use soroban_sdk::crypto::bn254::{Bn254G1Affine, Fr};

#[contract]
pub struct Contract;

[contractimpl]
impl Contract {
  pub fn multiply_point(env: Env, point: Bn254G1Affine, scalar: U256) -> Bn254G1Affine {
    // Convert the U256 multiplier to a BN254 scalar value
    let scalar = Fr::from(scalar);
  
    // Multiply the provided point by the provided value
    env.crypto().bn254().g1_mul(&point, &scalar)
  }
}
