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
   preserving the crate-root public API. Completed by [PR #422](https://github.com/EffortlessMetrics/xbrlkit/pull/422), stacked on PR #395.
3. Extract context and dimension validation orchestration into `validate.rs`,
   while preserving the crate-root public API. Completed by [PR #424](https://github.com/EffortlessMetrics/xbrlkit/pull/424), stacked on PR #422.
4. Reconcile finding construction only where a clear, tested responsibility
   boundary remains.
5. Partition tests by responsibility after the production seams stabilize.
   Completed by [PR #425](https://github.com/EffortlessMetrics/xbrlkit/pull/425).
6. Extract the repeated finding-construction shapes into a private helper
   module, keeping rule IDs, messages, and public APIs unchanged. This is the
   current stacked slice.

## Completed prior slice

Move `validate_typed_dimension_value` and its decimal, integer, date, datetime,
boolean, and URI helpers into `typed_values.rs`. Keep the module private and
call it from the existing public validation path. No validation rule, finding
shape, public symbol, dependency, or scenario contract changes.

## Completed validation slice

Move `validate_context_dimensions`, its private `validate_dimension_member`
helper, and `is_descendant_member` into the private `validate.rs` module.
Re-export the two public functions from `lib.rs` so callers continue to
resolve the same crate-root public symbols. Keep report-level
`validate_fact_dimensions`, result aggregation, finding construction, and the
existing test location outside this slice.

## Completed test-partition slice

Move the existing `#[cfg(test)]` unit-test module from `lib.rs` into the
private `tests.rs` module. Preserve the test names, fixtures, assertions, and
production visibility. This is a layout-only change; finding construction and
new test coverage remain separate. Completed by PR #425.

## Current finding-construction slice

Move the five repeated `ValidationFinding` construction shapes used by the
crate-root orchestration and context/member validation into a private
`findings.rs` module. Keep the existing fields, rule IDs, messages, and
call-site behavior unchanged; callers delegate to named helpers only.

## Acceptance criteria

- Existing dimensional validation behavior remains unchanged.
- The typed-value, result/summary, context-validation, and test responsibilities
  each have one focused module with no duplicate production implementation in
  `lib.rs`.
- `validate_context_dimensions` and `is_descendant_member` remain available at
  the crate root with unchanged public names and signatures.
- All existing dimensional-rules unit tests continue to execute with their
  existing names and assertions.
- Finding construction has one private implementation location for missing
  context, missing required dimension, unknown dimension, invalid member, and
  missing domain findings.
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
- No report-level orchestration, result aggregation, or new dimensional rules.
- No additional test partitioning or new finding shapes beyond the five
  existing construction paths.

## Rollback

Revert the extraction commit. The refactor has no persisted state or migration.
