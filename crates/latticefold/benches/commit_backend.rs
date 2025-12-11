use std::env;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use latticefold::{
    arith::Witness,
    commitment::{AjtaiCommitmentScheme, CommitmentBackend},
    decomposition_parameters::test_params::GoldilocksDP,
};

fn should_run_backend_bench() -> bool {
    match env::var("BACKEND_BENCH").or_else(|_| env::var("BACKEND")) {
        Ok(flag) if flag == "0" => false,
        _ => true,
    }
}

fn bench_ajtai_backend(c: &mut Criterion) {
    if !should_run_backend_bench() {
        return;
    }

    let mut rng = ark_std::test_rng();
    let w_len = 1 << 8;
    let witness = Witness::<cyclotomic_rings::rings::GoldilocksRingNTT>::rand::<_, GoldilocksDP>(
        &mut rng, w_len,
    );
    let scheme = AjtaiCommitmentScheme::rand(4, witness.f.len(), &mut rng);
    let backend = CommitmentBackend::Ajtai(&scheme);

    c.bench_with_input(
        BenchmarkId::new("backend_commit", "ajtai"),
        &w_len,
        |b, _| {
            b.iter(|| {
                let _ = witness
                    .commit_with_backend::<GoldilocksDP>(backend)
                    .unwrap();
            });
        },
    );
}

#[cfg(feature = "swifft")]
fn bench_swifft_backend(c: &mut Criterion) {
    if !should_run_backend_bench() {
        return;
    }
    if let Ok(flag) = std::env::var("SWIFFT_BENCH") {
        if flag == "0" {
            return;
        }
    }

    let mut rng = ark_std::test_rng();
    let w_len = 1 << 8;
    let witness = Witness::<cyclotomic_rings::rings::GoldilocksRingNTT>::rand::<_, GoldilocksDP>(
        &mut rng, w_len,
    );
    let scheme = latticefold::commitment::SwifftCommitmentScheme::rand(&mut rng);
    let backend = CommitmentBackend::Swifft(&scheme);

    c.bench_with_input(
        BenchmarkId::new("backend_commit", "swifft"),
        &w_len,
        |b, _| {
            b.iter(|| {
                let _ = witness
                    .commit_with_backend::<GoldilocksDP>(backend)
                    .unwrap();
            });
        },
    );
}

#[cfg(not(feature = "swifft"))]
criterion_group!(benches, bench_ajtai_backend);

#[cfg(feature = "swifft")]
criterion_group!(benches, bench_ajtai_backend, bench_swifft_backend);

criterion_main!(benches);
