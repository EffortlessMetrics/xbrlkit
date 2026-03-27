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
8. **Comment on ISSUE: implementation starting**
9. **Comment on PR: implementation complete**
10. Label PR: `autonomous`, `ready-for-review`

## Output

### GitHub Comment on ISSUE (Required)

**Starting Implementation Template:**
```
## 🤖 Implementation Starting

Plan approved. Beginning implementation.

### 📋 Plan Reference
`.mend/plans/ISSUE-{number}.md`

### 🔄 Status
- **Branch**: `feat/ISSUE-{number}-{desc}` (creating)
- **Phase**: Implementation in progress
- **ETA**: {rough estimate}

Will update this issue when PR is ready.

---
*builder-implement agent*
```

### GitHub Comment on PR (Required)

**Implementation Complete Template:**
```
## 🤖 Implementation Complete

I've implemented the approved plan.

### 📋 Implementation Summary

#### Changes Made
- **Files created**: {count}
- **Files modified**: {count}
- **Tests added**: {count}

#### Plan Adherence
{How closely the implementation followed the plan}

#### Deviations (if any)
- {Any changes from plan and why}

### ✅ Local Validation
- Build: ✅
- Tests: ✅
- Clippy: ✅
- Alpha-check: ✅

### 📄 Related
- **Issue**: #{issue-number}
- **Plan**: `.mend/plans/ISSUE-{number}.md`

### 🔄 Next Steps
Entering review pipeline. Next agent: `reviewer-quality`

---
*builder-implement agent*
```

**BLOCKED Template (on Issue):**
```
## 🤖 Implementation BLOCKED

Cannot proceed with implementation as planned.

### ❌ Blocker
{Why implementation is blocked}

### 📝 Deviation Required
{What would need to change}

### 🔄 Options
1. Revise plan and re-approve
2. Escalate to human for decision

---
*builder-implement agent*
```

### Label Actions
- Add `building` label to issue
- Create PR with `autonomous`, `ready-for-review` labels
- Remove `building` label when complete

## Safety
- Follows approved plan closely
- Local validation must pass before push
- PR includes plan reference
- Can escalate if plan insufficient
- Always comment on BOTH issue and PR
