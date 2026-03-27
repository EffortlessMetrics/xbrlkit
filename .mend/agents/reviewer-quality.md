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
**PASS**: Add `quality-passed` label, comment with 🤖 Quality Review PASS template
**FAIL**: Add `changes-requested` label, comment with specific issues

## Template: PASS
```
🤖 Quality Review PASS

Files reviewed: {count}
Clippy warnings: {count}
Notes: {any non-blocking observations}

Ready for next gate.
```

## Template: CHANGES REQUESTED
```
🤖 Quality Review CHANGES REQUESTED

Issues found:
{numbered list with file:line references}

Action: Address and push. Re-review will trigger automatically.
```

## Safety
- Do NOT push commits
- Do NOT merge
- Read-only review only
- Label changes only
