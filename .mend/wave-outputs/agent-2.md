# BDD Step Handler Analysis

## File Analyzed
`/tmp/xbrlkit-wt/test-003/crates/xbrlkit-bdd-steps/src/lib.rs`

## Step Handlers Search Results

### 1. "Given a validation report receipt"
**Status: NOT FOUND**

The `handle_given` function in `lib.rs` currently handles:
- `"the feature grid is compiled"`
- `"the profile pack \"...\"` (pattern match)
- `"the fixture directory \"...\"` or `"the fixture \"...\"` (pattern match)

**Conclusion:** No step handler exists for "Given a validation report receipt".

---

### 2. "When I package the receipt for cockpit"
**Status: NOT FOUND**

The `handle_when` function in `lib.rs` currently handles:
- `"I validate the filing"` | `"I validate duplicate facts"` | `"I resolve the DTS"`
- `"I export the canonical report to JSON"`
- `"I bundle the selector \"...\"` (pattern match)

**Conclusion:** No step handler exists for "When I package the receipt for cockpit".

---

### 3. "Then the sensor report is emitted"
**Status: NOT FOUND**

The `handle_then` function in `lib.rs` currently handles:
- `"the validation report has no error findings"`
- `"the taxonomy resolution succeeds"`
- `"the concept set is:"`
- `"the export report receipt is emitted"`

Plus parameterized assertions via `handle_parameterized_assertion` for:
- `"the bundle manifest lists scenario \"...\"`
- `"the validation report contains rule \"...\"`
- `"the validation report does not contain rule \"...\"`
- Count-based assertions (IXDS members, namespaces, facts)

**Conclusion:** No step handler exists for "Then the sensor report is emitted".

---

## Summary

| Step | Status |
|------|--------|
| Given a validation report receipt | ❌ Missing |
| When I package the receipt for cockpit | ❌ Missing |
| Then the sensor report is emitted | ❌ Missing |

All three requested step handlers are **not implemented** in the current `lib.rs`.
