#![cfg(feature = "swifft")]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use latticefold::commitment::SwifftCommitmentScheme;
use rand::RngCore;

fn bench_swifft_commits(c: &mut Criterion) {
    if let Ok(flag) = std::env::var("SWIFFT_BENCH") {
        if flag == "0" {
            return;
        }
    }

    let mut rng = rand::thread_rng();
    let scheme = SwifftCommitmentScheme::rand(&mut rng);

    for size in [56usize, 256, 1024, 4096] {
        c.bench_with_input(BenchmarkId::new("commit_bytes", size), &size, |b, &size| {
            let mut data = vec![0u8; size];
            rng.fill_bytes(&mut data);
            b.iter(|| {
                let _ = scheme.commit_bytes(&data);
            });
        });
    }
}

criterion_group!(benches, bench_swifft_commits);
criterion_main!(benches);
