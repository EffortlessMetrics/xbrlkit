# Active Work Queue — xbrlkit

## Agentic SDLC Status
Workflow defined in `.mend/agentic-sdlc-workflow.md`. Agents in `.mend/agents/`.

## Current Sprint

### In Review (Agentic Pipeline)
| PR | Description | CI | Quality | Tests | Arch | Integ | Human | Status |
|----|-------------|----|---------|-------|------|-------|-------|--------|
| #97 | Taxonomy loader BDD scenarios | 🟢 | — | — | — | — | — | 🟡 Ready for agent review |
| #99 | Autonomous workflow infra | 🟢 | — | — | — | — | — | 🟡 Ready for agent review |

### Blocked / Waiting
| Item | Blocker | ETA |
|------|---------|-----|
| Legacy PR cleanup (#11-15) | User decision on stale PRs | — |

### Queue (Next Up)
| Priority | Item | Confidence | Value |
|----------|------|------------|-------|
| P0 | Enable agentic review cron jobs | High | Process |
| P0 | Create GitHub labels | High | Process |
| P1 | Close/refresh legacy PRs #11-15 | Medium | Hygiene |
| P2 | CLI dimension inspection extension | High | Feature |
| P2 | Typed dimension validation scenarios | High | Feature |

## Decisions Made
1. **No auto-merge** — Merge requires agentic review gate + human approval
2. **Multi-pass review** — Quality → Tests → Arch → Integ → Human → Merge
3. **Bounce allowed** — Any gate can request changes

## Background Processes
| Job | Status | Last Run | Finding |
|-----|--------|----------|---------|
| CI Health | ✅ Active | Hourly | Green |
| Friction Scan | ✅ Active | Every 6h | — |
| Review Scheduler | 🟡 Defined | — | Ready to enable |
| Tree Cleanup | 🟡 Defined | — | Ready to enable |

---
*Updated: Agentic SDLC workflow phase 1 complete*
