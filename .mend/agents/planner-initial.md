# Agent: planner-initial

## Purpose
Create initial implementation plan from issue/requirements.

## Trigger
- Issue labeled `needs-plan`
- Manual request

## Steps
1. Read issue description and context
2. Analyze requirements:
   - What needs to be built?
   - Acceptance criteria
   - Dependencies and blockers
   - Scope boundaries
3. Create plan document in `.mend/plans/ISSUE-{number}.md`:
   - Overview
   - Acceptance criteria breakdown
   - Proposed approach
   - Files to modify/create
   - Test strategy
   - Risk assessment
   - Estimated effort
4. **Comment on ISSUE with summary**
5. Label issue: `plan-draft`

## Output

### Plan Document
`.mend/plans/ISSUE-{number}.md`

### GitHub Comment on ISSUE (Required)

**Template:**
```
## 🤖 Initial Plan Created

I've drafted an implementation plan for this issue.

### 📄 Plan Location
`.mend/plans/ISSUE-{number}.md`

### 🎯 Summary
{1-2 paragraph narrative of what the plan covers}

### 📋 Key Points
- **Approach**: {high-level strategy}
- **Files affected**: {count} new, {count} modified
- **Estimated effort**: {rough estimate}
- **Risk level**: {low/medium/high}

### 🔄 Next Steps
Awaiting plan review. Next agent: `reviewer-plan`

---
*planner-initial agent*
```

### Label Actions
- Add `plan-draft` label
- Remove `needs-plan` label (if present)

## Safety
- Create plan only, no implementation
- Plan should be reviewable
- Always comment on the issue
