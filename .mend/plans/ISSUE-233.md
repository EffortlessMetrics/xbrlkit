# Collision-resistant taxonomy cache keys — Issue #233

**Status:** Ready for review
**Issue:** #233
**Stream:** Taxonomy-loader reliability

## Objective

Replace delimiter-based URL-to-filename conversion with a SHA-256 digest of
the complete URL. Distinct external URLs must not select the same taxonomy
cache file, and the resulting name must remain safe and bounded for the local
filesystem.

## Root cause and trust boundary

`TaxonomyLoader::url_to_cache_path` currently replaces a small set of URL
characters with `_`. This is a lossy transformation of externally supplied
URL text; for example, `http://a/b` and `http://a_b` select the same path.
The cache key is not an authorization decision, but collisions can serve the
wrong taxonomy content, so the entire URL is hashed before it reaches the
filesystem path.

## Acceptance criteria

- [ ] Distinct URLs that previously collided produce distinct cache paths.
- [ ] The same URL produces the same path on repeated calls.
- [ ] Cache filenames are exactly 64 lowercase hexadecimal characters.
- [ ] Cache read and write call sites use the same helper without API changes.
- [ ] Existing taxonomy-loader behavior remains green under focused and
  workspace checks.

## Dependency and migration boundary

Add the direct `sha2 = "0.10"` dependency and update `Cargo.lock` through Cargo.
No separate hex-encoding dependency is needed because the digest implements
lowercase hexadecimal formatting. Existing delimiter-based cache files are
not migrated; they are ephemeral and may already contain collision-ambiguous
content, so the new key space intentionally starts clean.

## Proof

```text
cargo test -p taxonomy-loader --locked
cargo fmt --all --check
cargo clippy -p taxonomy-loader --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo xtask doctor
cargo xtask feature-grid
cargo xtask impact --changed crates/taxonomy-loader/src/lib.rs --changed crates/taxonomy-loader/Cargo.toml --changed Cargo.lock
git diff --check
```

## Non-goals

- Do not change HTTP fetching, URL validation, cache contents, or public APIs.
- Do not migrate or delete pre-existing cache files.
- Do not add cache-hit instrumentation or schema-import tracking; those are
  separate concerns referenced by older work around this issue.

## Rollback

Revert the manifest, lockfile, helper implementation, tests, and plan. No
persisted repository data or external service state is modified.
