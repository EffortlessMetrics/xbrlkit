# agentic-friction Micro-Agent

## Purpose
Identify friction points and ergonomic issues in agentic interfaces and observability hooks.

## Trigger Label
- **Input:** `agentic-observability-passed`
- **Output:** `agentic-friction-passed` OR `agentic-friction-failed`

## Scope
Perform focused review on agentic system ergonomics:

1. **Instrumentation Friction**
   - Check span attribute naming consistency
   - Verify log level appropriateness (no spam, no silent failures)
   - Validate error context propagation

2. **API Ergonomics**
   - Agent/task creation ergonomics
   - Configuration discoverability
   - Sensible defaults present

3. **Debuggability**
   - Tracing covers key decision points
   - State transitions observable
   - Error paths traceable

4. **Cross-Cutting Concerns**
   - Cancellation handling
   - Timeout propagation
   - Resource cleanup

## Commands

```bash
# Navigate to repo
cd /root/.openclaw/xbrlkit

# Find agentic code (instrumentation, tracing, agent interfaces)
find . -name "*.rs" -type f | xargs grep -l "span\|instrument\|Agent\|Task" 2>/dev/null | head -20

# Check for tracing instrumentation
grep -r "#\[tracing::instrument" --include="*.rs" . | wc -l
grep -r "#\[instrument" --include="*.rs" . | wc -l

# Check span attribute patterns
grep -r "span.record\|span!" --include="*.rs" . | head -20

# Look for error handling in agentic contexts
grep -rn "thiserror\|anyhow" --include="*.rs" . | grep -i "agent\|task\|instrument" | head -20
```

## Review Criteria

| Check | Pass Criteria |
|-------|---------------|
| Consistent naming | Span attributes follow conventions |
| Appropriate verbosity | No excessive logging at info level |
| Error context | Errors carry sufficient context |
| Ergonomic APIs | Agent creation/configuration is clean |
| Observability gaps | Key paths have tracing coverage |

## Label Transition

On success:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "agentic-friction-passed"
```

On failure:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "agentic-friction-failed"
gh pr comment <PR_NUMBER> --body "## Agentic Friction Review Failed

Friction points identified:
- [List issues]

Please address and re-trigger agentic pipeline."
```
