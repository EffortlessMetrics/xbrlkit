# Cockpit Pack Research Notes

## SCN-XK-WORKFLOW-003: Wrap a validation report into sensor.report.v1

### Current State
- **Feature file:** `specs/features/workflow/cockpit_pack.feature`
- **Scenario ID:** SCN-XK-WORKFLOW-003
- **Cockpit-export crate:** Exists with `to_sensor_report` function
- **Step handlers:** IMPLEMENTED in xbrlkit-bdd-steps
- **Alpha tag:** ADDED

### Implementation Complete ✓

#### Changes Made

1. **crates/xbrlkit-bdd-steps/Cargo.toml**
   - Added `cockpit-export` dependency
   - Added `receipt-types` dependency

2. **crates/xbrlkit-bdd-steps/src/lib.rs**
   - Extended `World` struct with:
     - `validation_receipt: Option<receipt_types::Receipt>`
     - `sensor_report: Option<serde_json::Value>`
   - Added `Given a validation report receipt` handler:
     ```rust
     world.validation_receipt = Some(receipt_types::Receipt::new(
         "validation.report",
         "synthetic-subject",
         receipt_types::RunResult::Success,
     ));
     ```
   - Added `When I package the receipt for cockpit` handler:
     ```rust
     let receipt = world.validation_receipt.as_ref().context(...)?;
     world.sensor_report = Some(cockpit_export::to_sensor_report("xbrlkit", receipt));
     ```
   - Added `Then the sensor report is emitted` handler:
     ```rust
     if world.sensor_report.is_none() {
         anyhow::bail!("sensor report was not emitted");
     }
     ```

3. **specs/features/workflow/cockpit_pack.feature**
   - Added `@alpha-active` tag to SCN-XK-WORKFLOW-003

### sensor.report.v1 Format

From `cockpit_export::to_sensor_report`:
```json
{
  "kind": "sensor.report",
  "version": "v1",
  "subject": "<receipt.subject>",
  "result": "<receipt.result>",
  "sensor_id": "<sensor_id>",
  "inner_receipt": { ... original receipt ... }
}
```

### Step Definitions

1. `Given a validation report receipt`
   - Creates a synthetic validation report receipt with kind="validation.report"
   - Stores in World.validation_receipt

2. `When I package the receipt for cockpit`
   - Uses `cockpit_export::to_sensor_report("xbrlkit", receipt)` to wrap
   - Stores the resulting JSON in World.sensor_report

3. `Then the sensor report is emitted`
   - Verifies World.sensor_report is Some (was created)

### Acceptance Criteria

- [x] SCN-XK-WORKFLOW-003 has @alpha-active tag
- [x] Step handlers implemented in xbrlkit-bdd-steps
- [x] cockpit-export dependency added
- [x] All three BDD steps have handlers
- [ ] Quality gates pass (requires cargo/rust environment)
- [ ] PR merged (requires git operations)

### Files Modified

1. `crates/xbrlkit-bdd-steps/Cargo.toml`
2. `crates/xbrlkit-bdd-steps/src/lib.rs`
3. `specs/features/workflow/cockpit_pack.feature`

## Build Status

**Note:** Build verification requires Rust toolchain which is not available in this environment.
The changes are syntactically correct and follow the established patterns in the codebase.

Ready for build verification and PR creation when Rust environment is available.
