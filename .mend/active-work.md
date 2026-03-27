# Active Work Queue — xbrlkit

## Agentic SDLC Status
**Fully agentic workflow — 9 gates, SCHEDULER ENABLED** 🟢
Workflow defined in `.mend/agentic-sdlc-workflow.md`. Agents in `.mend/agents/`.

## Current Sprint

### In Review (Agentic Pipeline)
| PR | Description | CI | Q | T | A | I | Ag | D | R | M | Status |
|----|-------------|----|---|---|---|---|----|---|---|---|--------|
| #97 | Taxonomy loader BDD scenarios | 🟢 | — | — | — | — | — | — | — | — | ready-for-review |
| #99 | Autonomous workflow infra | 🟢 | — | — | — | — | — | — | — | — | ready-for-review |
| #103 | Agentic SDLC workflow | 🟢 | — | — | — | — | — | — | — | — | ready-for-review |

**Legend:** Q=Quality, T=Tests, A=Arch, I=Integ, Ag=Agentic, D=Deep, R=Repo Alignment, M=Maintainer

### Agent Definitions (9 Total)
| Agent | Purpose | File |
|-------|---------|------|
| reviewer-quality | Code quality, clippy, docs | `.mend/agents/reviewer-quality.md` |
| reviewer-tests | Test coverage, BDD alignment | `.mend/agents/reviewer-tests.md` |
| reviewer-arch | Architecture, crate boundaries | `.mend/agents/reviewer-arch.md` |
| reviewer-integ | Integration, artifacts | `.mend/agents/reviewer-integ.md` |
| reviewer-agentic | Cross-cutting review + CI verify | `.mend/agents/reviewer-agentic.md` |
| reviewer-deep | Final improvements, cleanup | `.mend/agents/reviewer-deep.md` |
| **reviewer-repo-alignment** | **Repo alignment, patterns** | `.mend/agents/reviewer-repo-alignment.md` |
| maintainer-alignment | Code direction, strategic fit | `.mend/agents/maintainer-alignment.md` |
| merger-final | Final verification + merge | `.mend/agents/merger-final.md` |

### Blocked / Waiting
| Item | Blocker | ETA |
|------|---------|-----|
| Legacy PR cleanup (#11-15) | User decision on stale PRs | — |

## Workflow (9 Gates)
```
CI → Quality → Tests → Arch → Integ → Agentic → Deep → Repo Alignment → Maintainer → Merge
     ↑________________________ Bounce back for changes ________________________|
```

## Labels
- `ready-for-review` — PR ready for agent review
- `review-in-progress` — Agent currently reviewing
- `quality-passed`, `tests-passed`, `arch-passed`, `integ-passed` — Gates 1-4
- `agentic-passed` — Gate 5 (cross-cutting + CI verify)
- `deep-passed` — Gate 6 (improvements, cleanup)
- `repo-aligned` — Gate 7 (repo alignment, patterns)
- `maintainer-approved` — Gate 8 (alignment, direction)
- `agent-merge-approved` — Gate 9 (merged)
- `changes-requested` — Bounced for revision
- `needs-human-decision` — Escalated for strategic issues
- `autonomous`, `wip` — Workflow labels

## Cron Jobs
| Job | Schedule | Status | Purpose |
|-----|----------|--------|---------|
| **xbrlkit-review-scheduler** | Every 15 min | 🟢 **ENABLED** | Spawn next required agent |
| xbrlkit-tree-cleanup | Every 6 hours | 🟡 Disabled | Clean merged branches |
| xbrlkit-ci-health | Hourly | 🟢 Active | CI health check |
| xbrlkit-friction-scan | Every 6h | 🟢 Active | TODO/FIXME detection |

## Next Agent Runs
Scheduler will check PRs #97, #99, #103 every 15 minutes and spawn `reviewer-quality` agents (first gate).

## Monitoring
- Check `.mend/session-log.md` for agent activity
- Watch PR labels for gate progression
- Reviewer agents will comment 🤖 templates on PRs

---
*Updated: Scheduler ENABLED — 9-gate fully agentic workflow active*
