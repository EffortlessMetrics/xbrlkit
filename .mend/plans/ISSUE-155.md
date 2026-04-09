# Plan: [FR-012] Split oversized xbrlkit-bdd-steps module

## Issue Reference
- Issue: #155
- Created: 2026-04-09

## Problem Statement
The `xbrlkit-bdd-steps` crate has grown to **1,625 lines** in a single file (`crates/xbrlkit-bdd-steps/src/lib.rs`), making it difficult to maintain and navigate. The monolithic structure creates cognitive load, increases merge conflict risk, and hinders testability.

## Goals
1. **Modularize**: Split the 1,625-line file into domain-specific modules
2. **Maintain Functionality**: Ensure zero behavioral changes
3. **Improve Testability**: Enable unit testing of individual step handlers
4. **Reduce Duplication**: Remove duplicate selector logic (coordinate with #144)
5. **Improve Maintainability**: Each module should be under 500 lines

## Approach
Use a layered architecture that separates concerns:
- **World layer**: Contains all context structs and World state
- **Step dispatcher**: Central routing for Given/When/Then steps
- **Domain handlers**: Grouped by functionality (Given, When, Then)
- **Utilities**: Shared parsing and helper functions

## Implementation Steps

### Phase 1: Analysis & Preparation
1. **Audit current code**: Document all structs, functions, and their dependencies
2. **Identify shared utilities**: Find common parsing logic and helpers
3. **Map dependencies**: Understand which handlers depend on which context structs
4. **Review #144**: Coordinate with selector logic deduplication

### Phase 2: Create World Module
1. Create `src/world.rs` containing:
   - All 8 context structs (DimensionContext, TaxonomyContext, etc.)
   - World struct definition
   - World initialization and cleanup
   - Context accessor methods
2. Move imports and dependencies to module level
3. Add module-level documentation

### Phase 3: Create Steps Module Structure
1. Create `src/steps/mod.rs` with:
   - Step dispatcher functions
   - Common error handling
   - Shared type definitions
2. Create `src/steps/given.rs` for all Given step handlers (~400 lines)
3. Create `src/steps/when.rs` for all When step handlers (~300 lines)
4. Create `src/steps/then.rs` for all Then step handlers (~400 lines)
5. Create `src/steps/parse.rs` for step parsing utilities (~100 lines)

### Phase 4: Migrate Domain Handlers
1. **Dimension validation steps** (~250 lines)
   - Move dimension/member validation logic
2. **Taxonomy loader steps** (~200 lines)
   - Move loading and caching logic
3. **Streaming parser steps** (~150 lines)
   - Move large file handling logic
4. **Context completeness steps** (~200 lines)
   - Move context validation logic
5. **Filing manifest steps** (~100 lines)
   - Move EDGAR filings logic
6. **Bundle/grid steps** (~150 lines)
   - Move feature grid operations
7. **CLI steps** (~100 lines)
   - Move command-line operations

### Phase 5: Update lib.rs
1. Replace contents with re-exports only
2. Re-export: `World`, step functions, context structs
3. Maintain public API compatibility

### Phase 6: Address Duplication (coordinate with #144)
1. Identify `select_matching_scenarios` and `selector_matches` duplicates
2. Extract to shared crate or remove if already consolidated
3. Update imports in both `xbrlkit-bdd-steps` and `xtask`

### Phase 7: Documentation & Cleanup
1. Add module-level documentation for each file
2. Document public structs and functions
3. Run `cargo xtask doctor` to verify
4. Run full test suite

## Files to Modify

### New Files
| File | Lines (est.) | Purpose |
|------|--------------|---------|
| `src/world.rs` | ~300 | World struct and context structs |
| `src/steps/mod.rs` | ~200 | Step dispatcher and common utilities |
| `src/steps/given.rs` | ~400 | Given step handlers |
| `src/steps/when.rs` | ~300 | When step handlers |
| `src/steps/then.rs` | ~400 | Then step handlers |
| `src/steps/parse.rs` | ~100 | Step parsing utilities |

### Modified Files
| File | Change |
|------|--------|
| `src/lib.rs` | Replace with re-exports only (~50 lines) |
| `Cargo.toml` | Update if new dependencies needed for modularity |

### Coordination Files (with #144)
| File | Action |
|------|--------|
| `xtask/src/main.rs` | Use shared selector logic |

## Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| Public API breakage | Maintain exact re-exports in lib.rs; verify with `cargo check` |
| Import resolution errors | Use `pub use` statements; test compilation at each step |
| Test failures | Run tests after each phase; use `cargo xtask test-ac` |
| Merge conflicts with parallel work | Coordinate timing; use small incremental PRs |
| Coordination with #144 | Communicate with assignee; extract shared logic first |
| Missed functionality during split | Create comprehensive checklist from current file audit |

## Testing Strategy

### Unit Tests
- Test individual step handlers in isolation
- Mock World context for handler tests
- Test parsing utilities with edge cases

### Integration Tests
- Run existing BDD scenarios: `cargo xtask test-ac`
- Verify feature grid operations: `cargo xtask feature-grid`
- Run impact analysis: `cargo xtask impact --changed crates/xbrlkit-bdd-steps`

### BDD Scenarios
- All existing scenarios should pass without modification
- No changes to `.feature` files expected

## Dependencies

### Blocked By
- #144 - Consolidate selector matching logic duplication (optional but recommended)

### Related To
- #148 - Optimize BDD World struct memory usage
- #149 - Review xbrlkit-bdd-steps dependency footprint

## Definition of Done
- [ ] `src/lib.rs` reduced to ~50 lines of re-exports
- [ ] All 6 new module files created and populated
- [ ] Each module under 500 lines
- [ ] No functional changes to behavior
- [ ] All existing tests pass: `cargo xtask test-ac`
- [ ] Module-level documentation added for each file
- [ ] Public API unchanged (verified by `cargo check`)
- [ ] Selector logic duplication addressed (coordinate with #144)
- [ ] `cargo xtask doctor` passes
- [ ] PR reviewed and merged

## Timeline Estimate
- **Phase 1-2**: 2 hours (Analysis + World module)
- **Phase 3-4**: 3 hours (Steps structure + migration)
- **Phase 5-6**: 1 hour (lib.rs + deduplication)
- **Phase 7**: 1 hour (Documentation + cleanup)
- **Total**: ~7 hours
