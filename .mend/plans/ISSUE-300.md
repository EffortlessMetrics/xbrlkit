# Plan: Share Rule-ID Sanitization (#300)

**Issue:** #300
**Status:** Slice 1 ready for review

## Source-truth reconciliation

Earlier issue comments proposed placing the helper in `xbrl-report-types`,
while the latest deep and repository-alignment reviews selected a dedicated
`xbrlkit-utils` crate. This slice follows the latest decision so DTO types do
not become a home for unrelated string utilities.

## Selected slice

Create one zero-dependency `xbrlkit-utils` implementation of
`sanitize_for_rule_id` and make `numeric-rules` and `efm-rules` use it without
changing generated rule-ID behavior.

## Acceptance criteria

- Only one `sanitize_for_rule_id` implementation remains in the workspace.
- Both validation crates call the shared helper.
- Existing rule-ID output remains unchanged for current consumers.
- The shared helper has focused tests for ASCII, separators/non-ASCII input,
  and empty input.
- The new workspace crate has documented public API and workspace linting.

## Proof

- `cargo test -p xbrlkit-utils --locked --offline`
- `cargo test -p numeric-rules --locked --offline`
- `cargo test -p efm-rules --locked --offline`
- strict Clippy for all three packages
- `cargo fmt --all -- --check`
- `cargo xtask feature-grid`
- `cargo xtask doctor`
- `git diff --check`
- search confirming one helper definition and both imports

## Non-goals

- Do not change `xtask` filesystem-name sanitization.
- Do not change rule-ID formats or validation semantics.
- Do not add unrelated utilities or refactor other finding construction.
