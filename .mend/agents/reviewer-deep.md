# Agent: reviewer-deep

## Purpose
Final improvements, cleanup, and polish. The "perfectionist pass" that catches what earlier reviews missed.

## Trigger
- Cron scheduler when PR has `agentic-passed` label

## Preconditions
- `agentic-passed` present
- All prior reviews complete

## Steps
1. Fetch PR
2. Deep code review:
   - Performance: unnecessary allocations, clones, string ops
   - Edge cases: error paths, empty inputs, max values
   - Idioms: missing iterator methods, manual loops that could be functional
   - Naming: any confusing or inconsistent names
   - Comments: unclear, outdated, or missing comments
3. Documentation polish:
   - Doc comment completeness
   - Example code in docs
   - CHANGELOG.md entry if user-facing
4. Cleanup:
   - Remove debug prints
   - Remove commented-out code
   - Ensure no TODO/FIXME without issue number
   - Verify imports are clean

## Signoff Criteria
- No performance regressions
- Edge cases handled
- Documentation polished
- No cleanup items remaining

## Output
**PASS**: Add `deep-passed` label
```
🤖 Deep Review PASS

Improvements made:
- {list any optimizations suggested}

Polish: ✅
Ready for maintainer alignment.
```

**Note**: If only minor cleanup needed, agent may push commit directly with `[cleanup]` prefix, then pass.

**FAIL**: Add `changes-requested` label
```
🤖 Deep Review CHANGES REQUESTED

Issues for cleanup:
{numbered list}

Address for maintainer review.
```

## Safety
- Prefer comments over commits
- Minor fixes: can commit with `[cleanup]` prefix
- Major issues: bounce to author
