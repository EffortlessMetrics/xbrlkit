# agentic-final Micro-Agent

## Purpose
Final cross-cutting review of agentic systems before marking agentic phase complete.

## Trigger Label
- **Input:** `agentic-friction-passed`
- **Output:** `agentic-final-passed` OR `agentic-final-failed`

## Scope
Perform final validation of agentic system completeness:

1. **Integration Check**
   - All agentic components work together
   - No orphaned instrumentation
   - Consistent patterns across codebase

2. **Documentation**
   - Agent usage documented
   - Configuration options explained
   - Examples present for complex flows

3. **Completeness**
   - All agentic-observability items addressed
   - All agentic-friction items resolved
   - No TODOs/FIXMEs in agentic code

4. **Standards Compliance**
   - Follows project conventions
   - No redundant implementations
   - Proper module structure

## Commands

```bash
# Navigate to repo
cd /root/.openclaw/xbrlkit

# Check for TODOs/FIXMEs in agentic code
find . -name "*.rs" -type f | xargs grep -l "span\|instrument\|Agent\|Task" 2>/dev/null | xargs grep -n "TODO\|FIXME\|XXX" | head -20

# Verify no duplicate instrumentation
grep -r "#\[tracing::instrument" --include="*.rs" . | awk -F: '{print $1}' | sort | uniq -c | sort -rn | head -10

# Check module structure for agentic code
grep -r "mod.*agent\|mod.*task\|mod.*instrument" --include="*.rs" . | head -10
```

## Review Criteria

| Check | Pass Criteria |
|-------|---------------|
| No TODOs | No unresolved TODO/FIXME in agentic code |
| Complete coverage | All identified issues resolved |
| Consistent patterns | Follows established conventions |
| Documentation | Usage patterns documented |

## Label Transition

On success:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "agentic-final-passed"
gh pr edit <PR_NUMBER> --add-label "agentic-passed"
```

On failure:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "agentic-final-failed"
gh pr comment <PR_NUMBER> --body "## Agentic Final Review Failed

Issues remaining:
- [List issues]

Please address before proceeding to deep review."
```
