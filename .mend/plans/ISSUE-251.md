# Plan: Split xbrlkit-bdd-steps into Domain Modules (Issue #251)

## Overview

`crates/xbrlkit-bdd-steps/src/lib.rs` is a 1,642-line monolith containing all BDD step definitions for the entire xbrlkit workspace. The single `handle_given`, `handle_when`, `handle_then`, and `handle_parameterized_assertion` functions each exceed 300–500 lines with `#[allow(clippy::too_many_lines)]` suppressions. This makes the file difficult to navigate, review, and edit in parallel — any change to dimension steps risks conflicts with taxonomy loader steps, and the cognitive load of reading the entire file to understand one domain is unnecessarily high.

This plan proposes a pure refactor: split the monolithic `lib.rs` into domain-specific modules while preserving exact behavior. No functional changes. All existing BDD tests must pass unchanged.

## Acceptance Criteria Breakdown

### AC-1: Group step definitions by domain
- **Requirement**: Step definitions must be organized into modules reflecting the feature-file taxonomy under `specs/features/`.
- **Domains identified**:
  1. **Core** — shared types, `World` state, dispatcher, and cross-cutting helpers
  2. **Validation** — filing validation, duplicate facts, DTS resolution, IXDS assembly, export (`specs/features/foundation/`, `specs/features/inline/`)
  3. **Dimensions** — explicit/typed dimension validation, hypercubes, domain-member checking (`specs/features/taxonomy/dimensions.feature`)
  4. **Grid** — feature grid compilation, scenario bundling (`specs/features/workflow/bundle.feature`, `specs/features/workflow/feature_grid.feature`)
  5. **Cockpit** — cockpit export packaging, sensor reports, filing manifest (`specs/features/workflow/cockpit_pack.feature`, `specs/features/foundation/filing_manifest.feature`)
  6. **CLI** — `describe-profile --json`, alpha readiness gate (`specs/features/cli/`, `specs/features/workflow/alpha_check.feature`)
  7. **Context** — context completeness and decimal precision (`specs/features/foundation/context_completeness.feature`, `specs/features/sec/decimal_precision.feature`)
  8. **Streaming** — streaming parser, memory bounds, missing context detection (`specs/features/performance/streaming_parser.feature`)
  9. **Taxonomy** — taxonomy loader, caching, schema imports (`specs/features/taxonomy/taxonomy_loader.feature`, `specs/features/taxonomy/standard_locations.feature`)

### AC-2: Extract each group into its own module
- **Requirement**: Each domain lives in its own `src/{domain}.rs` file.
- **Module exports**: Each module exposes `pub fn handle_given(...)`, `pub fn handle_when(...)`, `pub fn handle_then(...)` with the same signatures as the current private functions.
- **Dispatcher pattern**: `lib.rs` chains through domain modules in a defined order. Each handler returns `Ok(true)` if it consumed the step, `Ok(false)` if the step is unrelated to that domain.

### AC-3: Keep shared helpers in `core.rs` or `lib.rs`
- **Requirement**: Functions used by multiple domains (`ensure_taxonomy_loader`, `execution`, `assert_declared_inputs_match`, `create_synthetic_taxonomy`, `parse_count_suffix`, `select_matching_scenarios`, `selector_matches`) remain in a single shared location.
- **Data structures**: `Step`, `World`, and all `*Context` structs stay in `lib.rs` because they are the shared state container.

### AC-4: No functional changes — pure refactor
- **Requirement**: Zero behavioral diffs. Every step handler must execute identically before and after the split.
- **Verification**: `cargo test --workspace`, `cargo xtask alpha-check`, and any existing BDD runners must pass without modification.

### AC-5: All existing BDD tests pass
- **Requirement**: The full test matrix — unit tests, integration tests, and the alpha readiness gate — must remain green.
- **Scope**: This includes tests in `crates/xbrlkit-bdd`, `crates/xbrlkit-bdd-steps`, and workspace-level tests that exercise the step definitions.

## Proposed Approach

### Architectural Pattern: Domain Dispatch

The current `run_step` function dispatches through four giant functions:

```rust
fn run_step(world, scenario, step) {
    if handle_given(world, scenario, step)? { return Ok(()); }
    if handle_when(world, scenario, step)?   { return Ok(()); }
    handle_then(world, step)
}
```

After modularization, the dispatcher becomes:

