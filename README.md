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
The `g1_add()` function is a native host function for elliptic-curve point addition on the G1 subgroup. The function takes two points on the G1 as arguments, they have to be in `Bn254G1Affine` format:

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

The test creates a point based on a byte array, and another point which is a negated version of the first point. When these two points are added, the result should be (0,0) point because `G+(-G) = 0`.

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








### Example
This example uses the `g1_mul()` function to multiply a point and a scalar value, and then check if the result is as expected. The code for this example is [here](/contracts/multiply).

The `multiply_points()` is a very simple contract function, it takes a point and a value as arguments and return the point calculated from the points multiplication. The example test shows how a point on the curve can be defined and used for this function. 

```rust
pub fn multiply_point(env: Env, point: Bn254G1Affine, scalar: U256) -> Bn254G1Affine {
  // Convert the U256 multiplier to a BN254 scalar value
  let scalar = Fr::from(scalar);

  // Multiply the provided point by the provided value
  env.crypto().bn254().g1_mul(&point, &scalar)
}
```

The test creates a point based on a byte array, and another point which is a negated version of the first point. When these two points are added, the result should be (0,0) point because `G+(-G) = 0`.

```rust
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
```




## pairing_check()


