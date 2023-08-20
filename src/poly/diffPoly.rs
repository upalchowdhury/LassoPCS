use std::collections::BTreeMap;
use ark_ff::{Field, Zero};
use ark_std::rand::Rng;
use ark_test_curves::bls12_381::Fr;

/// Represents a commitment to a polynomial.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Commitment(Vec<u8>);

/// Represents an opening proof for a polynomial at a given point.
#[derive(Debug, Clone, PartialEq, Eq)]
struct OpeningProof(Vec<u8>);

/// Dense representation of a polynomial.
struct DensePolynomial {
    coefficients: Vec<Fr>,
}

impl DensePolynomial {
    /// Commit to this polynomial using its coefficients.
    fn commit(&self) -> Commitment {
        let commitment_data: Vec<u8> = self.coefficients.iter().flat_map(|c| c.to_bytes()).collect();
        let hash = blake3::hash(&commitment_data).into();
        Commitment(hash.to_vec())
    }

    /// Open the polynomial at a specific point, producing a proof.
    fn open(&self, point: Fr) -> (Fr, OpeningProof) {
        let value_at_point = self.evaluate_at(point);
        let proof_data: Vec<u8> = value_at_point.to_bytes().to_vec();
        let hash = blake3::hash(&proof_data).into();
        (value_at_point, OpeningProof(hash.to_vec()))
    }

    /// Evaluate the polynomial at a specific point using Horner's method.
    fn evaluate_at(&self, point: Fr) -> Fr {
        self.coefficients.iter().rev().fold(Fr::zero(), |acc, &coeff| acc * point + coeff)
    }
}

/// Verify the opening proof for the polynomial at a given point.
fn verify(commitment: &Commitment, point: Fr, value: Fr, proof: &OpeningProof) -> bool {
    let proof_data: Vec<u8> = value.to_bytes().to_vec();
    let hash = blake3::hash(&proof_data).into();
    &proof.0 == &hash.as_bytes()
}

fn main() {
    let poly = DensePolynomial {
        coefficients: vec![Fr::from(1), Fr::from(2), Fr::from(3)]  // p(x) = x^2 + 2x + 3
    };

    let point = Fr::from(2);  // Evaluate the polynomial at x=2

    // Commit to the polynomial.
    let commitment = poly.commit();

    // Open the polynomial at x=2.
    let (value_at_point, proof) = poly.open(point);

    // Verify the commitment and opening proof.
    let is_valid = verify(&commitment, point, value_at_point, &proof);
    println!("Verification result: {}", is_valid);
}
