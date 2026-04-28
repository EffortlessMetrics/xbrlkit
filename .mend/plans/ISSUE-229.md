# Plan: Decompose God Object World and Remove Clippy Suppressions (Issue #229)

## Overview

`crates/xbrlkit-bdd-steps/src/lib.rs` has grown to **1,642 lines** — the largest single file in the crate graph — with four functions suppressing `clippy::too_many_lines` and a `World` struct holding **20+ fields**. Additionally, `crates/xbrlkit-cli/src/main.rs` carries a **crate-level** `#![allow(clippy::too_many_lines)]`.

This plan refactors the BDD step crate into maintainable submodules, extracts helpers from the oversized functions, groups the remaining loose `World` fields into focused context objects, and removes the CLI-level suppression.

---

## Current State

| Metric | Value |
|--------|-------|
| `xbrlkit-bdd-steps/src/lib.rs` | **1,642 lines** |
| `World` field count | **20 fields** (lines 53–72) |
| `handle_given` | ~472 lines (suppressed at line 211) |
| `handle_when` | ~376 lines (suppressed at line 684) |
| `handle_then` | ~176 lines (suppressed at line 1061) |
| `handle_parameterized_assertion` | ~404 lines (suppressed at line 1238) |
| `xbrlkit-cli/src/main.rs` | **225 lines**, crate-level `allow(clippy::too_many_lines)` |

The `World` struct already contains four "sub-contexts":
- `DimensionContext`
- `ContextCompletenessContext`
- `StreamingContext`
- `TaxonomyLoaderContext`

But the remaining **16 fields** are still stored directly on `World`, creating the god-object problem.

---

## Proposed Approach

### Phase 1: Extract Loose Fields into Focused Contexts (Structural)

Group the 16 unwrapped fields into **4 new context structs** based on usage patterns in `handle_*` functions:

| New Context | Fields Moved | Used By |
|-------------|--------------|---------|
| `ExecutionContext` | `execution`, `validation_receipt` | `handle_when`, `handle_then` |
| `FilingContext` | `filing_manifest`, `filing_receipt`, `sensor_report` | `handle_when`, `handle_then` |
| `BundleContext` | `bundle_manifest`, `compiled_grid` | `handle_when`, `handle_then` |
| `CliContext` | `cli_output`, `cli_json_output`, `cli_exit_code` | `handle_when`, `handle_then` |

The `World` struct shrinks from **20 fields → 8 fields**:

```rust
pub struct World {
    pub repo_root: PathBuf,
    pub grid: FeatureGrid,
    pub profile_id: Option<String>,
    pub fixture_dirs: Vec<PathBuf>,
    pub dimension_context: DimensionContext,
    pub context_completeness_context: ContextCompletenessContext,
    pub streaming_context: StreamingContext,
    pub taxonomy_loader_context: TaxonomyLoaderContext,
    pub execution_context: ExecutionContext,
    pub filing_context: FilingContext,
    pub bundle_context: BundleContext,
    pub cli_context: CliContext,
}
```

All existing `world.execution` accesses become `world.execution_context.execution`, etc.

### Phase 2: Split `lib.rs` into Submodules (Organizational)

Create a module tree under `xbrlkit-bdd-steps/src/`:

```
lib.rs              (~200 lines: public re-exports, World, Step, run_scenario)
contexts.rs         (all 8 context structs + impl World::new)
given.rs            (handle_given + extracted helpers)
when.rs             (handle_when + extracted helpers)
then.rs             (handle_then + handle_parameterized_assertion + extracted helpers)
helpers.rs          (parse_count_suffix, select_matching_scenarios, selector_matches,
                     create_synthetic_taxonomy, ensure_taxonomy_loader, execution,
                     assert_declared_inputs_match, run_step)
```

Each submodule gets its own `#[allow(...)]` only if absolutely necessary — goal is **zero clippy suppressions for length**.

### Phase 3: Extract Helper Functions from Suppressed Functions (Behavioral)

| Source Function | Extracted Helpers | Est. Lines Saved |
|-----------------|-------------------|------------------|
| `handle_given` | `handle_given_profile`, `handle_given_fixture`, `handle_given_dimension`, `handle_given_typed_dimension`, `handle_given_taxonomy` | ~300 |
| `handle_when` | `handle_when_execution`, `handle_when_export`, `handle_when_streaming`, `handle_when_cli` | ~200 |
| `handle_then` | `handle_then_validation`, `handle_then_filing`, `handle_then_bundle`, `handle_then_grid` | ~80 |
| `handle_parameterized_assertion` | `handle_assertion_rule`, `handle_assertion_count`, `handle_assertion_bundle`, `handle_assertion_grid`, `handle_assertion_context` | ~250 |

Each extracted helper becomes a **top-level private fn** in its respective submodule. The parent functions reduce to thin dispatch tables (~30–50 lines each).

### Phase 4: Remove CLI-Level Suppression

`xbrlkit-cli/src/main.rs` is only 225 lines; the crate-level `#![allow(clippy::too_many_lines)]` is likely a copy-paste artifact from the BDD crate.

- **Action**: Remove `#![allow(clippy::too_many_lines)]` from `main.rs`
- **If clippy fires**: extract subcommands into `commands.rs` or break `run()` into `run_validate()`, `run_bundle()`, etc.

---

## Files to Modify/Create

### New Files (4)
1. `crates/xbrlkit-bdd-steps/src/contexts.rs` — all context structs
2. `crates/xbrlkit-bdd-steps/src/given.rs` — Given step handlers
3. `crates/xbrlkit-bdd-steps/src/when.rs` — When step handlers
4. `crates/xbrlkit-bdd-steps/src/then.rs` — Then step handlers + parameterized assertions

