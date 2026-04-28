# Plan: Move Selector Matching Logic to scenario_contract Crate

**Stream:** Code Quality / Refactoring
**Issue:** #213
**Date:** 2026-04-21
**Status:** 🔨 Ready for Build

---

## Overview

Selector matching logic for scenario selection is currently duplicated between `xtask/src/main.rs` and `crates/xbrlkit-bdd-steps/src/utils.rs`. Both files implement nearly identical `selector_matches()` and `select_matching_scenarios()` functions that operate on `FeatureGrid` and `ScenarioRecord` types defined in the `scenario-contract` crate.

This plan moves the logic to the `scenario-contract` crate as methods on the types themselves, eliminating duplication and making the logic discoverable via IDE autocomplete.

---

## Acceptance Criteria Breakdown

| # | Criterion | Location | Verification |
|---|-----------|----------|------------|
| 1 | Add `matches_selector()` method to `ScenarioRecord` | `crates/scenario-contract/src/lib.rs` | Unit tests in same crate |
| 2 | Add `select_by_selector()` method to `FeatureGrid` | `crates/scenario-contract/src/lib.rs` | Unit tests in same crate |
| 3 | Replace implementation in `xtask` | `xtask/src/main.rs` | Delete `selector_matches` and `select_matching_scenarios` functions; call methods on types instead |
| 4 | Replace implementation in `xbrlkit-bdd-steps` | `crates/xbrlkit-bdd-steps/src/utils.rs` | Delete `selector_matches` and `select_matching_scenarios` functions; call methods on types instead |
| 5 | Add unit tests for selector matching logic | `crates/scenario-contract/src/lib.rs` | Test all match variants: `scenario_id`, `ac_id`, `req_id`, `@scenario_id`, `@ac_id`, no-match |
| 6 | Verify all existing usage continues to work | Full test suite | `cargo test` passes across workspace |

---

## Proposed Approach

### Phase 1: Add Methods to `scenario-contract`

Add two methods to `crates/scenario-contract/src/lib.rs`:

```rust
impl ScenarioRecord {
    pub fn matches_selector(&self, selector: &str) -> bool {
        self.scenario_id == selector
            || self.ac_id.as_deref() == Some(selector)
            || self.req_id.as_deref() == Some(selector)
            || format!("@{}", self.scenario_id) == selector
            || self.ac_id
                .as_ref()
                .is_some_and(|ac| format!("@{ac}") == selector)
    }
}

impl FeatureGrid {
    pub fn select_by_selector(&self, selector: &str) -> Vec<ScenarioRecord> {
        self.scenarios
            .iter()
            .filter(|s| s.matches_selector(selector))
            .cloned()
            .collect()
    }
}
```

**Why `scenario-contract`?**
- The types (`ScenarioRecord`, `FeatureGrid`) are defined there
- Pure logic with zero external dependencies
- Used by both `xtask` and `xbrlkit-bdd-steps` (and others)
- The crate currently has no tests — adding tests here improves coverage

### Phase 2: Remove Duplication from `xtask`

In `xtask/src/main.rs`:
- Delete `selector_matches()` function (lines ~249–258)
- Delete `select_matching_scenarios()` function (lines ~241–247)
- Update call sites at lines 112 and 150 to use `grid.select_by_selector(selector)`
- Update test imports to use `FeatureGrid::select_by_selector` or method syntax

### Phase 3: Remove Duplication from `xbrlkit-bdd-steps`

In `crates/xbrlkit-bdd-steps/src/utils.rs`:
- Delete `selector_matches()` function (lines ~67–75)
- Delete `select_matching_scenarios()` function (lines ~58–65)
- Update call site in `when.rs` line 199 to use `world.grid.select_by_selector(&selector)`

### Phase 4: Add Unit Tests

Add `#[cfg(test)]` module to `crates/scenario-contract/src/lib.rs` with tests covering:

| Selector Pattern | Expected Match |
|------------------|----------------|
| Exact `scenario_id` | ✅ |
| Exact `ac_id` | ✅ |
| Exact `req_id` | ✅ |
| `@` + `scenario_id` (tag syntax) | ✅ |
| `@` + `ac_id` (tag syntax) | ✅ |
| Non-existent ID | ❌ |
| Partial match | ❌ |

---

## Files to Modify

