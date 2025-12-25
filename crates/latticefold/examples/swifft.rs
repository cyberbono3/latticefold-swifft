//! Minimal example showing how to hash a witness with SWIFFT.

#[cfg(feature = "swifft")]
use ark_std::test_rng;
#[cfg(feature = "swifft")]
use latticefold::{
    arith::Witness,
    commitment::{CommitmentBackend, SwifftCommitmentScheme},
    decomposition_parameters::DecompositionParams,
    transcript::{poseidon::PoseidonTranscript, Transcript},
};
#[cfg(feature = "swifft")]
use cyclotomic_rings::rings::{GoldilocksChallengeSet, GoldilocksRingNTT};

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
    let witness = Witness::<GoldilocksRingNTT>::rand::<_, ExampleDP>(&mut rng, WITNESS_LEN);

    // Random SWIFFT key and commitment backend.
    let scheme = SwifftCommitmentScheme::rand(&mut rng);
    let mut transcript =
        PoseidonTranscript::<GoldilocksRingNTT, GoldilocksChallengeSet>::default();
    let backend = CommitmentBackend::Swifft(&scheme);
    let digest = witness
        .commit_with_backend_and_absorb::<ExampleDP>(backend, &mut transcript)
        .expect("SWIFFT commit should succeed");
    let challenge = transcript.get_challenge();
    let digest_bytes = digest.to_bytes().expect("digest bytes should be available");

    println!(
        "SWIFFT digest ({} bytes): {:02x?}",
        digest_bytes.len(),
        digest_bytes
    );
    println!("Transcript challenge: {challenge:?}");
}

#[cfg(not(feature = "swifft"))]
fn main() {
    eprintln!("Enable the `swifft` feature to run this example.");
}
