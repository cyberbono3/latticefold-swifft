#!/usr/bin/env markdown
# Ajtai vs SWIFFT commitment microbenchmarks

This repo ships lightweight Criterion benches to get a quick feel for commitment throughput:
- `commit_backend`: compares Ajtai `commit_ntt` on a small witness vs. SWIFFT byte hashing (feature-gated).
- `commit_witness`: compares Ajtai witness commit vs. SWIFFT hashing of serialized witness bytes.
- `swifft`: exercises SWIFFT byte commits at a few sizes.
- `swifft_backend`: commits a witness via `CommitmentBackend::Swifft` and absorbs into a transcript.
- `swifft_profile`: breaks SWIFFT into serialize/hash/total to identify bottlenecks.

These benches are deliberately small/fast; they do **not** reflect full protocol workloads.

## Backend notes

- Ajtai commitments remain the homomorphic commitment used in protocol folding.
- SWIFFT is exposed as a binding digest via `CommitmentDigest` and is feature-gated.
- SWIFFT witness hashing is domain-separated (ring type name, decomposition params, witness length).
- Use `CommitmentDigest::absorb_into` (or `Witness::commit_with_backend_and_absorb`) for backend-agnostic transcript absorption.
- Example: `cargo run --example swifft --features swifft` uses `CommitmentBackend::Swifft` and absorbs the digest into a transcript.
- For repeated SWIFFT commits, reuse `SwifftCommitmentBuffer` and `serialize_witness_bytes_into` to avoid per-call allocations.
- Enable `swifft-parallel` to use Rayon-backed compression (`--features swifft-parallel`).

## How to run

Ajtai only:
```bash
BACKEND_BENCH=1 cargo bench --bench commit_backend -- --warm-up-time 1 --measurement-time 3
```

Ajtai + SWIFFT (enable feature):
```bash
BACKEND_BENCH=1 SWIFFT_BENCH=1 cargo bench --features swifft --bench commit_backend -- --warm-up-time 1 --measurement-time 3
```

Ajtai + SWIFFT witness-aligned:
```bash
BACKEND_BENCH=1 SWIFFT_BENCH=1 cargo bench --features swifft --bench commit_witness -- --warm-up-time 1 --measurement-time 3
```

Ajtai + SWIFFT witness-aligned (include large sizes):
```bash
BACKEND_BENCH=1 SWIFFT_BENCH=1 SWIFFT_LARGE=1 cargo bench --features swifft --bench commit_witness -- --warm-up-time 1 --measurement-time 3
```

SWIFFT-only microbench:
```bash
SWIFFT_BENCH=1 cargo bench --features swifft --bench swifft -- --warm-up-time 1 --measurement-time 3
```

SWIFFT profiling (serialize vs hash vs total):
```bash
SWIFFT_BENCH=1 cargo bench --features swifft --bench swifft_profile -- --warm-up-time 1 --measurement-time 3
```

SWIFFT backend + transcript absorption:
```bash
SWIFFT_BENCH=1 cargo bench --features swifft --bench swifft_backend -- --warm-up-time 1 --measurement-time 3
```

Environment flags:
- `BACKEND_BENCH=0` skips `commit_backend`.
- `SWIFFT_BENCH=0` skips SWIFFT benches even when the feature is enabled.
- `SWIFFT_LARGE=1` adds larger witness sizes (4096, 16384) to `commit_witness`.

You can adjust sizes by editing:
- Ajtai: `KAPPA`/`N` in `crates/latticefold/benches/commit_backend.rs`.
- SWIFFT bytes: `BYTES_LEN` in `commit_backend.rs` and the sizes array in `benches/swifft.rs`.
- SWIFFT backend sizes: `SIZES` in `crates/latticefold/benches/swifft_backend.rs`.
- Witness commit sizes: `SIZES` in `crates/latticefold/benches/commit_witness.rs`.
- SWIFFT profiling sizes: `sizes` in `crates/latticefold/benches/swifft_profile.rs`.

## Recording results Ajtai + SWIFFT (enable feature)

Latest runs (Criterion means):

| bench                 | size   | mean time | notes                      |
|-----------------------|--------|-----------|----------------------------|
| ajtai_commit_ntt      | N=64   | 43.7 µs   | KAPPA=4                    |
| ajtai_commit_ntt      | N=256  | 175.0 µs  | KAPPA=4                    |
| ajtai_commit_ntt      | N=1024 | 699.0 µs  | KAPPA=4                    |
| swifft_commit_bytes   | 56 B   | 31.1 µs   | `--features swifft`        |
| swifft_commit_bytes   | 256 B  | 194.5 µs  | `--features swifft`        |
| swifft_commit_bytes   | 1024 B | 843.0 µs  | `--features swifft`        |
| swifft_backend_commit_absorb | N=64   | 10.79 ms | commit + absorb           |
| swifft_backend_commit_absorb | N=256  | 43.03 ms | commit + absorb           |
| swifft_backend_commit_absorb | N=1024 | 170.53 ms | commit + absorb          |
| ajtai_commit_witness  | N=64   | 227.46 µs | witness commit            |
| ajtai_commit_witness  | N=256  | 913.75 µs | witness commit            |
| ajtai_commit_witness  | N=1024 | 3.63 ms  | witness commit            |
| swifft_commit_witness_bytes | N=64   | 10.79 ms | hash witness bytes       |
| swifft_commit_witness_bytes | N=256  | 42.70 ms | hash witness bytes       |
| swifft_commit_witness_bytes | N=1024 | 169.44 ms | hash witness bytes      |




Test machine (for the numbers above):
- OS: `Darwin 25.1.0 arm64`
- CPU: Apple Silicon (T6000-class; brand string blocked by sandbox)
- Rust: `rustc 1.87.0-nightly (f4a216d28 2025-03-02)`

Include machine details (CPU, OS, Rust toolchain) for reproducibility when recording new runs.

## SWIFFT profiling summary

`swifft_profile` shows serialization cost is negligible compared to SWIFFT compression:

- N=64: serialize ~3.49 µs, hash ~10.68 ms, total ~10.68 ms
- N=256: serialize ~13.93 µs, hash ~42.24 ms, total ~42.12 ms
- N=1024: serialize ~56.28 µs, hash ~168.08 ms, total ~167.79 ms

Large sizes (SWIFFT_LARGE=1) continue the same pattern:
- N=4096: serialize ~227 µs, hash ~671.5 ms, total ~672.0 ms
- N=16384: serialize ~0.93 ms, hash ~2.70 s (total not captured before timeout)

On this machine, `swifft-parallel` regressed across tested sizes (e.g., N=1024 hash ~225.7 ms vs ~168.1 ms without parallel).
