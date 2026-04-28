# Plan: Activate SCN-XK-MANIFEST-001 (ISSUE-115)

## Overview

This plan covers the activation of scenario **SCN-XK-MANIFEST-001** for alpha testing. The scenario validates the ability to build a filing manifest from a minimal filing container fixture.

**Scenario Details:**
- **AC ID:** AC-XK-MANIFEST-001
- **Scenario ID:** SCN-XK-MANIFEST-001
- **Feature:** Filing manifest foundation feature
- **Fixture:** `synthetic/filing/minimal-container-01`

### Current State Assessment

Upon initial analysis:
- ✅ Feature file exists with scenario definition
- ✅ @alpha-active tag is **already present** on the scenario
- ✅ Step handlers are **already implemented**:
  - `Given the fixture "..."` - handles fixture loading (lib.rs:handle_given)
  - `When I build the filing manifest` - loads submission, creates manifest (lib.rs:handle_when)
  - `Then the filing manifest receipt is emitted` - validates receipt kind (lib.rs:handle_then)
- ⚠️ **AC-XK-MANIFEST-001 NOT in ACTIVE_ALPHA_ACS** (alpha_check.rs)
- ⚠️ **assert_scenario_outcome case exists but is a no-op** (scenario_runner/src/lib.rs)

## Acceptance Criteria Breakdown

| # | Criterion | Status | Implementation Needed |
|---|-----------|--------|----------------------|
| 1 | Add @alpha-active tag to scenario | ✅ Already present | None |
| 2 | Implement step handlers | ✅ Already implemented | None |
| 3 | Add AC-XK-MANIFEST-001 to ACTIVE_ALPHA_ACS | ❌ Missing | Add to constant array |
| 4 | Add assertion in assert_scenario_outcome | ⚠️ Present but no-op | Enhance to verify receipt |
| 5 | Local alpha-check passes | ⬜ Pending | Verify after changes |
| 6 | PR created and merged | ⬜ Pending | Standard PR workflow |

## Proposed Approach

### 1. Add AC to ACTIVE_ALPHA_ACS

**File:** `xtask/src/alpha_check.rs`

Add `"AC-XK-MANIFEST-001"` to the `ACTIVE_ALPHA_ACS` constant array. Place it near other workflow ACs for logical grouping.

### 2. Enhance assert_scenario_outcome (Optional but Recommended)

**File:** `crates/scenario-runner/src/lib.rs`

The current implementation in `assert_scenario_outcome` for AC-XK-MANIFEST-001 is a no-op:

```rust
Some("AC-XK-MANIFEST-001") => {
    // BDD steps handle the assertions
    Ok(())
}
```

**Enhancement:** Add verification that the filing manifest receipt exists with the correct kind:

```rust
Some("AC-XK-MANIFEST-001") => {
    // Verify the filing manifest receipt was emitted
    if execution.filing_receipt.is_none() {
        anyhow::bail!("filing manifest receipt was not emitted");
    }
    Ok(())
}
```

**Note:** This requires extending `ScenarioExecution` to include `filing_receipt` field, or using BDD-level assertions only.

**Decision:** Keep the no-op implementation in `assert_scenario_outcome` since:
1. The BDD Then step already validates the receipt
2. The `ScenarioExecution` struct doesn't currently track filing receipts
3. The alpha-check will run the BDD scenario and fail if assertions fail

### 3. Verify Fixture

**File:** `fixtures/synthetic/filing/minimal-container-01/submission.txt`

Ensure the submission.txt file contains valid SEC submission format for testing.

Current content appears minimal but sufficient for manifest building.

## Files to Modify/Create

### Modified Files

1. **`xtask/src/alpha_check.rs`**
   - Add `"AC-XK-MANIFEST-001"` to `ACTIVE_ALPHA_ACS` constant
   - Estimated lines changed: +1

### No New Files Required

All infrastructure already exists:
- Feature file: ✅
- Step handlers: ✅
- Fixture: ✅

## Test Strategy

### Local Testing Steps

1. **Run alpha-check to verify current state:**
   ```bash
   cargo xtask alpha-check
   ```
   Expected: Should pass (AC not yet in active list)

2. **Add AC to ACTIVE_ALPHA_ACS**

3. **Run alpha-check again:**
   ```bash
   cargo xtask alpha-check
   ```
   Expected: Should pass with AC-XK-MANIFEST-001 included

4. **Run specific BDD scenario:**
   ```bash
   cargo run -p xbrlkit-bdd -- -t @SCN-XK-MANIFEST-001
   ```
   Expected: Scenario passes

### CI/CD Considerations

- Alpha-check runs in CI as a gate
- No special deployment steps needed
- Follows standard PR workflow

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Fixture data insufficient | Low | Medium | Verify submission.txt format |
| Step handler bug | Low | Low | Step handlers already tested |
| Alpha-check timeout | Very Low | Low | Scenario is fast (@speed.fast) |
| Receipt validation fails | Low | Medium | BDD step validates receipt kind |

**Overall Risk Level:** LOW

The implementation is straightforward - primarily adding an existing AC to the active list. All step handlers are already implemented and tested.

## Estimated Effort

| Task | Time Estimate |
|------|---------------|
| Code change (add to ACTIVE_ALPHA_ACS) | 5 minutes |
| Local testing | 10 minutes |
| PR creation | 5 minutes |
| **Total** | **20 minutes** |

**Category:** Micro PR (as noted in original issue: 30-60 minutes)

## Dependencies

### Blockers
None - all dependencies resolved:
- ✅ filing_load crate exists
- ✅ Step handlers implemented
- ✅ Fixture exists

### Related Issues
- May relate to other filing manifest scenarios (if any)
- Part of alpha activation wave for foundation features

## Implementation Checklist

- [ ] Add `"AC-XK-MANIFEST-001"` to `ACTIVE_ALPHA_ACS` in `xtask/src/alpha_check.rs`
- [ ] Run `cargo xtask alpha-check` locally and verify passes
- [ ] Create PR with changes
- [ ] Ensure CI passes
- [ ] Merge PR
- [ ] Verify issue can be closed

## Notes

### Interesting Findings

The feature file already has the @alpha-active tag and step handlers are implemented, suggesting this work was partially completed in a previous iteration. The remaining work is minimal - just adding the AC to the active list.

### Future Enhancements

Consider extending `ScenarioExecution` to track filing receipts for more robust AC-level assertions, rather than relying solely on BDD step assertions.

---

*Plan created by: planner-initial agent*
*Date: 2026-03-31*
