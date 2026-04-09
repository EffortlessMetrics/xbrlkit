# deep-perf Micro-Agent

## Purpose
Performance review of code changes - identifying bottlenecks, unnecessary allocations, and optimization opportunities.

## Trigger Label
- **Input:** `agentic-passed`
- **Output:** `deep-perf-passed` OR `deep-perf-failed`

## Scope
Analyze PR for performance characteristics:

1. **Allocation Patterns**
   - Unnecessary String/Vec allocations
   - Clone-heavy code paths
   - Opportunities for borrowing
   - String vs &str usage

2. **Algorithmic Complexity**
   - O(n²) or worse loops
   - Repeated computations
   - Inefficient data structure choices
   - Iterator chain optimization

3. **Async/Await Efficiency**
   - Unnecessary .await points
   - Blocking operations in async
   - Spawn vs block_on usage
   - Cancellation safety

4. **I/O and Parsing**
   - Buffer sizing
   - Streaming vs buffering
   - Parse efficiency (nom, serde)
   - XML/XBRL specific optimizations

## Commands

```bash
# Navigate to repo
cd /root/.openclaw/xbrlkit

# Find potentially allocation-heavy code
grep -rn "\.clone()\|\.to_string()\|\.to_owned()" --include="*.rs" . | head -30

# Check for String where &str might suffice
grep -rn "fn.*String\|-> String" --include="*.rs" . | grep -v "//" | head -20

# Look for loop patterns that might be inefficient
grep -rn "for .* in \|\.iter()\|\.into_iter()" --include="*.rs" . | head -20

# Check async patterns
grep -rn "\.await\|spawn\|block_on" --include="*.rs" . | head -20

# Find collect() calls (potential allocation points)
grep -rn "\.collect::<\|\.collect()" --include="*.rs" . | head -20
```

## Review Criteria

| Check | Pass Criteria |
|-------|---------------|
| No obvious allocations | No unnecessary clones/tostring in hot paths |
| Efficient iterators | Iterator chains are optimized |
| Borrowing | &str preferred over String where possible |
| Async efficiency | No blocking in async, minimal await points |

## Label Transition

On success:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "deep-perf-passed"
```

On failure:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "deep-perf-failed"
gh pr comment <PR_NUMBER> --body "## Deep Performance Review Failed

Performance issues identified:
- [List issues with specific file:line references]

Please optimize before proceeding."
```