### Modified Files (4)
1. `crates/xbrlkit-bdd-steps/src/lib.rs` — shrink to ~200 lines (re-exports, `World`, `Step`, `run_scenario`)
2. `crates/xbrlkit-bdd-steps/src/helpers.rs` — extract shared utilities (new file, counted above)
3. `crates/xbrlkit-bdd-steps/Cargo.toml` — no changes expected (no new deps)
4. `crates/xbrlkit-cli/src/main.rs` — remove `#![allow(clippy::too_many_lines)]`

---

## Test Strategy

- **Behavioral parity**: every existing BDD scenario must pass without modification
- **Clippy gates**:
  - `cargo clippy -- -D clippy::too_many_lines` on `xbrlkit-bdd-steps`
  - `cargo clippy -- -D warnings` on `xbrlkit-cli`
- **Size targets**:
  - `lib.rs` < 250 lines
  - each submodule < 250 lines (extract further if needed)
  - `World` ≤ 12 fields

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Submodule split breaks intra-crate visibility | Low | High | Use `pub(crate)`; add `mod.rs` declarations |
| Field migration introduces compilation churn | Medium | Low | Mechanical change; compiler guides all renames |
| Extracted helpers need shared mutable state | Low | Medium | Pass `&mut World` or `&mut SpecificContext` |
| BDD step text matching relies on field order | Low | High | No behavior changes; only structural moves |
| CLI suppression removal reveals real issues | Low | Low | File is only 225 lines; extract if needed |

---

## Estimated Effort

| Task | Estimate |
|------|----------|
| Create 4 new context structs + update `World` | 1h |
| Migrate all field accesses across handlers | 2h |
| Split `lib.rs` into submodules | 2h |
| Extract helpers from `handle_given` | 2h |
| Extract helpers from `handle_when` | 2h |
| Extract helpers from `handle_then` + parameterized | 2h |
| Remove CLI suppression + fix any fallout | 0.5h |
| Run full BDD suite + clippy checks | 1h |
| **Total** | **~12.5 hours** |

---

## Definition of Done

- [ ] `World` has ≤ 12 fields (4 legacy contexts + 4 new contexts)
- [ ] `xbrlkit-bdd-steps/src/lib.rs` < 250 lines
- [ ] Zero `#[allow(clippy::too_many_lines)]` in `xbrlkit-bdd-steps`
- [ ] Zero `#![allow(clippy::too_many_lines)]` in `xbrlkit-cli`
- [ ] `cargo clippy -- -D warnings` passes on both crates
- [ ] All existing BDD scenarios pass (`cargo xtask alpha-check` or equivalent)
- [ ] No behavior changes in step matching logic

## Deep Review

### Edge Cases Considered
- **Downstream crate `xbrlkit-bdd` accesses `world.execution`, `world.profile_id`, `world.fixture_dirs` directly** (lines 40–42 of `crates/xbrlkit-bdd/src/lib.rs`): The plan's mechanical migration step covers this, but it should be explicitly listed as a file to update in Phase 1. `world.execution = None` will become `world.execution_context.execution = None`.
- **Existing `execution(world: &World)` helper function** (line ~177): Already provides `&ScenarioExecution` extraction. After Phase 1, this may be redundant or should be updated to access `execution_context.execution`.
- **`helpers.rs` listed under "Modified Files" but is a new file**: Minor documentation inconsistency; should be moved to "New Files".
- **`World` has `pub` fields and is imported by `xbrlkit-bdd`**: Moving fields into contexts is technically an API-breaking change for the one downstream consumer. The single-crate consumer limits blast radius.

### Risk Assessment
| Risk | Severity | Mitigation in Plan |
|------|----------|-------------------|
| `xbrlkit-bdd` compilation breaks during Phase 1 field migration | Low | Add explicit mention of `crates/xbrlkit-bdd/src/lib.rs` to the files-modified list |
| Submodule visibility: `run_step` in `lib.rs` must call `handle_given`/`when`/`then` across module boundaries | Low | Use `pub(crate)`; straightforward Rust module pattern |
| Over-bundling: `repo_root`/`grid`/`profile_id`/`fixture_dirs` are runner config, not domain state | Low | Plan correctly keeps them on `World`; bundling them purely to hit ≤8 would reduce clarity |
| `execution()` helper collides with new `ExecutionContext` naming | Low | Rename helper to `get_execution()` or fold into `ExecutionContext` accessor |

### Alternatives Considered
- **Trait-based World decomposition**: Would add indirection and boilerplate without benefit for a purely mechanical refactor. Rejected.
- **Macro-based field access**: Over-engineered for a one-time migration. Rejected.
- **≤8 fields target**: Would require bundling `repo_root`/`grid`/`profile_id`/`fixture_dirs` into a `RunnerContext`. Adds nesting for config fields that don't share lifecycle or semantics. The chosen ≤12 target is more pragmatic.

### Watch During Implementation
- Ensure `crates/xbrlkit-bdd/src/lib.rs` is updated in the same commit as Phase 1 (field migration) to keep CI green.
- Verify `run_scenario` can still dispatch to `handle_given`/`when`/`then` after submodule split — `pub(crate)` visibility is sufficient.
- Check if the existing `execution(world: &World)` helper becomes dead code after context migration.
- Run `cargo clippy -- -D clippy::too_many_lines` on the BDD crate after each phase, not just at the end, to catch regressions early.

---
*Plan created by planner-initial agent*
*Deep review by reviewer-deep-plan agent — PASS with watch-items*
