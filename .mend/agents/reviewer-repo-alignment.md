# Agent: reviewer-repo-alignment

## Purpose
Repository-level alignment check on the PLAN. Ensures proposed implementation fits structurally and conventionally with the existing codebase.

## Trigger
- Issue labeled `deep-plan-reviewed`

## Preconditions
- Plan document exists in `.mend/plans/`
- `deep-plan-reviewed` label present

## Steps
1. Read plan document `.mend/plans/ISSUE-{number}.md`
2. Structural alignment (planned):
   - Proposed file locations follow crate conventions
   - Module structure consistent with existing patterns
   - Naming conventions match repo style
3. Pattern consistency (planned):
   - Error handling approach matches existing code
   - Planned logging/tracing usage consistent
   - Testing patterns match existing tests
4. Convention compliance:
   - Follows `.github/CONTRIBUTING.md` if exists
   - File headers/license comments planned
5. Cross-reference check:
   - Similar features in codebase? (reuse opportunities)
   - Related modules need updates?
   - Impact on existing patterns

## Signoff Criteria
- Planned structure aligns with repo conventions
- Patterns are consistent
- No convention violations in the plan

## Output
**PASS**: Label `repo-aligned` on issue
```
🤖 Repo Alignment PASS

Planned structure: ✅
Patterns: ✅
Conventions: ✅

Ready for implementation (builder agent).
```

**CHANGES**: Label `plan-needs-work`
```
🤖 Repo Alignment CHANGES NEEDED

Alignment issues in plan:
{specific violations}

Revise plan before implementation.
```

## Safety
- Reviews PLAN only, not code
- Can bounce plan back to planning phase
- Focus on consistency with existing codebase
