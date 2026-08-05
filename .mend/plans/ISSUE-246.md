# ISSUE-246: Missing feature file: package_check.feature

## Problem Statement
The spec_ledger.yaml tracks AC-XK-WORKFLOW-004 ("Verify publishable crates package for crates.io"), and issue #218 references a file with scenario SCN-XK-WORKFLOW-006. However, the file does not exist in the repository.

## Proposed Solution
1. Create `specs/features/workflow/package_check.feature` with the package check scenario
2. Implement the 3 missing BDD step handlers (Given/When/Then for cargo publish --dry-run workflow)
3. Add `package-check` tag once the file and handlers exist

## Affected Files
- `specs/features/workflow/package_check.feature` (to create)
- `specs/features/workflow/package_check.meta.yaml` (to create)
- `crates/xbrlkit-bdd-steps/src/lib.rs` (add step handlers)
- `specs/spec_ledger.yaml` (verify reference)

## Related
- Issue #218 (tracks activation of this scenario)
- AC-XK-WORKFLOW-004 in spec_ledger.yaml

## Estimated Effort
Small — 2-3 hours. Creating a feature file + 3 simple step handlers that delegate to existing `cargo package` logic.

## Acceptance Criteria
- [ ] `package_check.feature` exists at expected location
- [ ] Scenario SCN-XK-WORKFLOW-006 is defined with proper tags
- [ ] All 3 BDD steps have handlers in `xbrlkit-bdd-steps`
- [ ] Running `cargo xtask bdd --tags @alpha-active` includes this scenario and it passes
- [ ] `package-check` tag is present on the scenario

---

## Deep Review

**Reviewer:** reviewer-deep-plan agent  
**Date:** 2026-04-23  
**Verdict:** CHANGES NEEDED  

### Critical Finding: Factual Premise Is Incorrect

After verifying against the actual codebase, the core claim of this issue — that `package_check.feature` does not exist — is **false**.

| Claim | Actual State |
|-------|-------------|
| File does not exist | ✅ EXISTS at `specs/features/workflow/package_check.feature` (commit `94d1f54`, 2026-03-30) |
| Sidecar missing | ✅ EXISTS at `specs/features/workflow/package_check.meta.yaml` |
| Spec ledger has dangling reference | ✅ CORRECTLY references `specs/features/workflow/package_check.feature` |
| No xtask logic | ✅ `PackageCheck` command + `package_check()` function fully implemented in `xtask/src/main.rs` |

### What Is Actually Missing

The **real** gap is in the BDD step handler layer (`crates/xbrlkit-bdd-steps/src/lib.rs`). The 3 scenario steps have **no handlers**:

1. `Given the publishable workspace crates declare crates.io-compatible manifests` — no `handle_given` match
2. `When I run the package readiness check` — no `handle_when` match
3. `Then the publishable workspace crates package successfully` — no `handle_then` match

Additionally, the scenario lacks the `@alpha-active` tag (only has `@speed.fast`), which is why it cannot be executed end-to-end via `cargo xtask bdd --tags @alpha-active`.

### Recommendation

1. **Do not create the feature file** — it already exists.
2. **Implement 3 BDD step handlers** in `xbrlkit-bdd-steps/src/lib.rs` that delegate to the existing `package_check()` logic (or replicate its `cargo metadata` → `cargo package --allow-dirty --locked --list` approach).
3. **Add `@alpha-active` tag** once handlers are in place.
4. **Consider closing this issue** and transferring remaining work to #218, which more accurately describes the actual problem (unactivated scenario with missing BDD handlers).

### Watchpoints for Implementer

- The existing `package_check()` uses `--list` (dry-run), not actual packaging. The `Then` handler should verify this dry-run succeeds.
- `package_is_publishable()` filters out workspace-only crates (`publish: Some([])`). The `Given` handler should verify this filtering.
- BDD runner routes through `handle_given` → `handle_when` → `handle_then`. New handlers must return `Ok(true)` (Given/When) or `Ok(())` (Then) to avoid falling through to `unsupported BDD step`.
- `--allow-dirty` may mask uncommitted changes. Consider whether this flag is appropriate for CI contexts.

### Effort Re-estimate

**1-2 hours** (down from 2-3). File and xtask logic already exist; only BDD handlers + tag changes needed.

### Risk Assessment

- **Low risk**. Existing `package_check()` has unit tests in `xtask/src/main.rs`.
- **No hidden dependencies**. Workflow-layer scenario; does not touch core parsing/validation.

### Code Verification Details

- **File path check:** `specs/features/workflow/package_check.feature` — confirmed present, lines 1-14 define Scenario SCN-XK-WORKFLOW-006.
- **Sidecar check:** `specs/features/workflow/package_check.meta.yaml` — confirmed present, maps to crates `[xtask, xbrlkit-core]`.
- **Spec ledger check:** `specs/spec_ledger.yaml` — confirmed AC-XK-WORKFLOW-04 points to correct file path.
- **Step handler check:** `crates/xbrlkit-bdd-steps/src/lib.rs` — searched `handle_given`, `handle_when`, `handle_then` bodies; no match for any of the 3 package-check step texts.
- **xtask logic check:** `xtask/src/main.rs` — `package_check()`, `publishable_packages()`, `run_cargo_package()`, `package_is_publishable()` all present and tested (lines 196-264 approximately).
- **Tag check:** `@alpha-active` is NOT present on SCN-XK-WORKFLOW-006; only `@speed.fast`.
