#![allow(dead_code)]
#![allow(unused_imports)]

use ark_ff::{Field, PrimeField, Fp2, BigInteger};
use ark_bls12_381::Fq as F;
use ark_std::{One, Zero, UniformRand, test_rng};


fn main() {
    let mut rng = ark_std::test_rng();

    // select random value from the field
    let a = F::rand(&mut rng);

    let modulus = <F as PrimeField>::MODULUS;

    // show that 1 + 1 = 2
    let sum = F::one() + F::one();
    assert_eq!(sum, F::one().double());

    
    // show that the multiplicative inverse of a number multiplied by itself equals one.
    assert_eq!(a * a.inverse().unwrap(), F::one());

    // show that a value raised to the power of the modulus is equal to itself
    // use the `pow` function to raise to a power
    assert_eq!(a.pow(modulus), a);
}