| File | Action | Lines |
|------|--------|-------|
| `crates/scenario-contract/src/lib.rs` | Add `matches_selector()` and `select_by_selector()` methods; add `#[cfg(test)]` module | ~50 new lines |
| `xtask/src/main.rs` | Delete `selector_matches` and `select_matching_scenarios`; update call sites and tests | ~−25 lines |
| `crates/xbrlkit-bdd-steps/src/utils.rs` | Delete `selector_matches` and `select_matching_scenarios` | ~−20 lines |
| `crates/xbrlkit-bdd-steps/src/when.rs` | Update call to use method syntax | 1 line |

**No new files to create.**

---

## Test Strategy

### Unit Tests (New)

Add to `crates/scenario-contract/src/lib.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn test_scenario() -> ScenarioRecord {
        ScenarioRecord {
            scenario_id: "SCN-XK-WORKFLOW-002".to_string(),
            ac_id: Some("AC-XK-WORKFLOW-002".to_string()),
            req_id: Some("REQ-XK-WORKFLOW".to_string()),
            feature_file: "specs/features/workflow/bundle.feature".to_string(),
            sidecar_file: "specs/features/workflow/bundle.meta.yaml".to_string(),
            layer: "workflow".to_string(),
            module: "bundle".to_string(),
            crates: vec!["xtask".to_string()],
            fixtures: Vec::new(),
            profile_pack: None,
            receipts: vec!["bundle.manifest.v1".to_string()],
            allowed_edit_roots: vec!["specs/features/workflow".to_string(), "xtask".to_string()],
            suite: Some("synthetic".to_string()),
            speed: Some("fast".to_string()),
        }
    }

    #[test]
    fn matches_scenario_id() {
        assert!(test_scenario().matches_selector("SCN-XK-WORKFLOW-002"));
    }

    #[test]
    fn matches_ac_id() {
        assert!(test_scenario().matches_selector("AC-XK-WORKFLOW-002"));
    }

    #[test]
    fn matches_req_id() {
        assert!(test_scenario().matches_selector("REQ-XK-WORKFLOW"));
    }

    #[test]
    fn matches_scenario_id_tag() {
        assert!(test_scenario().matches_selector("@SCN-XK-WORKFLOW-002"));
    }

    #[test]
    fn matches_ac_id_tag() {
        assert!(test_scenario().matches_selector("@AC-XK-WORKFLOW-002"));
    }

    #[test]
    fn does_not_match_nonexistent() {
        assert!(!test_scenario().matches_selector("AC-XK-DOES-NOT-EXIST"));
    }

    #[test]
    fn select_by_selector_returns_matching() {
        let grid = FeatureGrid {
            scenarios: vec![test_scenario()],
        };
        assert_eq!(grid.select_by_selector("AC-XK-WORKFLOW-002").len(), 1);
        assert!(grid.select_by_selector("NONEXISTENT").is_empty());
    }
}
```

### Regression Tests (Existing)

- `xtask` tests: `selector_matching_supports_ids_and_tags` → update to call method directly
- Full workspace test: `cargo test --workspace` to verify no breakage

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Call sites missed during refactor | Low | Medium | `grep -r "select_matching_scenarios\|selector_matches"` across entire repo before committing |
| Method naming conflicts with future API | Low | Low | Names (`matches_selector`, `select_by_selector`) are descriptive and unlikely to conflict |
| `xtask` tests break due to import changes | Low | Low | Tests are in the same file; update imports alongside deletions |
| `format!` in hot path (performance) | Very Low | Low | `format!` is already used in both existing implementations; no regression |

**Overall risk: LOW** — this is a pure refactor with no behavioral changes.

---

## Estimated Effort

- **Implementation:** 30 minutes (mechanical refactor)
- **Testing:** 20 minutes (write unit tests + run full suite)
- **Review/iteration:** 15 minutes
- **Total:** ~1 hour

---

## Branch Strategy

```
mend/issue-213-selector-matching
├── Commit 1: Add matches_selector() and select_by_selector() to scenario-contract + unit tests
├── Commit 2: Replace xtask implementation with method calls
├── Commit 3: Replace xbrlkit-bdd-steps implementation with method calls
└── Commit 4: Verify full test suite passes
```

---

## Notes

- The `scenario-contract` crate currently has zero tests. This change adds the first test coverage to the crate.
- Both `xtask` and `xbrlkit-bdd-steps` already depend on `scenario-contract`, so no dependency changes are needed.
- The `format!("@{ac}")` syntax (implicit named argument) is used in the existing `xtask` implementation but not in `xbrlkit-bdd-steps`. The plan preserves the `xtask` style as the canonical implementation since it's slightly more idiomatic Rust 2024.
