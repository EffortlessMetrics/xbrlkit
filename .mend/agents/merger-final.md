# Agent: merger-final

## Purpose
Final verification and merge execution. The last gate — executes merge operation.

## Trigger
- Cron scheduler when PR has `maintainer-approved` label

## Preconditions (9 Gates)
- `quality-passed`
- `tests-passed`
- `arch-passed`
- `integ-passed`
- `agentic-passed`
- `deep-passed`
- `repo-aligned`
- `maintainer-approved`

## Steps
1. Fetch PR: `gh pr checkout {number}`
2. Verify all 9 gate labels present
3. Final CI check:
   - `cargo build --workspace`
   - `cargo test --workspace`
   - `cargo xtask alpha-check`
4. Verify commit message follows convention
5. Verify CHANGELOG.md updated if user-facing
6. Execute merge:
   - `gh pr merge {number} --squash --delete-branch`
7. Update `.mend/active-work.md` — move to merged section

## Signoff Criteria
All 8 prior gates passed + final CI green.

## Template: MERGED
```
🤖 AGENTIC MERGE COMPLETE

All 9 gates satisfied:
- ✅ CI green (initial)
- ✅ Quality review
- ✅ Test review
- ✅ Architecture review
- ✅ Integration review
- ✅ Agentic review
- ✅ Deep improvements
- ✅ Repo alignment
- ✅ Maintainer alignment

Merged: {merge-commit-sha}
Branch: deleted
```

## Output Actions
1. Add `agent-merge-approved` label
2. Merge with squash
3. Delete branch
4. Update active-work.md
5. Log to `.mend/merge-log.md`

## Safety
- Only executes if ALL 8 prior labels present
- Final CI must pass
- No human approval required — fully agentic
- Merge operation is atomic
