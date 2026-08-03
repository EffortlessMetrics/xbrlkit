# Issue #407: Artifact-backed benchmark baseline comparison

## Status

Stacked follow-up to PR #408. This branch must land after the benchmark
package is present on `main`.

## Problem

The benchmark harness can run locally, but a pull request cannot compare its
measurements with `main` until a baseline is produced by a workflow run on
`main`. Checking out `main` inside the first benchmark PR is invalid because
that branch does not yet contain the benchmark package.

## Design

- Run the baseline job only on `main` pushes and pull requests.
- On `main`, run the same benchmark targets with a short, fixed Criterion
  configuration and save `target/criterion` under a cache key containing the
  lockfile hash and run id.
- On pull requests, restore only a baseline with the same lockfile hash using
  the default-branch cache scope. Do not save PR output as a baseline.
- If no matching baseline exists, emit an explicit warning and report the
  comparison as unavailable while leaving the job advisory.
- Upload `target/criterion` from every baseline-job run as a reviewable artifact
  with a bounded retention period.

## Acceptance criteria

- [x] A named `main` Criterion baseline is generated on pushes to `main`.
- [x] Pull requests restore a matching baseline through an explicit cache-key
      and lockfile-hash contract.
- [x] Pull requests run the same benchmark targets against the restored
      baseline and upload the Criterion output.
- [x] Missing baselines are reported as unavailable, not as a passing
      comparison; the job remains advisory.
- [x] The workflow never checks out `main` inside a pull-request job.
- [ ] Hosted proof after #408 lands: verify a real `main` baseline cache and a
      pull-request comparison artifact on GitHub Actions.

## Proof commands

```text
cargo bench --manifest-path crates/xbrlkit-bench/Cargo.toml --locked --offline --bench taxonomy_loader -- --save-baseline main --noplot --sample-size 10 --warm-up-time 1 --measurement-time 0.1
cargo bench --manifest-path crates/xbrlkit-bench/Cargo.toml --locked --offline --bench taxonomy_loader -- --baseline main --noplot --sample-size 10 --warm-up-time 1 --measurement-time 0.1
cargo fmt --all --check
```

Hosted proof must additionally verify the cache key, artifact name, commit
context, and explicit unavailable-baseline warning path.

## Non-goals

- No performance threshold or merge blocker.
- No network-backed benchmarks.
- No production behavior or public API changes.
- No baseline restore when the lockfile hash differs.

## Claim boundary

This change provides repeatable CI comparison mechanics and reviewable output.
It does not establish that a benchmark regression is meaningful across runner
types, nor does it define an acceptable performance threshold.
