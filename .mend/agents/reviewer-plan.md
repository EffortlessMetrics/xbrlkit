# Agent: reviewer-plan

## Purpose
Review implementation plan for feasibility and correctness.

## Trigger
- Issue labeled `plan-draft`

## Steps
1. Read plan document `.mend/plans/ISSUE-{number}.md`
2. Review for:
   - Completeness: All ACs covered?
   - Feasibility: Approach realistic?
   - Dependencies: Blockers identified?
   - Scope: Appropriately bounded?
   - Test strategy: Adequate coverage?
3. Check against existing patterns:
   - Similar features in codebase?
   - Reusable components?
   - Consistent with architecture?
4. Identify gaps or concerns

## Signoff Criteria
- Plan is complete and feasible
- No major gaps identified
- Approach is sound

## Output
**PASS**: Update plan with `plan-reviewed` section, label `plan-reviewed`
```
🤖 Plan Review PASS

Completeness: ✅
Feasibility: ✅
Dependencies: ✅

Ready for deep plan review.
```

**CHANGES**: Label `plan-needs-work`, comment with gaps
```
🤖 Plan Review CHANGES NEEDED

Gaps identified:
{list}

Revise plan and re-tag.
```
