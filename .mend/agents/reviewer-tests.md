# Agent: reviewer-tests

## Purpose
Verify test coverage and BDD alignment.

## Trigger
- Cron scheduler when PR has `quality-passed` label

## Steps
1. Fetch PR
2. Verify feature files have matching meta.yaml sidecars
3. Check BDD steps:
   - All Given/When/Then steps have handlers
   - No orphaned steps in lib.rs
   - New steps follow naming convention
4. Check unit tests:
   - New logic has unit tests
   - Edge cases covered (empty input, max values, errors)
   - No `#[ignore]` without reason in comment
5. Run: `cargo test --workspace`
6. Run: `cargo xtask alpha-check`

## Signoff Criteria
- All tests pass
- Alpha-check passes
- Feature file tags match meta.yaml
- No test gaps in new code

## Template: PASS
```
🤖 Test Review PASS

Scenarios checked: {count}
Unit tests: {count} new
Coverage: {assessment}
Alpha-check: ✅

Ready for next gate.
```

## Template: FAIL
```
🤖 Test Review CHANGES REQUESTED

Missing coverage:
{list of untested code paths}

Orphans:
{steps without handlers}

Fix and push for re-review.
```
