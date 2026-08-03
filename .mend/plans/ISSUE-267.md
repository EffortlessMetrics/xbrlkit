# Plan: Offline-by-default taxonomy loading with opt-in HTTP

**Issue:** #267
**Selected slice:** feature-gate the existing synchronous HTTP path
**Status:** Ready for review

## Objective

Make `taxonomy-loader` offline by default. Local XSD and linkbase paths must
remain available without a network dependency, while callers that intentionally
load HTTP(S) entrypoints opt into the existing blocking `reqwest` path through
the `http` feature.

## Current source facts

- `taxonomy-loader` currently declares non-optional `reqwest` with its blocking
  client and a 30-second timeout.
- `TaxonomyLoader::fetch_content` dispatches HTTP(S) inputs to that blocking
  client and local inputs to filesystem reads.
- `xbrlkit-cli` and `xbrlkit-bdd-steps` are the current workspace consumers that
  need explicit HTTP behavior.
- The repository has no network-dependent taxonomy-loader tests; acceptance
  runs are offline by policy.

## Scope and design

- Add an empty default feature set and an opt-in `http` feature that owns the
  optional `reqwest` dependency.
- Compile the HTTP client, timeout, and fetch path only when `http` is enabled.
- Return `UnsupportedUrl` for HTTP(S) inputs when the feature is disabled,
  without constructing a network client.
- Opt the CLI and BDD step crates into `taxonomy-loader/http` because they are
  the existing sync consumers that may load remote taxonomies.
- Document the feature and the offline default in the crate README.
- Add a deterministic taxonomy-loader acceptance scenario that checks the
  manifest-level offline default without using live network access.

## Acceptance criteria

- `cargo test -p taxonomy-loader --no-default-features --locked` passes and
  includes a regression test proving HTTP(S) input fails closed.
- `cargo test -p taxonomy-loader --features http --locked` passes, including
  the existing invalid-scheme test.
- The default dependency tree excludes `reqwest`; the explicit `http` tree
  includes it.
- CLI and BDD step consumers compile with their explicit feature opt-in.
- `AC-XK-TAX-LOAD-009` is represented in the feature ledger and passes its
  selector path.
- Existing public loader APIs and local filesystem behavior remain unchanged.
- Formatting, workspace tests, Clippy, repository doctor, feature-grid, and
  impact analysis pass.

## Proof commands

```text
cargo test -p taxonomy-loader --no-default-features --locked
cargo test -p taxonomy-loader --features http --locked
cargo tree -p taxonomy-loader --no-default-features -e normal
cargo tree -p taxonomy-loader --features http -e normal
cargo check -p xbrlkit-cli --locked
cargo check -p xbrlkit-bdd-steps --locked
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo xtask doctor
cargo xtask feature-grid
cargo xtask test-ac AC-XK-TAX-LOAD-009
cargo xtask impact --changed <changed-paths>
git diff --check
```

## Non-goals

- Redesigning the public API as async.
- Replacing `reqwest` with `ureq`.
- Changing timeout, cancellation, or visited-state semantics.
- Adding live-network tests or making normal acceptance runs networked.
- Changing cache-key behavior; that is handled by the separate #233 slice.

## Rollback and follow-up

Rollback is a dependency and feature declaration reversal plus removal of the
conditional branches; local loading remains the compatibility anchor. Async
loading, cancellation, timeout policy, and network-test infrastructure remain
separate follow-up work unless a later issue makes one a prerequisite.