```rust
fn run_step(world, scenario, step) {
    if core::handle_given(world, scenario, step)?     { return Ok(()); }
    if taxonomy::handle_given(world, scenario, step)?  { return Ok(()); }
    if dimensions::handle_given(world, scenario, step)? { return Ok(()); }
    if validation::handle_given(world, scenario, step)? { return Ok(()); }
    if grid::handle_given(world, scenario, step)?      { return Ok(()); }
    if cockpit::handle_given(world, scenario, step)?   { return Ok(()); }
    if cli::handle_given(world, scenario, step)?       { return Ok(()); }
    if context::handle_given(world, scenario, step)?   { return Ok(()); }
    if streaming::handle_given(world, scenario, step)? { return Ok(()); }
    // ... same pattern for handle_when, handle_then
}
```

Order matters only for steps that share prefixes; the existing monolithic file already establishes precedence implicitly. We preserve that exact precedence in the dispatcher chain.

### Why This Pattern

- **Minimal risk**: Each domain module is a pure cut-and-paste of existing code. No rewrites.
- **Preserved visibility**: All step handlers remain private within their modules; only the dispatcher calls them.
- **No API breakage**: `xbrlkit-bdd-steps` is a `publish = false` internal crate. Its public API (`Step`, `World`, `run_scenario`) stays unchanged.
- **Future-proof**: Adding a new domain means adding a new module and a single line in the dispatcher — no more appending to a 500-line function.

### Domain-to-Line Mapping (Estimated from Current `lib.rs`)

| Domain | Lines (approx) | Current Location |
|--------|---------------|------------------|
| Core + dispatcher | ~200 | Top of `lib.rs`, bottom helpers |
| Validation | ~140 | `handle_given` (validation receipt), `handle_when` (filing validate), `handle_then` (report assertions), `handle_parameterized_assertion` (rules, facts, IXDS) |
| Dimensions | ~290 | `handle_given` (dimension setup), `handle_when` (dimension validation), `handle_then` (pass/fail findings) |
| Grid | ~135 | `handle_given` (grid sidecars), `handle_when` (compile, bundle), `handle_then` (bundle assertions), `handle_parameterized_assertion` (grid contains) |
| Cockpit | ~85 | `handle_given` (receipt), `handle_when` (package, filing manifest), `handle_then` (sensor report, filing receipt) |
| CLI | ~115 | `handle_given` (profile), `handle_when` (describe-profile, alpha gate), `handle_then` (JSON valid, profile fields, alpha pass) |
| Context | ~275 | `handle_given` (contexts, facts), `handle_when` (completeness, decimal precision), `handle_then` (findings, errors), `handle_parameterized_assertion` (context-missing) |
| Streaming | ~290 | `handle_given` (streaming setup), `handle_when` (streaming validation, fact parsing), `handle_then` (memory, facts, contexts, units), `handle_parameterized_assertion` (streaming checks) |
| Taxonomy | ~220 | `handle_given` (loader setup), `handle_when` (load taxonomy), `handle_then` (dimensions, domains, members, cache), `handle_parameterized_assertion` (taxonomy checks) |

## Files to Modify/Create

### New Files (9)
1. `crates/xbrlkit-bdd-steps/src/core.rs` — Shared helpers: `ensure_taxonomy_loader`, `execution`, `assert_declared_inputs_match`, `create_synthetic_taxonomy`, `parse_count_suffix`, `select_matching_scenarios`, `selector_matches`
2. `crates/xbrlkit-bdd-steps/src/validation.rs` — Filing validation, DTS resolution, duplicate facts, export, IXDS assembly
3. `crates/xbrlkit-bdd-steps/src/dimensions.rs` — Explicit/typed dimension validation, hypercubes, domain-member pairs
4. `crates/xbrlkit-bdd-steps/src/grid.rs` — Feature grid compilation, scenario bundling, bundle assertions
5. `crates/xbrlkit-bdd-steps/src/cockpit.rs` — Cockpit export, sensor reports, filing manifest
6. `crates/xbrlkit-bdd-steps/src/cli.rs` — `describe-profile --json`, alpha readiness gate
7. `crates/xbrlkit-bdd-steps/src/context.rs` — Context completeness and decimal precision validation
8. `crates/xbrlkit-bdd-steps/src/streaming.rs` — Streaming parser, memory bounds, fact/context/unit collection
9. `crates/xbrlkit-bdd-steps/src/taxonomy.rs` — Taxonomy loader, cache management, schema imports

### Modified Files (1)
1. `crates/xbrlkit-bdd-steps/src/lib.rs` — Reduced from ~1,642 lines to ~200 lines. Retains `Step`, `World`, all `*Context` structs, `run_scenario`, `run_step`, and the domain-dispatch logic. Adds `mod` declarations for all new modules.

### Unchanged Files
- `crates/xbrlkit-bdd-steps/Cargo.toml` — No dependency changes required; this is an intra-crate refactor.
- All feature files under `specs/features/` — No Gherkin changes.
- All test files in other crates — No test changes; the public API of `xbrlkit-bdd-steps` is unchanged.

## Test Strategy

