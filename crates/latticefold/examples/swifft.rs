#![cfg(feature = "swifft")]

//! Minimal example showing how to hash a witness with SWIFFT.

use ark_std::test_rng;
use latticefold::{
    arith::Witness, commitment::SwifftCommitmentScheme,
    decomposition_parameters::test_params::GoldilocksDP,
};
use rand::Rng;

/// Generates a random witness and produces a SWIFFT digest of its serialized CCS form.
fn main() {
    let mut rng = test_rng();
    const WITNESS_LEN: usize = 128;

    // Build a random witness.
    let witness = Witness::<cyclotomic_rings::rings::GoldilocksRingNTT>::rand::<_, GoldilocksDP>(
        &mut rng,
        WITNESS_LEN,
    );

    // Random SWIFFT key and commitment.
    let scheme = SwifftCommitmentScheme::rand(&mut rng);
    let digest = witness
        .swifft_commit(&scheme)
        .expect("SWIFFT commit should succeed");

    println!(
        "SWIFFT digest ({} bytes): {:02x?}",
        digest.as_bytes().len(),
        digest.as_bytes()
    );
}
