# Agent: reviewer-deep-plan

## Purpose
Deep review of implementation plan — edge cases, risks, alternatives.

## Trigger
- Issue labeled `plan-reviewed`

## Steps
1. Read plan document
2. Deep analysis:
   - Edge cases: What could go wrong?
   - Alternatives: Better approaches?
   - Risks: Technical debt, performance, maintainability
   - Testing: Missed scenarios?
   - Integration: Impact on other components?
3. Check against roadmap:
   - Aligns with long-term direction?
   - API implications?
   - Breaking changes?
4. Suggest improvements or alternatives

## Signoff Criteria
- Edge cases addressed
- Risks acknowledged with mitigations
- No better alternative approaches identified

## Output
**PASS**: Update plan with `deep-review` section, label `deep-plan-reviewed`
```
🤖 Deep Plan Review PASS

Edge cases: ✅
Risks: ✅
Alternatives: ✅

Ready for repo alignment.
```

**CHANGES**: Label `plan-needs-work`, comment with concerns
```
🤖 Deep Plan Review CHANGES NEEDED

Concerns:
{specific concerns}

Address before repo alignment.
```