### Phase 1: Pre-Refactor Baseline
Before touching any code, run the full test suite to establish a green baseline:
```bash
cargo test --workspace
cargo xtask alpha-check
```

### Phase 2: Per-Domain Verification
After extracting each domain module, run targeted tests:
```bash
cargo test -p xbrlkit-bdd-steps
cargo test -p xbrlkit-bdd
```

### Phase 3: Post-Refactor Full Verification
After all modules are extracted and `lib.rs` is slimmed:
```bash
cargo test --workspace
cargo xtask alpha-check
cargo clippy --workspace --all-targets
```

### Phase 4: Diff Validation
Run a diff of the compiled artifact or use `cargo expand` to verify no semantic changes crept in during the move. The `World` struct and all `run_scenario` behavior should be byte-for-byte equivalent in effect.

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Accidental semantic change during cut-and-paste | Medium | High | Move one domain at a time; compile and test after each move; use `git diff` to verify only lines moved, not modified |
| Module visibility errors (private types, missing imports) | Medium | Medium | Add `pub` or `pub(crate)` only where strictly needed; `clippy` and `cargo check` catch these immediately |
| Clippy lints re-emerge in smaller modules | Low | Low | The `#[allow(clippy::too_many_lines)]` attributes move with their functions; new modules may need `#[allow(clippy::module_inception)]` or similar if clippy complains about module naming |
| Merge conflicts with in-flight PRs touching `lib.rs` | Medium | High | Schedule this refactor when BDD step PRs are low; the `feat/ISSUE-155-split-bdd-steps` and `feat/ISSUE-178-bdd-steps-refactor` branches suggest this has been attempted before — check those branches for prior art |
| Test flakiness unrelated to refactor | Low | Medium | Baseline test run establishes flakiness fingerprint; only investigate failures that differ from baseline |

## Estimated Effort

| Task | Estimate |
|------|----------|
| Baseline test run and branch setup | 15 min |
| Extract `core.rs` (shared helpers) | 30 min |
| Extract `taxonomy.rs` | 45 min |
| Extract `dimensions.rs` | 45 min |
| Extract `streaming.rs` | 45 min |
| Extract `context.rs` | 30 min |
| Extract `validation.rs` | 30 min |
| Extract `grid.rs` | 30 min |
| Extract `cockpit.rs` | 20 min |
| Extract `cli.rs` | 20 min |
| Slim `lib.rs` to dispatcher + types | 30 min |
| Compile fix + clippy pass | 30 min |
| Full test suite verification | 30 min |
| `cargo xtask alpha-check` verification | 15 min |
| **Total** | **~6.5 hours** |

## Context & Prior Art

The repository already has branches related to this effort:
- `feat/ISSUE-155-split-bdd-steps` (local)
- `feat/ISSUE-155-split-bdd-steps-v2` (local)
- `feat/ISSUE-178-bdd-steps-refactor` (local + origin)
- `feat/ISSUE-178-refactor-bdd-handlers` (local + origin)

These branches may contain prior attempts at this refactor. Before implementation, the builder agent should inspect these branches for:
1. Domain groupings they chose (convergence/divergence from this plan)
2. Any pitfalls encountered (compile errors, test regressions)
3. Whether any of those branches are close to completion and could be resumed rather than starting from scratch.

## Next Steps After Plan Approval

1. **Review this plan** — `reviewer-plan` agent checks domain grouping and dispatch ordering.
2. **Check prior branches** — builder inspects `feat/ISSUE-155-split-bdd-steps` and `feat/ISSUE-178-bdd-steps-refactor` for reusable work.
3. **Implement domain-by-domain** — one module per commit for clean history and easy bisection.
4. **Full test verification** — ensure zero regressions.
5. **PR review and merge**.

## Deep Review

**Status: CHANGES NEEDED** (see findings below)

### Prior Art Discovery

Substantial prior art exists that the original plan did not account for:

- **Branch `feat/ISSUE-178-bdd-steps-refactor`** (commit `bd2d59d`) already split `lib.rs` into step-type modules: `given.rs` (~493 LOC), `when.rs` (~388 LOC), `then.rs` (~520 LOC), `utils.rs` (~76 LOC), `types.rs` (~181 LOC). `lib.rs` reduced to ~18 lines.
- **Branch `pr-254`** (commit `a43693d`) refined this into `given_steps.rs`, `when_steps.rs`, `then_steps.rs`, `parsing.rs`. `lib.rs` is now ~319 lines. This branch merges cleanly into `main` with **zero conflicts** and includes additional commits (stub resolution, builder migration, cargo fmt).
- The prior art chose **step-type decomposition** rather than **domain decomposition**.

### Concerns

