# Plan: ISSUE-233 — Fragile cache path generation in taxonomy-loader

## Problem

`url_to_cache_path()` generates cache filenames by simple string replacement, which can cause collisions.

## Fix

Replace naive string replacement with SHA-256 digest of the URL as the cache filename. This eliminates collision risk entirely.

## Changes

1. `crates/taxonomy-loader/src/lib.rs`:
   - Update `url_to_cache_path()` to use `sha2::Sha256` + `hex::encode()`
   - Update `test_url_to_cache_path()` with correct expected hash

2. `crates/taxonomy-loader/Cargo.toml`:
   - Add `sha2 = "0.10"` and `hex = "0.4"` dependencies

## Validation

- `cargo fmt` ✓
- `cargo clippy` ✓
- `cargo test` ✓ (all 13 tests pass)
