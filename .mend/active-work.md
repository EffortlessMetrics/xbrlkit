# Active Work Queue — xbrlkit

**⚠️ AUTO-GENERATED: This file is recreated if missing. Do not rely on it for critical state.**

## Agentic SDLC Status
**FULLY OPERATIONAL** 🟢🟢
Planning scheduler: ENABLED | Review scheduler: ENABLED
13 agents, 13 gates. End-to-end agentic.

## Workflow
```
Issue → Plan → Plan Review → Deep Plan → Repo Alignment → Build → CI → Quality → Tests → Arch → Integ → Agentic → Deep → Maintainer → Merge
```

## Current Sprint

### In Planning (Scheduler Active)
| Issue | Description | Label | Next Agent |
|-------|-------------|-------|------------|
| #100 | Taxonomy Loader BDD | **needs-plan** | planner-initial |
| #101 | Legacy PR Cleanup | **needs-plan** | planner-initial |
| #102 | ADR: HTTP client architecture | **needs-plan** | planner-initial |

### In Review (Code Phase)
| PR | CI | Q | T | A | I | Ag | D | M | Status |
|----|----|---|---|---|---|----|---|---|--------|
| #97 | 🟢 | — | — | — | — | — | — | — | ready-for-review |
| #99 | 🟢 | — | — | — | — | — | — | — | ready-for-review |

**Legend:** Q=Quality, T=Tests, A=Arch, I=Integ, Ag=Agentic, D=Deep, M=Maintainer

## Cron Jobs
| Job | Schedule | Status |
|-----|----------|--------|
| **xbrlkit-planning-scheduler** | Every 15 min | 🟢 **ENABLED** |
| **xbrlkit-review-scheduler** | Every 15 min | 🟢 **ENABLED** |
| xbrlkit-tree-cleanup | Every 6 hours | 🟡 Disabled |
| xbrlkit-ci-health | Hourly | 🟢 Active |

## Monitoring
- `.mend/plans/` — Watch for new plan documents
- `.mend/session-log.md` — Agent activity log
- PR labels — Gate progression
- Issue labels — Planning phase progression

---
*Status: BOTH SCHEDULERS ENABLED — Full agentic workflow operational*
*Note: This file format is deprecated. Migrate to GitHub-native tracking (issues, labels, comments).*
