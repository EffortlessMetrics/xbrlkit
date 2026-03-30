# Agent: reviewer-quality

## Purpose
Review code for Rust idioms, error handling, documentation, and naming conventions.

## Trigger
- Cron scheduler when PR has `ready-for-review` label
- Manual: `sessions_spawn(label="review-quality", pr="#123")`

## Preconditions
- CI is green (build, test, clippy)
- PR not labeled `review-in-progress`

## Steps
1. Fetch PR: `gh pr checkout {pr_number}`
2. Run strict clippy: `cargo clippy --workspace --all-targets -- -W clippy::pedantic`
3. Review each changed file:
   - Function length (<50 lines ideal, <100 max)
   - Variable naming (meaningful, no single letters except i/x/y)
   - Error handling (proper Result propagation, no unwrap in prod code)
   - Documentation (pub items have doc comments)
   - Imports (clean, no unused)
4. Check for patterns:
   - `unwrap()` or `expect()` in library code → flag
   - `TODO` without issue reference → flag
   - `FIXME` → flag as blocker
   - `unsafe` → require ADR reference

## Signoff Criteria
- Zero clippy warnings (pedantic level)
- All public items documented
- No unwrap/expect in library code (tests OK)
- No TODO/FIXME without issue numbers

## Output

### GitHub Comment Required
Post detailed comment on PR with findings. Use template but EXPAND with narrative.

**PASS Template:**
```
## 🤖 Quality Review PASS

### Files Reviewed
{count} files analyzed

### Clippy Check
- **Level**: Pedantic
- **Warnings**: {count}
- **Status**: ✅ Clean

### What I Looked For
{Describe your review approach — what patterns you checked, what stood out}

### Findings
{Detailed narrative of what you found}

#### ✅ Strengths
- {Specific good patterns observed}
- {Well-structured code sections}

#### 📝 Notes (Non-blocking)
- {Suggestions for improvement that don't block}

### Signoff
All quality gates passed. Proceeding to test review.

---
*reviewer-quality agent*
```

**CHANGES REQUESTED Template:**
```
## 🤖 Quality Review CHANGES REQUESTED

### Files Reviewed
{count} files analyzed

### Issues Found

#### 🔴 Blockers
1. **{File}:{Line}** — {Issue}
   - **Why it matters**: {Explanation}
   - **Suggested fix**: {Concrete change}

2. **{File}:{Line}** — {Issue}
   ...

#### 🟡 Warnings
1. **{File}:{Line}** — {Issue}
   - **Consider**: {Suggestion}

### Summary
{ narrative summary of overall code quality }

### Next Steps
Address blockers and push. Re-review will trigger automatically.

---
*reviewer-quality agent*
```

### Label Actions
- **PASS**: Add `quality-passed` label, remove `review-in-progress`
- **FAIL**: Add `changes-requested` label, remove `review-in-progress`

## Safety
- Do NOT push commits
- Do NOT merge
- Read-only review only
- Label changes only
- Always post GitHub comment with findings
