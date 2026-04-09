# Plan: [FR-007] Review xbrlkit-bdd-steps dependency footprint

## Issue Reference
- Issue: #149
- Created: 2026-04-09

## Problem Statement

The `xbrlkit-bdd-steps` crate has a heavy dependency footprint with 17 direct dependencies, including almost all other workspace crates. This creates a heavy compile-time dependency graph and increases build times. The crate currently pulls in:

- Workspace external: `anyhow`, `serde_json`, `walkdir` (3)
- Workspace internal: 14 different internal crates including:
  - `scenario-contract`, `scenario-runner` (test infrastructure)
  - `dimensional-rules`, `taxonomy-dimensions`, `xbrl-contexts` (domain logic)
  - `cockpit-export`, `receipt-types` (reporting)
  - `filing-load`, `edgar-attachments` (filing handling)
  - `xbrlkit-feature-grid`, `sec-profile-types` (feature management)
  - `context-completeness`, `xbrl-report-types` (validation)
  - `numeric-rules`, `xbrl-stream`, `taxonomy-loader` (processing)

This monolithic structure means that any change to any of these 14 crates triggers a rebuild of `xbrlkit-bdd-steps`, slowing down CI and local development.

## Goals

1. **Primary Goal:** Reduce the dependency footprint of `xbrlkit-bdd-steps` by at least 40% (from 17 to ~10 or fewer direct dependencies)
2. **Secondary Goal:** Improve CI build times by breaking the crate into smaller, focused sub-crates that can compile independently
3. **Tertiary Goal:** Maintain backward compatibility for existing BDD scenarios during the migration

## Approach

The approach follows a **gradual decomposition strategy**:

1. **Analysis Phase:** Audit actual usage of each dependency in the step definitions
2. **Feature-Gating Phase:** Introduce feature flags for optional functionality before physical splitting
3. **Extraction Phase:** Split into focused sub-crates based on functional domains
4. **Migration Phase:** Update dependent crates to use the new structure

The decomposition will be based on functional domains identified in the step handlers:
- Core step execution and World state management
- Dimension validation steps
- Taxonomy loading steps
- Context completeness steps
- Streaming parser steps
- CLI/profile steps
- Filing/bundle steps

## Implementation Steps

### Step 1: Dependency Usage Audit

**Task:** Create a detailed dependency usage report

1. Run `cargo tree -p xbrlkit-bdd-steps` to analyze the full dependency tree
2. For each of the 17 direct dependencies, document:
   - Which step handlers use it
   - Whether it's used in `World` struct fields
   - Whether it's used in `Given`/`When`/`Then` handlers
   - Compile-time vs runtime usage
3. Categorize dependencies into:
   - **Essential:** Required for core step execution (scenario-contract, scenario-runner)
   - **Domain-specific:** Used only in specific step categories
   - **Optional:** Can be feature-gated

**Expected Output:** `docs/dependency-audit-149.md`

### Step 2: Introduce Feature Flags

**Task:** Add feature flags to `xbrlkit-bdd-steps` before physical splitting

Modify `crates/xbrlkit-bdd-steps/Cargo.toml`:

```toml
[features]
default = ["dimension", "taxonomy", "context", "streaming", "cli", "filing"]
dimension = ["dimensional-rules", "taxonomy-dimensions", "xbrl-contexts"]
taxonomy = ["taxonomy-loader", "taxonomy-dimensions"]
context = ["context-completeness", "xbrl-contexts", "xbrl-report-types"]
streaming = ["xbrl-stream"]
cli = ["sec-profile-types", "cockpit-export"]
filing = ["filing-load", "edgar-attachments"]
validation = ["numeric-rules", "receipt-types"]
```

Update `src/lib.rs` to gate module sections with `#[cfg(feature = "...")]`

### Step 3: Extract Core Crate

**Task:** Create `xbrlkit-bdd-core` with essential dependencies

