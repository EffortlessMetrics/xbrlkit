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
5. **Comment on ISSUE with deep review findings**
6. Update plan with `deep-review` section

## Output

### GitHub Comment on ISSUE (Required)

**PASS Template:**
```
## 🤖 Deep Plan Review PASS

I've conducted a deep analysis of the implementation plan.

### 🔍 Deep Analysis

#### Edge Cases Considered
- {Edge case 1}: {how plan addresses it}
- {Edge case 2}: {how plan addresses it}

#### Risk Assessment
| Risk | Severity | Mitigation in Plan |
|------|----------|-------------------|
| {risk} | {level} | {mitigation} |

#### Alternatives Considered
{Better approaches evaluated and why current plan was chosen}

### 📝 Findings

#### ✅ Well Addressed
- {Aspects the plan handles well}

#### ⚠️ Watch During Implementation
- {Things to monitor}

### 🔄 Next Steps
Proceeding to repo alignment check. Next agent: `reviewer-repo-alignment`

---
*reviewer-deep-plan agent*
```

**CHANGES NEEDED Template:**
```
## 🤖 Deep Plan Review CHANGES NEEDED

I've identified concerns during deep analysis.

### 🔍 Concerns

#### {Category}
**Concern**: {specific issue}
- **Impact**: {what could go wrong}
- **Suggested revision**: {how to address}

### 📝 Risk Analysis

#### Unaddressed Risks
- {risks not covered in plan}

#### Alternative Approaches to Consider
- {better ways to solve the problem}

### 🔄 Next Steps
Revise plan to address concerns and re-tag with `plan-reviewed`.

---
*reviewer-deep-plan agent*
```

### Label Actions
- **PASS**: Add `deep-plan-reviewed` label
- **CHANGES**: Add `plan-needs-work` label

## Safety
- Deep analysis, not surface level
- Always comment on the issue
- Suggest alternatives when appropriate
