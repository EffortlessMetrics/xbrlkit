# Plan: Make test setup failures descriptive and fallible (Issue #209)

## Problem

Twenty-six test setup and lookup paths in `xbrl-contexts` and
`taxonomy-loader` use `unwrap()`. When a fixture stops parsing or an expected
context/taxonomy node is missing, the failure gives no operation-specific
diagnostic and still uses panic-shaped test plumbing.

## Selected seam

- `crates/xbrl-contexts/src/lib.rs` test module
- `crates/taxonomy-loader/src/schema.rs` test module
- `crates/taxonomy-loader/src/linkbase.rs` test module

## Acceptance criteria

- Replace all 26 scoped test `unwrap()` calls with fallible propagation or
  contextual `Option` errors.
- Keep the existing test assertions and behavior coverage unchanged.
- Include the failed operation, expected identifier, or fixture role in each
  propagated diagnostic.
- Leave production behavior and unrelated test modules unchanged.

## Proof

- `cargo test -p xbrl-contexts -p taxonomy-loader --locked --offline`
- `cargo clippy -p xbrl-contexts -p taxonomy-loader --all-targets --locked --offline -- -D warnings`
- `cargo fmt --all --check`
- `git diff --check`
- Scoped source audit confirms no `unwrap()` remains in the three test modules.

## Policy alignment

The issue originally proposed replacing `unwrap()` with `expect()`. The current
repository Rust policy requires fallible test setup by default, so this plan
uses `Result<(), String>` and contextual `?` propagation instead of retaining
panic-shaped tests.

## Non-goals and rollback

- Do not redesign production parsing APIs or add new fixtures.
- Do not convert unrelated assertion macros or test modules in this PR.
- Revert the single commit if diagnostics or test behavior regress.
