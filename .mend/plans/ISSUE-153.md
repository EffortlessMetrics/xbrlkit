# Plan: [FR-010] Extract duplicate sanitize_for_rule_id utility

**Issue:** #153  
**Status:** plan-draft  
**Created:** 2026-04-22  
**Agent:** planner-initial

---

## Overview

The `sanitize_for_rule_id` utility function is duplicated identically in two validation rule crates. This plan extracts it to a single shared location, updates both call sites, and preserves all existing behavior and tests.

## Current State

### Duplicate Implementations

**Location 1:** `crates/numeric-rules/src/lib.rs` (lines 114–124)

```rust
fn sanitize_for_rule_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}
```

**Location 2:** `crates/efm-rules/src/lib.rs` (lines 123–133)

Identical implementation — 11 lines of duplicated logic.

### Note on Issue Description

The issue description lists `xtask/src/main.rs` (lines 297–307) as the second duplicate location. Upon codebase inspection, `xtask/src/main.rs` does **not** contain `sanitize_for_rule_id`; it contains a different `sanitize()` function with distinct logic (preserves `-` and `_`, does not uppercase). The actual duplicate is in `efm-rules/src/lib.rs`.

## Proposed Approach

### Shared Location: `xbrl-report-types`

Both `numeric-rules` and `efm-rules` already declare `xbrl-report-types` as a workspace dependency. Adding the utility there requires **zero new dependency edges** and avoids circular dependency issues.

> **Why not `xbrlkit-core`?** The issue suggests `xbrlkit-core` as preferred, but `xbrlkit-core` is a facade crate that re-exports from lower-level crates including `xbrl-report-types`. Adding `xbrlkit-core` as a dependency to `numeric-rules` or `efm-rules` would create a dependency cycle.

### Implementation Steps

1. **Add utility to `xbrl-report-types`**
   - Add `pub fn sanitize_for_rule_id(value: &str) -> String` to `crates/xbrl-report-types/src/lib.rs`
   - Consider adding a `util` module if future utilities are anticipated; inline is acceptable for a single function

2. **Update `numeric-rules`**
   - Remove local `sanitize_for_rule_id` definition from `crates/numeric-rules/src/lib.rs`
   - Add `use xbrl_report_types::sanitize_for_rule_id;` (or rely on existing `use xbrl_report_types::*` if present)

3. **Update `efm-rules`**
   - Remove local `sanitize_for_rule_id` definition from `crates/efm-rules/src/lib.rs`
   - Ensure `xbrl_report_types::sanitize_for_rule_id` is accessible

## Files to Modify

| File | Action |
|------|--------|
| `crates/xbrl-report-types/src/lib.rs` | Add `pub fn sanitize_for_rule_id` |
| `crates/numeric-rules/src/lib.rs` | Remove local `fn sanitize_for_rule_id` |
| `crates/efm-rules/src/lib.rs` | Remove local `fn sanitize_for_rule_id` |

**Total:** 0 new files, 3 modified files.

## Test Strategy

- **Behavior preservation:** No functional changes. Existing tests in `numeric-rules` and `efm-rules` that assert on rule ID formatting continue to pass unchanged.
- **Compilation gate:** `cargo check` across `numeric-rules` and `efm-rules` must pass.
- **Test gate:** `cargo test -p numeric-rules` and `cargo test -p efm-rules` must pass.

## Risk Assessment

| Risk | Level | Mitigation |
|------|-------|------------|
| Breaking rule ID format | Low | Function body is byte-for-byte identical; no behavior change |
| Import resolution conflict | Low | `xbrl-report-types` is already a dependency of both crates |
| Issue description mismatch | Low | Issue mentions `xtask` but actual duplicate is in `efm-rules`; plan covers the real duplication |

## Estimated Effort

**Small** — single-function extraction with two call-site updates. Expected implementation + verification: < 30 minutes.

## Acceptance Criteria

- [ ] Single source of truth for `sanitize_for_rule_id` in `xbrl-report-types`
- [ ] `numeric-rules` uses shared version (local definition removed)
- [ ] `efm-rules` uses shared version (local definition removed)
- [ ] All existing tests pass without modification
- [ ] No functional changes to rule ID formatting behavior

---

## plan-reviewed

**Reviewer:** reviewer-plan agent  
**Date:** 2026-04-22  
**Verdict:** PASS (with minor suggestions)

### Findings
- Target location (`xbrl-report-types`) is correct — avoids dependency cycle that `xbrlkit-core` would create
- Codebase analysis is accurate — duplicate confirmed in `efm-rules`, `xtask` function is correctly identified as different logic
- Scope is appropriately bounded
- Risk assessment is honest and low

### Suggestions (non-blocking)
1. Add unit test for extracted function in `xbrl-report-types` to prevent future regressions
2. Consider `util` module if more shared utilities are anticipated

### Gaps — none material
No blockers, no missing acceptance criteria, no dependency issues.

### Label Actions
- `plan-reviewed` added to issue #153
- `planning-in-progress` removed from issue #153

---

*Next step: Implementation → assign to implementation agent*
