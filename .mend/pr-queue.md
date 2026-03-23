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
| #30 | #8 | ✅ Merged | `e6d1b06` | Worktree-aware repo root |
| #28 | #9 | ✅ Merged | `b3bde6a` | Required facts unit tests |
| #27 | - | ✅ Merged | `9bd61ba` | Pre-push script |
| #26 | - | ✅ Merged | `bad1dbe` | Lint cleanup |

## Current Queue

| # | Issue | Stream | Stage | Ready |
|---|-------|--------|-------|-------|
| 1 | #7 | C: Test Infra | 📋 Ready | Synthetic fixture |
| 2 | - | D: Taxonomy | 📋 Discovery | Create research issue |

## Parallel Work Streams

| Stream | Focus | Status |
|--------|-------|--------|
| **A: SEC Compliance** | Required facts | ✅ Complete |
| **B: Developer Experience** | xtask worktree, pre-push | ✅ Complete |
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

1. **#7** — Synthetic fixture (Test Infra, 2-3h)
2. **New issue** — Taxonomy loader research (Core, high value)

Or say "proceed" and I'll pick highest value ready item.
