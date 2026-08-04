# Plan: Deduplicate dimension Given-step prefixes (Issue #348)

## Problem

`xbrlkit-bdd-steps` applies the same `DimensionContext` mutation for the
normal and negative dimension/member Given steps, duplicating the parser
branches and making future variants easy to diverge.

## Selected slice

Chain the equivalent prefixes with `or_else` while preserving the existing
captured value, mutation, and `Ok(true)` dispatch behavior.

## Files

- `crates/xbrlkit-bdd-steps/src/lib.rs`
- `.mend/plans/ISSUE-348.md`

## Acceptance criteria

- Normal and negative dimension prefixes still populate the same field.
- Normal and invalid member prefixes still populate the same field.
- No scenario text, metadata, schema, or public API changes are needed.
- The duplicate parser branches are removed without introducing new panic
  surfaces.

## Proof

- `cargo test -p xbrlkit-bdd-steps --locked --offline`
- `cargo clippy -p xbrlkit-bdd-steps --all-targets --locked --offline -- -D warnings`
- Relevant dimension BDD selectors, feature-grid, doctor, formatting, and
  `git diff --check`.

## Non-goals

- No tokenizer or regex-based step parser redesign.
- No changes to the semantic meaning of `unknown` or `invalid` scenarios.
- No changes to the protected root worktree or unrelated BDD handlers.

## Rollback

Revert the focused parser extraction and plan file; no persisted data or
schema migration is involved.
