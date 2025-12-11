# Ajtai vs SWIFFT commitment performance

This note explains how to benchmark the existing Ajtai commitment path against the experimental SWIFFT hashing path.

## What is measured
- `commit_backend` bench: calls `commit_with_backend` on a small Goldilocks witness (default `w_len = 256`). Ajtai uses the legacy matrix multiply; SWIFFT hashes the serialized witness bytes.
- `swifft` bench: directly exercises SWIFFT byte commits at several input sizes (56, 256, 1024, 4096 bytes).

## How to run
Run from the repo root.

Ajtai only:
```bash
BACKEND_BENCH=1 cargo bench --bench commit_backend -- --warm-up 1 --measurement-time 10
```

SWIFFT (enable feature):
```bash
BACKEND_BENCH=1 SWIFFT_BENCH=1 cargo bench --features swifft --bench commit_backend --bench swifft -- --warm-up 1 --measurement-time 10
```

Notes:
- Set `BACKEND_BENCH=0` or `SWIFFT_BENCH=0` to skip either bench.
- Increase `--measurement-time` for more stable numbers; `--warm-up` can be raised if the first iterations are noisy.
- To benchmark different witness sizes, tweak `w_len` in `crates/latticefold/benches/commit_backend.rs`.

## Reporting template
Record the Criterion mean/median for each backend. Example table (fill with your run):

| backend  | witness_len | params          | mean time | notes                  |
|----------|-------------|-----------------|-----------|------------------------|
| Ajtai    | 256         | kappa=4         | _fill_    | default bench profile  |
| SWIFFT   | 256         | key_len=1024    | _fill_    | `--features swifft`    |
| SWIFFT   | 1024 bytes  | direct commit   | _fill_    | from `swifft` bench    |

Include the Criterion output snippet (mean, std dev) alongside the table for reproducibility, and note CPU/machine details.
