# ISSUE-149: Review xbrlkit-bdd-steps dependency footprint

**Status:** plan-draft  
**Agent:** planner-initial  
**Created:** 2026-04-22  

---

## Overview

The `xbrlkit-bdd-steps` crate carries 19 direct dependencies (3 external, 16 workspace), making it one of the heaviest compile-time nodes in the workspace graph. This is a test-only crate (`publish = false`), but because it sits between `xbrlkit-bdd` and the rest of the workspace, its dependency bloat propagates to CI build times and downstream test compilation.

This plan proposes a **two-phase approach**: first feature-gate domains to make the dependency graph conditional, then evaluate whether physical crate splitting delivers enough value to justify the complexity.

---

## Acceptance Criteria

1. [ ] Each dependency in `xbrlkit-bdd-steps/Cargo.toml` is audited for actual usage in `src/lib.rs`
2. [ ] Unused or opportunistic dependencies are removed
3. [ ] Domain-specific functionality is behind feature flags (e.g., `dimensions`, `streaming`, `validation`, `cli`, `taxonomy-loader`)
4. [ ] `xbrlkit-bdd` compiles and passes tests with default features
5. [ ] CI build time for `xbrlkit-bdd-steps` is measured before and after (documented, not necessarily improved)
6. [ ] A decision record exists for whether to proceed with physical crate splitting

---

## Dependency Audit

### External Dependencies

| Crate | Used For | Domain | Action |
|-------|----------|--------|--------|
| `anyhow` | Error handling throughout | Core | Keep |
| `serde_json` | CLI JSON output parsing (`describe_profile`) | CLI | Feature-gate: `cli` |
| `walkdir` | Feature sidecar discovery, alpha tag scanning | Core/Grid | Keep in core |

### Workspace Dependencies

| Crate | Lines of Usage | Step Domains | Proposed Feature |
|-------|---------------|--------------|------------------|
| `scenario-contract` | 50+ | Core (FeatureGrid, ScenarioRecord, BundleManifest) | Core (always) |
| `scenario-runner` | 30+ | Core execution (validate, resolve, export) | Core (always) |
| `receipt-types` | 20+ | Cockpit, filing manifest, validation receipts | Core (always) |
| `xbrlkit-feature-grid` | 10+ | Feature grid compilation | Core (always) |
| `xbrl-contexts` | 80+ | Dimensions, context completeness, taxonomy loader | `dimensions` + `context-completeness` + `taxonomy-loader` |
| `taxonomy-dimensions` | 60+ | Dimension validation steps | `dimensions` |
| `dimensional-rules` | 20+ | Dimension validation steps | `dimensions` |
| `xbrl-report-types` | 40+ | Context completeness, decimal precision | `context-completeness` + `decimal-precision` |
| `context-completeness` | 15+ | Context completeness validation | `context-completeness` |
| `numeric-rules` | 10+ | Decimal precision validation | `decimal-precision` |
| `xbrl-stream` | 40+ | Streaming parser steps | `streaming` |
| `cockpit-export` | 10+ | Cockpit sensor report packaging | Core (always) — small, always used |
| `filing-load` | 15+ | Filing manifest building | Core (always) — small, always used |
| `edgar-attachments` | 10+ | Filing manifest types | Core (always) — small, always used |
| `sec-profile-types` | 15+ | CLI profile loading | `cli` |
| `taxonomy-loader` | 40+ | Taxonomy loader steps | `taxonomy-loader` |

### Summary

- **Core dependencies (always needed):** 9 crates — `anyhow`, `walkdir`, `scenario-contract`, `scenario-runner`, `receipt-types`, `xbrlkit-feature-grid`, `cockpit-export`, `filing-load`, `edgar-attachments`
- **Feature-gatable dependencies:** 10 crates — `serde_json`, `xbrl-contexts`, `taxonomy-dimensions`, `dimensional-rules`, `xbrl-report-types`, `context-completeness`, `numeric-rules`, `xbrl-stream`, `sec-profile-types`, `taxonomy-loader`

---

## Proposed Approach

### Phase 1: Feature-Gating (Recommended)

Introduce Cargo features in `xbrlkit-bdd-steps` to make domain-specific dependencies conditional:

```toml
[features]
default = ["dimensions", "context-completeness", "decimal-precision", "streaming", "cli", "taxonomy-loader"]
dimensions = ["dep:dimensional-rules", "dep:taxonomy-dimensions", "dep:xbrl-contexts"]
context-completeness = ["dep:context-completeness", "dep:xbrl-contexts", "dep:xbrl-report-types"]
decimal-precision = ["dep:numeric-rules", "dep:xbrl-report-types"]
streaming = ["dep:xbrl-stream"]
cli = ["dep:sec-profile-types", "dep:serde_json"]
taxonomy-loader = ["dep:taxonomy-loader", "dep:xbrl-contexts", "dep:taxonomy-dimensions"]
```

**Code changes:**
- Wrap each domain's `World` context fields in `#[cfg(feature = ...)]`
- Wrap each domain's `handle_given`/`handle_when`/`handle_then` blocks in `#[cfg(feature = ...)]`
- Provide `#[cfg(not(feature = ...))]` stub implementations that return `Ok(false)` for disabled step handlers
- Update `xbrlkit-bdd/Cargo.toml` to depend on `xbrlkit-bdd-steps` with default features (no change in behavior)
- Add a CI job that builds `xbrlkit-bdd-steps` with minimal features to verify compile-time reduction

