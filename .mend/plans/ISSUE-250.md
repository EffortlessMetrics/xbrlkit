# Issue #250: Criterion benchmarks for hot paths

## Status

Implementation slice for the benchmark harness and deterministic PR compile gate.
Runtime baseline comparison is tracked separately in issue #407.

## Problem

The repository has no repeatable performance measurements for taxonomy loading,
DTS resolution, or validation orchestration. Changes to these paths therefore
have no local benchmark signal.

## Design

- Add a private `crates/xbrlkit-bench` workspace package with three Criterion
  targets.
- Generate taxonomy fixtures in a unique `TempDir` before measurement. The
  fixtures exercise local file reads, recursive schema imports, definition
  linkbase parsing, and dimension relationships without network access. Any
  fixture setup failure aborts the benchmark instead of silently dropping a
  configured workload size.
- Use synthetic profile and report inputs for DTS and validation measurements so
  benchmark runs are deterministic and do not depend on release data or the
  network.
- Keep owned validation results out of the measured teardown interval so the
  validation timings describe the validation calls rather than result cleanup.
- Compile the benchmark targets in a dedicated CI job on every push and pull
  request. Runtime comparison against a `main` baseline is intentionally a
  follow-up: `main` does not yet contain a benchmark package or baseline
  artifact, so checking out `main` from this first benchmark PR cannot produce a
  valid comparison.

## Acceptance criteria for this slice

- [x] Criterion is declared once in workspace dependencies and used by the
      private benchmark package.
- [x] Taxonomy loading, DTS resolution, and validation pipeline benchmark
      targets exist.
- [x] Inputs are synthetic or local and do not perform network I/O.
- [x] Taxonomy fixture setup is outside measured iterations and validates that
      recursive loading and linkbase relationships succeed.
- [x] Every configured taxonomy workload is required, and fixture failures
      terminate the benchmark with the workload parameters in the diagnostic.
- [x] Validation benchmark results are dropped outside the measured interval.
- [x] Benchmark targets compile in CI with `--locked --no-run`.
- [ ] A stable main-vs-PR runtime baseline comparison exists; issue #407 owns
      this follow-up after the package lands on `main`.

## Proof commands

```text
cargo bench --manifest-path crates/xbrlkit-bench/Cargo.toml --locked --no-run
cargo fmt --all --check
cargo clippy -p xbrlkit-bench --all-targets --locked --offline -- -D warnings
cargo test --workspace --locked --offline
```

## Non-goals

- No runtime validation behavior changes.
- No public crate or API support promise.
- No network-backed or release-scale taxonomy benchmark.
- No automatic performance regression threshold before a stable baseline is
  available.

## Claim boundary

The benchmarks provide repeatable local workload definitions and prove that the
targets build. They do not establish that the current implementation is fast,
that a change is faster, or that a measurement is comparable across different
machines until a baseline policy is added.
