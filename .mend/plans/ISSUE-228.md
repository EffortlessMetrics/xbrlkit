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

## Acceptance criteria

- Only one `Period` enum definition exists in the three affected crates.
- Existing context and streaming parsing behavior remains unchanged.
- Existing public names compile without downstream call-site changes.
- The shared type retains debug/equality and serde behavior, with `Forever` as
  its default.
- A focused test proves the shared model's default contract.
- A plan artifact records the scope, proof, non-goals, and rollback path.

## Proof

Run from the isolated issue worktree with a task-specific target directory:

```text
cargo test -p xbrl-report-types -p xbrl-contexts -p xbrl-stream -p xbrlkit-bdd-steps --locked --offline
cargo clippy -p xbrl-report-types -p xbrl-contexts -p xbrl-stream -p xbrlkit-bdd-steps --all-targets --locked --offline -- -D warnings
cargo fmt --all --check
```

The proof establishes package tests, lint cleanliness, and formatting for the
affected ingestion/model surfaces. It does not establish a full workspace or
hosted CI result.

## Non-goals

- No parser validation, date normalization, or semantic change.
- No migration of unrelated context or streaming structs.
- No new conversion API or public crate support promise.
- No merge or release action.

## Rollback

Revert the bounded source, manifest, test, and plan changes. Existing public
crate-local names remain the compatibility boundary, so rollback does not
require downstream edits.
