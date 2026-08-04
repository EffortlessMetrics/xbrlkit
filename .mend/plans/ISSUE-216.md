# Plan: Route Taxonomy Cache Warnings Through Tracing (Issue #216)

## Problem

`taxonomy-loader` writes a non-fatal cache-write warning directly to stderr.
That bypasses application logging configuration and makes the warning hard for
library consumers to capture or suppress.

## Decision and selected slice

Use the workspace's direct `tracing` dependency and replace the `eprintln!`
with a structured `tracing::warn!` event at the cache-write failure boundary.
Keep cache-write failures non-fatal, matching the current loader behavior.

The event records the error text and a stable non-fatal operation label, but
does not include the external URL or local cache path, which avoids leaking
credentials in query strings or user filesystem details into logs.

## Acceptance criteria

- `taxonomy-loader` emits a structured warning instead of writing directly to
  stderr.
- Cache-write failure remains non-fatal and the fetched content is returned.
- Scenario `AC-XK-TAX-LOAD-010` exercises the warning and non-fatal return path
  with deterministic fetched content and an unwritable cache fixture.
- The workspace manifest, lockfile, and crate manifest remain consistent.
- A focused source test/gate proves the affected crate and its consumers still
  compile and pass their existing behavior tests.
- The plan records the observability and dependency decisions.

## Proof

```text
cargo fmt --all --check
cargo test -p taxonomy-loader -p xbrlkit-bdd-steps -p xbrlkit-cli --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo xtask alpha-check
cargo xtask package-check
cargo xtask bdd --tags @SCN-XK-TAX-LOAD-009
cargo xtask feature-grid
```

These checks establish the affected crate behavior, workspace lint, active
alpha, and package gates. They do not replace hosted CI or prove that an
application has installed a tracing subscriber.

On the current `main` harness, `cargo xtask test-ac AC-XK-TAX-LOAD-010` is not
the proof command for this BDD-only scenario: after the metadata is valid, the
fixture-oriented path fails because the taxonomy fixture has no HTML members.
The declared BDD dispatch in PR #389 is the prerequisite for canonical AC
execution. Until that contract lands, the exact tagged BDD command above is
the authoritative behavioral witness for this slice.

## Non-goals

- No change to cache failure severity or loader return types.
- No logging of URLs, cache paths, response bodies, or credentials.
- No logging backend/subscriber configuration for applications.
- No unrelated taxonomy-loader refactor.

## Rollback

Revert the workspace dependency, taxonomy-loader manifest/source, lockfile,
and plan changes. The previous non-fatal cache-write behavior remains the
fallback.
