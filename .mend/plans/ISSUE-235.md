# Plan: Remove production JSON serialization unwraps from the CLI (Issue #235)

## Problem

The `inspect-contexts --json` and `inspect-taxonomy --json` command paths use
`unwrap()` around JSON serialization. A serialization failure would abort the
CLI instead of returning the existing `anyhow::Result` error path.

## Selected seam

- `crates/xbrlkit-cli/src/main.rs`
- `InspectContexts` JSON output
- `InspectTaxonomy` JSON output

## Acceptance criteria

- AC-235-001: `InspectContexts` propagates JSON serialization errors with
  operation context.
- AC-235-002: `InspectTaxonomy` propagates JSON serialization errors with
  operation context.
- AC-235-003: these CLI JSON paths contain no production `unwrap()` calls.
- AC-235-004: successful human-readable and JSON command behavior is unchanged.

## Proof

- `cargo test -p xbrlkit-cli --locked --offline`
- `cargo clippy -p xbrlkit-cli --all-targets --locked --offline -- -D warnings`
- `cargo fmt --all --check`
- `git diff --check`
- `cargo run -p xbrlkit-cli --locked --offline -- --help`
- targeted source audit for `unwrap()` in `crates/xbrlkit-cli/src/main.rs`

## Non-goals

- Do not change the separate `workspace_root()` `expect`; that belongs to a
  different issue and is outside the two JSON serialization call sites.
- Do not redesign CLI output or add new dependencies.
- Do not add a scenario solely for this behavior-preserving error-plumbing
  change; the existing CLI contract and targeted compile/test proof cover it.

## Risk and rollback

The change only replaces panic-shaped error handling with the already-used
`anyhow::Context` pattern in `main()`. Rollback is a single commit revert if
CLI output or exit behavior changes unexpectedly.
