# Plan: Refactor BDD Step Handlers — Registry Pattern (Issue #210)

## Overview

This plan addresses Issue #210, which identifies excessive complexity in the BDD step handler functions within `crates/xbrlkit-bdd-steps`. The current implementation uses monolithic `if-let` match chains in `handle_given()`, `handle_when()`, `handle_then()`, and `handle_parameterized_assertion()` — each exceeding Clippy's `too_many_lines` threshold (50 lines). Four `#[allow(clippy::too_many_lines)]` suppressions mask this structural debt.

The refactor replaces these imperative match chains with a **registry pattern**: each step pattern is registered with a discrete handler function. This eliminates the suppressions, improves testability, and makes the supported step vocabulary discoverable by reading the registration code rather than parsing 400-line match chains.

## Acceptance Criteria Breakdown

### AC-210-001: Remove all `too_many_lines` suppressions
- All four `#[allow(clippy::too_many_lines)]` attributes are removed from `crates/xbrlkit-bdd-steps/src/lib.rs`.
- `cargo clippy --package xbrlkit-bdd-steps` passes without warnings.

### AC-210-002: Implement registry pattern for Given handlers
- A `StepRegistry` type supports registering pattern strings with handler closures.
- `handle_given()` is refactored into ~25 discrete handler functions, each under 50 lines.
- Given handlers are registered in a `build_given_registry()` function.

### AC-210-003: Implement registry pattern for When handlers
- `handle_when()` is refactored into ~15 discrete handler functions, each under 50 lines.
- When handlers are registered in a `build_when_registry()` function.

### AC-210-004: Implement registry pattern for Then handlers
- `handle_then()` and `handle_parameterized_assertion()` are refactored into ~20 discrete handler functions, each under 50 lines.
- Then handlers are registered in a `build_then_registry()` function.

### AC-210-005: Existing BDD tests pass without modification
- All `cargo test --workspace` BDD scenarios continue to pass.
- No `.feature` files or scenario metadata are changed.
- The external contract of `run_scenario()` and `World` remains unchanged.

### AC-210-006: Add unit tests for at least 3 handlers
- At least 3 individual handler functions have standalone unit tests exercising their logic in isolation (no full scenario execution required).
- Tests live in `crates/xbrlkit-bdd-steps/src/` inline modules or a new `tests/` directory.

## Proposed Approach

### Registry Design

```rust
// src/registry.rs
use std::sync::Arc;

type StepHandler = Arc<dyn Fn(&mut World, &ScenarioRecord, &Step) -> anyhow::Result<bool>>;

pub struct StepRegistry {
    handlers: Vec<(String, StepHandler)>,
}

impl StepRegistry {
    pub fn new() -> Self {
        Self { handlers: Vec::new() }
    }

    pub fn register<F>(&mut self, pattern: &str, handler: F)
    where
        F: Fn(&mut World, &ScenarioRecord, &Step) -> anyhow::Result<bool> + 'static,
    {
        self.handlers.push((pattern.to_string(), Arc::new(handler)));
    }

    pub fn handle(
        &self,
        world: &mut World,
        scenario: &ScenarioRecord,
        step: &Step,
    ) -> anyhow::Result<bool> {
        for (pattern, handler) in &self.handlers {
            if step_matches(pattern, &step.text) {
                return handler(world, scenario, step);
            }
        }
        Ok(false) // not handled
    }
}

fn step_matches(pattern: &str, text: &str) -> bool {
    // Exact match for now; parameterized patterns use strip_prefix in the handler
    pattern == text
}
```

**Why `Arc<dyn Fn>` over `Box<dyn Fn>`?** `Arc` enables cheap cloning of the registry if needed for parallel test execution; for this single-threaded use case, either works. The issue description uses `Box`; we'll use `Arc` for forward compatibility.

**Why closures instead of trait objects?** Each handler captures different `World` state mutation patterns. A closure captures the mutable borrow naturally. A trait-based approach would require `&mut World` on every method call, which is equivalent but more boilerplate.

### File Layout

Current state: all logic in a single 1606-line `src/lib.rs`.

Post-refactor:

```
crates/xbrlkit-bdd-steps/src/
├── lib.rs              # World, Step, run_scenario, run_step, registry wiring (~200 lines)
├── registry.rs         # StepRegistry, StepHandler, step_matches (~60 lines)
├── given.rs            # Given handler functions + build_given_registry() (~300 lines)
├── when.rs             # When handler functions + build_when_registry() (~250 lines)
├── then.rs             # Then handler functions + build_then_registry() (~350 lines)
├── handlers/
│   └── mod.rs          # (optional) shared handler utilities if needed
└── tests/              # (new) unit tests for individual handlers
    └── handler_tests.rs
```

