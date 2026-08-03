# Issue #330: Remove the unused `render-md` CLI dependency

## Current seam

The dependency audit verified that `xbrlkit-cli` declares `render-md`, but its source contains no import or use of the crate. The `render-md` workspace package remains an independently tracked orphan under issue #339.

## Selected slice

Remove only the unused `render-md.workspace` dependency from `crates/xbrlkit-cli/Cargo.toml` and refresh the lockfile dependency edge. Keep the workspace member and package itself unchanged so issue #339 remains the owner of orphaned-crate disposition.

## Acceptance criteria

- `xbrlkit-cli` has no unused `render-md` dependency edge.
- `render-md` remains a workspace member and is not deleted or redesigned.
- The lockfile matches the manifest graph.
- CLI compilation, tests, formatting, and dependency metadata checks pass.

## Proof commands

```text
cargo check -p xbrlkit-cli --locked --offline
cargo test -p xbrlkit-cli --locked --offline
cargo metadata --format-version 1 --locked --offline
cargo xtask doctor
cargo fmt --all -- --check
git diff --check
```

## Non-goals and rollback

This slice does not delete `render-md`, change CLI behavior, remove any workspace member, or resolve the broader orphan inventory in #339. Rollback is the manifest, lockfile, and plan change only.
