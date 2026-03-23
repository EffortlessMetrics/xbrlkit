# xbrlkit Autonomous PR Queue

**Mission:** Build a modern Rust XBRL processor through agentic quality.
**Principle:** Smooth → Clean → Researched → Verified → Stateful = Parallelizable Throughput
**Process:** Issue → Research → Plan → Build → Review → Merge. See `.mend/workflow.md`

## Quality Gates (Non-Negotiable)
1. `cargo fmt --all --check`
2. `cargo clippy --workspace --all-targets -- -D warnings`
3. `cargo test --workspace`
4. `cargo xtask alpha-check`

## Parallel Work Streams

| Stream | Focus | Current | Blockers |
|--------|-------|---------|----------|
| **A: SEC Compliance** | Required facts, inline restrictions | #9 research complete | User decision: close or expand |
| **B: Developer Experience** | xtask worktree, pre-push hooks | #8 ready | None |
| **C: Test Infrastructure** | Synthetic fixtures, scenarios | #7 ready | None |
| **D: Taxonomy Core** | Dimension loading, validation | Discovery | Needs research issue |

## Issue Queue

| # | Issue | Stream | Stage | Notes |
|---|-------|--------|-------|-------|
| 9 | Required Facts Validation | A | 🔍 Research Complete | Implementation done — close or expand scope? |
| 8 | xtask worktree-aware | B | 📋 Ready | Runtime worktree detection |
| 7 | Synthetic fixture | C | 📋 Ready | Active alpha validation path |
| - | Taxonomy Loader | D | 📋 Discovery | Load dimensions from actual files |

## Definition of Done (Per Issue)

- [ ] Research comment on issue (findings, spec refs, prior art)
- [ ] Plan comment on issue (approach, API, tests, risks)
- [ ] Build PR (implementation + tests + docs)
- [ ] Review comments (issues found + explanations)
- [ ] Refine commits (fixes + what/why explanations)
- [ ] CI green (all 4 gates)
- [ ] Merge with detailed summary
- [ ] Update this queue

## Parallelization Rules

1. **Stream Isolation:** Different crates = no conflict
2. **Interface First:** Define API before parallel implementation
3. **No Shared State:** Each issue owns its context
4. **Friction Logs:** Shared learning, not shared work

## Next Actions (Awaiting User)

1. **Issue #9:** Close as complete, or expand required facts list?
2. **Stream Priority:** Which stream should I pursue first?
3. **New Issues:** Create taxonomy loader research issue?
