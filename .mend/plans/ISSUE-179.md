# Plan: [Tech Debt] Clean up unused dependencies found by cargo-machete

## Issue Reference
- Issue: #179
- Created: 2026-04-09

## Problem Statement
cargo-machete analysis found 10 potentially unused dependencies across 6 crates in the xbrlkit workspace. These unused dependencies increase build times, binary sizes, and maintenance overhead. The analysis may contain false positives (e.g., dependencies used via macros or feature flags) that need careful verification before removal.

## Goals
1. Verify each flagged dependency to confirm actual unused status
2. Remove confirmed unused dependencies from Cargo.toml files
3. Update Cargo.lock after dependency cleanup
4. Ensure all tests pass after cleanup

## Approach
1. Run cargo-machete locally to reproduce the findings
2. For each flagged dependency, verify usage in source code (direct usage, macros, feature flags)
3. Remove confirmed unused dependencies from relevant Cargo.toml files
4. Run cargo check and cargo test to ensure build integrity
5. Update Cargo.lock via cargo update or cargo metadata

## Implementation Steps

### Phase 1: Reproduce and Verify (1-2 hours)
1. Run `cargo machete` to confirm the reported findings
2. Examine each flagged crate's source code for actual dependency usage
3. Document which dependencies are safe to remove vs. false positives

| Crate | Flagged Dependency | Verification Status | Action |
|-------|-------------------|---------------------|--------|
| validation-run | context-completeness | Pending | TBD |
| dimensional-rules | serde | Pending | TBD |
| dimensional-rules | thiserror | Pending | TBD |
| receipt-types | serde_json | Pending | TBD |
| scenario-contract | serde_json | Pending | TBD |
| unit-rules | serde | Pending | TBD |
| xbrlkit-cli | render-md | Pending | TBD |
| xtask | sec-profile-types | Pending | TBD |
| xtask | serde_yaml | Pending | TBD |
| xtask | validation-run | Pending | TBD |
| xtask | walkdir | Pending | TBD |
| xtask | xbrl-report-types | Pending | TBD |

### Phase 2: Remove Unused Dependencies (1 hour)
1. Edit each crate's Cargo.toml to remove confirmed unused dependencies
2. Verify no remaining references in source code

### Phase 3: Build Verification (30 minutes)
1. Run `cargo check --workspace` to ensure clean build
2. Run `cargo test --workspace` to verify tests pass
3. Run `cargo clippy --workspace` to catch any issues

### Phase 4: Lockfile Update (15 minutes)
1. Run `cargo update` to refresh Cargo.lock
2. Verify no unintended side effects

## Files to Modify
- `crates/validation-run/Cargo.toml` - Remove context-completeness (if confirmed unused)
- `crates/dimensional-rules/Cargo.toml` - Remove serde, thiserror (if confirmed unused)
- `crates/receipt-types/Cargo.toml` - Remove serde_json (if confirmed unused)
- `crates/scenario-contract/Cargo.toml` - Remove serde_json (if confirmed unused)
- `crates/unit-rules/Cargo.toml` - Remove serde (if confirmed unused)
- `crates/xbrlkit-cli/Cargo.toml` - Remove render-md (if confirmed unused)
- `xtask/Cargo.toml` - Remove sec-profile-types, serde_yaml, validation-run, walkdir, xbrl-report-types (if confirmed unused)
- `Cargo.lock` - Auto-updated via cargo

## Risks & Mitigation

| Risk | Mitigation |
|------|------------|
| False positive (macro usage) | Search for `#[derive(` patterns and macro imports before removal |
| False positive (feature flags) | Check Cargo.toml for feature-gated dependencies |
| Build breakage after removal | Run full `cargo check` and `cargo test` before committing |
| Runtime-only dependencies | Verify dependencies aren't loaded dynamically at runtime |
| Test-only dependencies | Check dev-dependencies section and test imports |

## Testing Strategy
- **Unit tests:** Run `cargo test --workspace` to ensure no regressions
- **Integration tests:** Run focused acceptance tests via `cargo xtask test-ac`
- **Build verification:** `cargo check --workspace` and `cargo clippy --workspace`
- **Dependency verification:** Re-run `cargo machete` to confirm clean report

## Definition of Done
- [ ] All 10 flagged dependencies verified for actual usage
- [ ] Confirmed unused dependencies removed from respective Cargo.toml files
- [ ] `cargo check --workspace` passes without errors
- [ ] `cargo test --workspace` passes
- [ ] `cargo machete` reports no unused dependencies (or only documented false positives)
- [ ] Cargo.lock updated and committed
- [ ] PR created with summary of changes

## Notes
- cargo-machete can have false positives for dependencies used via derive macros (serde, thiserror)
- Consider adding cargo-machete to CI pipeline to prevent future accumulation (separate issue)
- Document any false positives found for future reference
