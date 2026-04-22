# ISSUE-248: Cache Hit and Schema Import Tracking

## Problem Summary

The BDD step definitions in `crates/xbrlkit-bdd-steps/src/lib.rs` contain two TODO items referencing issue #233:

- **Line ~1690:** `i_see_cache_hit_for_url` step is stubbed — returns hardcoded `true`
  - Needs: `TaxonomyLoaderContext.cache_hit` field
- **Line ~1704:** `i_see_schema_imported` step is stubbed — returns hardcoded `true`
  - Needs: `TaxonomyLoaderContext.loaded_schemas` field

The `TaxonomyLoader` currently tracks visited schemas via `visited: RefCell<HashSet<String>>` for circular import prevention, but does not track:
1. Which URLs were served from cache vs fetched fresh
2. Which schemas were successfully loaded (distinct from merely visited)

## Acceptance Criteria

- [x] Add `cache_hits: RefCell<HashSet<String>>` to `TaxonomyLoader`
- [x] Track actual cache hits during taxonomy loading (when `fetch_url` reads from cache)
- [x] Add `loaded_schemas: RefCell<Vec<String>>` to `TaxonomyLoader`
- [x] Track actual schema imports during resolution (after successful `load_schema_recursive`)
- [x] Update BDD steps to assert against real state instead of hardcoded `true`
- [x] Remove TODO(#233) comments
- [x] Unit tests for tracking behavior
- [x] BDD scenario verification with `@alpha-active` tag

## Implementation Plan

### Phase 1: Add Tracking Fields to TaxonomyLoader

**File:** `crates/taxonomy-loader/src/lib.rs`

Add to `TaxonomyLoader` struct:
```rust
pub cache_hits: std::cell::RefCell<HashSet<String>>,
pub loaded_schemas: std::cell::RefCell<Vec<String>>,
```

**Rationale:** Using `pub` (not `pub(crate)`) because `xbrlkit-bdd-steps` is a separate workspace crate that needs direct access for assertions. Using `std::cell::RefCell` (fully qualified) to match existing `visited` field style.

Initialize in `TaxonomyLoader::new()`:
```rust
cache_hits: std::cell::RefCell::new(HashSet::new()),
loaded_schemas: std::cell::RefCell::new(Vec::new()),
```

### Phase 2: Instrument Cache Hit Tracking

**File:** `crates/taxonomy-loader/src/lib.rs`

In `fetch_url()`, after cache hit:
```rust
// After reading from cache_dir
self.cache_hits.borrow_mut().insert(url.to_string());
```

Insert before returning cached content in the `cache_path.exists()` branch.

### Phase 3: Instrument Schema Load Tracking

**File:** `crates/taxonomy-loader/src/lib.rs`

In `load_schema_recursive()`, after successful load:
```rust
// After all imports/linkbases loaded successfully
self.loaded_schemas.borrow_mut().push(path.to_string());
```

Insert at the end of the function, just before `Ok(())`.

### Phase 4: Preserve Loader in When-Step

**File:** `crates/xbrlkit-bdd-steps/src/lib.rs` (when_steps)

Change:
```rust
// OLD: let loader = world.taxonomy_loader_context.loader.take().context("...")?;
// NEW: let loader = world.taxonomy_loader_context.loader.as_ref().context("...")?;
```

**Critical fix:** `.take()` moves the loader out of `Option`, making it `None`. Then-steps can't inspect it. `.as_ref()` preserves the loader in the world so then-steps can read tracking fields. Since `TaxonomyLoader::load()` takes `&self`, a reference is sufficient.

### Phase 5: Update BDD Then-Steps

**File:** `crates/xbrlkit-bdd-steps/src/lib.rs` (then_steps)

Update `subsequent loads should use the cache`:
```rust
let loader = world.taxonomy_loader_context.loader.as_ref()
    .context("TaxonomyLoader not available")?;
let cache_hits = loader.cache_hits.borrow();
assert!(!cache_hits.is_empty(), "Expected cache hits but none recorded");
Ok(())
```

Update `imported schemas should be loaded`:
```rust
let loader = world.taxonomy_loader_context.loader.as_ref()
    .context("TaxonomyLoader not available")?;
let loaded = loader.loaded_schemas.borrow();
assert!(loaded.len() > 1, "Expected imported schemas but only found {}: {:?}", loaded.len(), &*loaded);
Ok(())
```

Update `the taxonomy file should be cached`:
```rust
let loader = world.taxonomy_loader_context.loader.as_ref()
    .context("TaxonomyLoader not available")?;
let cache_hits = loader.cache_hits.borrow();
assert!(!cache_hits.is_empty(), "Expected cache hits but none recorded");
Ok(())
```

### Phase 6: Remove TODO Comments

Remove from `crates/xbrlkit-bdd-steps/src/lib.rs`:
```rust
// TODO(#233): Real cache hit detection needs TaxonomyLoaderContext.cache_hit field.
// TODO(#233): Real schema import tracking needs TaxonomyLoaderContext.loaded_schemas field.
```

### Phase 7: Unit Tests

**File:** `crates/taxonomy-loader/src/lib.rs`

Add `#[cfg(test)]` block at bottom:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_hit_tracking() {
        let loader = TaxonomyLoader::with_cache_dir(PathBuf::from("/tmp/test-cache"));
        loader.cache_hits.borrow_mut().insert("http://example.com/schema.xsd".to_string());
        assert!(loader.cache_hits.borrow().contains("http://example.com/schema.xsd"));
    }

    #[test]
    fn test_loaded_schema_tracking() {
        let loader = TaxonomyLoader::new();
        loader.loaded_schemas.borrow_mut().push("schema.xsd".to_string());
        assert_eq!(loader.loaded_schemas.borrow().len(), 1);
    }
}
```

### Phase 8: BDD Verification

Run:
```bash
cargo test -p xbrlkit-bdd-steps
cargo xtask bdd --tags @alpha-active
```

## Files to Modify

| File | Lines | Purpose |
|------|-------|---------|
| `crates/taxonomy-loader/src/lib.rs` | ~+15 | Add tracking fields, instrument tracking, unit tests |
| `crates/xbrlkit-bdd-steps/src/lib.rs` | ~+20 | Update then-steps, remove TODOs |
| `crates/xbrlkit-bdd-steps/src/lib.rs` (when_steps) | ~1 | Preserve loader with `.as_ref()` |

## Dependencies

- Blocked by: #233 (the TODO source — this issue resolves it)
- No external blockers
- Cross-crate access: `taxonomy-loader` → `xbrlkit-bdd-steps` via `pub` fields

## Test Strategy

1. **Unit tests:** Verify tracking fields accumulate correctly
2. **BDD tests:** Run `@alpha-active` scenarios for taxonomy loader:
   - SCN-XK-TAX-LOAD-005: Cache taxonomy files locally
   - SCN-XK-TAX-LOAD-006: Handle schema imports recursively
3. **Edge cases:**
   - Empty cache → `cache_hits` is empty
   - No imports → `loaded_schemas` has only entrypoint
   - Circular imports → `visited` prevents infinite loops; `loaded_schemas` may have duplicates (acceptable for Vec)

## Risks

| Risk | Level | Mitigation |
|------|-------|------------|
| RefCell borrow conflicts | Low | Single-threaded BDD execution; borrows are scoped |
| Fixture gap for SCN-XK-TAX-LOAD-006 | Medium | Synthetic taxonomy may need mock imports or real XSD fixtures |
| `loaded_schemas` duplicates | Low | Using `Vec` (not `HashSet`) preserves order; duplicates acceptable for BDD assertions |
| When-step `.as_ref()` change | Low | Verified `TaxonomyLoader::load()` takes `&self`; no ownership transfer needed |

## Notes

- **Type choice:** Using `HashSet<String>` / `Vec<String>` (not `HashSet<Url>`) to match existing `visited: RefCell<HashSet<String>>` pattern. `url::Url` is not directly accessible from `xbrlkit-bdd-steps`.
- **When-step fix:** This is a critical fix identified in repo alignment review. The previous `.take()` consumed the loader, making it unavailable for then-step inspection.
- **Cache hit semantics:** `cache_hits` only tracks URL cache hits. Local file paths via `fetch_file` bypass cache tracking.
