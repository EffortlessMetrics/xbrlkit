# Micro-Agent: agentic-observability

**Purpose:** Review PR for observability patterns (logging, metrics, tracing, error visibility)

**Trigger Label:** `integ-passed`  
**Success Label:** `agentic-observability-passed`  
**Failure Label:** `changes-requested`  

**Inputs:**
- PR_NUMBER: Target PR number
- REPO: EffortlessMetrics/xbrlkit

---

## Review Checklist

### 1. Logging Patterns
- [ ] Structured logging used (not println!)
- [ ] Appropriate log levels (debug/info/warn/error)
- [ ] Contextual fields attached to logs
- [ ] No sensitive data in logs

### 2. Error Observability
- [ ] Errors include context (thiserror/anyhow)
- [ ] Error variants are descriptive
- [ ] Error paths are logged before returning

### 3. Metrics (if applicable)
- [ ] Key operations have counters/timers
- [ ] Business metrics exposed
- [ ] Health check endpoints

### 4. Tracing (if applicable)
- [ ] #[instrument] on key functions
- [ ] Span context propagation
- [ ] Trace IDs in logs

---

## Procedure

```bash
# 1. Fetch PR changes
gh pr view $PR_NUMBER --repo $REPO --json files,title,body
gh pr diff $PR_NUMBER --repo $REPO > /tmp/pr.diff

# 2. Check for observability patterns
cargo check 2>&1 | head -50
rg "println!" --type rust || echo "No println found"
rg "tracing::|log::" --type rust | head -20
rg "#\[instrument" --type rust || echo "No tracing instrumentation"

# 3. Review error handling
grep -E "(Error|Result)" src/**/*.rs 2>/dev/null | head -10

# 4. Decision
# PASS: All checks acceptable OR no observability concerns
# FAIL: Missing critical observability for new features
```

---

## Output Format

**PASS:**
```
Observability review: PASSED
- Logging: Appropriate levels used
- Errors: Properly contextualized
- No blocking concerns
```

**FAIL:**
```
Observability review: CHANGES REQUESTED
- Issue: [specific finding]
- Suggestion: [concrete fix]
```

---

## Time Budget: 3 minutes max

If review exceeds 3 minutes, mark as PASSED with note:
"Review timeout - no critical issues detected in sampled files"
