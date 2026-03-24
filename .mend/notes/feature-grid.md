# Feature Grid Research Notes

## Objective
Activate SCN-XK-WORKFLOW-001 by implementing step handlers for feature grid generation validation.

## Current State Analysis

### Feature File (specs/features/workflow/feature_grid.feature)
```gherkin
@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Feature grid

  @AC-XK-WORKFLOW-001
  @SCN-XK-WORKFLOW-001
  @speed.fast
  Scenario: Compile a feature grid
    Given the repo has feature sidecars
    When I compile the feature grid
    Then the feature grid contains scenario "SCN-XK-IXDS-002"
```

**Missing:** `@alpha-active` tag

### Sidecar (specs/features/workflow/feature_grid.meta.yaml)
```yaml
feature_id: FEAT-XK-WORKFLOW-GRID
layer: workflow
module: feature-grid
scenarios:
  SCN-XK-WORKFLOW-001:
    ac_id: AC-XK-WORKFLOW-001
    req_id: REQ-XK-WORKFLOW
    crates: [xbrlkit-feature-grid, xtask]
    fixtures: []
    receipts: [feature.grid.v1, scenario.run.v1]
    suite: synthetic
    speed: fast
    allowed_edit_roots:
      - crates/xbrlkit-feature-grid
      - xtask
      - specs/features/workflow
```

### Existing Infrastructure

1. **xbrlkit-feature-grid crate** (`crates/xbrlkit-feature-grid/src/lib.rs`)
   - `compile(root: &Path) -> anyhow::Result<FeatureGrid>`
   - Walks `specs/features` for `.meta.yaml` sidecars
   - Builds `FeatureGrid` with `Vec<ScenarioRecord>`

2. **xtask** (`xtask/src/main.rs`)
   - `cargo xtask feature-grid` command exists
   - Calls `xbrlkit_feature_grid::compile(&repo_root)`
   - Writes to `artifacts/feature.grid.v1.json`

3. **BDD Steps** (`crates/xbrlkit-bdd-steps/src/lib.rs`)
   - Pattern: `handle_given()`, `handle_when()`, `handle_then()`
   - World state contains `grid: FeatureGrid`
   - Existing bundle pattern: "Given the feature grid is compiled"

## Step Handlers Required

### 1. `Given the repo has feature sidecars`
- **Location:** `handle_given()` in `xbrlkit-bdd-steps`
- **Implementation:** Check that sidecar files exist (at least one `.meta.yaml` in `specs/features`)
- **Pattern:** Similar to "the feature grid is compiled" but lighter - just verify presence

### 2. `When I compile the feature grid`
- **Location:** `handle_when()` in `xbrlkit-bdd-steps`
- **Implementation:** Call `xbrlkit_feature_grid::compile(&world.repo_root)`
- **Storage:** Store result in world (need to add field to World struct)
- **World extension:** Add `compiled_grid: Option<FeatureGrid>` field

### 3. `Then the feature grid contains scenario "{scenario_id}"`
- **Location:** `handle_then()` in `xbrlkit-bdd-steps`  
- **Implementation:** Check if scenario_id exists in `world.compiled_grid.scenarios`
- **Pattern:** Similar to bundle assertions

## Implementation Plan

### Phase 1: Update World Struct
Add to `crates/xbrlkit-bdd-steps/src/lib.rs`:
```rust
pub struct World {
    // ... existing fields ...
    pub compiled_grid: Option<FeatureGrid>,
}
```

### Phase 2: Implement Step Handlers

**Given:**
```rust
if step.text == "the repo has feature sidecars" {
    // Check at least one .meta.yaml exists in specs/features
    return Ok(true);
}
```

**When:**
```rust
if step.text == "I compile the feature grid" {
    world.compiled_grid = Some(xbrlkit_feature_grid::compile(&world.repo_root)?);
    return Ok(true);
}
```

**Then:**
```rust
if let Some(scenario_id) = step.text.strip_prefix("the feature grid contains scenario \"") {
    let scenario_id = scenario_id.trim_end_matches('"');
    let grid = world.compiled_grid.as_ref().context("grid not compiled")?;
    if !grid.scenarios.iter().any(|s| s.scenario_id == scenario_id) {
        anyhow::bail!("scenario {} not found in grid", scenario_id);
    }
    return Ok(());
}
```

### Phase 3: Add @alpha-active Tag
Update `specs/features/workflow/feature_grid.feature` to add `@alpha-active` tag.

## Testing

Run with: `cargo xtask bdd --tags @alpha-active`

Expected:
- SCN-XK-WORKFLOW-001 passes
- Receipt written to `artifacts/runs/scenario.run.v1.json`
