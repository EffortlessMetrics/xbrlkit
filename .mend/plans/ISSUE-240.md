# Issue #240: centralize oracle-comparison receipt construction

## Problem

`oracle-compare` and `xbrlkit-interop-tests` each construct the same
`oracle.compare` warning receipt. The duplicated field mapping can drift if the
receipt contract changes.

## Selected slice

Add one owned factory to `receipt-types` and re-export it under each crate's
existing public function name:

- `oracle-compare::comparison_receipt`;
- `xbrlkit-interop-tests::interop_receipt`.

The public names and emitted receipt shape remain unchanged.

## Acceptance criteria

- `receipt-types` contains the only `oracle.compare` receipt construction.
- Both existing crate-level functions remain available as re-exports.
- The focused test proves kind, version, subject, and warning result.
- Workspace metadata and lockfile remain unchanged.

## Proof

```text
cargo test -p receipt-types -p oracle-compare -p xbrlkit-interop-tests --locked --offline
cargo clippy -p receipt-types -p oracle-compare -p xbrlkit-interop-tests --all-targets --locked --offline -- -D warnings
cargo check --workspace --locked --offline
cargo fmt --all --check
cargo xtask doctor
git diff --check
```

## Non-goals

- Do not remove or publish either currently unreferenced crate.
- Do not change receipt schemas, serialization, or oracle execution behavior.
- Do not add a new dependency or redesign the receipt model.

## Rollback

Revert the slice commit; the two wrappers can temporarily return to their
local constructors without changing their public signatures.
