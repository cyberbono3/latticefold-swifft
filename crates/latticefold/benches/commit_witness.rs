use std::env;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use latticefold::{
    arith::Witness,
    commitment::AjtaiCommitmentScheme,
    decomposition_parameters::DecompositionParams,
};

#[cfg(feature = "swifft")]
use latticefold::commitment::{SwifftCommitmentBuffer, SwifftCommitmentScheme};

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

fn bench_ajtai_witness_commit(c: &mut Criterion) {
    if !should_run() {
        return;
    }

    let mut rng = ark_std::test_rng();
    const KAPPA: usize = 4;
    const SIZES: &[usize] = &[64, 256, 1024];

    for &n in SIZES {
        let witness = Witness::<cyclotomic_rings::rings::GoldilocksRingNTT>::rand::<_, BenchDP>(
            &mut rng,
            n,
        );
        let scheme = AjtaiCommitmentScheme::rand(KAPPA, witness.f.len(), &mut rng);

        c.bench_with_input(BenchmarkId::new("ajtai_commit_witness", n), &n, |b, _| {
            b.iter(|| witness.commit::<BenchDP>(&scheme).expect("commit should succeed"));
        });
    }
}

#[cfg(feature = "swifft")]
fn bench_swifft_witness_bytes(c: &mut Criterion) {
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
        let bytes = witness
            .serialize_witness_bytes::<BenchDP>()
            .expect("serialize witness should succeed");
        let scheme = SwifftCommitmentScheme::rand(&mut swifft_rng);
        let mut buffer = SwifftCommitmentBuffer::default();

        c.bench_with_input(BenchmarkId::new("swifft_commit_witness_bytes", n), &n, |b, _| {
            b.iter(|| scheme.commit_bytes_with_buffer(&bytes, &mut buffer));
        });
    }
}

#[cfg(not(feature = "swifft"))]
criterion_group!(benches, bench_ajtai_witness_commit);

#[cfg(feature = "swifft")]
criterion_group!(benches, bench_ajtai_witness_commit, bench_swifft_witness_bytes);

criterion_main!(benches);
