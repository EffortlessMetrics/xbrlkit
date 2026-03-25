# Context Completeness Validation — Research Notes

**Issue:** #83  
**Date:** 2026-03-25  
**Status:** 🔍 Research → 📐 Plan

---

## Problem Statement

XBRL facts reference contexts via `context_ref` attributes. If a fact references a context that doesn't exist in the report, it's a data integrity error that can cause downstream processing failures.

**Example:**
```xml
<fact contextRef="ctx-2024">1000</fact>
<!-- But no context with id="ctx-2024" exists -->
```

## SEC/EFM Requirements

While not explicitly called out in EFM validation rules, context completeness is fundamental to XBRL validity:
- Every `contextRef` must resolve to a defined `<context>` element
- Context IDs are case-insensitive per XBRL 2.1 specification
- Missing contexts indicate malformed XBRL instance documents

## Implementation Options

### Option A: Standalone Crate (context-completeness)

**Pros:**
- Follows established pattern (numeric-rules, unit-rules)
- Clean separation of concerns
- Testable in isolation

**Cons:**
- Another small crate
- Requires integration step

### Option B: Extend xbrl-contexts Crate

**Pros:**
- Context validation belongs with context handling
- No new crate needed

**Cons:**
- Mixes parsing and validation concerns
- Less modular

### Decision: Option A

Following the precedent set by numeric-rules and unit-rules, create a dedicated `context-completeness` crate for validation logic.

## Validation Logic

```rust
pub fn validate_context_completeness(
    facts: &[Fact],
    contexts: &ContextSet,
) -> Vec<ValidationFinding> {
    // For each fact, check if context_ref exists in ContextSet
    // Generate finding if missing
}
```

## Finding Structure

- **Rule ID:** `SEC-CONTEXT-001`
- **Severity:** Error
- **Message:** "Fact references undefined context: '{context_ref}'"

## Edge Cases

1. **Case sensitivity:** XBRL context IDs are case-insensitive — "ctx-1" and "CTX-1" are the same
2. **Empty context_ref:** Facts may have no context_ref (should this be an error?)
3. **Duplicate context IDs:** First one wins (parsing behavior)

## Dependencies

- `xbrl-report-types` (for Fact, ValidationFinding)
- `xbrl-contexts` (for ContextSet)

## Related Work

- `crates/numeric-rules/` — pattern for validation crates
- `crates/unit-rules/` — pattern for validation crates
- `crates/xbrl-contexts/` — ContextSet data structure

## Next Steps

1. Create `crates/context-completeness/` crate
2. Implement `validate_context_completeness()` function
3. Add unit tests
4. Wire into `validation-run/src/lib.rs`
5. Add BDD scenarios
6. Register AC for alpha-check
