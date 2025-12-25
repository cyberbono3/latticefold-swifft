#![cfg(feature = "swifft")]

use std::env;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use latticefold::{
    arith::Witness,
    commitment::{CommitmentBackend, SwifftCommitmentScheme},
    decomposition_parameters::DecompositionParams,
    transcript::poseidon::PoseidonTranscript,
};

#[derive(Clone)]
struct BenchDP;

impl DecompositionParams for BenchDP {
    const B: u128 = 1 << 15;
    const L: usize = 5;
    const B_SMALL: usize = 2;
    const K: usize = 15;
}

fn should_run() -> bool {
    match env::var("BACKEND_BENCH").or_else(|_| env::var("BACKEND")) {
        Ok(flag) if flag == "0" => false,
        _ => true,
    }
}

fn bench_swifft_backend(c: &mut Criterion) {
    if !should_run() {
        return;
    }
    if let Ok(flag) = env::var("SWIFFT_BENCH") {
        if flag == "0" {
            return;
        }
    }

    let mut rng = ark_std::test_rng();
    let mut swifft_rng = rand::thread_rng();
    const SIZES: &[usize] = &[64, 256, 1024];

    for &n in SIZES {
        let witness = Witness::<cyclotomic_rings::rings::GoldilocksRingNTT>::rand::<_, BenchDP>(
            &mut rng,
            n,
        );
        let scheme = SwifftCommitmentScheme::rand(&mut swifft_rng);

        c.bench_with_input(
            BenchmarkId::new("swifft_backend_commit_absorb", n),
            &n,
            |b, _| {
                b.iter(|| {
                    let mut transcript = PoseidonTranscript::<
                        cyclotomic_rings::rings::GoldilocksRingNTT,
                        cyclotomic_rings::rings::GoldilocksChallengeSet,
                    >::default();
                    let backend = CommitmentBackend::Swifft(&scheme);
                    let digest = witness
                        .commit_with_backend_and_absorb::<BenchDP>(backend, &mut transcript)
                        .expect("commit should succeed");
                    let _ = digest.to_bytes().expect("digest bytes should be available");
                });
            },
        );
    }
}

criterion_group!(benches, bench_swifft_backend);
criterion_main!(benches);
