# Plan: Activate SCN-XK-WORKFLOW-002: Bundle AC into bounded context packet

**Issue:** [#113](https://github.com/EffortlessMetrics/xbrlkit/issues/113)  
**Agent:** planner-initial  
**Date:** 2026-03-31

---

## Overview

This plan activates scenario SCN-XK-WORKFLOW-002 ("Bundle an AC into a bounded context packet") for alpha testing by adding the AC-XK-WORKFLOW-002 entry to the ACTIVE_ALPHA_ACS list in the alpha check system.

The scenario and its step handlers are already implemented and working with `@alpha-active` tags. The only missing piece is registering AC-XK-WORKFLOW-002 in the ACTIVE_ALPHA_ACS constant so that `cargo alpha-check` will execute the BDD-based test for this acceptance criterion.

---

## Acceptance Criteria Breakdown

| # | Criterion | Current Status | Required Action |
|---|-----------|----------------|-----------------|
| 1 | Add @alpha-active tag to scenario | ✅ **COMPLETE** - Both SCN-XK-WORKFLOW-002 and SCN-XK-WORKFLOW-004 have @alpha-active | None |
| 2 | Implement step handlers | ✅ **COMPLETE** - All 4 step handlers exist in xbrlkit-bdd-steps/src/lib.rs | None |
| 3 | Add AC-XK-WORKFLOW-002 to ACTIVE_ALPHA_ACS | ❌ **MISSING** - Not in the list, only has a comment | Add to list |
| 4 | Add assertion in assert_scenario_outcome | ✅ **COMPLETE** - Handler exists in scenario-runner/src/lib.rs | None |
| 5 | Local alpha-check passes | ⏳ **PENDING** - Depends on criterion #3 | Verify after changes |
| 6 | PR created and merged | ⏳ **PENDING** | Create PR after verification |

### Step Handlers Status

All required BDD step handlers are already implemented:

- **Given the feature grid is compiled** - Implemented in `handle_given()` (line ~675)
- **When I bundle the selector "..."** - Implemented in `handle_when()` (line ~875)
- **Then the bundle manifest lists scenario "..."** - Implemented in `handle_parameterized_assertion()` (line ~1231)
- **Then bundling fails because no scenario matches** - Implemented in `handle_then()` (line ~1085)

---

## Proposed Approach

### Phase 1: Add AC-XK-WORKFLOW-002 to ACTIVE_ALPHA_ACS

Modify `xtask/src/alpha_check.rs` to add "AC-XK-WORKFLOW-002" to the ACTIVE_ALPHA_ACS list.

**Current state:**
```rust
const ACTIVE_ALPHA_ACS: &[&str] = &[
    // ... other ACs ...
    "AC-XK-WORKFLOW-003", // tested via @alpha-active BDD tag for sensor report
    // AC-XK-WORKFLOW-002 and AC-XK-MANIFEST-001 tested via @alpha-active BDD tags
];
```

**Proposed change:**
```rust
const ACTIVE_ALPHA_ACS: &[&str] = &[
    // ... other ACs ...
    "AC-XK-WORKFLOW-002", // tested via @alpha-active BDD tag for bundle
    "AC-XK-WORKFLOW-003", // tested via @alpha-active BDD tag for sensor report
    // AC-XK-MANIFEST-001 tested via @alpha-active BDD tag
];
```

### Phase 2: Verify alpha-check passes

Run the alpha check to ensure the scenario executes correctly:

```bash
cargo alpha-check
```

Expected output: 17 scenarios selected for @alpha-active (was 16)

### Phase 3: Create PR

Create a micro PR with the single-line change.

---

## Files to Modify/Create

### Modify

| File | Lines | Description |
|------|-------|-------------|
| `xtask/src/alpha_check.rs` | ~28 | Add "AC-XK-WORKFLOW-002" to ACTIVE_ALPHA_ACS array, update comment |

### No Changes Required (Already Complete)

| File | Status |
|------|--------|
| `specs/features/workflow/bundle.feature` | ✅ @alpha-active tags present |
| `crates/xbrlkit-bdd-steps/src/lib.rs` | ✅ All step handlers implemented |
| `crates/scenario-runner/src/lib.rs` | ✅ assert_scenario_outcome handles AC-XK-WORKFLOW-002 |

---

## Test Strategy

### Pre-merge Testing

1. **Unit test:** Verify the AC is selectable
   ```bash
   cargo test -p xbrlkit-bdd-steps
   ```

2. **Integration test:** Run alpha check
   ```bash
   cargo alpha-check
   ```
   - Expected: All steps pass
   - Expected: bdd:@alpha-active step shows 17 scenarios (was 16)

3. **BDD test:** Verify scenarios execute
   ```bash
   cargo test -p xbrlkit-bdd -- --test-threads=1
   ```

### Post-merge Verification

- CI pipeline alpha-check job should pass
- Monitor for any flaky test behavior

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Alpha check fails due to incomplete fixtures | Low | Medium | Step handlers already tested and working |
| BDD step handler regression | Low | Low | Step handlers are stable, used by other tests |
| Performance impact on alpha-check | Negligible | Low | Bundle scenarios are @speed.fast, minimal overhead |
| Comment typo or formatting | Low | Negligible | Follow existing code style |

**Overall Risk Level: LOW**

The implementation is minimal (single line addition) and all supporting infrastructure is already in place and tested.

---

## Estimated Effort

| Task | Estimate |
|------|----------|
| Add AC-XK-WORKFLOW-002 to ACTIVE_ALPHA_ACS | 5 minutes |
| Update comment | 2 minutes |
| Local testing (cargo alpha-check) | 5 minutes |
| PR creation | 5 minutes |
| **Total** | **~15-20 minutes** |

---

## Dependencies

- ✅ Step handlers (xbrlkit-bdd-steps) - Complete
- ✅ Scenario definition (bundle.feature) - Complete  
- ✅ assert_scenario_outcome handler - Complete
- ⏳ This plan must be approved by reviewer-plan agent

---

## Notes

### Related Implementation Notes

From `.mend/notes/bundle-scenarios.md`:
> The implementation is complete. The bundle scenarios SCN-XK-WORKFLOW-002 and SCN-XK-WORKFLOW-004 both pass with @alpha-active tags.
> The only remaining task is to add AC-XK-WORKFLOW-002 to ACTIVE_ALPHA_ACS to complete the activation.

### AC-XK-MANIFEST-001

This AC is also mentioned in the comment as being tested via @alpha-active BDD tag, but is not part of this issue. It should remain as a comment or be activated separately.

---

## Plan Status: DRAFT

Awaiting review by `reviewer-plan` agent.
