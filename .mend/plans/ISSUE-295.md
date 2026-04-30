# Plan: Update xbrl-stream to use workspace thiserror 2.0.12 (Issue #295)

## Overview

This plan addresses Issue #295, which identifies a dependency version mismatch in the `xbrl-stream` crate. The workspace root specifies `thiserror = "2.0.12"` in `[workspace.dependencies]`, but `crates/xbrl-stream/Cargo.toml` declares `thiserror = "1.0"` directly without using the workspace dependency.

This inconsistency causes Cargo to compile two versions of `thiserror` during builds, increasing clean build time and binary size. The fix is straightforward: update `xbrl-stream` to use the workspace-defined `thiserror` version.

## Acceptance Criteria Breakdown

### AC-1: Update xbrl-stream/Cargo.toml
- **Requirement**: Change `thiserror = "1.0"` to `thiserror.workspace = true`
- **Location**: `crates/xbrl-stream/Cargo.toml`
- **Verification**: `cargo check` and `cargo build` pass without errors

### AC-2: Verify no duplicate thiserror versions
- **Requirement**: After the change, only one version of `thiserror` should appear in the dependency tree
- **Verification**: Run `cargo tree -p xbrl-stream -i thiserror` and confirm single version

### AC-3: Confirm API compatibility
- **Requirement**: The `xbrl-stream` crate compiles successfully with `thiserror` 2.0.12
- **Verification**: All existing tests in `xbrl-stream` pass

## Proposed Approach

### Background: thiserror 1.0 → 2.0 Migration

The `xbrl-stream` crate uses `thiserror` for a simple error enum (`StreamError`) with basic derive macro features:
- `#[derive(Debug, thiserror::Error)]`
- `#[error("...")]` message formatting
- `#[from]` attribute for automatic conversion
- `#[source]` for error chaining

These features are fully compatible between `thiserror` 1.0 and 2.0. The 2.0 release was primarily about:
1. Removing deprecated functionality from 1.0
2. MSRV (Minimum Supported Rust Version) changes
3. Internal improvements

Since the workspace already uses `thiserror` 2.0.12 successfully in other crates (`dimensional-rules`, `taxonomy-dimensions`, `taxonomy-loader`, `xbrl-contexts`), the migration risk is minimal.

### Implementation Steps

1. **Modify Cargo.toml**: Update the dependency declaration from `"1.0"` to `.workspace = true`
2. **Verify compilation**: Run `cargo check` and `cargo build` for the package
3. **Run tests**: Execute the test suite for `xbrl-stream`
4. **Verify deduplication**: Confirm only one `thiserror` version exists in the dependency tree

## Files to Modify/Create

### Modified Files (1)
1. **`crates/xbrl-stream/Cargo.toml`**
   - Change line: `thiserror = "1.0"` → `thiserror.workspace = true`
   - This aligns with other crates in the workspace that already use `thiserror.workspace = true`

### New Files (0)
No new files are required for this change.

## Test Strategy

### Build Verification
```bash
# Check compilation
cargo check -p xbrl-stream

# Full build
cargo build -p xbrl-stream

# Check entire workspace builds correctly
cargo build --workspace
```

### Test Execution
```bash
# Run xbrl-stream tests
cargo test -p xbrl-stream

# Run tests with all features (if applicable)
cargo test -p xbrl-stream --all-features
```

### Dependency Verification
```bash
# Verify no duplicate thiserror versions
cargo tree -p xbrl-stream -i thiserror

# The output should show only one version (2.0.12), not two
```

### Expected Test Results
All tests in `crates/xbrl-stream/src/lib.rs` should pass:
- `parses_simple_fact` - Basic fact parsing
- `parses_multiple_facts` - Multiple fact handling
- `handles_empty_xbrl` - Empty document handling
- `parses_context_definition` - Context parsing
- `parses_unit_definition` - Unit parsing

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Breaking API changes between thiserror 1.0 and 2.0 | Very Low | Medium | Test before/after; workspace already uses 2.0.12 elsewhere |
| Build failures in dependent crates | Low | Medium | Run full workspace build; CI will catch any issues |
| Runtime behavior changes | Very Low | High | `thiserror` 2.0 is primarily a cleanup release; no logic changes |
| Increased binary size (if migration fails) | Very Low | Low | Verify with `cargo tree -i thiserror` |

### Risk Notes
- The workspace already successfully uses `thiserror` 2.0.12 in multiple crates
- The `xbrl-stream` error types use only basic `thiserror` features that are stable across major versions
- No `#[backtrace]` or other advanced features are used that might have changed

## Estimated Effort

| Task | Estimate |
|------|----------|
| Modify Cargo.toml | 5 min |
| Verify compilation | 5 min |
| Run tests | 5 min |
| Verify dependency deduplication | 5 min |
| **Total** | **~20 minutes** |

## Implementation Commands

### One-line fix
```bash
# Edit crates/xbrl-stream/Cargo.toml
# Change:
#   thiserror = "1.0"
# To:
#   thiserror.workspace = true
```

### Verification commands
```bash
# Quick check
cargo check -p xbrl-stream

# Full verification
cargo test -p xbrl-stream
cargo tree -p xbrl-stream -i thiserror | grep "thiserror"
```

## Dependencies

- **Rust toolchain**: The workspace requires Rust 1.92+
- **Cargo**: For dependency resolution and building
- **Workspace thiserror**: Version 2.0.12 is already defined at root

## Rollback Plan

If issues are discovered:
1. Revert the change in `crates/xbrl-stream/Cargo.toml`
2. Restore `thiserror = "1.0"`
3. Rebuild to confirm the previous state works

---
*Plan created by planner-initial agent*
