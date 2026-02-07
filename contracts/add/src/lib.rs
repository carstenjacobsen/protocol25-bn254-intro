#![no_std]
use soroban_sdk::{contract, contractimpl, Env, BytesN, U256, String, Vec};
use soroban_sdk::crypto::bn254::{Bn254G1Affine};

#[contract]
pub struct Contract;

[contractimpl]
impl Contract {
  pub fn add_points(env: Env, point_1: Bn254G1Affine, point_2: Bn254G1Affine) -> Bn254G1Affine {
      env.crypto().bn254().g1_add(&point_1, &point_2)
  }
}