### Handler Decomposition Strategy

Each current `if let Some(...) = step.text.strip_prefix(...)` block becomes a named function. Example transformation:

**Before (in `handle_given`):**
```rust
if let Some(profile_id) = step.text.strip_prefix("the profile pack \"") {
    let profile_id = profile_id.trim_end_matches('"').to_string();
    if scenario.profile_pack.as_deref() != Some(profile_id.as_str()) {
        anyhow::bail!("feature file profile pack {profile_id} does not match scenario metadata");
    }
    world.profile_id = Some(profile_id);
    return Ok(true);
}
```

**After:**
```rust
// in given.rs
fn handle_profile_pack(world: &mut World, scenario: &ScenarioRecord, step: &Step) -> anyhow::Result<bool> {
    let Some(profile_id) = step.text.strip_prefix("the profile pack \"") else {
        return Ok(false);
    };
    let profile_id = profile_id.trim_end_matches('"').to_string();
    if scenario.profile_pack.as_deref() != Some(profile_id.as_str()) {
        anyhow::bail!("feature file profile pack {profile_id} does not match scenario metadata");
    }
    world.profile_id = Some(profile_id);
    Ok(true)
}

// in lib.rs registry setup
registry.register("the profile pack \"*\"", handle_profile_pack);
```

Wait — the `step_matches` function won't work with wildcards. The current pattern matching is done via `strip_prefix` inside each handler, not via glob matching. So `step_matches` should be a prefix check, or better: each handler tries its own prefix and returns `Ok(false)` on mismatch. The registry simply calls handlers in order until one returns `Ok(true)` or an error.

Revised `step_matches`:
```rust
fn step_matches(_pattern: &str, _text: &str) -> bool {
    // Always true — handlers self-filter via strip_prefix.
    // The registry string is documentation/ordering only.
    true
}
```

This is the cleanest approach: the registry controls invocation order, each handler is responsible for its own matching. The pattern string in `register()` serves as documentation of what step text the handler expects.

### Handler Categories

**Given handlers (~25 patterns):**
1. Profile pack
2. Fixture directory
3. Taxonomy dimension stubs (×3)
4. Dimension context setup (×2: known/unknown)
5. Member setup (×2: valid/invalid)
6. Concept setup
7. Required dimension
8. Context without dimension
9. Typed dimension (×2: simple, with type)
10. Typed member value
11. Feature grid compiled
12. Repo has feature sidecars
13. Validation receipt
14. SEC profile configured
15. Alpha scenarios implemented
16. XBRL report contexts (×2: single, multiple)
17. Fact referencing concept
18. Facts referencing concepts
19. Numeric fact with value
20. Streaming parser available
21. XBRL filing size (×3: larger than, smaller than, large with facts)
22. Missing context refs
23. Streaming parser with custom handler
24. Taxonomy loader available
25. Taxonomy schema with dimensions
26. Taxonomy definition linkbase
27. Taxonomy with typed dimensions
28. Taxonomy with hypercubes
29. Taxonomy URL to load
30. Cache directory configured
31. Taxonomy with imports
32. Loaded taxonomy with dimensions

**When handlers (~15 patterns):**
1. Validate filing / duplicate facts / resolve DTS (exact match group)
2. Export canonical report to JSON
3. Compile feature grid
4. Validate dimension-member pair
5. Validate fact dimensions
6. Validate typed dimension value
7. Bundle selector
8. Build filing manifest
9. Package receipt for cockpit
10. Run describe-profile --json
11. Run alpha readiness gate
12. Context completeness validation
13. Decimal precision validation
14. Streaming validation (×4: validate, check needed, context validation, facts encountered)
15. Load taxonomy

**Then handlers (~20 patterns):**
1. Validation pass/fail (×2)
2. No findings
3. Specific finding reported
4. No validation errors
5. Specific error reported
6. Report has no error findings
7. Taxonomy resolution succeeds
8. Concept set
9. Export report receipt
10. Bundling fails
11. Sensor report
12. Filing manifest receipt
13. Valid JSON output
14. Profile contains fields
15. Alpha readiness checks pass
16. Rule contains / not contains (×2)
17. IXDS member count
18. Taxonomy namespace count
19. Report fact count
20. Bundle scenario listing
21. Feature grid scenario
22. Context completeness (×4: missing error, no findings, error count, rule ID)
23. Streaming parser (×7: memory, facts, context refs, DOM recommendation, streaming option, missing refs, line numbers)
24. Handler receives facts
25. Contexts collected
26. Units available
27. Taxonomy dimensions (×8: dimensions, explicit domains, domain members, parent-child, typed types, valid XSD, hypercubes, dimension-domain refs)
28. Taxonomy cached
29. Cache subsequent loads
30. Imported schemas
31. All dimension definitions

