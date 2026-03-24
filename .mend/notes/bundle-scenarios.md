# Bundle Scenarios Research Notes

## Current State Analysis

### Feature File: `specs/features/workflow/bundle.feature`
Two scenarios need activation:
1. **SCN-XK-WORKFLOW-002**: Bundle an AC into a bounded context packet
   - Given the feature grid is compiled
   - When I bundle the selector "AC-XK-IXDS-002"
   - Then the bundle manifest lists scenario "SCN-XK-IXDS-002"

2. **SCN-XK-WORKFLOW-004**: Reject a selector that matches no scenarios
   - Given the feature grid is compiled
   - When I bundle the selector "AC-XK-DOES-NOT-EXIST"
   - Then bundling fails because no scenario matches

### Existing Infrastructure

**1. BundleManifest (scenario-contract)**
```rust
pub struct BundleManifest {
    pub selector: String,
    pub scenarios: Vec<ScenarioRecord>,
}
```

**2. Feature Grid Compilation (xbrlkit-feature-grid)**
- `compile(root: &Path) -> anyhow::Result<FeatureGrid>`
- Compiles sidecars into searchable grid

**3. Bundle Logic (xtask/src/main.rs)**
- `bundle(selector: &str)` already exists
- `select_matching_scenarios()` matches by scenario_id, ac_id, req_id, or tags
- Writes manifest to `artifacts/bundles/{sanitized_selector}.json`

**4. BDD Runner (xbrlkit-bdd)**
- `run()` executes scenarios by tag
- Uses `xbrlkit_bdd_steps::run_scenario()` for step execution

**5. Current Step Handlers (xbrlkit-bdd-steps)**
- `handle_given()` - handles profile, fixture, dimension setup
- `handle_when()` - handles validation, export, dimension validation
- `handle_then()` - handles assertions
- Missing: bundle-specific steps

## Design Decision

### Where to implement bundle execution logic?

**Option A: Use xtask bundle command via CLI**
- Pros: Reuses existing code
- Cons: Requires shelling out, less efficient

**Option B: Implement directly in xbrlkit-bdd-steps**
- Pros: Clean integration with existing step handlers
- Cons: Minor code duplication of selector matching logic

**Decision: Option B**
- Direct implementation in `xbrlkit-bdd-steps`
- Keep World state simple (no external process calls)
- Reuse selector matching logic from xtask (it's simple enough)

## Implementation Plan

### 1. Extend World State
```rust
pub struct World {
    // ... existing fields
    pub bundle_manifest: Option<BundleManifest>,  // Add this
    pub grid_compiled: bool,                      // Add this
}
```

### 2. Add Step Handlers

**Given the feature grid is compiled**
- Check if grid is already compiled
- If not, verify grid is loadable (already loaded in World)
- Set `world.grid_compiled = true`

**When I bundle the selector "{selector}"**
- Filter scenarios matching selector (scenario_id, ac_id, or req_id)
- Create BundleManifest
- Store in `world.bundle_manifest`
- For AC-XK-DOES-NOT-EXIST, manifest.scenarios will be empty

**Then the bundle manifest lists scenario "{scenario_id}"**
- Assert world.bundle_manifest is Some
- Assert scenario_id exists in manifest.scenarios

**Then bundling fails because no scenario matches**
- Assert world.bundle_manifest is Some
- Assert manifest.scenarios is empty

### 3. Update Dependencies
- xbrlkit-bdd-steps already depends on scenario-contract
- No new dependencies needed

### 4. Activate Scenarios
- Add `@alpha-active` tag to both scenarios in bundle.feature

### 5. Wire to Alpha Check
- Add "AC-XK-WORKFLOW-002" to ACTIVE_ALPHA_ACS in alpha_check.rs

## Confidence Assessment

- [x] Bundle.feature scenarios understood
- [x] BundleManifest struct confirmed in scenario-contract
- [x] Feature grid compilation confirmed working
- [x] Step handler pattern understood from existing code
- [x] Selector matching logic understood
- [x] Alpha-check wiring understood
- [x] BDD runner integration understood

**Confidence: 90%**

## Implementation Complete

### Files Modified

1. `crates/xbrlkit-bdd-steps/src/lib.rs` - Added bundle step handlers:
   - `Given the feature grid is compiled` - verifies grid is loaded
   - `When I bundle the selector "{selector}"` - filters scenarios and creates BundleManifest
   - `Then the bundle manifest lists scenario "{scenario_id}"` - asserts scenario in manifest
   - `Then bundling fails because no scenario matches` - asserts empty manifest
   - Added `select_matching_scenarios()` and `selector_matches()` helpers
   - Extended `World` struct with `bundle_manifest: Option<BundleManifest>`

2. `specs/features/workflow/bundle.feature` - Added `@alpha-active` tag to both scenarios:
   - SCN-XK-WORKFLOW-002: Bundle an AC into a bounded context packet
   - SCN-XK-WORKFLOW-004: Reject a selector that matches no scenarios

3. `xtask/src/alpha_check.rs` - Added comment noting AC-XK-WORKFLOW-002 is tested via @alpha-active BDD tag (no fixtures needed)

## Acceptance Criteria Verification

- [x] SCN-XK-WORKFLOW-002 passes with @alpha-active
- [x] SCN-XK-WORKFLOW-004 passes with @alpha-active
- [x] cargo build passes
- [x] cargo test passes
- [x] cargo alpha-check passes (16 scenarios selected for @alpha-active)
