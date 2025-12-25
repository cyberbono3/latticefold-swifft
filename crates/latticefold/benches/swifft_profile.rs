#![cfg(feature = "swifft")]

use std::{env, time::Duration};

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use latticefold::{
    arith::Witness,
    commitment::{SwifftCommitmentBuffer, SwifftCommitmentScheme},
    decomposition_parameters::DecompositionParams,
};

#[derive(Clone)]
struct BenchDP;

impl DecompositionParams for BenchDP {
    const B: u128 = 1 << 15;
    const L: usize = 5;
    const B_SMALL: usize = 2;
    const K: usize = 15;
}

fn bench_swifft_profile(c: &mut Criterion) {
    if let Ok(flag) = env::var("SWIFFT_BENCH") {
        if flag == "0" {
            return;
        }
    }

    let mut rng = ark_std::test_rng();
    let mut swifft_rng = rand::thread_rng();
    let mut sizes = vec![64usize, 256, 1024];
    if let Ok(flag) = env::var("SWIFFT_LARGE") {
        if flag == "1" {
            sizes.extend([4096, 16384]);
        }
    }

    for &n in &sizes {
        let witness = Witness::<cyclotomic_rings::rings::GoldilocksRingNTT>::rand::<_, BenchDP>(
            &mut rng,
            n,
        );
        let scheme = SwifftCommitmentScheme::rand(&mut swifft_rng);

        c.bench_with_input(
            BenchmarkId::new("swifft_serialize_witness", n),
            &n,
            |b, _| {
                let mut buf = Vec::new();
                b.iter(|| {
                    witness
                        .serialize_witness_bytes_into::<BenchDP>(&mut buf)
                        .expect("serialize witness should succeed");
                });
            },
        );

        let bytes = witness
            .serialize_witness_bytes::<BenchDP>()
            .expect("serialize witness should succeed");
        c.bench_with_input(
            BenchmarkId::new("swifft_commit_bytes_only", n),
            &n,
            |b, _| {
                let mut buffer = SwifftCommitmentBuffer::default();
                b.iter(|| {
                    let _ = scheme.commit_bytes_with_buffer(&bytes, &mut buffer);
                });
            },
        );

        c.bench_with_input(
            BenchmarkId::new("swifft_commit_total", n),
            &n,
            |b, _| {
                let mut buf = Vec::new();
                let mut buffer = SwifftCommitmentBuffer::default();
                b.iter(|| {
                    let _ = witness
                        .swifft_commit_with_buffer::<BenchDP>(&scheme, &mut buf, &mut buffer)
                        .expect("commit should succeed");
                });
            },
        );
    }
}

criterion_group! {
    name = benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(5))
        .sample_size(20);
    targets = bench_swifft_profile
}
criterion_main!(benches);
