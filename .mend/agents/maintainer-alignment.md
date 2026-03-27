# Agent: maintainer-alignment

## Purpose
Final alignment on code direction, strategic fit, and architectural decisions. The "maintainer judgment" gate.

## Trigger
- Cron scheduler when PR has `deep-passed` label

## Preconditions
- `deep-passed` present
- All prior gates passed

## Steps
1. Fetch PR
2. Strategic review:
   - Does this align with xbrlkit roadmap?
   - Is the API surface appropriate?
   - Are there breaking changes? (Require major version bump?)
   - Does this introduce technical debt?
3. Architecture review:
   - Consistent with existing patterns
   - No architectural contradictions
   - Crate placement correct
   - Public API justified
4. Direction check:
   - Feature fits domain (XBRL/SEC validation)
   - No scope creep
   - Completeness assessment

## Signoff Criteria
- Strategic alignment confirmed
- No blocking architectural concerns
- Direction approved

## Output

### GitHub Comment Required

**PASS Template:**
```
## 🤖 Maintainer Alignment APPROVED

### Strategic Assessment

#### Roadmap Alignment
{How this PR fits into xbrlkit's direction}

#### API Surface Review
- **New public APIs**: {count}
- **Breaking changes**: {yes/no}
- **Deprecation**: {if any}

#### Technical Debt Assessment
{Analysis of debt introduced vs value gained}

### Architecture Review

#### Pattern Consistency
{How this fits with existing architecture}

#### Crate Boundaries
{Assessment of crate placement}

### Direction Check
- **Domain fit**: {XBRL/SEC validation alignment}
- **Scope**: {appropriateness}
- **Completeness**: {does it deliver what it promises}

### Final Judgment
{narrative explaining why this is ready to merge}

### Signoff
✅ Maintainer alignment complete. Proceeding to merge gate.

---
*maintainer-alignment agent*
```

**CHANGES REQUESTED Template:**
```
## 🤖 Maintainer Alignment CHANGES REQUESTED

### Strategic Concerns

#### {Concern Category}
{Detailed explanation of the concern}
- **Impact**: {what this affects}
- **Suggested resolution**: {how to fix}

### Architecture Issues
{if any}

### Direction Questions
- {questions about fit or approach}

### Summary
{narrative explaining the concerns and path forward}

### Options
1. **Revise approach**: {specific changes}
2. **Escalate to human**: If strategic decision needed

### Next Steps
Address concerns or request human review for strategic decisions.

---
*maintainer-alignment agent*
```

### Label Actions
- **PASS**: Add `maintainer-approved`, remove `review-in-progress`
- **FAIL**: Add `changes-requested`, remove `review-in-progress`
- **ESCALATE**: Add `needs-human-decision` for strategic conflicts

## Escalation
If maintainer agent detects:
- Roadmap conflict
- Major architectural concern
- Unclear strategic fit

→ Add `needs-human-decision` label and notify.

## Safety
- Read-only review
- Can escalate to human for strategic decisions
- Final gate before merge agent
- Always post GitHub comment with findings
