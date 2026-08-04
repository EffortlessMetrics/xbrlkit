# Plan: Split dimensional-rules validation responsibilities (Issue #338)

## Problem

`crates/dimensional-rules/src/lib.rs` combines public validation orchestration,
taxonomy lookup, typed-value validation, summaries, and tests in one large
module. The file is currently about 870 lines and is approaching the repository
review and parallel-edit threshold.

## Multi-PR sequence

1. Extract typed-dimension value validators into `typed_values.rs` while
   preserving private visibility and all public APIs. Completed by [PR #395](https://github.com/EffortlessMetrics/xbrlkit/pull/395).
2. Extract the result model and summary aggregation into `result.rs`, while
   preserving the crate-root public API. This is the current stacked slice.
3. Extract context and dimension validation orchestration into a dedicated
   module after the production result seam is reviewed.
4. Reconcile finding construction only where a clear, tested responsibility
   boundary remains.
5. Partition tests by responsibility after the production seams stabilize.

## Completed prior slice

Move `validate_typed_dimension_value` and its decimal, integer, date, datetime,
boolean, and URI helpers into `typed_values.rs`. Keep the module private and
call it from the existing public validation path. No validation rule, finding
shape, public symbol, dependency, or scenario contract changes.

## Current slice

Move `DimensionalValidationResult`, `DimensionalSummary`, `collect_findings`,
and `summarize_results` into the private `result.rs` module. Re-export those
items from `lib.rs` so callers continue to resolve the same crate-root public
symbols. Keep the existing test location and validation orchestration in this
slice; test partitioning and orchestration extraction remain separate.

## Acceptance criteria

- Existing dimensional validation behavior remains unchanged.
- The typed-value helpers and result/summary responsibilities each have one
  focused private module with no duplicate implementation in `lib.rs`.
- `DimensionalValidationResult`, `DimensionalSummary`, `collect_findings`, and
  `summarize_results` remain available at the crate root with unchanged public
  names, signatures, and field visibility.
- Package tests, Clippy, formatting, and workspace compilation pass.

## Proof

- `cargo test -p dimensional-rules --locked --offline`
- `cargo clippy -p dimensional-rules --all-targets --locked --offline -- -D warnings`
- `cargo check --workspace --locked --offline`
- `cargo fmt --all --check`
- `git diff --check`
- `cargo xtask doctor`
- `cargo xtask impact --changed crates/dimensional-rules/src/lib.rs`

## Non-goals

- No validation behavior changes or new dimensional rules.
- No public module promotion or public API redesign.
- No context/dimension orchestration extraction, broad test relocation, or
  finding-construction refactor.

## Rollback

Revert the extraction commit. The refactor has no persisted state or migration.
