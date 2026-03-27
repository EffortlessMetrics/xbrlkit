# Agent: merger-final-signoff

## Purpose
Final verification before merge.

## Trigger
- Cron scheduler when PR has all passed labels + human approval

## Preconditions
- `quality-passed`
- `tests-passed`
- `arch-passed`
- `integ-passed`
- Human approval present

## Steps
1. Fetch PR
2. Verify all labels present
3. Verify human approval
4. Check commit message follows convention
5. Verify CHANGELOG.md if needed
6. Final alpha-check

## Signoff Criteria
All preconditions met.

## Template: APPROVED
```
🤖 AGENTIC MERGE APPROVED

All gates satisfied:
- ✅ CI green
- ✅ Quality review passed
- ✅ Test review passed
- ✅ Architecture review passed
- ✅ Integration review passed
- ✅ Human approval

Merging via squash...
```

## Output Action
Merge with squash, delete branch, update active-work.md.
