# Protocol 25 BN254 Intro

Protocol 25, also known as X-Ray, introduced three new ZK Proof primitives in Soroban. Future protocol upgrades will add primitives, but let's take a look at what was included in this current protocol, and how to use the new primitives.

### BN254 curve subgroups
The BN254 elliptic curve has two subgroups, G1 and G2. G1 is the primary subgroup and the addition and multiplication operations included in Protocol 25 X-Ray are performed on points in the G1 subgroup. Points in the G1 subgroup are (x,y) pairs of field element that satisfy the curve equation `y² = x³ + 3`.

G2 is the second subgroup used in BN254, and each coordinate of a G2 point is itself a pair of field elements, meaning a G2 point requires 4 field elements to represent (compared to 2 for G1). G2 consists of points on a twisted curve, and the curve equation for G2 over the twist is typically `y² = x³ + b/ξ` (a "sextic twist" of the original curve, where ξ is a specific element).

There are plenty of resources describing BN254 if you are interested in the cryptographics of DN254.

### What's new?
BN254 is a widely used pairing-friendly elliptic curve, and is often used in ZK Proofs systems. In this upgrade these three primitives were added to Soroban:

* **g1_add()** - used for point addition
* **g1_mul()** - used for scalar multiplication
* **pairing_check()** - used for pairing checks

Besides the primitives above, three new BN254 types have been added:

* **Bn254G1Affine** - represents a point (x,y) on the BN254 elliptic curve, in the primary subgroup G1
* **Bn254G2Affine** - represents a point ((x1,y1)(x2,y2)) on the BN254 elliptic curve, in the secondary subgroup G2
* **Fr** - represents a scalar field

For more information about X-Ray/Protocol 25, see the [blog post](https://stellar.org/blog/developers/announcing-stellar-x-ray-protocol-25).


## g1_add()
The `g1_add()` function is a native host function for elliptic-curve point addition in the G1 subgroup. The function takes two points in the G1 as arguments, they have to be in `Bn254G1Affine` format:

```rust
pub fn g1_add(&self, p0: &Bn254G1Affine, p1: &Bn254G1Affine) -> Bn254G1Affine
```

The sum of `p0` and `p1` is returned as a new point in the `Bn254G1Affine` format.

### Example
This example uses the `g1_add()` function to add two points, and then check if the result is as expected. The code for this example is [here](/contracts/add).

The `add_points()` is a very simple contract function, it takes two points as arguments and return the point calculated from the points addition. The example test shows how points on the curve can be defined and used for this function. 

```rust
pub fn add_points(env: Env, point_1: Bn254G1Affine, point_2: Bn254G1Affine) -> Bn254G1Affine {
  env.crypto().bn254().g1_add(&point_1, &point_2)
}
```

The test creates a point based on a byte array, and another point which is a negated version of the first point. When these two points are added, the result should be (0,0) point since `G+(-G) = 0`.

```rust
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

  // Negate the point (has same x but negated y)
  let neg_point = -point.clone();

  // Call the function and get the result of the addition
  let result = client.add_points(&point, &neg_point);

  // Create a (0,0) point on the curve
  let zero_point = Bn254G1Affine::from_array(&env, &[0u8; 64]);
  
  // Check if the add_points() function returns a (0,0) point
  assert_eq!(result, zero_point);
}
```

## g1_mul()
The `g1_mul()` function is a native host function for elliptic-curve point multiplication in the G1 subgroup. The function takes a point in G1 and a scalar value as arguments:

```rust
pub fn g1_mul(&self, p0: &Bn254G1Affine, scalar: &Fr) -> Bn254G1Affine
```

The result of `p0` multiplied by `scalar` is returned as a new point in the `Bn254G1Affine` format.

### Example
This example uses the `g1_mul()` function to multiply a point and a scalar value, and then check if the result is as expected. The code for this example is [here](/contracts/multiply).

The `multiply_points()` is a very simple contract function, it takes a point and a value as arguments and return the point calculated from the point multiplication. The example test shows how a point on the curve can be defined and used for this function. 

```rust
pub fn multiply_point(env: Env, point: Bn254G1Affine, scalar: U256) -> Bn254G1Affine {
  // Convert the U256 multiplier to a BN254 scalar value
  let scalar = Fr::from(scalar);

  // Multiply the provided point by the provided value
  env.crypto().bn254().g1_mul(&point, &scalar)
}
```

The test creates a point based on a byte array, and use the value `1` as the value the point is multiplied by. The result should be the same as the created point since `G*1 = G`.

```rust
#[test]
fn test_multiply_point() {
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
```

## pairing_check()
The `pairing_check()` function is a native host function for multi-pairing checking between vectors of points in G1 and G2. The function takes the parameters `vp1`, a vector of G1 points, and `vp2`, a vector of G2 points, and verifies if the product of all pairings is equal to `1`. 

```rust
pub fn pairing_check(&self, vp1: Vec<Bn254G1Affine>, vp2: Vec<Bn254G2Affine>) -> bool
```

If the product of the pairings is `1`, the function returns `true`, otherwise `false`.

### Example
This example uses the `pairing_check()` to verify the equation `e(P1, Q1) * e(P2, Q2) = 1` for points in the G1 and G2 subgroups. The code for this example is [here](/contracts/pairing).

The `simple_pairing_check()` contract function takes two G1 points and two G2 points and checks if the `e(P1, Q1) * e(P2, Q2) = 1` is true or not. The points are just test points, but could be proof from a ZK proofs system. The function takes the four points, add them to a G1Affine point vector and a G2Affine point vector, and calls the `pairing_check()` host function with the two vectors as parameters. 

The host function will return either `true` or `false` depending on the check outcome. The function will panic if the provided points are not valid, so random data cannot be used, the points have to exist in the curve.

```rust
pub fn simple_pairing_check(env: Env, p1: Bn254G1Affine, p2: Bn254G1Affine, q1: Bn254G2Affine, q2: Bn254G2Affine) -> bool {
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
```

The test 

```rust
#[test]
fn test_simple_pairing_check() {
  // This test is a simple pairing check, it verifies that 
  // e(P1, Q1) * e(P2, Q2) = 1 is true for the provided G1
  // and G2 points. The points p1, p2, q1 and q2 are derived
  // from two point byte arrays.
  
  let env = Env::default();
  let contract_id = env.register(Contract, ());
  let client = ContractClient::new(&env, &contract_id);

  // Create a byte array for a G1 point
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
  let result = client.simple_pairing_check(&p1, &p2, &q1, &q2);

  // Check if the simple_pairing_check() function returns true
  assert_eq!(result, true);
}
```
