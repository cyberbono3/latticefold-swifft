//! Minimal example showing how to hash a witness with SWIFFT.

#[cfg(feature = "swifft")]
use ark_std::test_rng;
#[cfg(feature = "swifft")]
use latticefold::{
    arith::Witness, commitment::SwifftCommitmentScheme,
    decomposition_parameters::DecompositionParams,
};

#[cfg(feature = "swifft")]
#[derive(Clone)]
struct ExampleDP;

#[cfg(feature = "swifft")]
impl DecompositionParams for ExampleDP {
    const B: u128 = 1 << 15;
    const L: usize = 5;
    const B_SMALL: usize = 2;
    const K: usize = 15;
}

/// Generates a random witness and produces a SWIFFT digest of its serialized CCS form.
#[cfg(feature = "swifft")]
fn main() {
    let mut rng = test_rng();
    const WITNESS_LEN: usize = 128;

    // Build a random witness.
    let witness = Witness::<cyclotomic_rings::rings::GoldilocksRingNTT>::rand::<_, ExampleDP>(
        &mut rng,
        WITNESS_LEN,
    );

    // Random SWIFFT key and commitment.
    let scheme = SwifftCommitmentScheme::rand(&mut rng);
    let digest = witness
        .swifft_commit::<ExampleDP>(&scheme)
        .expect("SWIFFT commit should succeed");

    println!(
        "SWIFFT digest ({} bytes): {:02x?}",
        digest.as_bytes().len(),
        digest.as_bytes()
    );
}

#[cfg(not(feature = "swifft"))]
fn main() {
    eprintln!("Enable the `swifft` feature to run this example.");
}
