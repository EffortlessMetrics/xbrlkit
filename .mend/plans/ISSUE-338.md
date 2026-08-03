# Plan: Split dimensional-rules validation responsibilities (Issue #338)

## Problem

`crates/dimensional-rules/src/lib.rs` combines public validation orchestration,
taxonomy lookup, typed-value validation, summaries, and tests in one large
module. The file is currently about 870 lines and is approaching the repository
review and parallel-edit threshold.

## Multi-PR sequence

1. Extract typed-dimension value validators into `typed_values.rs` while
   preserving private visibility and all public APIs. This is the current PR.
2. Extract context and dimension validation orchestration into a dedicated
   module after the first extraction is reviewed.
3. Reconcile finding construction and summary helpers only where a clear,
   tested responsibility boundary remains.
4. Partition tests by responsibility after the production seams stabilize.

## Current slice

Move `validate_typed_dimension_value` and its decimal, integer, date, datetime,
boolean, and URI helpers into `typed_values.rs`. Keep the module private and
call it from the existing public validation path. No validation rule, finding
shape, public symbol, dependency, or scenario contract changes.

## Acceptance criteria

- Existing dimensional validation behavior remains unchanged.
- The typed-value helpers have one focused module and no duplicate
  implementation remains in `lib.rs`.
- Public exports and function signatures remain unchanged.
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
- No public module or API promotion in this first extraction.
- No broad test relocation or unrelated finding-builder refactor.

## Rollback

Revert the extraction commit. The refactor has no persisted state or migration.
