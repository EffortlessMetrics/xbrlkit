# xbrlkit Autonomous PR Queue

**Mission:** Build a modern Rust XBRL processor through agentic quality.
**Principle:** Smooth → Clean → Researched → Verified → Stateful = Parallelizable Throughput
**Process:** Issue → Research → Plan → Build → Review → Merge. See `.mend/workflow.md`

## Quality Gates (Non-Negotiable)
1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo xtask alpha-check`

## Completed Today

| PR | Issue | Status | Commit | Notes |
|----|-------|--------|--------|-------|
| #28 | #9 | ✅ Merged | `b3bde6a` | Unit tests for required facts |
| #26 | - | ✅ Merged | `bad1dbe` | Lint cleanup |
| #27 | - | ✅ Merged | `9bd61ba` | Pre-push script |

## Current Queue

| # | Issue | Stream | Stage | Ready |
|---|-------|--------|-------|-------|
| 1 | #8 | B: DevEx | 📋 Ready | xtask worktree-aware |
| 2 | #7 | C: Test Infra | 📋 Ready | Synthetic fixture |
| 3 | - | D: Taxonomy | 📋 Discovery | Create research issue |

## Parallel Work Streams

| Stream | Focus | Status |
|--------|-------|--------|
| **A: SEC Compliance** | Required facts | ✅ Complete |
| **B: Developer Experience** | xtask worktree, pre-push | 📋 Ready |
| **C: Test Infrastructure** | Synthetic fixtures | 📋 Ready |
| **D: Taxonomy Core** | Dimension loading | 📋 Discovery |

## Autonomous Infrastructure

| File | Purpose |
|------|---------|
| `scripts/pre-push.sh` | Quality gates |
| `scripts/autonomous-pr.sh` | Full workflow |
| `.mend/workflow.md` | Process definition |
| `.mend/mission.md` | Project mission |
| `.mend/pr-queue.md` | This queue |

## Autonomous Mode Active

**Triggers for action:**
- CI green → auto-merge
- Issue ready with AC → start research
- User says "proceed" / "continue"

**Triggers for human contact:**
- CI failure needing intervention
- Architecture decision required
- Confidence < 60%

## Next (Awaiting Pick)

Options:
1. **#8** — xtask worktree-aware (DevEx, quick win)
2. **#7** — Synthetic fixture (Test Infra, foundation)
3. **New issue** — Taxonomy loader research (Core, high value)

Your call, or I'll pick highest value ready item.
