# Plan: Deterministic taxonomy-loader HTTP tests (Issue #211)

## Status

Plan draft, research reconciled against `origin/main` at `e4d941c`.

This issue is a testability and determinism work item. It is not evidence that
the current unit-test suite already performs external HTTP requests.

## Current-state findings

- `TaxonomyLoader::new` starts without a stored HTTP client.
- `TaxonomyLoader::with_cache_dir` builds a blocking `reqwest` client, while
  `fetch_url` lazily builds one when the loader has no stored client.
- The checked-in `taxonomy-loader` tests cover construction, cache-path
  derivation, and rejection of an unsupported URL scheme.
- The schema and linkbase tests parse inline XML and do not load a remote URL.
- No current test provides deterministic coverage for successful HTTP loading,
  HTTP status failures, transport failures, or remote cache hit/miss behavior.

The original issue wording says tests "may trigger actual HTTP calls", but that
behavior was not reproduced from the current test code. The durable problem is
missing deterministic coverage and the absence of an explicit test transport
seam.

## Goal

Make taxonomy-loader's remote-loading behavior testable without external
network access, while preserving the existing public loading behavior and
constructors unless a compatibility-reviewed API change is proven necessary.

## Proposed sequence

### Slice 1: establish the transport seam and characterization coverage

After the currently active taxonomy-loader production lanes have settled,
introduce the smallest seam that lets unit tests supply deterministic responses.
Prefer a private crate-internal transport abstraction or adapter around the
blocking client. Keep the default production path backed by the existing
blocking `reqwest` client and keep `load_taxonomy`, `TaxonomyLoader::new`, and
`TaxonomyLoader::with_cache_dir` source-compatible.

Add fixture-free tests for:

1. a successful remote response containing a minimal schema;
2. a non-success HTTP status, such as 404;
3. a transport error without waiting for the 30-second production timeout;
4. rejection of unsupported schemes before transport use;
5. a cache miss followed by a write; and
6. a cache hit that avoids the transport.

The test double must record requests sufficiently to prove cache hits do not
perform a second request. Tests must not depend on public Internet hosts,
wall-clock sleeps, or process-global mutable state.

### Slice 2: document the testing contract

Update `crates/taxonomy-loader/README.md` with the local-only testing rule and
the distinction between the default production transport and the deterministic
test transport. Keep operational HTTP behavior and opt-in policy in the
existing implementation issues rather than duplicating those contracts here.

## Acceptance criteria

- [ ] The current issue description is represented accurately: the gap is
      missing deterministic remote-path coverage, not a reproduced external
      network call in the existing tests.
- [ ] A crate-internal transport seam or equivalent test adapter exists without
      requiring tests to contact an external host.
- [ ] Success, HTTP failure, transport failure, unsupported scheme, cache miss,
      and cache hit behavior are covered by focused tests.
- [ ] The cache-hit test proves that the transport is not called after a cached
      response is available.
- [ ] Existing public constructors and local-path loading remain compatible.
- [ ] `crates/taxonomy-loader/README.md` documents the no-network test rule.
- [ ] Normal tests pass with offline dependency resolution and no network
      permission.

## Likely files and seams

- `crates/taxonomy-loader/src/lib.rs`
- `crates/taxonomy-loader/src/error.rs`
- a new crate-private transport module or test-support module, if the selected
  seam warrants one
- `crates/taxonomy-loader/README.md`

Do not edit the BDD feature grid, schema contracts, or unrelated taxonomy
loader refactors in this issue.

## Dependencies and ordering

Reconcile the implementation branch with the current heads of the taxonomy
loader lanes before coding. At the time this plan was written, the relevant
open PRs included cache-key handling (#386), HTTP opt-in behavior (#387),
tracing for cache warnings (#400), cache/import tracking (#401), and namespace
refactoring (#428). The implementation should either follow those changes or
be restacked narrowly so that it does not overwrite their ownership.

## Proof commands

From the implementation worktree:

```text
cargo fmt --all -- --check
cargo test -p taxonomy-loader --locked --offline
cargo clippy -p taxonomy-loader --all-targets --locked --offline -- -D warnings
git diff --check
```

If a local network-denial harness is available, run it as an additional proof.
The focused Cargo test is the required proof and must not be reported as
network-isolated unless the test process was actually run without network
access.

## Non-goals

- changing the production HTTP timeout or URL policy;
- making HTTP loading opt-in or changing CLI behavior;
- redesigning cache-key generation or cache freshness;
- adding live remote taxonomy fixtures;
- changing public error or DTO contracts without a separate compatibility
  decision;
- broad taxonomy-loader refactoring.

## Risks and rollback

The main risk is coupling a test seam to the public loader API while adjacent
production PRs are changing the same module. Keep the first implementation
private and behavior-preserving. If the seam proves to require a public API
change, stop and split that compatibility decision into a separate issue/PR.

Rollback is a single PR revert. No persisted data, wire format, or external
service state is changed.

## Claim boundary

Passing focused tests will establish deterministic coverage of the selected
transport and cache branches. It will not establish reachability or correctness
of third-party taxonomy servers, full filing validation, or release readiness.
