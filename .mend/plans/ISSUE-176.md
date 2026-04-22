# ISSUE-176 Plan: Create Shared Test Utilities Crate

## Overview

Multiple crates across the xbrlkit workspace duplicate test helper functions for creating test data (Facts, Contexts, ProfilePacks, DimensionTaxonomies). This plan creates a centralized `xbrlkit-test-helpers` dev-dependency crate to eliminate duplication, ensure consistency when `Fact` or related structs change, and reduce test boilerplate.

## Current State — Duplicated Test Utilities

### Fact Constructors (6 locations across 5 crates)

| Location | Function | Signature | Purpose |
|----------|----------|-----------|---------|
| `crates/numeric-rules/src/lib.rs` | `fact()` | `(concept, value) -> Fact` | Minimal fact with defaults |
| `crates/numeric-rules/src/decimal_precision.rs` | `fact_with_decimals()` | `(concept, value, decimals) -> Fact` | Fact with decimals attribute |
| `crates/efm-rules/src/lib.rs` | (inline) | Manual `Fact { ... }` × 4 | Inline construction, no helper |
| `crates/unit-rules/src/validator.rs` | `create_test_fact()` | `(concept, unit_ref) -> Fact` | Fact with unit reference |
| `crates/context-completeness/src/lib.rs` | `create_test_fact()` | `(concept, context_ref) -> Fact` | Fact with context reference |
| `crates/duplicate-facts/src/lib.rs` | `fact()` | `(value) -> Fact` | Fact with fixed concept/context |

### Context Constructors (2 crates)

| Location | Function | Signature | Purpose |
|----------|----------|-----------|---------|
| `crates/context-completeness/src/lib.rs` | `create_test_context()` | `(id) -> Context` | Basic context with instant period |
| `crates/dimensional-rules/src/lib.rs` | `create_test_context_with_dims()` | `(id, dims) -> Context` | Context with explicit dimensions |
| `crates/dimensional-rules/src/lib.rs` | `create_test_context_with_typed_dim()` | `(id, value) -> Context` | Context with typed dimension |

### Profile/Taxonomy Constructors (2 crates)

| Location | Function | Signature | Purpose |
|----------|----------|-----------|---------|
| `crates/efm-rules/src/lib.rs` | `profile()` | `() -> ProfilePack` | Basic SEC profile pack |
| `crates/efm-rules/src/lib.rs` | `profile_with_required_facts()` | `() -> ProfilePack` | Profile with required facts list |
| `crates/dimensional-rules/src/lib.rs` | `create_test_taxonomy()` | `() -> DimensionTaxonomy` | Taxonomy with explicit dimensions |
| `crates/dimensional-rules/src/lib.rs` | `create_test_typed_taxonomy()` | `(value_type) -> DimensionTaxonomy` | Taxonomy with typed dimension |

### Additional Duplication Patterns

- **ValidationFinding inline construction**: Repeats across all validation crates with nearly identical field initialization
- **EntityIdentifier defaults**: `scheme: "http://www.sec.gov/CIK"`, `value: "0000320193"` hardcoded in multiple places
- **Period defaults**: `Period::Instant("2024-12-31".to_string())` repeated in 3+ locations

## Proposed Solution: `xbrlkit-test-helpers` Crate

### Crate Design

```
crates/xbrlkit-test-helpers/
├── Cargo.toml          # dev-only dependency, no runtime deps
├── src/
│   └── lib.rs          # All test helper modules
```

**Cargo.toml characteristics:**
- `version = "0.1.0-alpha.1"` (workspace version)
- No `[lib]` crate-type restrictions — standard rlib
- Dependencies: `xbrl-report-types`, `xbrl-contexts`, `sec-profile-types`, `taxonomy-dimensions`
- All downstream crates add it as `dev-dependencies` only

### API Surface

#### Module: `fact` — Test Fact Builders

