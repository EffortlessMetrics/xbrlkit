# Handoff: ISSUE-233 — Fragile cache path generation in taxonomy-loader

## Issue

https://github.com/EffortlessMetrics/xbrlkit/issues/233

## Problem Statement

`url_to_cache_path()` in `crates/taxonomy-loader/src/lib.rs` generated cache filenames by replacing `/`, `:`, `?`, `&`, `=` with underscores. Different URLs could collide to the same filename (e.g., `http://a/b` and `http://a_b` both become `http__a_b`).

## Solution Implemented

Replaced the naive string-replacement approach with a SHA-256 digest of the URL, hex-encoded to a 64-character filename. This provides:
- **Zero collision risk** (cryptographic hash)
- **Fixed filename length** (64 chars, filesystem-safe)
- **Deterministic output** (same URL → same cache path)

## Files Changed

- `crates/taxonomy-loader/src/lib.rs` — `url_to_cache_path()` + `test_url_to_cache_path()`
- `crates/taxonomy-loader/Cargo.toml` — added `sha2 = "0.10"`, `hex = "0.4"`
- `Cargo.lock` — updated automatically

## Quality Gates

| Gate | Status |
|------|--------|
| `cargo fmt` | ✅ |
| `cargo clippy` | ✅ |
| `cargo test` | ✅ (13 tests pass) |

## Branch

`feature/ISSUE-233-cache-path`

## PR

(TBD — pushed after this handoff)
