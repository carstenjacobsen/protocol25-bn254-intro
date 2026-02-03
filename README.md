# Protocol 25 BN254 Intro

Protocol 25, also known as X-Ray, introduced three new ZK Proof primitives in Soroban. Future protocol upgrades will add primitives, but let's take a look at what was included in this current protocol, and how to use the new primitives.

### What's new?
BN254 is a widely used pairing-friendly elliptic curve, and is often used in ZK Proofs systems. In this upgrade these three primitives were added to Soroban:

* **g1_add()** - used for point addition
* **g1_mul()** - used for scalar multiplication
* **pairing_check()** - used for pairing checks

For more information about X-Ray/Protocol 25, see the [blog post](https://stellar.org/blog/developers/announcing-stellar-x-ray-protocol-25).

## g1_add()
The `g1_add()` function is a native host function for elliptic-curve point addition on the BN254 curve’s G1 group. 



