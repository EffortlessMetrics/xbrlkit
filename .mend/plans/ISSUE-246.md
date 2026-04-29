# ISSUE-246 Plan: Missing BDD Step Handlers for package_check.feature

## Status
- **Issue:** #246 — `[Scout] Missing feature file: package_check.feature referenced but does not exist`
- **Related:** #218 — `[Scout] Unactivated scenario: package_check.feature has 1 scenario (AC-XK-WORKFLOW-004) without @alpha-active`
- **Plan created:** 2026-04-28
- **Risk level:** Low

## Discovery

The scout report in #246 states the feature file is missing, but the file **does exist** at:

```
specs/features/workflow/package_check.feature
```

The actual gap is that the **BDD step handlers** for the scenario's three steps are **not implemented** in `crates/xbrlkit-bdd-steps/src/lib.rs`. The `xtask` crate already contains the working `package_check()` function that runs `cargo package --allow-dirty --locked --list` for each publishable crate. The BDD framework (`xbrlkit-bdd` + `xbrlkit-bdd-steps`) cannot execute this scenario end-to-end because the Given/When/Then mappings are absent.

## Current State

### Feature file (exists)
```gherkin
@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Package check

  @AC-XK-WORKFLOW-004
  @SCN-XK-WORKFLOW-006
  @speed.fast
  Scenario: Verify publishable crates package for crates.io
    Given the publishable workspace crates declare crates.io-compatible manifests
    When I run the package readiness check
    Then the publishable workspace crates package successfully
```

### Spec ledger (correctly references the file)
- `AC-XK-WORKFLOW-004` → `specs/features/workflow/package_check.feature`

### Missing: 3 BDD step handlers
| Step | Type | Status |
|------|------|--------|
| `Given the publishable workspace crates declare crates.io-compatible manifests` | Given | **MISSING** |
| `When I run the package readiness check` | When | **MISSING** |
| `Then the publishable workspace crates package successfully` | Then | **MISSING** |

### Existing implementation
- `xtask/src/main.rs:fn package_check()` — runs `cargo metadata` to find publishable crates, then `cargo package --allow-dirty --locked --list` for each.

## Approach

Bridge the existing `package_check()` logic into the BDD step framework by adding three handlers to `crates/xbrlkit-bdd-steps/src/lib.rs`:

1. **Given** — verify publishable crates exist (reuse `publishable_packages()` logic, or set up a `package_check_context` on `World`).
2. **When** — run the package readiness check (invoke the cargo packaging logic, capturing success/failure).
3. **Then** — assert all publishable crates packaged successfully.

After handlers are in place, add `@alpha-active` tag to `package_check.feature` so the scenario is picked up by the BDD runner.

## Files Affected

### Modified (3 files)
1. `crates/xbrlkit-bdd-steps/src/lib.rs` — add 3 step handlers
2. `specs/features/workflow/package_check.feature` — add `@alpha-active` tag
3. `crates/xbrlkit-bdd-steps/Cargo.toml` — may need `cargo_metadata` dependency (if reusing logic inline; otherwise keep as-is)

### No new files needed

## Implementation Details

### Step 1: Add `package_check_context` to `World`

```rust
#[derive(Debug, Clone, Default)]
pub struct PackageCheckContext {
    pub packages: Vec<String>,
    pub results: Vec<Result<(), String>>,
}
```

Add field `pub package_check_context: PackageCheckContext` to `World` and initialize in `World::new`.

### Step 2: Given handler

```rust
if step.text == "the publishable workspace crates declare crates.io-compatible manifests" {
    let packages = publishable_packages_from_metadata(&world.repo_root)?;
    if packages.is_empty() {
        anyhow::bail!("no publishable workspace crates found");
    }
    world.package_check_context.packages = packages;
    return Ok(true);
}
```

Extract `publishable_packages_from_metadata()` by factoring out the `cargo metadata` logic from `xtask/src/main.rs` into a shared helper, or inline a simplified version.

### Step 3: When handler

```rust
if step.text == "I run the package readiness check" {
    let packages = world.package_check_context.packages.clone();
    if packages.is_empty() {
        anyhow::bail!("no packages to check; Given step may be missing");
    }
    for package in &packages {
        let result = run_cargo_package_check(&world.repo_root, package);
        world.package_check_context.results.push(result);
    }
    return Ok(true);
}
```

### Step 4: Then handler

```rust
if step.text == "the publishable workspace crates package successfully" {
    let failures: Vec<_> = world
        .package_check_context
        .results
        .iter()
        .filter_map(|r| r.as_ref().err().cloned())
        .collect();
    if !failures.is_empty() {
        anyhow::bail!(
            "package check failed for {} crate(s): {}",
            failures.len(),
            failures.join("; ")
        );
    }
    return Ok(());
}
```

### Step 5: Add `@alpha-active`

Insert `@alpha-active` tag in `package_check.feature` alongside existing `@speed.fast`.

## Estimated Effort

- **Small** (~2–4 hours). The core logic already exists in `xtask`. The work is plumbing it into the BDD step framework and adding one struct field to `World`.

## Risk Assessment

- **Low.** No new dependencies required (already using `cargo` via `std::process::Command`). The `xtask` `package_check()` function is battle-tested. Risk is limited to BDD runner integration, which follows the exact same patterns as existing handlers in `lib.rs`.

## Acceptance Criteria

- [ ] `crates/xbrlkit-bdd-steps/src/lib.rs` handles all 3 steps for `SCN-XK-WORKFLOW-006`
- [ ] `cargo test -p xbrlkit-bdd-steps` passes (or relevant test suite)
- [ ] `xtask package-check` still works independently
- [ ] `xtask bdd --tags @alpha-active` selects and executes `SCN-XK-WORKFLOW-006`
- [ ] `@alpha-active` tag present on the scenario
- [ ] Issue #218 can be closed as resolved

## Next Steps

Awaiting plan review. Next agent: `reviewer-plan`

## plan-reviewed

- **Reviewer:** reviewer-plan agent
- **Date:** 2026-04-28
- **Result:** PASS
- **Findings:**
  - Plan correctly reframes the issue from "missing file" to "missing step handlers."
  - Approach is low-risk: bridges existing, battle-tested `xtask` logic into BDD framework.
  - Scope is appropriately bounded at 3 handlers + 1 tag + optional dependency.
  - One open decision: `cargo_metadata` dependency vs. inlined `cargo metadata` command. Implementer should choose and document.
  - Consider adding unit tests for error paths (empty package list, manifest parse failure) if not covered by BDD runner.

---
*planner-initial agent*
