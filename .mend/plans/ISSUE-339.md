# Plan: Reconcile unreferenced workspace crates (Issue #339)

## Problem

The workspace contains small crates that compile in isolation but are not
consumed by another workspace package. The prior issue list was stale because
`diff-run` is now consumed by `xbrlkit-core`, while sixteen other workspace
packages remain unreferenced. The set includes intentional executable and
test/support roots as well as minimal library boundaries, so zero reverse
dependencies alone is not a deletion recommendation.

## Current slice

Add a durable crate status map and link it from the README. Record only facts
confirmed by the current package graph and source, and leave deletion and
ownership decisions to explicit follow-up issues.

## Acceptance criteria

- The map names the sixteen current zero-reverse-dependency packages.
- The map records `diff-run` as consumed by `xbrlkit-core`.
- Status language does not imply approval to delete or a claim about external
  consumers.
- The README links to the map from the workspace-shape section.
- The plan and map identify the next required owner/integration decision.

## Proof

- `cargo metadata --format-version 1 --locked --offline`
- `cargo check --workspace --locked --offline`
- `cargo xtask doctor`
- Verify the README and plan links from the repository root.

## Non-goals

- No crate deletion or package rename.
- No dependency-graph changes.
- No ownership or roadmap assignment without a maintainer decision.

## Rollback

Revert the documentation commit. The inventory has no runtime or persisted-data
effect.
