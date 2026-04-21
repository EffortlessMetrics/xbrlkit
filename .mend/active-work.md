# Active Work Queue — xbrlkit

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

**No items currently in planning.** All planning-phase issues completed.

| Issue | Description | Label | Next Agent |
|-------|-------------|-------|------------|

### Completed Recently
| Issue | Description | PR |
|-------|-------------|-----|
| #100 | Taxonomy Loader BDD | #104 |
| #101 | Legacy PR Cleanup | #108 |
| #102 | ADR: HTTP client architecture | #107, #109 |

### In Review (Code Phase)

| PR | CI | Q | T | A | I | Ag | D | M | Status |
|----|----|---|---|---|---|----|---|---|--------|
| #126 | 🟢 | ✅ | ✅ | — | — | — | — | — | changes-requested |
| #127 | 🟢 | ✅ | ✅ | — | — | — | — | — | changes-requested |
| #128 | 🟢 | ✅ | ✅ | — | — | — | — | — | tests-alpha-failed |
| #129 | 🟢 | ✅ | ✅ | — | — | — | — | — | tests-alpha-failed |
| #134 | 🟢 | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | changes-requested |
| #138 | 🟢 | ✅ | ✅ | — | — | — | — | — | changes-requested |
| #180 | 🟢 | ✅ | ✅ | — | — | — | — | — | ready-for-review |
| #181 | 🟢 | ✅ | ✅ | — | — | — | — | — | ready-for-review |
| #182 | 🟢 | ✅ | ✅ | — | — | — | — | — | ready-for-review |
| #183 | 🟢 | ✅ | ✅ | — | — | — | — | — | quality-final-passed |
| #184 | 🟢 | ✅ | ✅ | — | — | — | — | — | quality-final-failed |

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
| **xbrlkit-planning-scheduler** | Every 15 min | 🟢 **ENABLED** |
| **xbrlkit-review-scheduler** | Every 15 min | 🟢 **ENABLED** |
| xbrlkit-tree-cleanup | Every 6 hours | 🟡 Disabled |
| xbrlkit-ci-health | Hourly | 🟢 Active |

## What Happens Now

### Planning Phase (Next 15 min)
1. Planning scheduler picks up issues #100, #101, #102
2. Spawns `planner-initial` agents
3. Agents create `.mend/plans/ISSUE-{n}.md` documents
4. Labels progress: needs-plan → plan-draft

### Review Phase (Ongoing)
1. Review scheduler checks PRs #97, #99, #103 every 15 min
2. Spawns `reviewer-quality` agents (first gate)
3. Labels progress through 8 review gates
4. Final `merger-final` executes merge

## Monitoring
- `.mend/plans/` — Watch for new plan documents
- `.mend/session-log.md` — Agent activity log
- PR labels — Gate progression
- Issue labels — Planning phase progression

---
*Status: BOTH SCHEDULERS ENABLED — Full agentic workflow operational*
