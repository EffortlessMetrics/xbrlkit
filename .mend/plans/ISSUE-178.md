# Plan: [Tech Debt] Refactor large BDD step handlers in xbrlkit-bdd-steps

**Issue:** [#178](https://github.com/EffortlessMetrics/xbrlkit/issues/178)  
**Status:** 📝 Draft  
**Created:** 2026-04-09

---

## Summary

The BDD step handlers in `crates/xbrlkit-bdd-steps/src/lib.rs` are excessively large and complex, with functions exceeding 200-400 lines and suppressed clippy warnings.

| Function | Lines | Suppression |
|----------|-------|-------------|
| `handle_given()` | ~400+ | `#[allow(clippy::too_many_lines)]` |
| `handle_when()` | ~300+ | `#[allow(clippy::too_many_lines)]` |
| `handle_then()` | ~200+ | `#[allow(clippy::too_many_lines)]` |
| `handle_parameterized_assertion()` | ~200+ | `#[allow(clippy::too_many_lines)]` |

---

## Goals

1. **Maintainability:** Break down monolithic handlers into smaller, testable functions
2. **Compilation Speed:** Reduce compilation time by eliminating monolithic structure
3. **Code Quality:** Remove clippy suppressions and follow Rust best practices
4. **Zero Regression:** Ensure no functional changes or test coverage loss

---

## Proposed Structure

```
crates/xbrlkit-bdd-steps/src/
├── lib.rs                    # Re-exports and module declarations
├── step_registry.rs          # Step registry pattern implementation
├── parser_utils.rs           # Common quote-parsing utilities
├── given_steps.rs            # Given step handlers
├── when_steps.rs             # When step handlers
├── then_steps.rs             # Then step handlers
└── parameterized_steps.rs    # Parameterized assertion handlers
```

---

## Implementation Strategy

### Phase 1: Analysis & Preparation
- [ ] Read current `lib.rs` to understand structure and dependencies
- [ ] Identify common patterns and extractable utilities
- [ ] Map step patterns to identify natural grouping boundaries

### Phase 2: Extract Common Utilities
- [ ] Create `parser_utils.rs` with shared quote-parsing logic
- [ ] Extract string matching helpers to reduce duplication

### Phase 3: Step Registry Pattern
- [ ] Create `step_registry.rs` implementing a registry pattern for step handlers
- [ ] Define traits for step handler types (Given, When, Then)

### Phase 4: Split Handlers
- [ ] Create `given_steps.rs` - migrate `handle_given()` content
- [ ] Create `when_steps.rs` - migrate `handle_when()` content
- [ ] Create `then_steps.rs` - migrate `handle_then()` content
- [ ] Create `parameterized_steps.rs` - migrate `handle_parameterized_assertion()` content

### Phase 5: Cleanup & Validation
- [ ] Remove `#[allow(clippy::too_many_lines)]` attributes
- [ ] Ensure all tests pass
- [ ] Verify no functional changes

---

## Acceptance Criteria

- [ ] All handler functions under 100 lines
- [ ] No `#[allow(clippy::too_many_lines)]` attributes remain
- [ ] All existing tests pass without modification
- [ ] Test coverage maintained at current level
- [ ] No functional changes to behavior
- [ ] Clippy passes without warnings on new code

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Breaking existing tests | Maintain exact function signatures during refactoring |
| Lost edge case handling | Add regression tests before refactoring |
| Merge conflicts with active work | Coordinate with team, target low-activity period |

---

## Notes

- This is a pure refactoring task - no functional changes expected
- Consider adding integration tests for step handlers if not present
- Document the step registry pattern for future maintainers

---

## Related

- Crate: `crates/xbrlkit-bdd-steps`
- Main file: `src/lib.rs`
