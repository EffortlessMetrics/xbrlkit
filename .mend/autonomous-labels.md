# Autonomous Workflow Labels

This document defines the GitHub label system for the autonomous workflow pipeline in xbrlkit.

## Overview

The autonomous workflow uses a structured label system to track issues and PRs through the planning and implementation pipeline. Labels are organized by phase and provide clear state visibility for both humans and agents.

## Label Taxonomy

### Planning Phase Labels

| Label | Color | Description | When Applied |
|-------|-------|-------------|--------------|
| `needs-plan` | `#F9D0C4` | Issue needs implementation plan | New autonomous issues lacking a plan |
| `planning-in-progress` | `#FBCA04` | Agent actively working on this issue | When an agent starts planning |
| `plan-draft` | `#FEF2C0` | Implementation plan drafted | Plan document created, pending review |
| `plan-reviewed` | `#FBCA04` | Plan feasibility reviewed | Initial review complete |
| `deep-plan-reviewed` | `#FBCA04` | Deep plan review complete | In-depth review with edge cases |
| `plan-needs-work` | `#B60205` | Plan needs revision | Review identified issues |
| `repo-aligned` | `#0E8A16` | Repo alignment and pattern consistency passed | Plan matches codebase patterns |

### Implementation Phase Labels

| Label | Color | Description | When Applied |
|-------|-------|-------------|--------------|
| `building` | `#D4C5F9` | Implementation in progress | When coding begins |
| `wip` | `#F9D0C4` | Work in progress | General WIP indicator |
| `in-progress` | `#D93F0B` | Work in progress - not ready for review | Active development |

### Review Phase Labels

| Label | Color | Description | When Applied |
|-------|-------|-------------|--------------|
| `ready-for-review` | `#FBCA04` | PR is ready for agent review | Awaiting automated review |
| `review-in-progress` | `#D4C5F9` | Agent review in progress | Review agent working |
| `changes-requested` | `#B60205` | Agent requested changes | Review found issues |
| `status/reviewed-needs-work` | `#f97583` | Reviewed, changes requested | Human review feedback |
| `status/ready-for-review` | `#c2e0c6` | Ready for next review pass | Fixes applied |
| `in-review` | `#FEF2C0` | Ready for human review | Awaiting maintainer |

### Quality Gate Labels

| Label | Color | Description | When Applied |
|-------|-------|-------------|--------------|
| `quality-passed` | `#0E8A16` | Quality review agent passed | Code quality checks passed |
| `arch-passed` | `#0E8A16` | Architecture review agent passed | Architecture validation passed |
| `tests-passed` | `#0E8A16` | Test review agent passed | All tests passing |
| `integ-passed` | `#0E8A16` | Integration review agent passed | Integration tests passed |
| `deep-passed` | `#0E8A16` | Deep improvements review passed | Deep review complete |
| `agentic-passed` | `#0E8A16` | Agentic cross-cutting review passed | Cross-cutting concerns OK |

### Merge Phase Labels

| Label | Color | Description | When Applied |
|-------|-------|-------------|--------------|
| `status/ready-to-merge` | `#0e8a16` | Ready for merge after review | All checks passed |
| `agent-merge-approved` | `#5319E7` | Merge agent approved and merged | Automated merge complete |
| `maintainer-approved` | `#5319E7` | Maintainer alignment approved | Human approval granted |

### Escalation Labels

| Label | Color | Description | When Applied |
|-------|-------|-------------|--------------|
| `needs-human-decision` | `#B60205` | Escalated for human strategic decision | Requires human judgment |
| `needs-human` | `#B60205` | Requires human decision | Blocked on human input |
| `status/blocked-on-X` | `#5319e7` | Waiting for dependency PR | External dependency |

### Agent-Specific Labels (Prefixed)

| Label | Color | Description |
|-------|-------|-------------|
| `agent/autonomous` | `#1D76DB` | Marks issues/PRs created or managed by autonomous workflow |
| `agent/in-review` | `#FBCA04` | Indicates items currently under active review by an agent |
| `agent/wip` | `#D93F0B` | Work in progress - not ready for review |
| `agent/needs-human` | `#D73A4A` | Requires human decision before proceeding |
| `agent/tech-debt` | `#7057FF` | Technical debt, legacy code, or maintenance tasks |

### Workflow Metadata Labels

| Label | Color | Description |
|-------|-------|-------------|
| `autonomous` | `#0052CC` | Part of autonomous workflow |
| `swarm-core` | `#1D76DB` | Primary repo work |
| `swarm-architectural` | `#C5DEF5` | Needs human judgment or ADR-level decision |
| `swarm-improve-docs` | `#0E8A16` | Docs / ADR / README improvement |
| `swarm-improve-tests` | `#5319E7` | Test quality / mutation / coverage improvement |
| `swarm-improve-devex` | `#FBCA04` | Developer experience / tooling improvement |
| `swarm-improve-infra` | `#D93F0B` | Infra / dependency / build hygiene improvement |
| `swarm-discovered` | `#BFD4F2` | Discovered while touching adjacent code |