### Phase 2: Physical Crate Splitting (Decision Record)

After Phase 1, measure whether feature-gating is sufficient. Physical splitting into `xbrlkit-bdd-core`, `xbrlkit-bdd-validation`, `xbrlkit-bdd-cli` would:

**Pros:**
- True compile-time isolation: changing validation code doesn't recompile core steps
- Cleaner domain boundaries enforced by the compiler
- Smaller incremental builds for CI

**Cons:**
- `World` struct is a monolithic state bag — splitting it requires either:
  - Shared `World` with extension traits (complexity)
  - Per-crate `World` wrappers (duplication)
  - Dynamic dispatch for step handlers (runtime cost)
- `xbrlkit-bdd` would need to depend on all three crates
- Workspace crate count increases (maintenance surface)
- Current crate is `publish = false` — the split is only for compile time

**Decision criterion:** If Phase 1 feature-gating reduces clean-build time by <20%, proceed to Phase 2. Otherwise, document feature-gating as sufficient.

---

## Files to Modify

### Phase 1

| File | Change |
|------|--------|
| `crates/xbrlkit-bdd-steps/Cargo.toml` | Add `[features]` section, mark optional deps with `optional = true` |
| `crates/xbrlkit-bdd-steps/src/lib.rs` | Wrap domain handlers in `#[cfg(feature = ...)]` |
| `crates/xbrlkit-bdd/Cargo.toml` | No change (default features preserve behavior) |
| `xtask/Cargo.toml` | No change (doesn't depend on bdd-steps directly) |
| `.github/workflows/*.yml` | Add minimal-feature build job for `xbrlkit-bdd-steps` |
| `crates/xbrlkit-bdd-steps/README.md` | Document feature flags (create if absent) |

### Phase 2 (if triggered)

| File | Change |
|------|--------|
| `crates/xbrlkit-bdd-core/Cargo.toml` | New crate: core dependencies only |
| `crates/xbrlkit-bdd-core/src/lib.rs` | Core step handlers + base `World` |
| `crates/xbrlkit-bdd-validation/Cargo.toml` | New crate: validation domains |
| `crates/xbrlkit-bdd-validation/src/lib.rs` | Validation step handlers + extension traits |
| `crates/xbrlkit-bdd-cli/Cargo.toml` | New crate: CLI steps |
| `crates/xbrlkit-bdd-cli/src/lib.rs` | CLI step handlers |
| `crates/xbrlkit-bdd/Cargo.toml` | Update to depend on all three |
| `Cargo.toml` | Update workspace members |

---

## Test Strategy

1. **Regression:** Run `cargo test -p xbrlkit-bdd-steps` and `cargo test -p xbrlkit-bdd` — all existing tests must pass with default features
2. **Feature isolation:** Run `cargo check -p xbrlkit-bdd-steps --no-default-features` — must compile (with stub handlers)
3. **Feature combination:** Run `cargo check -p xbrlkit-bdd-steps --features dimensions,streaming` — must compile
4. **Integration:** Run `cargo test -p xbrlkit-bdd` with various feature combinations
5. **Benchmark:** Document `cargo build -p xbrlkit-bdd-steps` time before and after (use `cargo clean` + `time cargo build`)

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `#[cfg]` spaghetti in `lib.rs` makes code unreadable | Medium | Medium | Use module-level `cfg` instead of inline; extract domain modules to separate files |
| `World` struct changes break downstream | Low | High | Keep `World` fields additive; use `Option<T>` + `Default` pattern already in place |
| Feature combination explosion | Low | Medium | Limit to 6 features; CI tests default + minimal + each individual feature |
| Phase 2 splitting proves too complex | Medium | High | Decision record documents why feature-gating is sufficient; no sunk-cost fallacy |

---

## Estimated Effort

| Phase | Tasks | Estimate |
|-------|-------|----------|
| Phase 1 | Dependency audit, feature flag design, code modification, tests, CI update | 2-3 hours |
| Phase 2 | Decision record, optional crate splitting | 1-2 hours (if triggered) |
| **Total** | | **2-5 hours** |

---

## Decision Log

- **Feature-gating over immediate splitting:** The `World` struct is a monolithic state container shared across all step handlers. Physical splitting requires either architectural refactoring of `World` or accepting cross-crate coupling. Feature-gating achieves 80% of the compile-time benefit with 20% of the complexity.
- **Keep `cockpit-export`, `filing-load`, `edgar-attachments` in core:** These are small, single-purpose crates that are always exercised by the core BDD flow (filing validation → receipt → cockpit export). Moving them behind features would save negligible compile time.
- **`xbrl-contexts` appears in multiple features:** This is acceptable — Cargo deduplicates dependencies. The feature flags control which *code paths* compile, not which deps download.

---

## Next Steps

1. Await plan review by `reviewer-plan` agent
2. If approved, assign to `builder-implement` agent for Phase 1 execution
3. After Phase 1, run benchmark and make Phase 2 go/no-go decision
