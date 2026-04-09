# Plan: [FR-011] Dimensional-rules unwrap panic risk

## Issue Reference
- Issue: #154
- Created: 2026-04-09

## Problem Statement
The `dimensional-rules` crate contains a potential panic risk at line 160 of `src/lib.rs`. The code uses `unwrap()` on a dimension lookup from a HashMap, which will cause a panic if the referenced dimension is not present in the taxonomy. This is production code that could crash when processing malformed or unexpected taxonomies.

## Goals
1. Remove the `unwrap()` call and implement proper error handling
2. Ensure graceful degradation when dimensions are missing
3. Add test coverage for the missing dimension scenario
4. Audit the crate for any other `unwrap()` calls in production code

## Approach
Replace the `unwrap()` with either:
- Option A: Propagate the error using `anyhow` for caller handling
- Option B: Log a validation finding and continue processing (graceful degradation)

The recommended approach is Option B (validation finding) as it aligns with the XBRL validation context and allows processing to continue while still reporting the issue.

## Implementation Steps
1. **Analyze the context** - Read the full function around line 160 to understand the flow
2. **Design the error handling** - Determine if this should be a hard error or validation finding
3. **Implement the fix** - Replace `unwrap()` with proper handling
4. **Add test case** - Create a test with a malformed taxonomy missing a dimension
5. **Audit for other unwraps** - Search the crate for other risky `unwrap()` calls
6. **Update documentation** - Document the new validation finding if applicable

## Files to Modify
- `crates/dimensional-rules/src/lib.rs` - Replace unwrap at line 160
- `crates/dimensional-rules/src/lib.rs` or `tests/` - Add test case for missing dimension
- `crates/dimensional-rules/CHANGELOG.md` - Document the fix

## Risks & Mitigation
| Risk | Mitigation |
|------|------------|
| Changing error handling behavior affects downstream callers | Review all call sites; maintain backward compatibility if possible |
| Missing edge cases in new error handling | Add comprehensive test coverage including malformed taxonomies |
| Other unwrap() calls in the crate | Audit and create follow-up issues if found |

## Testing Strategy
- **Unit tests**: Test the specific function with missing dimension scenarios
- **Integration tests**: Test with malformed XBRL taxonomies
- **BDD scenarios**: Add scenario for dimensional validation with missing dimensions

### Test Cases
1. Dimension exists - normal processing continues
2. Dimension missing - validation finding created, processing continues
3. Multiple missing dimensions - all reported, processing continues

## Definition of Done
- [ ] Implementation complete - unwrap removed, proper error handling added
- [ ] Unit tests passing for new error handling paths
- [ ] Integration tests passing with malformed taxonomies
- [ ] No other `unwrap()` calls exist in dimensional-rules production code
- [ ] Documentation updated (CHANGELOG, code comments if needed)
- [ ] PR reviewed and merged
