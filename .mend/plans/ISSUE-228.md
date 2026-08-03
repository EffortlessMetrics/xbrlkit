# Plan: Share the XBRL Period Model (Issue #228)

## Problem

`xbrl-contexts` and `xbrl-stream` define structurally overlapping period
enums. The duplicate types prevent callers from passing a parsed period
between the two ingestion surfaces without a conversion or a type-specific
branch.

## Selected slice

Make `xbrl-report-types` the owner of the shared `Period` model and preserve
the existing crate-local names as public re-exports:

- `xbrl_contexts::Period` remains available for context parsing callers.
- `xbrl_stream::StreamingPeriod` remains available for streaming callers.
- Both names refer to the same `xbrl_report_types::Period` type.
- `Forever` remains the default for compatibility with `xbrl-contexts`.
- `Unknown` is retained for streaming's incomplete/invalid-period state and
  becomes an available shared variant; context parsing does not emit it.
- The Serde JSON representation is recorded in the versioned
  `contracts/schemas/period.v1.json` contract.
- Scenario `AC-XK-PERIOD-001` passes a period through both public aliases.

## Acceptance criteria

- Only one `Period` enum definition exists in the three affected crates.
- Existing context and streaming parsing behavior remains unchanged.
- Existing public names remain available; exhaustive matches must handle the
  newly shared variants described in the migration note below.
- The shared type retains debug/equality and serde behavior, with `Forever` as
  its default.
- A focused test proves the shared model's default contract.
- A focused serialization test covers all four versioned wire variants.
- A deterministic BDD scenario proves the public context and streaming aliases
  accept the same period value.
- The scenario runner accepts fixture-free BDD contracts while continuing to
  reject fixture-free execution contracts that declare runner-owned receipts.
- A plan artifact records the scope, proof, non-goals, and rollback path.

## Proof

Run from the isolated issue worktree with a task-specific target directory:

```text
cargo test -p xbrl-report-types -p xbrl-contexts -p xbrl-stream -p xbrlkit-bdd-steps --locked --offline
cargo clippy -p xbrl-report-types -p xbrl-contexts -p xbrl-stream -p xbrlkit-bdd-steps --all-targets --locked --offline -- -D warnings
cargo fmt --all --check
cargo xtask test-ac AC-XK-PERIOD-001
cargo xtask bdd --tags @SCN-XK-PERIOD-001
cargo xtask feature-grid
cargo xtask schema-check
cargo test --workspace --locked --offline
cargo check --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo xtask alpha-check
```

The proof establishes package and workspace tests, lint cleanliness, formatting,
the fixture-free acceptance path, the tagged BDD alias scenario, and the active
alpha gate. It does not establish a hosted CI result.

## Compatibility and migration

This is an alpha API compatibility break for exhaustive matches. The public
aliases remain available, but sharing the two former enums requires both
aliases to expose the union of their variants:

- Context callers matching `xbrl_contexts::Period` must add an
  `xbrl_contexts::Period::Unknown` arm. Context parsing itself still emits only
  `Instant`, `Duration`, or `Forever`.
- Streaming callers matching `xbrl_stream::StreamingPeriod` must add a
  `xbrl_stream::StreamingPeriod::Forever` arm. Streaming parsing continues to
  use `Unknown` when the period cannot be determined.

The in-repository CLI match is updated in `crates/xbrlkit-cli/src/main.rs`.
Downstream consumers should add the corresponding arm before upgrading to
this shared model.

## Non-goals

- No parser validation, date normalization, or semantic change.
- No migration of unrelated context or streaming structs.
- No new conversion API or public crate support promise.
- No change to the existing Serde representation of the moved variants.
- No merge or release action.

## Rollback

Revert the bounded source, manifest, test, and plan changes. The public aliases
remain the compatibility boundary; consumers that adopted the shared model
must remove the additional match arms only when rolling back the type move.
