# Agent: reviewer-repo-alignment

## Purpose
Repository-level alignment check. Ensures PR fits structurally and conventionally with the existing codebase.

## Trigger
- Cron scheduler when PR has `deep-passed` label

## Preconditions
- `deep-passed` present
- All prior gates passed

## Steps
1. Fetch PR
2. Structural alignment:
   - File locations follow crate conventions
   - Module structure consistent with existing patterns
   - Naming conventions match repo style
   - Import organization (std, external, crate, super)
3. Pattern consistency:
   - Error handling patterns match existing code
   - Logging/tracing usage consistent
   - Serialization patterns consistent
   - Testing patterns match existing tests
4. Convention compliance:
   - Follows `.github/CONTRIBUTING.md` if exists
   - Commit message format
   - Branch naming (if checkable)
   - File headers/license comments
5. Cross-reference check:
   - Similar code exists elsewhere? (consistency)
   - Related modules updated if needed
   - Documentation cross-references valid

## Signoff Criteria
- Structural alignment verified
- Patterns consistent with repo conventions
- No convention violations

## Output
**PASS**: Add `repo-aligned` label
```
🤖 Repo Alignment PASS

Structure: ✅
Patterns: ✅
Conventions: ✅

Ready for maintainer alignment.
```

**FAIL**: Add `changes-requested` label
```
🤖 Repo Alignment CHANGES REQUESTED

Alignment issues:
{specific violations}

Fix for maintainer review.
```

## Safety
- Read-only review
- Can bounce back to author
- Focus on consistency, not subjective style
