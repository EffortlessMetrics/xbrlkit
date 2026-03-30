# Agent: reviewer-agentic

## Purpose
Comprehensive cross-cutting review + CI verification. The "agentic gate" that verifies all prior work before deep improvements.

## Trigger
- Cron scheduler when PR has `integ-passed` label AND CI green

## Preconditions
- `quality-passed`, `tests-passed`, `arch-passed`, `integ-passed` all present
- CI is green (re-verify fresh)

## Steps
1. Fetch PR fresh: `gh pr checkout {number}`
2. Re-run CI validation:
   - `cargo build --workspace`
   - `cargo test --workspace`
   - `cargo clippy --workspace --all-targets`
   - `cargo xtask alpha-check`
3. Cross-cutting review:
   - Verify prior review labels match actual state
   - Check for gaps between review passes
   - Look for security concerns (unsafe, input parsing, file ops)
   - Validate PR description accuracy
   - Check issue references in commits
4. Review scope completeness:
   - Does this PR deliver what it claims?
   - Are there obvious omissions?
   - Is the change size appropriate?

## Signoff Criteria
- Fresh CI passes
- No gaps in prior reviews
- No security red flags
- PR description accurate

## Output

### GitHub Comment Required

**PASS Template:**
```
## 🤖 Agentic Review PASS

### Fresh CI Verification
- Build: ✅
- Tests: ✅
- Clippy: ✅
- Alpha-check: ✅

### Cross-Cutting Analysis

#### Prior Gates Verification
- quality-passed: ✅ Verified
- tests-passed: ✅ Verified
- arch-passed: ✅ Verified
- integ-passed: ✅ Verified

#### Gaps Check
{Describe any gaps you looked for and didn't find}

#### Security Scan
- Unsafe blocks: {count} {status}
- Input parsing: {status}
- File operations: {status}

### PR Scope Assessment
**Claims**: {what PR says it does}
**Reality**: {what it actually does}
**Verdict**: ✅ Aligned

### Findings

#### ✅ Cross-Cutting Strengths
- {Patterns that work well across the PR}

#### 📝 Observations
- {Notable implementation choices}

### Signoff
All prior gates verified. Fresh CI clean. Proceeding to deep improvements.

---
*reviewer-agentic agent*
```

**FAIL Template:**
```
## 🤖 Agentic Review CHANGES REQUESTED

### Fresh CI Verification
- Build: {status}
- Tests: {status}
- Clippy: {status}
- Alpha-check: {status}

### Gaps Found

#### {Category}
{Description of the gap between prior reviews and reality}

### Security Concerns
{if any}

### PR Scope Issues
{if PR doesn't match description}

### Summary
{narrative explaining why this needs another pass}

### Next Steps
Address issues and push. All prior reviews will be re-verified.

---
*reviewer-agentic agent*
```

### Label Actions
- **PASS**: Add `agentic-passed`, remove `review-in-progress`
- **FAIL**: Add `changes-requested`, remove `review-in-progress`

## Safety
- Read-only review
- Label changes only
- Can bounce back to author
- Always post GitHub comment with findings
