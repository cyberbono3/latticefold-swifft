#!/usr/bin/env markdown
# Ajtai vs SWIFFT commitment microbenchmarks

This repo ships lightweight Criterion benches to get a quick feel for commitment throughput:
- `commit_backend`: compares Ajtai `commit_ntt` on a small witness vs. SWIFFT byte hashing (feature-gated).
- `swifft`: exercises SWIFFT byte commits at a few sizes.

These benches are deliberately small/fast; they do **not** reflect full protocol workloads.

## How to run

Ajtai only:
```bash
BACKEND_BENCH=1 cargo bench --bench commit_backend -- --warm-up-time 1 --measurement-time 3
```

Ajtai + SWIFFT (enable feature):
```bash
BACKEND_BENCH=1 SWIFFT_BENCH=1 cargo bench --features swifft --bench commit_backend -- --warm-up-time 1 --measurement-time 3
```

SWIFFT-only microbench:
```bash
SWIFFT_BENCH=1 cargo bench --features swifft --bench swifft -- --warm-up-time 1 --measurement-time 3
```

Environment flags:
- `BACKEND_BENCH=0` skips `commit_backend`.
- `SWIFFT_BENCH=0` skips SWIFFT benches even when the feature is enabled.

You can adjust sizes by editing:
- Ajtai: `KAPPA`/`N` in `crates/latticefold/benches/commit_backend.rs`.
- SWIFFT bytes: `BYTES_LEN` in `commit_backend.rs` and the sizes array in `benches/swifft.rs`.

## Recording results

Latest runs (Criterion means) for quick reference:

| bench                 | size   | mean time | notes                      |
|-----------------------|--------|-----------|----------------------------|
| ajtai_commit_ntt      | N=64   | 43.7 µs   | KAPPA=4                    |
| ajtai_commit_ntt      | N=256  | 175.0 µs  | KAPPA=4                    |
| ajtai_commit_ntt      | N=1024 | 699.0 µs  | KAPPA=4                    |
| swifft_commit_bytes   | 56 B   | 31.1 µs   | `--features swifft`        |
| swifft_commit_bytes   | 256 B  | 194.5 µs  | `--features swifft`        |
| swifft_commit_bytes   | 1024 B | 843.0 µs  | `--features swifft`        |

Replace or append with your own runs; capture mean/median/stddev from Criterion output.


Test machine (for the numbers above):
- OS: `Darwin 25.1.0 arm64`
- CPU: 
- Rust: `rustc 1.87.0-nightly (f4a216d28 2025-03-02)`

Include machine details (CPU, OS, Rust toolchain) for reproducibility when recording new runs.