```rust
use xbrl_report_types::Fact;

/// Builder for test Facts with sensible defaults.
pub struct TestFact {
    concept: String,
    context_ref: String,
    unit_ref: Option<String>,
    decimals: Option<String>,
    value: String,
    member: String,
}

impl TestFact {
    pub fn new(concept: &str, value: &str) -> Self;
    pub fn context_ref(mut self, ctx: &str) -> Self;
    pub fn unit_ref(mut self, unit: &str) -> Self;
    pub fn decimals(mut self, dec: &str) -> Self;
    pub fn member(mut self, member: &str) -> Self;
    pub fn build(self) -> Fact;
}

// Convenience free functions for common patterns
pub fn fact(concept: &str, value: &str) -> Fact;
pub fn fact_with_decimals(concept: &str, value: &str, decimals: &str) -> Fact;
pub fn fact_with_unit(concept: &str, value: &str, unit_ref: &str) -> Fact;
pub fn fact_with_context(concept: &str, value: &str, context_ref: &str) -> Fact;
```

#### Module: `context` — Test Context Builders

```rust
use xbrl_contexts::{Context, ContextSet, EntityIdentifier, Period, DimensionalContainer, DimensionMember};

pub struct TestContext {
    id: String,
    entity: EntityIdentifier,
    period: Period,
    scenario: Option<DimensionalContainer>,
    entity_segment: Option<DimensionalContainer>,
}

impl TestContext {
    pub fn new(id: &str) -> Self;
    pub fn with_entity(mut self, scheme: &str, value: &str) -> Self;
    pub fn with_instant_period(mut self, date: &str) -> Self;
    pub fn with_duration_period(mut self, start: &str, end: &str) -> Self;
    pub fn with_dimensions(mut self, dims: Vec<(&str, &str)>) -> Self;
    pub fn with_typed_dimension(mut self, dim: &str, value: &str) -> Self;
    pub fn build(self) -> Context;
}

// Convenience functions
pub fn test_context(id: &str) -> Context;
pub fn test_context_with_dims(id: &str, dims: Vec<(&str, &str)>) -> Context;
pub fn test_context_with_typed_dim(id: &str, dim: &str, value: &str) -> Context;

// ContextSet helpers
pub fn context_set(contexts: Vec<Context>) -> ContextSet;
```

#### Module: `profile` — Test Profile Builders

```rust
use sec_profile_types::{ProfilePack, InlineRules, AcceptedTaxonomies};

pub struct TestProfile {
    id: String,
    label: String,
    inline_rules: InlineRules,
    required_facts: Vec<String>,
    // ... other fields
}

impl TestProfile {
    pub fn new(id: &str) -> Self;
    pub fn with_banned_elements(mut self, elements: Vec<&str>) -> Self;
    pub fn with_banned_attributes(mut self, attrs: Vec<&str>) -> Self;
    pub fn with_required_facts(mut self, facts: Vec<&str>) -> Self;
    pub fn build(self) -> ProfilePack;
}

// Convenience functions
pub fn test_profile() -> ProfilePack;
pub fn test_profile_with_required_facts(facts: Vec<&str>) -> ProfilePack;
```

#### Module: `taxonomy` — Test Taxonomy Builders

```rust
use taxonomy_dimensions::{DimensionTaxonomy, Dimension, Domain, DomainMember, Hypercube};

pub struct TestTaxonomy;

impl TestTaxonomy {
    pub fn new() -> DimensionTaxonomy;
    pub fn with_explicit_dimension(tax: &mut DimensionTaxonomy, dim: &str, domain: &str, members: Vec<&str>);
    pub fn with_typed_dimension(tax: &mut DimensionTaxonomy, dim: &str, value_type: &str);
    pub fn link_concept_hypercube(tax: &mut DimensionTaxonomy, concept: &str, hypercube: &str);
}

// Convenience functions
pub fn test_taxonomy() -> DimensionTaxonomy;
pub fn test_typed_taxonomy(value_type: &str) -> DimensionTaxonomy;
```

#### Module: `finding` — ValidationFinding Builders

```rust
use xbrl_report_types::ValidationFinding;

pub fn error_finding(rule_id: &str, message: &str) -> ValidationFinding;
pub fn error_finding_with_subject(rule_id: &str, message: &str, subject: &str) -> ValidationFinding;
pub fn warning_finding(rule_id: &str, message: &str) -> ValidationFinding;
```

## Files to Create

