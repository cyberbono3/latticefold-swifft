use std::env;

use ark_std::UniformRand;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use latticefold::commitment::AjtaiCommitmentScheme;
#[cfg(feature = "swifft")]
use latticefold::commitment::SwifftCommitmentScheme;
#[cfg(feature = "swifft")]
use rand::RngCore;

fn should_run() -> bool {
    match env::var("BACKEND_BENCH").or_else(|_| env::var("BACKEND")) {
        Ok(flag) if flag == "0" => false,
        _ => true,
    }
}

fn bench_ajtai(c: &mut Criterion) {
    if !should_run() {
        return;
    }

    let mut rng = ark_std::test_rng();
    const KAPPA: usize = 4;
    const N: usize = 256;
    let witness: Vec<_> = (0..N)
        .map(|_| cyclotomic_rings::rings::GoldilocksRingNTT::rand(&mut rng))
        .collect();
    let scheme = AjtaiCommitmentScheme::rand(KAPPA, N, &mut rng);

    c.bench_with_input(BenchmarkId::new("ajtai_commit_ntt", N), &N, |b, _| {
        b.iter(|| scheme.commit_ntt(&witness).unwrap());
    });
}

#[cfg(feature = "swifft")]
fn bench_swifft(c: &mut Criterion) {
    if !should_run() {
        return;
    }
    if let Ok(flag) = env::var("SWIFFT_BENCH") {
        if flag == "0" {
            return;
        }
    }

    let mut rng = rand::thread_rng();
    const BYTES_LEN: usize = 1024;
    let mut data = vec![0u8; BYTES_LEN];
    rng.fill_bytes(&mut data);
    let scheme = SwifftCommitmentScheme::rand(&mut rng);

    c.bench_with_input(
        BenchmarkId::new("swifft_commit_bytes", BYTES_LEN),
        &BYTES_LEN,
        |b, _| {
            b.iter(|| scheme.commit_bytes(&data));
        },
    );
}

#[cfg(not(feature = "swifft"))]
criterion_group!(benches, bench_ajtai);

#[cfg(feature = "swifft")]
criterion_group!(benches, bench_ajtai, bench_swifft);

criterion_main!(benches);
