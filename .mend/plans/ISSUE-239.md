# Plan: Make JSON Export Fallible (Issue #239)

## Problem

`export-run::export_json` uses `expect` in a production path. A serialization
failure therefore aborts the caller instead of returning an actionable error.

## Selected slice

- Return `anyhow::Result<(String, Receipt)>` from `export_json`.
- Attach context while propagating the `serde_json` serialization error.
- Propagate export failures from `xbrlkit-cli`.
- Propagate export failures from `scenario-runner` when an export receipt is
  requested.
- Add a focused success test covering serialized output and receipt semantics.

`CanonicalReport` currently contains strings and collections of plain structs,
so its concrete `serde_json::Value` conversion has no representable failing
input. The implementation still preserves the serializer error boundary for
future report-model fields; an artificial serializer-failure test would not
exercise the public API's current input type.

## Acceptance criteria

- No production `expect` remains in `export_json`.
- Both callers handle the new `Result` without changing successful output or
  receipt contents.
- The export crate has focused regression coverage for successful JSON export.
- The issue plan records caller impact, proof, non-goals, and rollback.

## Proof

```text
cargo fmt --all --check
cargo test -p export-run -p scenario-runner -p xbrlkit-cli -p xbrlkit-bdd-steps --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo xtask alpha-check
cargo xtask package-check
```

These checks establish the affected package behavior and workspace lint,
alpha, and package gates. They do not replace hosted CI or a release check.

## Non-goals

- No change to the JSON schema or receipt wire shape.
- No redesign of `oim_normalize` or report validation.
- No unrelated CLI `unwrap` cleanup.
- No merge or release action.

## Rollback

Revert the export-run source, caller, test, dependency, lockfile, and plan
changes. The public behavior at successful call sites is otherwise unchanged.
