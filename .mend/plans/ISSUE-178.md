# Plan: ISSUE-178 — Refactor large BDD step handlers in xbrlkit-bdd-steps

## Overview

The BDD step handlers in `crates/xbrlkit-bdd-steps/src/lib.rs` have grown into monolithic functions totaling ~1,375 lines across four functions in a 1,606-line file. This makes the code hard to maintain, slow to compile, and difficult to test in isolation.

| Function | Lines | Suppression |
|----------|-------|-------------|
| `handle_given()` | 485 | `#[allow(clippy::too_many_lines)]` |
| `handle_when()` | 377 | `#[allow(clippy::too_many_lines)]` |
| `handle_then()` | 177 | `#[allow(clippy::too_many_lines)]` |
| `handle_parameterized_assertion()` | 336 | `#[allow(clippy::too_many_lines)]` |

This plan proposes a structural refactor: extract domain-oriented modules, remove the clippy suppressions, and improve testability — with zero functional changes.

---

## Acceptance Criteria

- [ ] Break down all four handlers into smaller, focused functions (target: ≤50 lines per function)
- [ ] Remove all `#[allow(clippy::too_many_lines)]` attributes
- [ ] Maintain 100% of existing test coverage (all BDD scenarios must still pass)
- [ ] No functional changes — purely structural refactor
- [ ] No new dependencies added to `Cargo.toml`

---

## Proposed Approach

### 1. Module Split (file-level)

Split `lib.rs` into domain-oriented modules:

```
crates/xbrlkit-bdd-steps/src/
├── lib.rs                    (public exports, `World`, `Step`, `run_scenario`, `run_step`)
├── given_steps.rs            (Given step handlers)
├── when_steps.rs             (When step handlers)
├── then_steps.rs             (Then step handlers + parameterized assertions)
├── parsing.rs                (Common quote/string parsing utilities)
└── mod.rs                    (module declarations — or use lib.rs as root)
```

**Rationale**: Each BDD keyword (`Given`, `When`, `Then`) represents a distinct phase in scenario execution. Separating them aligns with BDD semantics and makes it immediately obvious where to add new steps.

### 2. Functional Domain Grouping (within each module)

Within each step-type module, group handlers by functional domain:

**`given_steps.rs`:**
- `handle_profile_pack()` — profile pack setup
- `handle_fixture()` — fixture directory setup
- `handle_dimension_setup()` — dimension context initialization
- `handle_typed_dimension_setup()` — typed dimension context
- `handle_bundle_setup()` — bundle / feature grid setup
- `handle_cockpit_setup()` — validation receipt setup
- `handle_cli_setup()` — CLI profile setup
- `handle_alpha_setup()` — alpha scenario verification
- `handle_context_completeness_setup()` — context/fact initialization
- `handle_decimal_precision_setup()` — numeric fact setup
- `handle_streaming_setup()` — streaming parser setup
- `handle_taxonomy_loader_setup()` — taxonomy loader initialization

**`when_steps.rs`:**
- `handle_scenario_execution()` — validate filing, resolve DTS, etc.
- `handle_feature_grid_compile()` — feature grid compilation
- `handle_dimension_validation()` — dimension-member validation
- `handle_bundle_when()` — selector bundling, filing manifest
- `handle_cockpit_packaging()` — receipt packaging
- `handle_cli_when()` — CLI execution (describe-profile)
- `handle_alpha_when()` — alpha readiness gate
- `handle_context_completeness_when()` — context completeness validation
- `handle_decimal_precision_when()` — decimal precision validation
- `handle_streaming_when()` — streaming parser execution
- `handle_taxonomy_loader_when()` — taxonomy loading

**`then_steps.rs`:**
- `handle_dimension_assertions()` — validation pass/fail, findings
- `handle_decimal_precision_assertions()` — error assertions
- `handle_report_assertions()` — report-level assertions (no errors, concept sets, etc.)
- `handle_bundle_assertions()` — bundle manifest assertions
- `handle_feature_grid_assertions()` — compiled grid assertions
- `handle_cli_assertions()` — JSON output, profile fields
- `handle_alpha_assertions()` — exit code checks
- `handle_context_completeness_assertions()` — context missing errors
- `handle_streaming_assertions()` — memory, facts, contexts, units
- `handle_taxonomy_loader_assertions()` — dimension/domain/member assertions
- `handle_parameterized_assertions()` — rule counts, fact counts, namespace counts

### 3. Extract Common Parsing (`parsing.rs`)

Two patterns recur everywhere:

1. **Quoted string extraction**: `step.text.strip_prefix("...").map(|s| s.trim_end_matches('"'))`
2. **Multi-quoted parsing**: `split('"').enumerate().filter(|(i, _)| i % 2 == 1)`

Extract into:

```rust
// parsing.rs
pub fn extract_quoted(step: &str, prefix: &str) -> Option<String>
pub fn extract_all_quoted(step: &str) -> Vec<String>
pub fn parse_count_suffix(step: &str, prefix: &str, noun_stem: &str) -> Option<usize>
```

This eliminates the `#[allow(clippy::too_many_lines)]` drivers: the match arms become single-line `if let Some(x) = extract_quoted(...)` calls.

