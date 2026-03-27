# Active Work Queue — xbrlkit

## Agentic SDLC Status
**Full workflow — Planning → Build → Review → Merge** 🟢
13 agents, 13 gates. Fully agentic.

## Workflow
```
Issue → Plan → Plan Review → Deep Plan → Repo Alignment → Build → CI → Quality → Tests → Arch → Integ → Agentic → Deep → Maintainer → Merge
```

## Current Sprint

### In Planning
| Issue | Description | Status | Next Agent |
|-------|-------------|--------|------------|
| #100 | Taxonomy Loader BDD — PR #97 | 🟡 PR exists | In review phase |
| #101 | Legacy PR Cleanup | — | needs-plan |
| #102 | ADR: HTTP client architecture | — | needs-plan |

### In Review (Code Phase)
| PR | CI | Q | T | A | I | Ag | D | M | Status |
|----|----|---|---|---|---|----|---|---|--------|
| #97 | 🟢 | — | — | — | — | — | — | — | ready-for-review |
| #99 | 🟢 | — | — | — | — | — | — | — | ready-for-review |
| #103 | 🟢 | — | — | — | — | — | — | — | ready-for-review |

**Legend:** Q=Quality, T=Tests, A=Arch, I=Integ, Ag=Agentic, D=Deep, M=Maintainer

## Agent Definitions (13 Total)

### Planning Phase (4)
| Agent | Purpose | File |
|-------|---------|------|
| planner-initial | Create plan from issue | `.mend/agents/planner-initial.md` |
| reviewer-plan | Review plan feasibility | `.mend/agents/reviewer-plan.md` |
| reviewer-deep-plan | Deep plan review | `.mend/agents/reviewer-deep-plan.md` |
| reviewer-repo-alignment | Check plan vs repo patterns | `.mend/agents/reviewer-repo-alignment.md` |

### Implementation Phase (1)
| Agent | Purpose | File |
|-------|---------|------|
| builder-implement | Implement plan, create PR | `.mend/agents/builder-implement.md` |

### Review Phase (8)
| Agent | Purpose | File |
|-------|---------|------|
| reviewer-quality | Code quality, clippy | `.mend/agents/reviewer-quality.md` |
| reviewer-tests | Test coverage, BDD | `.mend/agents/reviewer-tests.md` |
| reviewer-arch | Architecture | `.mend/agents/reviewer-arch.md` |
| reviewer-integ | Integration | `.mend/agents/reviewer-integ.md` |
| reviewer-agentic | Cross-cutting + CI | `.mend/agents/reviewer-agentic.md` |
| reviewer-deep | Improvements, cleanup | `.mend/agents/reviewer-deep.md` |
| maintainer-alignment | Direction, strategy | `.mend/agents/maintainer-alignment.md` |
| merger-final | Verify + merge | `.mend/agents/merger-final.md` |

## Labels

### Planning
- `needs-plan` → `plan-draft` → `plan-reviewed` → `deep-plan-reviewed` → `repo-aligned`
- `plan-needs-work` — Bounce to revision

### Implementation
- `building` — Implementation in progress
- `ready-for-review` — Entering review pipeline

### Review
- `quality-passed` → `tests-passed` → `arch-passed` → `integ-passed` → `agentic-passed` → `deep-passed` → `maintainer-approved` → `agent-merge-approved`
- `changes-requested` — Bounce to revision

## Cron Jobs
| Job | Schedule | Status |
|-----|----------|--------|
| xbrlkit-planning-scheduler | Every 15 min | 🟢 Created (disabled) |
| **xbrlkit-review-scheduler** | Every 15 min | 🟢 **ENABLED** |
| xbrlkit-tree-cleanup | Every 6 hours | 🟡 Disabled |
| xbrlkit-ci-health | Hourly | 🟢 Active |

## Next Steps
- [ ] Enable xbrlkit-planning-scheduler cron job
- [ ] Test planning phase on new issue
- [ ] Monitor PR #97, #99, #103 through review pipeline

---
*Updated: Full 13-agent workflow — planning phase added, repo alignment in planning*