1. `crates/xbrlkit-test-helpers/Cargo.toml` — new crate manifest
2. `crates/xbrlkit-test-helpers/src/lib.rs` — crate root, module declarations
3. `crates/xbrlkit-test-helpers/src/fact.rs` — Fact builders
4. `crates/xbrlkit-test-helpers/src/context.rs` — Context builders
5. `crates/xbrlkit-test-helpers/src/profile.rs` — ProfilePack builders
6. `crates/xbrlkit-test-helpers/src/taxonomy.rs` — DimensionTaxonomy builders
7. `crates/xbrlkit-test-helpers/src/finding.rs` — ValidationFinding builders

## Files to Modify

### Add dev-dependency

Update `Cargo.toml` workspace dependencies section:
- Add `xbrlkit-test-helpers = { version = "0.1.0-alpha.1", path = "crates/xbrlkit-test-helpers" }`

Update each crate's `Cargo.toml` to add `[dev-dependencies]` entry:
- `crates/numeric-rules/Cargo.toml`
- `crates/efm-rules/Cargo.toml`
- `crates/unit-rules/Cargo.toml`
- `crates/context-completeness/Cargo.toml`
- `crates/duplicate-facts/Cargo.toml`
- `crates/dimensional-rules/Cargo.toml`

### Remove duplicated helpers and migrate imports

- `crates/numeric-rules/src/lib.rs` — Remove `fact()`; import from `xbrlkit_test_helpers::fact::fact`
- `crates/numeric-rules/src/decimal_precision.rs` — Remove `fact_with_decimals()`; use builder
- `crates/efm-rules/src/lib.rs` — Remove `profile()`, `profile_with_required_facts()`, inline Fact constructions; use helpers
- `crates/unit-rules/src/validator.rs` — Remove `create_test_fact()`; use builder
- `crates/context-completeness/src/lib.rs` — Remove `create_test_fact()`, `create_test_context()`; use helpers
- `crates/duplicate-facts/src/lib.rs` — Remove `fact()`; use helpers
- `crates/dimensional-rules/src/lib.rs` — Remove `create_test_taxonomy()`, `create_test_typed_taxonomy()`, `create_test_context_with_dims()`, `create_test_context_with_typed_dim()`; use helpers

### Update workspace membership

- `Cargo.toml` (workspace root) — Add `"crates/xbrlkit-test-helpers"` to `members` list

## Migration Strategy

### Phase 1: Create the crate
1. Create `crates/xbrlkit-test-helpers/` with all modules
2. Implement builder patterns for each test data type
3. Ensure all existing test helper signatures are supported (backward-compatible API)
4. Add internal unit tests for the test helpers themselves
5. Verify crate compiles with `cargo check -p xbrlkit-test-helpers`

### Phase 2: Migrate crates (one at a time)
1. Add `xbrlkit-test-helpers` to `[dev-dependencies]`
2. Replace local test helper functions with imports from new crate
3. Run `cargo test -p <crate>` to verify
4. Commit per crate for bisect-ability

**Recommended migration order** (least to most complex):
1. `duplicate-facts` — simplest, only `fact()` helper
2. `numeric-rules` — `fact()` and `fact_with_decimals()`
3. `unit-rules` — `create_test_fact()` with unit_ref
4. `context-completeness` — Fact + Context helpers
5. `efm-rules` — Profile packs + inline Fact constructions
6. `dimensional-rules` — Taxonomy + Context with dimensions

### Phase 3: Verification
1. Run full workspace test suite: `cargo test --workspace`
2. Verify no duplicated helper patterns remain: `grep -rn "fn fact\|fn create_test_fact\|fn create_test_context\|fn create_test_taxonomy\|fn profile()" --include="*.rs" crates/ | grep -v xbrlkit-test-helpers`
3. Expected: zero matches outside the new crate

## Acceptance Criteria

- [ ] New `xbrlkit-test-helpers` crate exists and compiles
- [ ] All existing test helper functions migrated from:
  - [ ] `numeric-rules`
  - [ ] `efm-rules`
  - [ ] `unit-rules`
  - [ ] `context-completeness`
  - [ ] `duplicate-facts`
  - [ ] `dimensional-rules`