### 4. Step Registry (optional enhancement)

Consider a lightweight registry pattern for the `run_step` dispatcher:

```rust
pub type GivenHandler = fn(&mut World, &ScenarioRecord, &Step) -> anyhow::Result<bool>;
pub type WhenHandler = fn(&mut World, &ScenarioRecord, &Step) -> anyhow::Result<bool>;
pub type ThenHandler = fn(&mut World, &Step) -> anyhow::Result<()>;

static GIVEN_HANDLERS: &[GivenHandler] = &[
    handle_profile_pack,
    handle_fixture,
    // ...
];
```

**Decision**: Defer to a follow-up issue. The module split alone is sufficient to close #178. A registry adds dispatch overhead and is only justified if step count grows significantly. Document as `ISSUE-178-followup` idea in plan.

---

## Files to Modify / Create

### New files (4)

| File | Purpose | Est. Lines |
|------|---------|-----------|
| `src/given_steps.rs` | All Given step handlers | ~400 |
| `src/when_steps.rs` | All When step handlers | ~300 |
| `src/then_steps.rs` | All Then + parameterized assertions | ~350 |
| `src/parsing.rs` | Common string/quote parsing utilities | ~40 |

### Modified files (2)

| File | Changes |
|------|---------|
| `src/lib.rs` | Keep `World`, `Step`, `run_scenario`, `run_step`, `execution()`, `assert_declared_inputs_match()`, `create_synthetic_taxonomy()`. Re-export handlers. Reduce from ~1,600 to ~250 lines. |
| `Cargo.toml` | Add `mod` declarations (none — handled in `lib.rs` with `mod given_steps;` etc.) |

---

## Test Strategy

**No new tests needed** — this is a pure refactor with zero functional changes. The existing BDD test suite (feature files in `specs/features/`) is the acceptance test.

**Validation steps:**
1. Run `cargo test -p xbrlkit-bdd-steps` — must pass
2. Run `cargo clippy -p xbrlkit-bdd-steps` — must pass with zero `too_many_lines` suppressions
3. Run `cargo test --workspace` — must pass (integration via scenario runner)
4. Run `cargo build -p xbrlkit-bdd-steps` — compile time should improve slightly

**Regression guard**: After each module extraction, run the full test suite before proceeding to the next module. Do not batch all changes and test at the end.

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|-----------|
| Accidentally changing behavior during move | Medium | High | Move code verbatim. No logic changes in this PR. Review diff carefully. |
| Import / visibility issues after module split | Low | Medium | `pub(crate)` on all handlers. `lib.rs` re-exports only `run_scenario`. |
| Merge conflicts with parallel work on BDD steps | Low | High | Complete refactor in single PR, short-lived branch. Coordinate with team. |
| Clippy still warns on extracted functions | Low | Low | If any extracted function exceeds 50 lines, further decompose. |
| `create_synthetic_taxonomy()` used by both `given_steps` and `when_steps` | Medium | Low | Keep in `lib.rs` or move to `parsing.rs` / new `test_utils.rs`. |

**Overall risk: Low**. This is a well-scoped mechanical refactor with a comprehensive existing test harness.

---

## Estimated Effort

- **Analysis & planning**: Done (this document)
- **Implementation**: 3–4 hours
  - Extract `parsing.rs`: 15 min
  - Extract `given_steps.rs`: 60 min
  - Extract `when_steps.rs`: 45 min
  - Extract `then_steps.rs`: 45 min
  - Clean up `lib.rs`: 15 min
  - Fix imports / visibility: 30 min
  - Run tests + clippy + fix issues: 30 min
- **Review & merge**: 1 hour

**Total: ~5 hours**

---

## Step-by-Step Execution Order

1. **Create `parsing.rs`** — extract `parse_count_suffix()`, `extract_quoted()`, `extract_all_quoted()`
2. **Create `given_steps.rs`** — move all `handle_given` logic verbatim
   - Keep `assert_declared_inputs_match()` in `lib.rs` (called by both `given` and `when`)
   - Each functional domain becomes a private function; `pub(crate) fn handle_given()` calls them in sequence
3. **Create `when_steps.rs`** — move all `handle_when` logic verbatim
   - Same pattern as `given_steps.rs`
4. **Create `then_steps.rs`** — move `handle_then()` and `handle_parameterized_assertion()` verbatim
   - Split into domain-specific assertion helpers
5. **Clean up `lib.rs`** — remove old handler bodies, add `mod` declarations, verify `use` statements
6. **Run full validation** — `cargo clippy`, `cargo test --workspace`

---

## Follow-up Ideas (out of scope for #178)

- **Step registry pattern**: Replace the sequential `if let` chains with a registry for O(1) dispatch on step prefixes.
- **Macro-generated step matchers**: Derive step handlers from annotated function names to eliminate boilerplate `strip_prefix` chains.
- **Per-domain submodules**: If any single domain (e.g., taxonomy loader) grows beyond ~100 lines, create `given_steps/taxonomy.rs` etc.

---

## Related

- Issue: #178 (this issue)
- Auto-generated by: friction-scan job

