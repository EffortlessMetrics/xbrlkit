## Summary
Multiple workflow and infrastructure scenarios are defined but not activated with `@alpha-active` tag.

## Affected Scenarios

### Workflow (5 scenarios)
| Scenario | File | AC Tag | Status |
|----------|------|--------|--------|
| SCN-XK-WORKFLOW-001 | workflow/feature_grid.feature | AC-XK-WORKFLOW-001 | Needs activation |
| SCN-XK-WORKFLOW-002 | workflow/bundle.feature | AC-XK-WORKFLOW-002 | Plan exists (#113) |
| SCN-XK-WORKFLOW-003 | workflow/cockpit_pack.feature | AC-XK-WORKFLOW-003 | Needs activation |
| SCN-XK-WORKFLOW-004 | workflow/bundle.feature | AC-XK-WORKFLOW-002 | Needs activation |
| SCN-XK-WORKFLOW-005 | workflow/alpha_check.feature | AC-XK-WORKFLOW-005 | Needs activation |

### Inline XBRL (2 scenarios)
- SCN-XK-IXDS-001, SCN-XK-IXDS-002
- File: `specs/features/inline/ixds_assembly.feature`

### Foundation (6 scenarios)
- SCN-XK-CONTEXT-001 through SCN-XK-CONTEXT-004 (context completeness)
- SCN-XK-DUPLICATES-001 (duplicate facts)
- SCN-XK-MANIFEST-001 (filing manifest)

### Taxonomy (2 scenarios)
- SCN-XK-TAXONOMY-001, SCN-XK-TAXONOMY-002

### Performance/Streaming (4 scenarios)
- SCN-XK-STREAM-001 through SCN-XK-STREAM-004

### Export (1 scenario)
- SCN-XK-EXPORT-001

### CLI (1 scenario)
- SCN-XK-CLI-001

## Blockers

### Missing Step Handlers (~45 total)

**Critical missing handlers:**
1. `When the document is validated` (5 occurrences)
2. `When I bundle the selector "..."` (partial)
3. `Then the bundle manifest lists scenario "..."`
4. `When I package the receipt for cockpit`
5. `When I run describe-profile --json`
6. Streaming parser assertions

## Immediate Action

**SCN-XK-WORKFLOW-002** has a complete activation plan (see `.mend/plans/ISSUE-113.md`). Ready to implement.

---
*Discovered by Scout Agent on 2026-04-01*
