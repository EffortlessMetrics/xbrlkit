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
6. **Comment on ISSUE with alignment findings**

## Output

### GitHub Comment on ISSUE (Required)

**PASS Template:**
```
## 🤖 Repo Alignment PASS

I've verified the plan aligns with repository patterns and conventions.

### 📁 Structural Alignment

#### File Locations
- **Proposed locations**: ✅ Follow crate conventions
- **Module structure**: ✅ Consistent with existing patterns

#### Naming Conventions
- **Style match**: ✅ Aligns with repo conventions

### 🔄 Pattern Consistency

#### Error Handling
{Assessment of planned error handling approach}

#### Testing Patterns
{How planned tests align with existing patterns}

### 📋 Convention Compliance
- **CONTRIBUTING.md**: ✅ Follows guidelines
- **License headers**: ✅ Planned appropriately

### 🔗 Cross-References
- **Similar features**: {identified for reference}
- **Related modules**: {that may need updates}
- **Reuse opportunities**: {identified}

### 📝 Assessment
{narrative about how well the plan fits the codebase}

### 🔄 Next Steps
✅ Repo alignment complete. Plan approved for implementation.

Builder agent will create branch and PR. Next agent: `builder-implement`

---
*reviewer-repo-alignment agent*
```

**CHANGES NEEDED Template:**
```
## 🤖 Repo Alignment CHANGES NEEDED

The plan has alignment issues with repository conventions.

### 📁 Structural Issues

#### {Issue}
- **Current plan**: {what's proposed}
- **Repo convention**: {what's expected}
- **Suggested change**: {how to align}

### 🔄 Pattern Inconsistencies

#### {Pattern}
- **Plan approach**: {what's planned}
- **Existing approach**: {what repo uses}
- **Recommendation**: {how to match}

### 📝 Summary
{narrative explaining the alignment gaps}

### 🔄 Next Steps
Revise plan for alignment and re-tag with `deep-plan-reviewed`.

---
*reviewer-repo-alignment agent*
```

### Label Actions
- **PASS**: Add `repo-aligned` label
- **CHANGES**: Add `plan-needs-work` label

## Safety
- Reviews PLAN only, not code
- Can bounce plan back to planning phase
- Focus on consistency with existing codebase
- Always comment on the issue
