---
slice: SCN-XK-WORKFLOW-003
type: scenario-activation
priority: P2
worktree: /tmp/xbrlkit-wt/cockpit-pack-003
branch: feature/cockpit-pack-003
parent: main
---

# Handoff: SCN-XK-WORKFLOW-003 — Cockpit Pack

## Context

The cockpit pack scenario packages validation reports into a sensor.report.v1 format for the cockpit dashboard. This is a simple wrapping/serialization task.

## Scenario (from specs/features/workflow/cockpit_pack.feature)

```gherkin
@SCN-XK-WORKFLOW-003
@speed.fast
Scenario: Wrap a validation report into sensor.report.v1
  Given a validation report receipt
  When I package the receipt for cockpit
  Then the sensor report is emitted
```

## Acceptance Criteria

- [ ] Add `@alpha-active` tag to the scenario
- [ ] Implement `Given a validation report receipt` step handler
- [ ] Implement `When I package the receipt for cockpit` step handler  
- [ ] Implement `Then the sensor report is emitted` step handler
- [ ] If new receipt type needed, extend `ScenarioExecution`
- [ ] Add AC to `ACTIVE_ALPHA_ACS` in `xtask/src/alpha_check.rs`
- [ ] Add assertion to `assert_scenario_outcome` (if applicable)
- [ ] `cargo test --workspace` passes
- [ ] `cargo xtask alpha-check` passes locally

## Code Excerpts

Current alpha-check active scenarios (from xtask/src/alpha_check.rs):
```rust
pub const ACTIVE_ALPHA_ACS: &[(&str, &str)] = &[
    ("AC-XK-DUPLICATES-001", "SCN-XK-DUPLICATES-001"),
    ("AC-XK-EXPORT-001", "SCN-XK-EXPORT-001"),
    // ... etc
];
```

Scenario execution types (from xtask/src/bdd/engine.rs):
```rust
pub enum ScenarioExecution {
    ValidationRun { ... },
    ExportReceipt { ... },
    IxdsReceipt { ... },
    // May need: CockpitPack { ... }
}
```

## Test Command

```bash
cargo test --workspace && cargo xtask alpha-check
```

## Known Pitfalls

- Check if `sensor.report.v1` schema already exists in `schemas/`
- If extending `ScenarioExecution`, update the serialization/deserialization
- The receipt type might already be covered by `ValidationRun` — check existing patterns

## Definition of Done

Green PR with:
- Scenario tagged @alpha-active
- Step handlers implemented
- Alpha-check updated and passing
- No manual steps required
