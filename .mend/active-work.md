# Active Work Queue — xbrlkit

## Agentic SDLC Status
**Fully agentic workflow** — 8 agent gates, no human review required.
Workflow defined in `.mend/agentic-sdlc-workflow.md`. Agents in `.mend/agents/`.

## Current Sprint

### In Review (Agentic Pipeline)
| PR | Description | CI | Q | T | A | I | Ag | D | M | Status |
|----|-------------|----|---|---|---|---|----|---|---|--------|
| #97 | Taxonomy loader BDD scenarios | 🟢 | — | — | — | — | — | — | — | ready-for-review |
| #99 | Autonomous workflow infra | 🟢 | — | — | — | — | — | — | — | ready-for-review |
| #103 | Agentic SDLC workflow | 🟢 | — | — | — | — | — | — | — | ready-for-review |

**Legend:** Q=Quality, T=Tests, A=Arch, I=Integ, Ag=Agentic, D=Deep, M=Maintainer

### Agent Definitions (8 Total)
| Agent | Purpose | File |
|-------|---------|------|
| reviewer-quality | Code quality, clippy, docs | `.mend/agents/reviewer-quality.md` |
| reviewer-tests | Test coverage, BDD alignment | `.mend/agents/reviewer-tests.md` |
| reviewer-arch | Architecture, crate boundaries | `.mend/agents/reviewer-arch.md` |
| reviewer-integ | Integration, artifacts | `.mend/agents/reviewer-integ.md` |
| reviewer-agentic | Cross-cutting review + CI verify | `.mend/agents/reviewer-agentic.md` |
| reviewer-deep | Final improvements, cleanup | `.mend/agents/reviewer-deep.md` |
| maintainer-alignment | Code direction, strategic fit | `.mend/agents/maintainer-alignment.md` |
| merger-final | Final verification + merge | `.mend/agents/merger-final.md` |

### Blocked / Waiting
| Item | Blocker | ETA |
|------|---------|-----|
| Legacy PR cleanup (#11-15) | User decision on stale PRs | — |

## Workflow
```
CI Green → Quality → Tests → Arch → Integ → Agentic → Deep → Maintainer → Merge
     ↑____________________ Bounce back for changes ____________________|
```

## Labels
- `ready-for-review` — PR ready for agent review
- `review-in-progress` — Agent currently reviewing
- `quality-passed`, `tests-passed`, `arch-passed`, `integ-passed` — Gates 1-4
- `agentic-passed` — Gate 5 (cross-cutting + CI verify)
- `deep-passed` — Gate 6 (improvements, cleanup)
- `maintainer-approved` — Gate 7 (alignment, direction)
- `agent-merge-approved` — Gate 8 (merged)
- `changes-requested` — Bounced for revision
- `autonomous`, `wip` — Workflow labels

## Cron Jobs (Created, Disabled)
| Job | Schedule | Purpose |
|-----|----------|---------|
| xbrlkit-review-scheduler | Every 15 min | Spawn next required agent |
| xbrlkit-tree-cleanup | Every 6 hours | Clean merged branches |
| xbrlkit-ci-health | Hourly | CI health check |
| xbrlkit-friction-scan | Every 6h | TODO/FIXME detection |

## Next Steps
- [ ] Enable review-scheduler cron
- [ ] Test full 8-gate pipeline on PR #103
- [ ] Measure latency per gate

---
*Updated: Fully agentic workflow — 8 gates, no human review required*
