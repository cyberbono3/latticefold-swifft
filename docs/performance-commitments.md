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

Capture the Criterion mean/median/stddev from the console output. A simple table template:

| bench                 | size    | mean time | std dev | notes |
|-----------------------|---------|-----------|---------|-------|
| ajtai_commit_ntt      | N=256   | _fill_    | _fill_  | KAPPA=4 |
| swifft_commit_bytes   | 1024 B  | _fill_    | _fill_  | `--features swifft` |

Include machine details (CPU, OS, Rust toolchain) for reproducibility.
