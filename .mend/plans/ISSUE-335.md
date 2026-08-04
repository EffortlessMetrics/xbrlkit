# Plan: Share Taxonomy Namespace Extraction (#335)

**Issue:** #335
**Status:** Slice 1 ready for review

## Selected slice

Move the identical `extract_namespaces` helper from `schema.rs` and
`linkbase.rs` into an internal `taxonomy-loader::util` module. Keep the helper
crate-private and preserve both existing call sites.

## Acceptance criteria

- One `extract_namespaces` implementation remains in `taxonomy-loader`.
- Schema and linkbase parsing both call the shared helper.
- No public API, parsing behavior, or schema/receipt contract changes.
- Existing taxonomy-loader tests continue to pass.

## Proof

- `cargo test -p taxonomy-loader --locked --offline`
- strict package Clippy
- `cargo fmt --all -- --check`
- `git diff --check`
- search confirming one helper definition and two call-site imports

## Non-goals

- Do not move the helper to `taxonomy-types` or add a dependency.
- Do not change namespace resolution, QName parsing, or taxonomy output.