### `lib.rs` Restructure

```rust
// lib.rs
mod registry;
mod given;
mod when;
mod then;

use registry::StepRegistry;
use scenario_contract::{FeatureGrid, ScenarioRecord};
use std::path::PathBuf;

// ... World, Step, DimensionContext, etc. definitions stay here ...

pub fn run_scenario(world: &mut World, scenario: &ScenarioRecord, steps: &[Step]) -> anyhow::Result<()> {
    // ... existing validation ...

    let given_registry = given::build_registry();
    let when_registry = when::build_registry();
    let then_registry = then::build_registry();

    for step in steps {
        if given_registry.handle(world, scenario, step)? {
            continue;
        }
        if when_registry.handle(world, scenario, step)? {
            continue;
        }
        then_registry.handle(world, scenario, step)?;
    }

    // ... existing execution check ...
    Ok(())
}
```

## Files to Modify/Create

### New Files (4)
1. `crates/xbrlkit-bdd-steps/src/registry.rs` — Registry type and `step_matches` helper.
2. `crates/xbrlkit-bdd-steps/src/given.rs` — Given handler functions and `build_registry()`.
3. `crates/xbrlkit-bdd-steps/src/when.rs` — When handler functions and `build_registry()`.
4. `crates/xbrlkit-bdd-steps/src/then.rs` — Then handler functions and `build_registry()`.
5. `crates/xbrlkit-bdd-steps/tests/handler_tests.rs` — Unit tests for individual handlers (at least 3).

### Modified Files (1)
1. `crates/xbrlkit-bdd-steps/src/lib.rs` — Remove all handler bodies, keep `World`, `Step`, `run_scenario`, `run_step`, and utility functions (`execution`, `assert_declared_inputs_match`, `create_synthetic_taxonomy`, `parse_count_suffix`). Wire registry.

### No Changes Required
- `Cargo.toml` — No new dependencies needed; existing workspace deps suffice.
- `.feature` files — External contract unchanged.
- Golden test files — No scenario IDs or metadata change.

## Test Strategy

### Integration Tests (Existing)
- `cargo test --workspace` must pass in full.
- `cargo xtask alpha-check` must pass (all `@alpha-active` scenarios).
- `cargo clippy --package xbrlkit-bdd-steps` must pass with zero warnings.

### Unit Tests (New)
Target 3+ handlers for standalone testing, selected for:
1. **State mutation clarity** — easy to assert on `World` changes.
2. **Error path coverage** — handlers with `anyhow::bail!` branches.
3. **No heavy dependencies** — avoid handlers that call `execute_scenario()` or filesystem I/O.

**Recommended test targets:**

| Handler | File | Why |
|---------|------|-----|
| `handle_profile_pack` | `given.rs` | Pure state mutation, clear error path |
| `handle_fixture_directory` | `given.rs` | Path manipulation, metadata validation |
| `handle_validation_pass` | `then.rs` | Pure assertion on `World` state, no I/O |

Example test shape:
```rust
#[test]
fn test_handle_profile_pack_success() {
    let mut world = World::new(PathBuf::from("/tmp"), FeatureGrid::default());
    let scenario = ScenarioRecord {
        profile_pack: Some("sec/efm-77/opco".to_string()),
        ..ScenarioRecord::default()
    };
    let step = Step { text: "the profile pack \"sec/efm-77/opco\"".to_string(), table: vec![] };
    let result = given::handle_profile_pack(&mut world, &scenario, &step);
    assert!(result.is_ok());
    assert!(result.unwrap());
    assert_eq!(world.profile_id, Some("sec/efm-77/opco".to_string()));
}

#[test]
fn test_handle_profile_pack_mismatch() {
    let mut world = World::new(PathBuf::from("/tmp"), FeatureGrid::default());
    let scenario = ScenarioRecord {
        profile_pack: Some("other/profile".to_string()),
        ..ScenarioRecord::default()
    };
    let step = Step { text: "the profile pack \"sec/efm-77/opco\"".to_string(), table: vec![] };
    let result = given::handle_profile_pack(&mut world, &scenario, &step);
    assert!(result.is_err());
}
```