- [ ] No duplicated `Fact` construction patterns remain in test code (verified by grep)
- [ ] No duplicated `Context` construction patterns remain in test code
- [ ] No duplicated `ProfilePack` construction patterns remain in test code
- [ ] No duplicated `DimensionTaxonomy` construction patterns remain in test code
- [ ] Full workspace test suite passes (`cargo test --workspace`)
- [ ] Crate is added to workspace `members` list
- [ ] All dependent crates have `xbrlkit-test-helpers` in `[dev-dependencies]` only

## Test Strategy

- The test-helpers crate itself should have unit tests verifying builders produce correct structures
- Each migrated crate's existing tests serve as regression tests — they must all pass unchanged
- No new test logic needed; this is a refactor

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Builder API doesn't cover all edge cases in existing tests | Medium | Medium | Start with exact reproduction of existing helpers; iterate if gaps found |
| Circular dependency if crate depends on validation crates | Low | High | Only depend on type crates (`xbrl-report-types`, `xbrl-contexts`, etc.); never on validation crates |
| Workspace compile time increase | Low | Low | Dev-dependency only; does not affect release builds |
| Breaking change when `Fact` struct evolves | N/A | N/A | This is the *benefit* — one place to update instead of N |

## Estimated Effort

- **Crate creation + API design**: 1–2 hours
- **Per-crate migration**: 15–30 minutes × 6 crates = 1.5–3 hours
- **Verification + cleanup**: 30 minutes
- **Total**: ~3.5–5.5 hours

## Dependencies

- No blockers. Self-contained refactor.
- Should be completed after any in-flight PRs touching the affected test files to avoid merge conflicts.

## References

- Issue: https://github.com/EffortlessMetrics/xbrlkit/issues/176
- Auto-generated by friction-scan job

## Deep Review

**Reviewer:** reviewer-deep-plan-176  
**Date:** 2026-04-22  
**Verdict:** PASS — No blockers. Core design is sound. Refinements noted below for implementation.

### Findings

1. **`normalize_context_id` inconsistency.** `context-completeness` normalizes context IDs in its helper; `dimensional-rules` does not. The `TestContext` builder should normalize by default with an explicit `raw_id()` escape hatch.

2. **`member` field default mismatch.** `duplicate-facts` defaults `member` to `"member-a.html"` while all others use `String::new()`. The builder should use the common default; document the explicit `.member()` call needed during migration.

3. **`unit_ref` signature mismatch.** `unit-rules` takes `Option<&str>` but the builder API takes `&str`. Add `maybe_unit_ref(Option<&str>)` or keep backward-compatible free functions.

4. **Inline `Fact { ... }` missed in plan.** `numeric-rules/src/decimal_precision.rs:253` has an inline `Fact { ... }` inside a test that the plan's function-based grep won't catch. Phase 3 verification should include inline struct construction.

5. **`TestTaxonomy` is not a true builder.** It uses `&mut DimensionTaxonomy` parameters, inconsistent with `TestFact`/`TestContext`/`TestProfile`. Recommend making it a consuming builder or renaming to `TaxonomyHelper`.

6. **Verification script incomplete.** Should check for inline `Fact {` and `Context {` in `#[cfg(test)]` modules, not just helper function names.

7. **`xbrlkit-bdd-steps` correctly out-of-scope.** Its inline constructions are BDD step DSL, not unit-test helpers. Note as future follow-up.

8. **No circular dependency risk confirmed.** New crate only depends on type crates; safe.

### Suggested Amendments

1. Add `normalize_context_id` to `TestContext::new()` with `raw_id()` escape hatch.
2. Fix `TestTaxonomy` to be a true consuming builder.
3. Expand Phase 3 verification to catch inline struct construction.
4. Document `member` default change for `duplicate-facts` migration.
5. Add `maybe_unit_ref(Option<&str>)` variant.
6. Add CI lint check (xtask) to prevent regression of inline test data construction.

### Actions

- GitHub comment posted: https://github.com/EffortlessMetrics/xbrlkit/issues/176#issuecomment-4294103227
- Label: `deep-plan-reviewed` added to issue #176.
