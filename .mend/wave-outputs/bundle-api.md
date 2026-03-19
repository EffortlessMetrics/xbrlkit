# Bundle API Analysis

## Result: CRATE NOT FOUND

The `bundle-selector` crate does NOT exist in the workspace.

## Existing Related Crates
- `xbrlkit-feature-grid` - Contains FeatureGrid compilation and search
- `scenario-contract` - Contains ScenarioRecord and related types
- `scenario-runner` - Contains scenario execution logic

## Feature Grid API (from xbrlkit-feature-grid/src/lib.rs)

### Public Function
```rust
pub fn compile(root: &Path) -> anyhow::Result<FeatureGrid>
```
Compiles feature sidecars (*.meta.yaml) into a searchable grid.

### ScenarioRecord Fields
- scenario_id, ac_id, req_id
- feature_file, sidecar_file  
- layer, module
- crates, fixtures, profile_pack
- receipts, allowed_edit_roots
- suite, speed

## Recommendation
Need to implement bundle-selector crate or add bundle logic to xbrlkit-feature-grid.
The bundle.feature tests require:
- Selecting scenarios by AC ID
- Generating bundle manifest receipts
- Bundle manifest v1 format

---
task: bundle_api_mapper
status: completed_with_gaps
crate_found: false
feature_grid_found: true
analyzed_at: 2026-03-19T09:52:27+08:00
