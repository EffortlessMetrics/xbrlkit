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

## Completed Recently (Phase 2)

| PR | Issue | Status | Commit | Notes |
|----|-------|--------|--------|-------|
| #74 | - | ✅ Complete | `02b8882` | Phase 2 complete marker |
| #72 | #6 | ✅ Complete | `abc337b` | Bundle + impact workflow docs |
| #70 | #2 | ✅ Complete | `1e0d050` | Maintainer command docs |
| #68 | - | ✅ Complete | `bf14912` | Roadmap Wave 2 complete |
| #67 | #57 | ✅ Complete | `ded1840` | Typed value validation |
| #65 | #56 | ✅ Complete | `05e974d` | Typed member handling |
| #63 | #55 | ✅ Complete | `65f3baf` | HTTP fetching for taxonomy-loader |
| #61 | #5 | ✅ Complete | `3828c3e` | Alpha-check JSON summary |
| #53 | SCN-XK-WORKFLOW-005 | ✅ Complete | `7379cc0` | Alpha gate scenario |
| #51 | #4 | ✅ Complete | `4a82af7` | Maintainer wrappers |

## Current State

**Phase 2: COMPLETE ✅**
- Wave 1 (Infrastructure): 3/3 ✅
- Wave 2 (Technical Debt): 3/3 ✅  
- Wave 3 (Documentation): 2/2 ✅

**Metrics:**
- 21 @alpha-active scenarios passing
- 60+ tests passing
- CI: Green

**Next:** Phase 3 planning

## Cron Schedule

| Job | Frequency | Purpose |
|-----|-----------|---------|
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

## Stale PRs Requiring Attention

None. All stale PRs closed on 2026-03-25.

## Phase 3: Feature Completeness — SEC Validation Rules

See `.mend/roadmap-phase-3.md` for full roadmap.

### ✅ Recently Discovered (Already Complete)

| Item | Issue | Status | Notes |
|------|-------|--------|-------|
| Required Facts Validation | #9 | ✅ Complete | Already implemented and active (AC-XK-SEC-REQUIRED-001/002 passing) |

### 🔨 Build (In Progress)

| Item | Issue | Description | Est. Effort |
|------|-------|-------------|-------------|
| Negative Value Validation | #80 | Detect negative values where prohibited by taxonomy | 3-4 days |

**Status:** 
- ✅ numeric-rules crate created
- ✅ Core validation logic implemented
- ✅ Wired into validation-run pipeline
- ✅ BDD scenarios added (5 scenarios)
- ⏳ Build and test

### 📐 Plan (Completed)

| Item | Issue | Description | Est. Effort |
|------|-------|-------------|-------------|
| Negative Value Validation | #80 | Implementation plan complete | — |

### 🔍 Research (Next Up)

| Item | Issue | Description | Est. Effort |
|------|-------|-------------|-------------|
| Decimal Precision Validation | #81 | Validate decimal attribute correctness | 2-3 days |

### 📋 Ready (Planned)

| Item | Issue | Description | Est. Effort |
|------|-------|-------------|-------------|
| Decimal Precision Validation | #81 | Validate decimal attribute correctness | 2-3 days |
| Unit Consistency Validation | #82 | Ensure unit references match fact types | 2-3 days |
| Context Completeness Validation | #83 | Ensure all facts reference valid contexts | 2 days |

### 📋 Planned (Future)

| Wave | Item | Priority | Description |
|------|------|----------|-------------|
| Wave 4 | Performance Optimization | P2 | Streaming parser, parallel validation, caching |
| Wave 5 | IFRS/ESEF Support | P2 | Extended taxonomy support |

## Actions Completed This Run
- ✅ Closed stale PRs #31, #36, #39
- ✅ Closed completed issues #4, #5
- ✅ Created Phase 3 roadmap (`.mend/roadmap-phase-3.md`)
- ✅ Discovered Issue #9 was already complete — closed with documentation
- ✅ Created Issue #80 (Negative Value Validation)
- ✅ Created Issue #81 (Decimal Precision Validation)
- ✅ Created Issue #82 (Unit Consistency Validation)
- ✅ Created Issue #83 (Context Completeness Validation)