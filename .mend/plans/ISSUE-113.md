# Plan: Issue #113 — Activate SCN-XK-WORKFLOW-002

## Context
SCN-XK-WORKFLOW-002 is a BDD-only synthetic scenario that tests bundling an AC into a bounded context packet. It has no fixtures (by design) and is tested via BDD step handlers.

## Current State
- Scenario exists in `specs/features/workflow/bundle.feature` with `@alpha-active` tag
- Step handlers implemented in `crates/xbrlkit-bdd-steps/src/lib.rs`
- `AC-XK-WORKFLOW-002` commented out in `ACTIVE_ALPHA_ACS` because `test_ac` requires fixtures

## Problem
The alpha_check runs `test_ac` for all ACs in `ACTIVE_ALPHA_ACS`, but `test_ac` calls `execute_scenario` which fails if no fixtures are provided. BDD-only scenarios need to be excluded from `test_ac` and only run via the BDD runner.

## Solution
Modify `alpha_check.rs` to:
1. Separate fixture-based ACs from BDD-only ACs
2. Run `test_ac` only for fixture-based ACs
3. BDD runner already picks up `@alpha-active` scenarios automatically

## Files to Modify
- `xtask/src/alpha_check.rs` — restructure ACTIVE_ALPHA_ACS handling

## Validation
```bash
cargo test --workspace
cargo xtask alpha-check
```

## Acceptance Criteria
- [x] `cargo xtask alpha-check` passes
- [x] BDD scenarios with `@alpha-active` tag run successfully
- [x] No regression in existing AC tests

## Implementation
- PR #129 created with fix
- Branch: `feat/ISSUE-113-bundle-ac-bounded-context`
- Labels: `autonomous`, `ready-for-review`

## Status
**COMPLETE** - Awaiting review

## Notes
- WORKFLOW-003 (cockpit pack) has same issue — will be handled separately or together
- This is a test infrastructure fix, not a scenario implementation
