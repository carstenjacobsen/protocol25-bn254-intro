





pub fn simple_pairing_check(env: Env, p1: Bn254G1Affine, p2: Bn254G1Affine, q1: Bn254G2Affine, q2: Bn254G2Affine) -> bool {
  // Simple pairing check: verify e(P1, Q1) * e(P2, Q2) = 1

  // Create vector of G1 points
  let mut g1_points = Vec::new(&env);
  g1_points.push_back(p1);
  g1_points.push_back(p2);

  // Create vector of G2 points
  let mut g2_points = Vec::new(&env);
  g2_points.push_back(q1);
  g2_points.push_back(q2);
  
  // Call pairing_check() with G1Affine and G2Affine vectors
  env.crypto().bn254().pairing_check(g1_points, g2_points)
}
