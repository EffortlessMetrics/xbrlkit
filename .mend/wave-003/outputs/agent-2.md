# Step Handler Analysis: cockpit_pack.feature

## Target Steps to Handle

From `specs/features/workflow/cockpit_pack.feature`:

```gherkin
Given a validation report receipt
When I package the receipt for cockpit
Then the sensor report is emitted
```

---

## Existing Step Handlers in `crates/xbrlkit-bdd-steps/src/lib.rs`

### Given Handlers (`handle_given`)

| Pattern | Status | Location |
|---------|--------|----------|
| `the profile pack "{profile_id}"` | ✅ EXISTING | Line ~83 |
| `the fixture directory "{fixture}"` | ✅ EXISTING | Line ~94 |
| `the fixture "{fixture}"` | ✅ EXISTING | Line ~94 |
| **Given a validation report receipt** | ❌ MISSING | - |

### When Handlers (`handle_when`)

| Pattern | Status | Location |
|---------|--------|----------|
| `I validate the filing` | ✅ EXISTING | Line ~118 |
| `I validate duplicate facts` | ✅ EXISTING | Line ~118 |
| `I resolve the DTS` | ✅ EXISTING | Line ~118 |
| `I export the canonical report to JSON` | ✅ EXISTING | Line ~126 |
| **When I package the receipt for cockpit** | ❌ MISSING | - |

### Then Handlers (`handle_then` + `handle_parameterized_assertion`)

| Pattern | Status | Location |
|---------|--------|----------|
| `the validation report has no error findings` | ✅ EXISTING | Line ~136 |
| `the taxonomy resolution succeeds` | ✅ EXISTING | Line ~139 |
| `the concept set is:` | ✅ EXISTING | Line ~142 |
| `the export report receipt is emitted` | ✅ EXISTING | Line ~150 |
| `the validation report contains rule "{rule_id}"` | ✅ EXISTING | Line ~169 |
| `the validation report does not contain rule "{rule_id}"` | ✅ EXISTING | Line ~177 |
| `the IXDS assembly receipt contains {count} member(s)` | ✅ EXISTING | Line ~185 |
| `the taxonomy resolution resolves at least {count} namespace(s)` | ✅ EXISTING | Line ~192 |
| `the report contains {count} fact(s)` | ✅ EXISTING | Line ~199 |
| **Then the sensor report is emitted** | ❌ MISSING | - |

---

## Missing Implementation Details

### 1. Given: `a validation report receipt`

**Current State:**
- No handler exists for this step
- The `World` struct has `execution: Option<ScenarioExecution>` which contains `validation_run: Option<ValidationRun>`
- Need to assert that a validation report exists (i.e., `validation_run` is Some)

**Implementation Notes:**
- This is a "setup assertion" step - it validates preconditions before the When step
- Should verify that `world.execution` exists AND contains a validation report
- May need to run validation first if not already executed (similar to other Given steps)

**Data Available:**
```rust
pub struct ScenarioExecution {
    pub validation_run: Option<ValidationRun>,
    pub taxonomy_resolution: Option<TaxonomyResolutionRun>,
    pub ixds_receipt: Option<Receipt>,
    pub export_receipt: Option<Receipt>,
}
```

### 2. When: `I package the receipt for cockpit`

**Current State:**
- No handler exists for this step
- There IS a `cockpit-export` crate at `crates/cockpit-export/src/lib.rs`
- The crate has: `pub fn to_sensor_report(sensor_id: &str, receipt: &Receipt) -> serde_json::Value`

**Implementation Notes:**
- Needs to take the validation report receipt and convert to sensor report format
- The cockpit-export function expects a `Receipt` from `receipt_types::Receipt`
- Need to determine how to extract the receipt from `ValidationRun`
- May need to extend `World` to store `sensor_report: Option<serde_json::Value>`

**Relevant Dependency:**
```rust
// crates/cockpit-export/src/lib.rs
pub fn to_sensor_report(sensor_id: &str, receipt: &Receipt) -> serde_json::Value {
    serde_json::json!({
      "kind": "sensor.report",
      "version": "v1",
      "subject": receipt.subject,
      "result": format!("{:?}", receipt.result).to_ascii_lowercase(),
      "sensor_id": sensor_id,
      "inner_receipt": receipt,
    })
}
```

### 3. Then: `the sensor report is emitted`

**Current State:**
- No handler exists for this step
- This would check that the sensor report was successfully created

**Implementation Notes:**
- Similar pattern to `the export report receipt is emitted`
- Would verify that `world.sensor_report` (new field) is `Some`
- Could optionally validate structure of the sensor report JSON

---

## World Struct Extension Needed

The `World` struct needs a new field to track the sensor report:

```rust
#[derive(Debug, Clone)]
pub struct World {
    pub repo_root: PathBuf,
    pub grid: FeatureGrid,
    pub profile_id: Option<String>,
    pub fixture_dirs: Vec<PathBuf>,
    pub execution: Option<ScenarioExecution>,
    pub sensor_report: Option<serde_json::Value>,  // NEW
}
```

---

## Recommended Implementation Order

1. **Add `sensor_report` field to `World`** - Required by both When and Then handlers
2. **Implement Given handler** - Validates validation report exists
3. **Implement When handler** - Calls `cockpit_export::to_sensor_report()`
4. **Implement Then handler** - Asserts sensor report was created

---

## Dependencies to Verify

- `cockpit-export` crate is available but may need to be added to `xbrlkit-bdd-steps/Cargo.toml` dependencies
- `receipt_types::Receipt` type needs to be accessible from the validation run

---

## Summary Table

| Step | Status | Complexity | Dependencies |
|------|--------|------------|--------------|
| `Given a validation report receipt` | MISSING | Low | Existing execution data |
| `When I package the receipt for cockpit` | MISSING | Medium | cockpit-export crate, Receipt extraction |
| `Then the sensor report is emitted` | MISSING | Low | World extension for sensor_report field |
