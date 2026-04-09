# deep-edge Micro-Agent

## Purpose
Edge case and boundary condition review - identifying potential panics, overflow scenarios, and robustness issues.

## Trigger Label
- **Input:** `deep-perf-passed`
- **Output:** `deep-edge-passed` OR `deep-edge-failed`

## Scope
Analyze PR for edge cases and boundary conditions:

1. **Panic Sources**
   - unwrap() / expect() calls
   - Index out of bounds: arr[i], slice[idx]
   - Division by zero risks
   - Integer overflow (wrapping vs checked)

2. **Input Validation**
   - Empty input handling
   - Maximum size limits
   - Malformed data handling
   - Unicode/encoding edge cases

3. **Resource Limits**
   - Stack overflow risks (deep recursion)
   - Memory exhaustion vectors
   - File handle limits
   - Timeout handling

4. **XBRL-Specific Edge Cases**
   - Empty XBRL fragments
   - Circular references in networks
   - Extremely deep nesting
   - Malformed XML entities

## Commands

```bash
# Navigate to repo
cd /root/.openclaw/xbrlkit

# Find unwrap() calls (potential panic sources)
grep -rn "\.unwrap()\|\.expect(" --include="*.rs" . | grep -v "test" | head -30

# Check for indexing operations
grep -rn "\[.*\]" --include="*.rs" . | grep -v "//\|test\|#\[" | head -30

# Find arithmetic operations that might overflow
grep -rn "\s*+\s*\|[^/]/[^/]\|\*\|%-" --include="*.rs" . | grep -v "//\|test" | head -30

# Check for division operations
grep -rn "/ " --include="*.rs" . | grep -v "//\|/\*\|//!" | head -20

# Look for recursion patterns
grep -rn "fn.*->.*Self\|recursion" --include="*.rs" . | head -20

# Find collect() with potential large allocations
grep -rn "\.collect::<Vec" --include="*.rs" . | head -20

# Check for file operations without error handling
grep -rn "fs::\|File::open\|read_to_string" --include="*.rs" . | grep -v "?\|match\|if let" | head -20
```

## Review Criteria

| Check | Pass Criteria |
|-------|---------------|
| Panic safety | No unwrap/expect in production code; use ? or proper error handling |
| Bounds checking | Array/slice access is bounds-checked or validated |
| Overflow safety | Arithmetic uses checked_* or saturating_* where overflow possible |
| Input validation | All public functions validate inputs before processing |
| Recursion safety | No unbounded recursion; tail recursion or iteration preferred |

## Label Transition

On success:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "deep-edge-passed"
```

On failure:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "deep-edge-failed"
gh pr comment <PR_NUMBER> --body "## Deep Edge Case Review Failed

Edge case issues identified:
- [List issues with specific file:line references]

Please add proper error handling and validation before proceeding."
```