## Workflow State Transitions

```
┌─────────────┐     ┌──────────────────┐     ┌─────────────┐
│  needs-plan │────▶│planning-in-progress│───▶│ plan-draft  │
└─────────────┘     └──────────────────┘     └─────────────┘
                                                     │
                         ┌──────────────────────────┘
                         ▼
              ┌─────────────────┐     ┌────────────────┐
              │  plan-reviewed  │────▶│deep-plan-review│
              └─────────────────┘     └────────────────┘
                         │                     │
                         ▼                     ▼
              ┌─────────────────┐     ┌────────────────┐
              │  repo-aligned   │◀────│ plan-needs-work│
              │   (planning     │     │   (loop back)  │
              │    complete)    │     └────────────────┘
              └─────────────────┘
                         │
                         ▼
              ┌─────────────────┐     ┌────────────────┐
              │    building     │────▶│  wip/in-progress│
              │                 │     │                │
              └─────────────────┘     └────────────────┘
                         │
                         ▼
              ┌─────────────────┐     ┌────────────────┐
              │ ready-for-review│────▶│review-in-progress│
              └─────────────────┘     └────────────────┘
                         │                     │
                         ▼                     ▼
              ┌─────────────────┐     ┌────────────────┐
              │  quality-passed │     │changes-requested│
              │  arch-passed    │◀────│ (loop back)    │
              │  tests-passed   │     └────────────────┘
              │  integ-passed   │
              └─────────────────┘
                         │
                         ▼
              ┌─────────────────┐     ┌────────────────┐
              │status/ready-to- │────▶│ agent-merge-   │
              │     merge       │     │  approved      │
              └─────────────────┘     └────────────────┘
```

## Color Conventions

- **🟢 Green (`#0E8A16`)**: Success states, passed gates, approved
- **🟡 Yellow/Orange (`#FBCA04`, `#FEF2C0`, `#F9D0C4`)**: In progress, pending, drafting
- **🔴 Red (`#B60205`, `#D73A4A`, `#f97583`)**: Blocked, needs work, failed
- **🟣 Purple (`#D4C5F9`, `#5319E7`, `#7057FF`)**: Agent-specific, infrastructure
- **🔵 Blue (`#1D76DB`, `#0052CC`)**: Autonomous workflow markers

## Usage Guidelines

### For Planning Agents
1. Apply `autonomous` + `needs-plan` to new issues
2. Switch to `planning-in-progress` when starting work
3. Update to `plan-draft` when plan is written
4. Apply `repo-aligned` after pattern validation

### For Builder Agents
1. Check for `repo-aligned` before starting
2. Apply `building` when implementation starts
3. Update to `ready-for-review` when PR is ready
4. Address `changes-requested` promptly

### For Review Agents
1. Apply `review-in-progress` when starting review
2. Use `changes-requested` with detailed comments
3. Apply quality gate labels as checks pass

### For Maintainers
1. Watch for `needs-human-decision` escalations
2. Apply `maintainer-approved` after human review
3. Use `status/blocked-on-X` for dependencies

## Label Management

### Creating Labels

```bash
# Planning phase
gh label create "needs-plan" --color "F9D0C4" --description "Issue needs implementation plan"
gh label create "planning-in-progress" --color "FBCA04" --description "Agent actively working on this issue"
gh label create "plan-draft" --color "FEF2C0" --description "Implementation plan drafted"
gh label create "plan-reviewed" --color "FBCA04" --description "Plan feasibility reviewed"
gh label create "deep-plan-reviewed" --color "FBCA04" --description "Deep plan review complete"
gh label create "repo-aligned" --color "0E8A16" --description "Repo alignment and pattern consistency passed"
gh label create "plan-needs-work" --color "B60205" --description "Plan needs revision"
```

### Updating Labels

```bash
gh label edit "label-name" --color "NEWCOLOR" --description "New description"
```

### Listing Labels

```bash
gh label list --search "planning"
```

## Automation Integration

Labels trigger automation via GitHub Actions:

- `ready-for-review` → Triggers agent review pipeline
- `needs-human` → Notifies maintainers
- `repo-aligned` → Allows implementation to proceed
- `agent-merge-approved` → Auto-merge if checks pass

## See Also

- [Development Workflow](workflow.md) - Main workflow documentation
- [PR Queue](pr-queue.md) - Active work tracking
- [Autonomous Log](autonomous-log.md) - Automation activity log
