# Issue #218: Activate the package-check scenario

## Current seam

`package_check.feature` and its sidecar exist on the selector-contract base supplied by PR #403, but the BDD handlers were missing and the scenario was not active in the alpha set.

## Selected slice

Add a fallible package-check context and handlers that:

- discover publishable workspace packages with `cargo metadata`;
- run the existing `cargo package --allow-dirty --locked --list` semantics deterministically;
- retain per-package results and report all failures; and
- activate `SCN-XK-WORKFLOW-006` after its handlers exist.

The scenario sidecar is updated to name the actual BDD-step crate and the lockfile surface used by this implementation.

The selector contract in PR #403 is a prerequisite and remains unchanged by this slice.

## Acceptance criteria

- Discover only publishable packages and reject an empty publishable set.
- Run the locked package listing for every discovered package and preserve diagnostics.
- Make the Then step fail with package names and command output when any package fails.
- Pass the offline focused BDD scenario and the active alpha set.
- Leave selector matching and unrelated BDD behavior unchanged.

## Proof commands

```text
cargo test -p xbrlkit-bdd-steps --locked --offline
cargo clippy -p xbrlkit-bdd-steps --all-targets --locked --offline -- -D warnings
cargo fmt --all -- --check
git diff --check
cargo xtask feature-grid
cargo run -p xtask --locked --offline -- bdd --tags @SCN-XK-WORKFLOW-006
cargo xtask alpha-check
```

## Non-goals and rollback

This slice does not change `xtask` package-check behavior, selector matching, dependency cleanup, archive crates, or unrelated package metadata. Rollback is limited to the implementation, scenario tag, and this plan.

## Deferred harness follow-up

The generic `cargo xtask test-ac AC-XK-WORKFLOW-004` path remains fixture-oriented and currently rejects this fixture-free BDD scenario before execution. The gap is recorded on issue #388; this slice uses the dedicated BDD selector and `alpha-check` witnesses instead.