### Coverage Check
- Run `cargo tarpaulin --package xbrlkit-bdd-steps` before/after to confirm no regression.
- Target: maintain or improve coverage percentage.

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Registry ordering breaks ambiguous step matching | Medium | High | Preserve existing handler order in registration; add regression test for ambiguous prefixes |
| Handler visibility changes break downstream integration tests | Low | Medium | Keep `run_scenario` and `World` pub; only internal handlers become private or crate-private |
| `Arc<dyn Fn>` closure overhead in tight loops | Low | Low | BDD step execution is not hot-path; registry lookups are O(n) where n ≈ 30 — negligible |
| Refactor introduces subtle state mutation bugs | Medium | High | Full integration test suite catches regressions; review each handler extraction carefully |
| `create_synthetic_taxonomy` and `parse_count_suffix` need visibility from new modules | Low | Low | Move to `lib.rs` as `pub(crate)` or keep in `lib.rs` and reference from submodules |
| Missing `#[allow(clippy::too_many_lines)]` reveals other clippy issues | Low | Low | Run `cargo clippy` before and after; fix any newly surfaced warnings |
| Unit tests require mocking `ScenarioRecord` or `World` | Medium | Low | `World::new` and `ScenarioRecord` are simple structs; construct directly in tests |

## Estimated Effort

| Task | Estimate |
|------|----------|
| Create `registry.rs` with `StepRegistry` and matching logic | 30 min |
| Extract `handle_given()` into `given.rs` with ~25 handlers | 2h |
| Extract `handle_when()` into `when.rs` with ~15 handlers | 1.5h |
| Extract `handle_then()` + `handle_parameterized_assertion()` into `then.rs` with ~20 handlers | 2.5h |
| Refactor `lib.rs` to wire registries, remove suppressions | 1h |
| Write unit tests for 3+ handlers | 1h |
| Run full test suite, fix regressions | 1h |
| Run clippy, fix warnings | 30 min |
| Code review self-check (handler parity) | 30 min |
| **Total** | **~10 hours** |

## Implementation Notes

### PR Checklist
- [ ] All 4 `#[allow(clippy::too_many_lines)]` removed
- [ ] `cargo clippy --package xbrlkit-bdd-steps` passes clean
- [ ] `cargo test --workspace` passes
- [ ] `cargo xtask alpha-check` passes
- [ ] At least 3 handler unit tests added
- [ ] No `.feature` files modified
- [ ] `World` and `run_scenario` signatures unchanged

### Review Focus Areas
1. **Handler parity**: Verify every `if let` / `if step.text ==` branch in the original code has a corresponding handler function.
2. **Error message preservation**: Ensure `anyhow::bail!` messages are identical to avoid breaking test assertions that match on error text.
3. **State mutation order**: Confirm `world` mutations happen in the same sequence as the original match chain.

### Next Steps After Plan Approval
1. Create `registry.rs`, `given.rs`, `when.rs`, `then.rs`.
2. Extract handlers one category at a time (Given → When → Then), running tests after each.
3. Remove suppressions from `lib.rs`.
4. Add unit tests.
5. Open PR with reference to Issue #210.

---
*Plan created by planner-initial agent*

## Deep Review

**Reviewer:** `reviewer-deep-plan` agent  
**Date:** 2026-04-22  
**Verdict:** PASS with recommendations  
**GitHub comment:** https://github.com/EffortlessMetrics/xbrlkit/issues/210#issuecomment-4294181193

### Overall Assessment

The plan is architecturally sound and correctly identifies the core problem: four `#[allow(clippy::too_many_lines)]` suppressions in a 1971-line `lib.rs` masking structural debt. The registry pattern is an appropriate solution. Several design details need resolution before implementation to avoid mid-PR refactoring.

### Critical: Handler Signature Mismatch

The plan proposes a single `StepHandler` type returning `anyhow::Result<bool>`. This works for Given/When handlers but **does not work for Then handlers**, which return `anyhow::Result<()>` and have no "try next handler" semantics.

**Required fix before implementation:**
```rust
type GivenWhenHandler = Arc<dyn Fn(&mut World, &ScenarioRecord, &Step) -> anyhow::Result<bool>>;
type ThenHandler = Arc<dyn Fn(&mut World, &Step) -> anyhow::Result<()>>;
```

The Then registry should emit `anyhow::bail!("unsupported BDD step: {}", step.text)` when no handler matches.

### Edge Cases Identified

