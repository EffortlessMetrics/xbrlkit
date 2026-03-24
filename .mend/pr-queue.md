# xbrlkit Autonomous PR Queue

**Mission:** Build a modern Rust XBRL processor through agentic quality.
**Principle:** Smooth → Clean → Researched → Verified → Stateful = Parallelizable Throughput
**Process:** Issue → Research → Plan → Build → Review → Merge. See `.mend/workflow.md`

## State Definitions

| Stage | Emoji | Meaning |
|-------|-------|---------|
| 📋 Ready | Ready for research/pickup | No blockers |
| 🔍 Research | Investigating, commenting findings | Issue analysis |
| 📐 Plan | Designing approach | API, tests, risks |
| 🔨 Build | Implementing | Code in progress |
| 🔄 Review | CI running, critique | PR open |
| ✅ Complete | Merged, closed | Done |
| ⏳ Blocked | Waiting on dependency | Human attention needed |

## Quality Gates (Non-Negotiable)
1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo xtask alpha-check`

## Completed Recently

| PR | Issue | Status | Commit | Notes |
|----|-------|--------|--------|-------|
| #37 | #35 | ✅ Complete | `5a577da` | Taxonomy loader crate |
| #32 | #7 | ✅ Complete | `2eeb94f` | Synthetic fixture ix:tuple |
| #30 | #8 | ✅ Complete | `e6d1b06` | Worktree-aware repo root |
| #28 | #9 | ✅ Complete | `b3bde6a` | Required facts unit tests |
| #27 | - | ✅ Complete | `9bd61ba` | Pre-push script |
| #26 | - | ✅ Complete | `bad1dbe` | Lint cleanup |

## Current Queue

| # | Issue | Stream | Stage | Blocker |
|---|-------|--------|-------|---------|
| - | - | - | - | - |

## In Progress

| # | Issue | Stream | Stage | Started |
|---|-------|--------|-------|---------|
| - | - | - | - | - |

## Parallel Work Streams

| Stream | Focus | Status |
|--------|-------|--------|
| **A: SEC Compliance** | Required facts | ✅ Complete |
| **B: Developer Experience** | xtask worktree, pre-push | ✅ Complete |
| **C: Test Infrastructure** | Synthetic fixtures | ✅ Complete |
| **D: Taxonomy Core** | Dimension loading | ✅ Complete |

## Autonomous Infrastructure

| File | Purpose |
|------|---------|
| `scripts/pre-push.sh` | Quality gates |
| `scripts/autonomous-pr.sh` | Full workflow |
| `scripts/update-queue.sh` | State management |
| `.mend/workflow.md` | Process definition |
| `.mend/mission.md` | Project mission |
| `.mend/plans/taxonomy-loader.md` | Current plan |

## Cron Schedule

| Job | Frequency | Purpose |
|-----|-----------|--------|
| xbrlkit-ci-health | 1h | Monitor CI |
| xbrlkit-queue-check | 2h | Pick ready items |

## Autonomous Rules

**Will act:**
- CI green → auto-merge
- 📋 Ready + no In Progress → start research
- User says "proceed"

**Will contact human:**
- ⏳ Blocked
- CI failure needing intervention
- Confidence < 60%

## Next

**In Progress:**
- Building `taxonomy-loader` crate — see `.mend/plans/taxonomy-loader.md`
- Will create PR `mend/issue-35-taxonomy-loader` when ready