#### 1. Prior Art Not Leveraged
**Concern**: The plan proposes starting from scratch with domain modules, ignoring the completed work on `pr-254`.
- **Impact**: Duplicated effort. The step-type split on `pr-254` is already tested, compiles, and merges cleanly. Rejecting it means redoing ~4–6 hours of verified work.
- **Suggested revision**: The plan should evaluate `pr-254`'s step-type approach against the domain approach. A viable path: merge `pr-254` first (mechanical, low-risk), then do a second pass to re-group by domain within each step-type module. Or, if domain modules are strongly preferred, base the work on `pr-254` so the file splitting and compile fixes are already done.

#### 2. LOC Count Discrepancy
**Concern**: The issue description states 1,971 LOC; the actual file is 1,642 LOC.
- **Impact**: Minor but signals the plan was written without reading the current file.
- **Suggested revision**: Update all references to 1,642 LOC.

#### 3. Domain Mapping Approximation
**Concern**: The domain-to-line mapping table is approximate and contains misclassifications.
- **Example**: "the feature grid is compiled" is a `Given` step (line 318) but the plan maps Grid to `handle_when`.
- **Impact**: During implementation, builders will discover steps in "wrong" domains, forcing ad-hoc decisions that risk inconsistent grouping.
- **Suggested revision**: Before implementation, scan all step text patterns in the four handler functions and produce an accurate step-to-domain mapping. The plan should include this as a prerequisite task (~30 min).

#### 4. Effort Estimate Optimism
**Concern**: The ~6.5 hour estimate assumes pure cut-and-paste.
- **Reality**: Splitting by domain requires **breaking apart** the monolithic `handle_given` (474 lines), `handle_when` (378 lines), `handle_then` (177 lines), and `handle_parameterized_assertion` (335 lines) functions. Each domain sub-handler needs its own `if` chain, and shared mutable state access (`world.*_context`) must be audited for cross-domain dependencies.
- **Impact**: Estimated effort is more likely **10–14 hours** for domain-based decomposition from scratch, or **3–5 hours** if building on top of `pr-254`.
- **Suggested revision**: Adjust estimate. Add a prerequisite: review `pr-254` and decide on step-type vs domain approach.

#### 5. Architectural Trade-off Missing
**Concern**: The plan does not discuss why domain modules are better than step-type modules.
- **Step-type pros**: Easier to implement (function-level cut-and-paste), preserves handler precedence naturally, matches Cucumber's Given/When/Then mental model.
- **Domain pros**: Better for domain-focused maintenance ("I need to fix all dimension steps").
- **Hybrid option**: `src/dimensions/given.rs`, `src/dimensions/when.rs`, `src/dimensions/then.rs` — domain folders with step-type files inside.
- **Suggested revision**: Add a decision record explaining why domain modules were chosen over step-type or hybrid.

#### 6. Dispatcher Precedence Risk
**Concern**: The plan states "Order matters only for steps that share prefixes" but does not identify which steps actually share prefixes.
- **Impact**: During the break-apart of monolithic handlers, a step that previously matched early in `handle_given` might now match later in a different domain module, subtly changing behavior if prefix matching overlaps.
- **Suggested revision**: Audit all `strip_prefix` patterns for overlap. Document the exact dispatch order rationale.

### Risk Reassessment

| Risk | Original Likelihood | Revised Likelihood | Mitigation |
|------|---------------------|-------------------|------------|
| Accidental semantic change during move | Medium | **High** | Domain break-apart is harder than function-level cut-and-paste. Add step-pattern audit as prerequisite. |
| Merge conflicts with in-flight PRs | Medium | **High** | `pr-254` already resolves this cleanly. Consider merging it first. |
| Underestimated effort | — | **Medium** | Revise estimate to 10–14h from scratch, or 3–5h building on `pr-254`. |
| Domain misclassification | — | **Medium** | Add prerequisite: scan all step texts and produce accurate mapping. |

### Recommended Revised Approach

1. **Inspect `pr-254`** (30 min): Review the completed step-type split. Decide if it satisfies the issue's intent or if domain modules are strictly required.
2. **If step-type is acceptable**: Merge `pr-254` (or cherry-pick `a43693d` + `82f807e` + `643320c`). Close #251 as resolved by prior work.
3. **If domain is required**: Base work on `pr-254`. Use its clean compile/test baseline. Decompose each of `given_steps.rs`, `when_steps.rs`, `then_steps.rs` into per-domain sub-modules (e.g., `src/given/dimensions.rs`, `src/given/taxonomy.rs`). This is much safer than splitting from the monolith directly.
4. **Step-pattern audit** (30 min): Produce definitive step-to-domain mapping before touching code.

---
*Plan created by planner-initial agent*
*Deep review by reviewer-deep-plan agent*
