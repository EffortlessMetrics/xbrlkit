# Plan: Extract duplicate sanitize_for_rule_id utility

## Issue Reference
- **Issue:** #153
- **Title:** [FR-010] Extract duplicate sanitize_for_rule_id utility
- **Type:** Technical Debt / Refactoring

## Problem Statement
The `sanitize_for_rule_id` function is duplicated identically in two locations:
- `crates/numeric-rules/src/lib.rs` (lines 114-124)
- `xtask/src/main.rs` (lines 297-307)

This violates the DRY principle and increases maintenance burden.

## Current State
```rust
fn sanitize_for_rule_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}
```

## Proposed Solution
Extract the function to a shared utility location in `xbrlkit-core` crate.

## Changes Required

### 1. Add to xbrlkit-core
- Create or update utility module in `crates/xbrlkit-core/src/`
- Add `sanitize_for_rule_id` function
- Export function publicly

### 2. Update numeric-rules crate
- Remove local implementation from `crates/numeric-rules/src/lib.rs`
- Update imports to use shared version from `xbrlkit-core`

### 3. Update xtask
- Remove local implementation from `xtask/src/main.rs`
- Update imports to use shared version from `xbrlkit-core`

## Acceptance Criteria
- [ ] Single source of truth for `sanitize_for_rule_id`
- [ ] Both `numeric-rules` and `xtask` updated to use shared version
- [ ] No functional changes to behavior
- [ ] All existing tests pass

## Estimated Complexity
Low - straightforward code move with import updates.
