# Agent: builder-implement

## Purpose
Implement the approved plan. Write code, tests, documentation per the plan.

## Trigger
- Issue labeled `repo-aligned`

## Preconditions
- Plan document exists and approved through all planning gates
- `repo-aligned` label present

## Steps
1. Read plan document `.mend/plans/ISSUE-{number}.md`
2. Create branch: `feat/ISSUE-{number}-{short-desc}`
3. Implement per plan:
   - Create/modify files as specified
   - Write unit tests
   - Write BDD scenarios if applicable
   - Update documentation
4. Run local validation:
   - `cargo build --workspace`
   - `cargo test --workspace`
   - `cargo clippy --workspace`
   - `cargo xtask alpha-check`
5. Commit with conventional commit format
6. Push branch
7. Create PR with:
   - Reference to issue
   - Summary of changes
   - Link to plan document
8. Label PR: `autonomous`, `ready-for-review`

## Output
**Complete**: Branch pushed, PR created, labeled `ready-for-review`
```
🤖 Implementation Complete

Branch: {branch-name}
PR: #{pr-number}
Status: ready-for-review

All local checks passed. Entering review pipeline.
```

**BLOCKED**: If implementation deviates significantly from plan
```
🤖 Implementation BLOCKED

Deviation from plan:
{explanation}

Requires plan revision or human decision.
```

## Safety
- Follows approved plan closely
- Local validation must pass before push
- PR includes plan reference
- Can escalate if plan insufficient
