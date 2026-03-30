# Agentic SDLC Workflow — xbrlkit

## Philosophy
End-to-end agentic workflow: Planning → Implementation → Review → Merge. No human gates.

## Full Workflow

```
Issue → Plan → Plan Review → Deep Plan Review → Repo Alignment → Build → CI → Quality → Tests → Arch → Integ → Agentic → Deep → Maintainer → Merge
```

## Phase 1: Planning

| Step | Agent | Trigger | Output |
|------|-------|---------|--------|
| 1 | `planner-initial` | Issue labeled `needs-plan` | Plan document + `plan-draft` |
| 2 | `reviewer-plan` | `plan-draft` label | `plan-reviewed` label |
| 3 | `reviewer-deep-plan` | `plan-reviewed` label | `deep-plan-reviewed` label |
| 4 | `reviewer-repo-alignment` | `deep-plan-reviewed` label | `repo-aligned` label |

**Repo Alignment** happens here — before any code is written. Checks if the planned implementation aligns with repo patterns and conventions.

## Phase 2: Implementation

| Step | Agent | Trigger | Output |
|------|-------|---------|--------|
| 5 | `builder-implement` | `repo-aligned` label | Branch + PR + `ready-for-review` |

## Phase 3: Code Review

| Step | Agent | Trigger | Output |
|------|-------|---------|--------|
| 6 | `reviewer-quality` | `ready-for-review` + CI green | `quality-passed` |
| 7 | `reviewer-tests` | `quality-passed` | `tests-passed` |
| 8 | `reviewer-arch` | `tests-passed` | `arch-passed` |
| 9 | `reviewer-integ` | `arch-passed` | `integ-passed` |
| 10 | `reviewer-agentic` | `integ-passed` | `agentic-passed` |
| 11 | `reviewer-deep` | `agentic-passed` | `deep-passed` |
| 12 | `maintainer-alignment` | `deep-passed` | `maintainer-approved` |
| 13 | `merger-final` | `maintainer-approved` | `agent-merge-approved` + merge |

## Agent Definitions

### Planning Phase
- `planner-initial.md` — Create implementation plan from issue
- `reviewer-plan.md` — Review plan for feasibility
- `reviewer-deep-plan.md` — Deep plan review (edge cases, risks)
- `reviewer-repo-alignment.md` — Check plan against repo patterns

### Implementation Phase
- `builder-implement.md` — Implement approved plan, create PR

### Review Phase
- `reviewer-quality.md` — Code quality, clippy, docs
- `reviewer-tests.md` — Test coverage, BDD alignment
- `reviewer-arch.md` — Architecture, crate boundaries
- `reviewer-integ.md` — Integration, artifacts
- `reviewer-agentic.md` — Cross-cutting review + CI verify
- `reviewer-deep.md` — Final improvements, cleanup
- `maintainer-alignment.md` — Code direction, strategic fit
- `merger-final.md` — Final verification + merge

## Labels

### Planning Phase
| Label | Meaning | Set By |
|-------|---------|--------|
| `needs-plan` | Issue needs implementation plan | Human or triage |
| `plan-draft` | Initial plan created | planner-initial |
| `plan-reviewed` | Plan feasibility verified | reviewer-plan |
| `deep-plan-reviewed` | Deep plan review complete | reviewer-deep-plan |
| `repo-aligned` | Plan aligns with repo patterns | reviewer-repo-alignment |
| `plan-needs-work` | Plan needs revision | Any planning agent |

### Implementation Phase
| Label | Meaning | Set By |
|-------|---------|--------|
| `building` | Implementation in progress | builder-implement |
| `ready-for-review` | PR ready for review pipeline | builder-implement |

### Review Phase
| Label | Meaning | Set By |
|-------|---------|--------|
| `review-in-progress` | Agent currently reviewing | Scheduler |
| `quality-passed` | Quality review complete | reviewer-quality |
| `tests-passed` | Test review complete | reviewer-tests |
| `arch-passed` | Architecture review complete | reviewer-arch |
| `integ-passed` | Integration review complete | reviewer-integ |
| `agentic-passed` | Agentic review complete | reviewer-agentic |
| `deep-passed` | Deep improvements complete | reviewer-deep |
| `maintainer-approved` | Maintainer alignment complete | maintainer-alignment |
| `agent-merge-approved` | Merge complete | merger-final |
| `changes-requested` | Bounced for revision | Any reviewer |
| `needs-human-decision` | Escalated for strategic issues | maintainer-alignment |
| `autonomous` | Part of autonomous workflow | Any agent |

## Cron Jobs

### `xbrlkit-planning-scheduler` (Every 15 min)
```
For each open issue with label:
  needs-plan → spawn planner-initial
  plan-draft → spawn reviewer-plan
  plan-reviewed → spawn reviewer-deep-plan
  deep-plan-reviewed → spawn reviewer-repo-alignment
  repo-aligned → spawn builder-implement
```

### `xbrlkit-review-scheduler` (Every 15 min)
```
For each open PR:
  If CI green AND no review-in-progress:
    Determine next agent from labels:
      ready-for-review → reviewer-quality
      quality-passed → reviewer-tests
      tests-passed → reviewer-arch
      arch-passed → reviewer-integ
      integ-passed → reviewer-agentic
      agentic-passed → reviewer-deep
      deep-passed → maintainer-alignment
      maintainer-approved → merger-final
```

## Bounce Behavior

Any agent can bounce back to previous phase:
- Planning agents → back to plan revision
- Review agents → back to author (with `changes-requested`)

After 3 bounces on same PR/issue → escalate to human (`needs-human-decision`)

## Safety

1. **No human gates** — Fully agentic end-to-end
2. **Repo alignment before build** — Pattern check on plan, not code
3. **Audit trail** — Every action logged
4. **Bounce limit** — Escalation after repeated failures
5. **Plan-first** — Implementation follows approved plan

## Directory Structure

```
.mend/
  agents/           # Agent definitions
  plans/            # Implementation plans (ISSUE-{n}.md)
  session-log.md    # Agent activity log
  active-work.md    # Current sprint tracking
```

---
*Full agentic workflow: Planning → Build → Review → Merge (13 agents, 13 gates)*
