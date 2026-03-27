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
**PASS**: Add `maintainer-approved` label
```
🤖 Maintainer Alignment APPROVED

Strategic fit: ✅
Architecture: ✅
Direction: ✅

Proceeding to merge gate.
```

**FAIL**: Add `changes-requested` label
```
🤖 Maintainer Alignment CHANGES REQUESTED

Alignment issues:
{specific concerns}

Requires author attention before merge.
```

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