1. Create new crate at `crates/xbrlkit-bdd-core/`
2. Move to this crate:
   - `Step` and `World` struct definitions (minimal version)
   - Core `run_scenario`, `run_step` functions
   - `assert_declared_inputs_match` function
   - Basic Given/When/Then handlers for scenario execution
3. Dependencies: `anyhow`, `serde_json`, `scenario-contract`, `scenario-runner`
4. Update workspace `Cargo.toml` to include the new member

**Files to Create:**
- `crates/xbrlkit-bdd-core/Cargo.toml`
- `crates/xbrlkit-bdd-core/src/lib.rs`
- `crates/xbrlkit-bdd-core/src/world.rs`
- `crates/xbrlkit-bdd-core/src/steps/mod.rs`

### Step 4: Extract Dimension Validation Crate

**Task:** Create `xbrlkit-bdd-dimension` for dimension-related steps

1. Create new crate at `crates/xbrlkit-bdd-dimension/`
2. Move to this crate:
   - `DimensionContext` and related context structs
   - All dimension-related Given steps ("a context with dimension...")
   - All dimension-related When steps ("I validate the dimension...")
   - All dimension-related Then steps
3. Dependencies: `xbrlkit-bdd-core`, `dimensional-rules`, `taxonomy-dimensions`, `xbrl-contexts`
4. Implement extension trait pattern for World

**Files to Create:**
- `crates/xbrlkit-bdd-dimension/Cargo.toml`
- `crates/xbrlkit-bdd-dimension/src/lib.rs`
- `crates/xbrlkit-bdd-dimension/src/context.rs`

### Step 5: Extract Taxonomy Loader Crate

**Task:** Create `xbrlkit-bdd-taxonomy` for taxonomy loading steps

1. Create new crate at `crates/xbrlkit-bdd-taxonomy/`
2. Move to this crate:
   - `TaxonomyLoaderContext` struct
   - All taxonomy loader Given/When/Then steps
   - `create_synthetic_taxonomy()` helper function
3. Dependencies: `xbrlkit-bdd-core`, `taxonomy-loader`, `taxonomy-dimensions`

### Step 6: Extract Streaming Parser Crate

**Task:** Create `xbrlkit-bdd-streaming` for streaming parser steps

1. Create new crate at `crates/xbrlkit-bdd-streaming/`
2. Move to this crate:
   - `StreamingContext` struct
   - All streaming-related Given/When/Then steps
3. Dependencies: `xbrlkit-bdd-core`, `xbrl-stream`

### Step 7: Update Main Crate as Facade

**Task:** Refactor `xbrlkit-bdd-steps` as a facade crate

1. Modify `crates/xbrlkit-bdd-steps/Cargo.toml`:
   - Remove direct dependencies on internal crates
   - Add dependencies on the new sub-crates
   - Keep features that re-export sub-crate features

2. Update `src/lib.rs`:
   - Re-export from sub-crates
   - Keep only integration code
   - Maintain backward compatibility for existing tests

**New Cargo.toml structure:**
```toml
[dependencies]
xbrlkit-bdd-core = { path = "../xbrlkit-bdd-core" }
xbrlkit-bdd-dimension = { path = "../xbrlkit-bdd-dimension", optional = true }
xbrlkit-bdd-taxonomy = { path = "../xbrlkit-bdd-taxonomy", optional = true }
xbrlkit-bdd-streaming = { path = "../xbrlkit-bdd-streaming", optional = true }
# ... other sub-crates

[features]
default = ["dimension", "taxonomy", "context", "streaming", "cli", "filing"]
dimension = ["dep:xbrlkit-bdd-dimension"]
taxonomy = ["dep:xbrlkit-bdd-taxonomy"]
# ...
```

### Step 8: Verification and Testing

**Task:** Ensure all tests pass after refactoring

