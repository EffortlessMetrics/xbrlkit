# Implementation Plan: ISSUE-118

## Overview

Activate scenario **SCN-XK-WORKFLOW-003** from `specs/features/workflow/cockpit_pack.feature` for alpha testing. This scenario validates the "cockpit pack" workflow that wraps a validation report receipt into a `sensor.report.v1` receipt.

### Background

The scenario tests the cockpit-export functionality which converts validation report receipts into sensor report format for external monitoring/integration. This is part of the workflow layer for the xbrlkit system.

## Current State Analysis

Based on code review, the following components are **already in place**:

| Component | Status | Location |
|-----------|--------|----------|
| Feature file with scenario | ✅ Complete | `specs/features/workflow/cockpit_pack.feature` |
| @alpha-active tag | ✅ Present | Feature file line 6 |
| @AC-XK-WORKFLOW-003 tag | ✅ Present | Feature file line 5 |
| Step handlers (Given/When/Then) | ✅ Implemented | `crates/xbrlkit-bdd-steps/src/lib.rs` |
| ACTIVE_ALPHA_ACS entry | ✅ Present | `xtask/src/alpha_check.rs` |
| assert_scenario_outcome assertion | ✅ Implemented | `crates/scenario-runner/src/lib.rs` |

### Step Handler Implementation Details

**Given:** `a validation report receipt` (lib.rs:319)
- Creates a synthetic validation receipt with `RunResult::Success`

**When:** `I package the receipt for cockpit` (lib.rs:872)
- Calls `cockpit_export::to_sensor_report("xbrlkit", receipt)`
- Stores result in `world.sensor_report`

**Then:** `the sensor report is emitted` (lib.rs:1155)
- Verifies `world.sensor_report.is_some()`

**AC Assertion:** (scenario-runner:276-281)
```rust
Some("AC-XK-WORKFLOW-003") => {
    if execution.sensor_receipt.is_none() {
        anyhow::bail!("sensor report receipt was not emitted");
    }
    Ok(())
}
```

## Acceptance Criteria Breakdown

- [x] Add @alpha-active tag to scenario
- [x] Define AC (e.g., AC-XK-WORKFLOW-003)
- [x] Implement step handlers: `Given a validation report receipt`, `When I package the receipt for cockpit`, `Then the sensor report is emitted`
- [x] Add AC to ACTIVE_ALPHA_ACS
- [x] Add assertion in assert_scenario_outcome
- [ ] Local alpha-check passes
- [ ] PR created and merged

## Proposed Approach

### Phase 1: Verification (5 minutes)
1. Run `cargo xtask alpha-check` to verify all components work together
2. Confirm BDD scenario executes successfully with @alpha-active tag
3. Verify sensor.report.v1 receipt is emitted correctly

### Phase 2: Documentation Update (10 minutes)
1. Update scenario metadata if needed
2. Ensure cockpit_pack.meta.yaml reflects correct receipts and crates

### Phase 3: PR Creation (15 minutes)
1. Create feature branch `activate-scn-xk-workflow-003`
2. Commit any necessary changes
3. Push and create PR with description
4. Link PR to issue #118

## Files to Modify/Create

### Verification Only (No Changes Expected)
| File | Purpose |
|------|---------|
| `specs/features/workflow/cockpit_pack.feature` | Verify tags present |
| `crates/xbrlkit-bdd-steps/src/lib.rs` | Verify step handlers exist |
| `xtask/src/alpha_check.rs` | Verify AC in ACTIVE_ALPHA_ACS |
| `crates/scenario-runner/src/lib.rs` | Verify assertion exists |

### Potential Updates
| File | Change |
|------|--------|
| `specs/features/workflow/cockpit_pack.meta.yaml` | Review/update if metadata stale |

## Test Strategy

### Alpha Check
```bash
cargo xtask alpha-check
```

Expected output:
- `test-ac:AC-XK-WORKFLOW-003` - passed
- `bdd:@alpha-active` - passed (includes SCN-XK-WORKFLOW-003)
- All golden file comparisons pass

### BDD Direct Run
```bash
cargo test -p xbrlkit-bdd-steps --features alpha-active
```

### Manual Verification
```bash
cargo run -- -p xbrlkit-cli validate-fixture --json <fixture>
# Verify sensor.report.v1 in output
```

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Step handler dependencies not satisfied | Low | Medium | Verify cockpit-export crate builds |
| Sensor report schema mismatch | Low | High | Check receipt-types version compatibility |
| Alpha-check environment issues | Medium | Low | Run in clean environment |
| Missing crate dependencies | Low | Medium | Review Cargo.toml for cockpit-export |

**Overall Risk Level:** LOW

All core components are already implemented. Primary risk is environment-specific issues during verification.

## Estimated Effort

- **Verification:** 5-10 minutes
- **Documentation review:** 10 minutes
- **PR creation:** 15 minutes
- **Total:** 30-45 minutes (micro PR)

## Notes

1. The implementation appears complete based on code review. This plan focuses on verification and activation.
2. The `cockpit-export` crate dependency should be verified as part of alpha-check.
3. Receipt type: `sensor.report.v1` is the expected output.
4. Related crates: cockpit-export, receipt-types, xtask

## Dependencies

- `cockpit-export` crate must be buildable
- `receipt-types` crate for Receipt structures
- `xbrlkit-bdd` and `xbrlkit-bdd-steps` for BDD execution

## Deep Review (2026-03-31)

**Reviewer:** reviewer-deep-plan agent  
**Status:** PASS ✅

### Edge Cases Analyzed
| Edge Case | Status |
|-----------|--------|
| Synthetic vs real validation receipt | ✅ Handled (synthetic appropriate for unit test) |
| Missing validation_receipt in When step | ✅ Handled (anyhow context propagation) |
| cockpit-export crate failure | ✅ Handled (dependency verification in plan) |
| Sensor report schema version mismatch | ✅ Handled (receipt-types crate consistency) |

### Risk Assessment Reviewed
| Risk | Severity | Mitigation Verified |
|------|----------|---------------------|
| Environment issues during verification | Low-Medium | Clean environment recommendation present |
| Integration failure between crates | Low | Components pre-built and tested |
| Schema drift in sensor.report.v1 | Medium | Typed Receipt structure used |
| Test coverage gap (content validation) | Medium | Noted as watch item |

### Alternatives Considered
1. Enhanced content validation in Then step — **Deferred** (aligns with AC definition)
2. Fixture-based vs synthetic receipt — **Rejected** (synthetic appropriate for unit test)
3. Full cockpit endpoint integration — **Out of scope** (alpha focuses on receipt generation)

### Watch Items for Implementation
1. Content validation depth — Then step only checks existence
2. Error diagnostic clarity if alpha-check fails
3. Receipt type versioning coordination

### Roadmap Alignment
- ✅ Contributes to Phase 2 workflow infrastructure goal
- ✅ Supports observability via sensor.report.v1
- ⚠️ Ensure metadata documentation in Wave 3

**Next Agent:** reviewer-repo-alignment

## References

- Feature file: `specs/features/workflow/cockpit_pack.feature`
- Meta file: `specs/features/workflow/cockpit_pack.meta.yaml`
- BDD steps: `crates/xbrlkit-bdd-steps/src/lib.rs`
- Alpha check: `xtask/src/alpha_check.rs`
- Scenario runner: `crates/scenario-runner/src/lib.rs`
