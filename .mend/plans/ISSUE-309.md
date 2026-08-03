# Issue 309: Extract duplicated utility functions

## Current state

Issue #309 identifies three duplicated helpers:

- `extract_namespaces` in `taxonomy-loader/src/schema.rs` and `linkbase.rs`;
- `resolve_path` in the same two files;
- `sanitize_for_rule_id` in `numeric-rules` and `efm-rules`.

The first two helpers are byte-for-byte equivalent apart from formatting and
have one clear same-crate owner. The sanitizer is a separate cross-crate
boundary and is intentionally not part of the first implementation slice.

## Selected PR slice

This PR creates `taxonomy-loader/src/util.rs`, moves `extract_namespaces` and
`resolve_path` there, and updates both parser modules to use the shared
implementations. It adds focused tests at the new utility seam while retaining
the existing parser tests as call-site regression coverage.

## Acceptance criteria

- AC-309-001: `extract_namespaces` has one implementation in
  `taxonomy-loader` and both schema and linkbase parsing use it.
- AC-309-002: `resolve_path` has one implementation in `taxonomy-loader` and
  both import and linkbase-reference extraction use it.
- AC-309-003: The shared helpers preserve current behavior for namespace
  collection, HTTP URLs, empty base directories, and relative paths.
- AC-309-004: Focused utility tests and the affected crate test suite pass.
- AC-309-005: No cross-crate dependency or public API is introduced by this
  slice.

## Proof

```text
cargo test -p taxonomy-loader --locked --offline
cargo clippy -p taxonomy-loader --all-targets --locked --offline -- -D warnings
cargo fmt --all -- --check
git diff --check
cargo xtask doctor
cargo xtask impact --changed crates/taxonomy-loader/src/util.rs
cargo xtask impact --changed crates/taxonomy-loader/src/schema.rs
cargo xtask impact --changed crates/taxonomy-loader/src/linkbase.rs
```

## Follow-up

The `sanitize_for_rule_id` duplication remains a separate candidate slice.
Before implementing it, recheck current dependency topology and decide whether
an existing lightweight crate is an appropriate host; do not widen this PR's
same-crate parser refactor.

## Non-goals and rollback

This slice does not change parsing behavior, path normalization, dependency
topology, public exports, or the cross-crate rule-ID utility. Rollback is the
single commit that adds `util.rs`, updates the two imports, and adds this plan.
