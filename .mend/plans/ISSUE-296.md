# Plan: Cache repeated scenario fixture reads (Issue #296)

## Problem

`scenario-runner` rereads identical `entrypoints.yaml`, `report.yaml`, and
HTML fixture contents whenever a scenario reuses a fixture directory. This
adds avoidable file-content I/O to repeated BDD and acceptance execution.

## Target seam

Keep the public `load_fixture_facts` and `load_html_members` APIs unchanged and
route their file reads, plus `load_entry_points`, through one internal,
process-local cache in `crates/scenario-runner/src/lib.rs`.

The cache stores raw file text by `PathBuf`, retains the file modification time
and length as metadata, and verifies the current text before serving a hit. The
content check is intentional: a same-size rewrite can preserve coarse file
timestamps, so metadata alone cannot safely invalidate the cache. The cache
uses one `Mutex` for consistent lock ordering and evicts least-recently-used
entries at a fixed capacity of 64 files. A cache miss still uses the existing
contextual read and parse errors.

## Acceptance criteria

- Repeated loads of unchanged fixture content can reuse the cache entry after
  content verification.
- A changed fixture is reread rather than serving stale content.
- The cache remains bounded and evicts the least-recently-used entry.
- Missing or malformed fixtures retain fallible, contextual errors.
- Existing scenario behavior and public function signatures remain unchanged.
- No external dependency is added.

## Proof

- Focused `scenario-runner` unit tests cover hit, invalidation, eviction, and
  loader behavior.
- `cargo test -p scenario-runner --locked`
- `cargo clippy -p scenario-runner --all-targets --locked -- -D warnings`
- `cargo fmt --all --check`
- `cargo xtask doctor`
- `cargo xtask feature-grid`
- `cargo xtask impact --changed crates/scenario-runner/src/lib.rs .mend/plans/ISSUE-296.md`
- `cargo xtask alpha-check`

## Non-goals

- No directory-listing cache; `load_html_members` continues to discover the
  current set of HTML files on every call.
- No cross-process or persistent cache.
- No scenario-result cache, taxonomy/profile cache, or API/schema change.
- No CI performance threshold or benchmark claim in this slice. The cache does
  not claim to eliminate the metadata/content read needed to verify freshness.

## Rollback

Revert the single implementation commit. The cache is in-memory only and has
no persisted state or migration.
