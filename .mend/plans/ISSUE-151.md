# Plan: [FR-010] Refactor duplicate taxonomy loader setup in BDD Given steps

**Issue:** #151  
**Status:** plan-draft  
**Created:** 2026-04-09

## Current State

The `handle_given` function in `crates/xbrlkit-bdd-steps/src/lib.rs` contains **6 identical 5-line code blocks** for setting up the taxonomy loader context:

```rust
world.taxonomy_loader_context.schema_path =
    Some("fixtures/synthetic/taxonomy/standard-location-01/schema.xsd".to_string());
world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
```

This pattern appears for:
- `a taxonomy schema with dimension elements`
- `a taxonomy definition linkbase with domain members`
- `a taxonomy with typed dimensions`
- `a taxonomy with hypercube elements`
- `a taxonomy schema that imports another schema`
- `a loaded taxonomy with dimension definitions` (partially different)

**Impact:**
- Code bloat in an already large file (~1,100 lines)
- Maintenance burden - changing fixture path requires 5+ edits
- Inconsistent with DRY principles

## Goal

Extract the repeated taxonomy loader setup pattern into a reusable helper function or macro to eliminate code duplication and improve maintainability.

## Constraints

- Keep all existing BDD test behaviors intact
- Maintain deterministic output (no behavior changes)
- Preserve receipt-backed test approach
- Changes should be local to `crates/xbrlkit-bdd-steps`

## Approach

### Option 1: Extract Helper Function (Preferred)
Create a helper function in `crates/xbrlkit-bdd-steps/src/lib.rs`:

```rust
fn setup_taxonomy_loader(world: &mut World, fixture_path: &str) {
    world.taxonomy_loader_context.schema_path = Some(fixture_path.to_string());
    world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
}
```

Then replace each 5-line block with:
```rust
setup_taxonomy_loader(
    world,
    "fixtures/synthetic/taxonomy/standard-location-01/schema.xsd"
);
```

### Option 2: Macro (If patterns vary more)
If some cases need slight variations, consider a macro:
```rust
macro_rules! setup_taxonomy_loader {
    ($world:expr, $path:expr) => {
        $world.taxonomy_loader_context.schema_path = Some($path.to_string());
        $world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
    };
}
```

## Implementation Steps

1. **Identify all duplicate blocks** (~341-345, ~351-355, ~361-365, ~371-375, ~401-405)
2. **Extract helper function** at an appropriate location in the file
3. **Replace all occurrences** with function calls
4. **Verify no behavior changes** - run `cargo xtask doctor` and BDD tests
5. **Update related issues** - reference #145 and #149

## References

- Issue: #151
- Related: #145 [FR-003] Deduplicate synthetic taxonomy creation in tests
- Related: #149 [FR-007] Review xbrlkit-bdd-steps dependency footprint
- File: `crates/xbrlkit-bdd-steps/src/lib.rs`
