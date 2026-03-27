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
4. Label issue: `plan-draft`

## Output
Plan document + `plan-draft` label

## Template
```markdown
# Plan: {Issue Title}

## Overview
{summary}

## Acceptance Criteria
- [ ] AC-001: {description}
- [ ] AC-002: {description}

## Approach
{technical approach}

## Files
- Create: `path/to/new.rs`
- Modify: `path/to/existing.rs`

## Tests
- Unit tests for {component}
- BDD scenarios for {feature}

## Risks
- {risk and mitigation}

## Estimate
{rough estimate}
```

## Safety
- Create plan only, no implementation
- Plan should be reviewable
