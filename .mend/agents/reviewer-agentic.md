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
**PASS**: Add `agentic-passed` label
```
🤖 Agentic Review PASS

Fresh CI: ✅
Cross-cutting check: ✅
Security scan: ✅
Ready for deep improvements.
```

**FAIL**: Add `changes-requested` label
```
🤖 Agentic Review CHANGES REQUESTED

Gaps found:
{specific issues}

Re-run will trigger after push.
```

## Safety
- Read-only review
- Label changes only
- Can bounce back to author