1. **Prefix ordering ambiguity**: Given steps like `"a context with dimension \"` must come *after* `"a context with unknown dimension \"` and `"a context with typed dimension \"`. A regression test should verify that for every `.feature` file step, exactly one handler claims responsibility.
2. **`match` vs `strip_prefix` in `handle_then`**: `handle_then` currently uses `match step.text.as_str()` for exact-match steps. The registry plan converts everything to prefix-based handlers with `step_matches` returning `true` universally, making pattern strings pure documentation. Consider a **hybrid registry**: `HashMap<String, ThenHandler>` for exact matches + `Vec` for prefix matches.
3. **Registry construction frequency**: Building registries on every `run_step()` call is wasteful. Build once per `run_scenario()` call.

### Alternative Approaches Evaluated

| Approach | Verdict |
|----------|---------|
| Registry (proposed) | ✅ Recommended |
| Enum-based dispatch | Overkill for this use case |
| Trait objects + impl blocks | Not worth the boilerplate |
| Hybrid: HashMap exact + Vec prefix | Worth considering for Then handlers |
| Keep as-is with macro-generated chains | Rejected — doesn't solve testability |

### Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Handler extraction introduces subtle logic bug | High | High | Add parity verification script |
| Error message text changes break assertions | Medium | High | Freeze messages exactly |
| Then registry `Ok(false)` vs `Ok(())` confusion | Medium | High | Fix signatures before coding |
| Unit tests only cover 3 of ~60 handlers | Certain | Medium | Integration tests catch regressions; blind spots in error paths remain |
| `pub(crate)` visibility for helper fns | Low | Low | Move `create_synthetic_taxonomy`/`parse_count_suffix` to shared `handlers/util.rs` |

The **highest risk is handler parity**: extracting ~60 handlers manually is error-prone. A **verification script** should count `strip_prefix`/`matches!`/`==` patterns in original `lib.rs` and assert the same count exists across new files.

### Testing Strategy: Additional Scenarios Needed

Beyond the plan's target of 3 unit tests, the implementation should cover:

1. **Registry ordering test**: Verify specific-prefix handlers win over generic ones.
2. **Error message preservation test**: Snapshot `anyhow::bail!` messages to prevent regression.
3. **Unsupported step fallback**: Assert `then_registry.handle()` fails with `unsupported BDD step` for unknown steps.
4. **Feature-file coverage audit**: CI-checkable script that greps all `.feature` step texts and verifies every non-`And`/`But` step has a registry entry.

**Recommended unit test targets** (expanding from 3 to 4):
1. `handle_profile_pack` (Given) — state mutation + metadata validation
2. `handle_validate_filing_group` (When) — `matches!` exact-match dispatch
3. `handle_validation_should_pass` (Then) — assertion on state, no I/O
4. `handle_parameterized_assertion` catch-all (Then) — fallback/error path

### Integration Impact

- No `.feature` file changes — ✅
- No `World`/`run_scenario` signature changes — ✅
- `Cargo.toml` unchanged — ✅
- `run_step()` needs updating to use pre-built registries — minor internal change
- `create_synthetic_taxonomy`/`parse_count_suffix` need `pub(crate)` visibility — move to shared module

### Roadmap Alignment

**Strongly aligned.** Phase 3 (SEC validation rules) will add many new BDD scenarios. A registry pattern makes adding new steps a single-line registration instead of editing a 500-line match chain. Clearing this debt now prevents suppressions from proliferating as new rule categories arrive. Supports Phase 3 Waves 4 (Streaming Parser) and 5 (Extended Taxonomy) by enabling isolated handler modules.

### API Implications

**No public API breaking changes.** `run_scenario()`, `World`, `Step`, and `run_step()` signatures remain unchanged. The four `handle_*` functions are private (`fn`, not `pub`).

### Recommendations (Pre-Implementation)

1. **Fix handler signatures** — Separate `GivenWhenHandler` from `ThenHandler`.
2. **Build registries once per scenario**, not per step.
3. **Add a parity verification script** — Count patterns in original `lib.rs`, assert same count in new files.
4. **Expand unit test target to 4 handlers**, including one Then error-path test.
5. **Consider a hybrid registry** — `HashMap` for exact-match Then steps, `Vec` for prefix-match steps.
6. **Document that `step_matches` returning `true` is intentional** — the registry is an ordered invocation list.
7. **Preserve error messages exactly** — any test asserting on error text will break otherwise.

### Action Items for Implementer

- [ ] Resolve handler signature mismatch (GivenWhenHandler vs ThenHandler)
- [ ] Implement lazy/once-per-scenario registry construction
- [ ] Add parity verification script or inline assertion
- [ ] Expand unit tests to 4 handlers with error-path coverage
- [ ] Consider hybrid exact-match HashMap for Then handlers
- [ ] Ensure `create_synthetic_taxonomy`/`parse_count_suffix` visibility is correct in new module layout
