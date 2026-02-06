# Protocol 25 BN254 Intro

Protocol 25, also known as X-Ray, introduced three new ZK Proof primitives in Soroban. Future protocol upgrades will add primitives, but let's take a look at what was included in this current protocol, and how to use the new primitives.

### BN254 curve subgroups
The BN254 elliptic curve has two subgroups, G1 and G2. G1 is the primary subgroup and the addition and multiplication operations included in Protocol 25 X-Ray are performed on points in the G1 subgroup. Points in the G1 subgroup are (x,y) pairs of field element that satisfy the curve equation `y² = x³ + 3`.

G2 is the second subgroup used in BN254, and each coordinate of a G2 point is itself a pair of field elements, meaning a G2 point requires 4 field elements to represent (compared to 2 for G1). G2 consists of points on a twisted curve, and the curve equation for G2 over the twist is typically `y² = x³ + b/ξ` (a "sextic twist" of the original curve, where ξ is a specific element).

### What's new?
BN254 is a widely used pairing-friendly elliptic curve, and is often used in ZK Proofs systems. In this upgrade these three primitives were added to Soroban:

* **g1_add()** - used for point addition
* **g1_mul()** - used for scalar multiplication
* **pairing_check()** - used for pairing checks

Besides the primitives above, three new BN254 types have been added:

* **Bn254G1Affine** - represents a point (x,y) on the BN254 elliptic curve, in the primary subgroup G1
* **Bn254G2Affine** - represents a point (x,y) on the BN254 elliptic curve, in the secondary subgroup G2
* **Fr** - represents a scalar field

For more information about X-Ray/Protocol 25, see the [blog post](https://stellar.org/blog/developers/announcing-stellar-x-ray-protocol-25).


## g1_add()
The `g1_add()` function is a native host function for elliptic-curve point addition on the BN254 curve’s G1 group. 


## g1_mul()



## pairing_check()