1. Run `cargo xtask doctor` to verify workspace health
2. Run `cargo xtask feature-grid` to verify feature grid compilation
3. Run `cargo test -p xbrlkit-bdd-steps` for unit tests
4. Run `cargo xtask test-ac` for acceptance tests
5. Verify feature flag combinations compile:
   ```bash
   cargo check -p xbrlkit-bdd-steps --no-default-features
   cargo check -p xbrlkit-bdd-steps --features dimension
   cargo check -p xbrlkit-bdd-steps --features taxonomy
   ```

### Step 9: Build Time Benchmarking

**Task:** Measure improvement in build times

1. Clean build baseline: `cargo clean && time cargo build -p xbrlkit-bdd-steps`
2. Incremental build test: touch a leaf crate, measure rebuild time
3. Document improvement in `docs/build-time-improvement-149.md`

Target: 20-30% reduction in clean build time, 50%+ reduction in incremental builds when changing domain-specific code.

## Files to Modify

| File Path | Reason |
|-----------|--------|
| `crates/xbrlkit-bdd-steps/Cargo.toml` | Add feature flags, refactor as facade |
| `crates/xbrlkit-bdd-steps/src/lib.rs` | Re-export from sub-crates, remove direct implementations |
| `Cargo.toml` (workspace) | Add new crate members |
| `crates/xbrlkit-bdd-core/Cargo.toml` | New core crate definition |
| `crates/xbrlkit-bdd-core/src/lib.rs` | Core step execution logic |
| `crates/xbrlkit-bdd-dimension/Cargo.toml` | New dimension validation crate |
| `crates/xbrlkit-bdd-dimension/src/lib.rs` | Dimension step implementations |
| `crates/xbrlkit-bdd-taxonomy/Cargo.toml` | New taxonomy loader crate |
| `crates/xbrlkit-bdd-taxonomy/src/lib.rs` | Taxonomy step implementations |
| `crates/xbrlkit-bdd-streaming/Cargo.toml` | New streaming parser crate |
| `crates/xbrlkit-bdd-streaming/src/lib.rs` | Streaming step implementations |

## Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| Breaking existing BDD scenarios | Maintain backward-compatible facade; gradual migration with feature flags |
| Increased crate maintenance overhead | Keep sub-crates small and focused; shared CI pipeline |
| Circular dependencies between sub-crates | Careful domain analysis; use trait-based extension pattern |
| Performance regression from trait objects | Benchmark before/after; prefer generics where possible |
| Merge conflicts during long-running refactor | Use feature-flag approach to enable incremental merging |
| Test coverage gaps | Run full BDD test suite after each extraction step |

## Testing Strategy

### Unit Tests
- Each new sub-crate should have its own test module
- Test individual step handlers with mock World state
- Verify feature flag combinations compile correctly

### Integration Tests
- Existing BDD scenarios serve as integration tests
- Run `cargo xtask test-ac` for acceptance criteria validation
- Verify all feature grid scenarios still pass

### BDD Scenarios
- No new BDD scenarios required for this refactoring
- Existing scenarios in `specs/features/**/*.feature` validate the refactoring
- Specific test cases for feature flag combinations:
  - `--no-default-features` (core only)
  - `--features dimension` (core + dimension)
  - `--features taxonomy` (core + taxonomy)
  - Full feature set (all features)

### Build Time Tests
- Measure clean build time before and after
- Measure incremental build time when touching leaf crates
- Target: Document 20%+ improvement in clean builds

## Definition of Done

- [ ] Implementation complete: All sub-crates extracted and facade crate functional
- [ ] Tests passing: All existing BDD scenarios pass (`cargo xtask test-ac`)
- [ ] Feature flags working: All feature combinations compile correctly
- [ ] Build time improvement documented: At least 20% reduction in clean build time
- [ ] Documentation updated: 
  - [ ] `docs/dependency-audit-149.md` with usage analysis
  - [ ] `docs/build-time-improvement-149.md` with benchmarks
  - [ ] README updated for each new sub-crate
- [ ] Backward compatibility: No changes required to existing scenario files
- [ ] CI pipeline updated: New crates included in workspace build
- [ ] Code review completed: PR reviewed and approved
