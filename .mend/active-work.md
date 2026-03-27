# Active Work Queue — xbrlkit

## Agentic SDLC Status
Workflow defined in `.mend/agentic-sdlc-workflow.md`. Agents in `.mend/agents/`.

## Current Sprint

### In Review (Agentic Pipeline)
| PR | Description | CI | Quality | Tests | Arch | Integ | Human | Status |
|----|-------------|----|---------|-------|------|-------|-------|--------|
| #97 | Taxonomy loader BDD scenarios | 🟢 | — | — | — | — | — | 🟡 ready-for-review |
| #99 | Autonomous workflow infra | 🟢 | — | — | — | — | — | 🟡 ready-for-review |
| #103 | Agentic SDLC workflow | 🟢 | — | — | — | — | — | 🟡 ready-for-review |

### Blocked / Waiting
| Item | Blocker | ETA |
|------|---------|-----|
| Legacy PR cleanup (#11-15) | User decision on stale PRs | — |

### Queue (Next Up)
| Priority | Item | Confidence | Value |
|----------|------|------------|-------|
| P0 | Enable review-scheduler cron (dry-run first) | High | Process |
| P1 | Close/refresh legacy PRs #11-15 | Medium | Hygiene |
| P2 | CLI dimension inspection extension | High | Feature |
| P2 | Typed dimension validation scenarios | High | Feature |

## Decisions Made
1. **No auto-merge** — Merge requires agentic review gate + human approval
2. **Multi-pass review** — Quality → Tests → Arch → Integ → Human → Merge
3. **Bounce allowed** — Any gate can request changes

## Labels Created
- `ready-for-review` — PR ready for agent review
- `review-in-progress` — Agent currently reviewing
- `quality-passed`, `tests-passed`, `arch-passed`, `integ-passed` — Review gates
- `in-review` — Ready for human review
- `changes-requested` — Bounced for revision
- `agent-merge-approved` — Merge agent completed
- `autonomous` — Part of autonomous workflow
- `wip` — Work in progress

## Cron Jobs (Created, Disabled)
| Job | Schedule | Purpose |
|-----|----------|---------|
| xbrlkit-review-scheduler | Every 15 min | Spawn reviewer agents for ready PRs |
| xbrlkit-tree-cleanup | Every 6 hours | Clean merged branches, stash uncommitted |
| xbrlkit-ci-health | Hourly | CI health check |
| xbrlkit-friction-scan | Every 6h | TODO/FIXME detection |
| xbrlkit-queue-check | Every 2h | Check active-work.md for queued items |

---
*Updated: Agentic SDLC Phase 1 complete — labels and cron jobs ready*
