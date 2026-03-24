# xbrlkit Roadmap — Phase 2

**Date:** 2026-03-24
**Status:** Wave 1 ✅, Wave 2 ✅, Wave 3 🔄
**Current State:** 21 @alpha-active scenarios passing, CI green

## Goals for Phase 2

1. **✅ Complete workflow infrastructure** — Self-testing alpha gate
2. **✅ Improve developer experience** — Stable maintainer command surface
3. **✅ Close technical debt** — Typed dimensions, HTTP fetching
4. **🔄 Enhance observability** — Machine-readable CI summaries

---

## Progress

### Wave 1: Infrastructure & DevEx ✅ COMPLETE

| # | Issue | Stream | Description | Status |
|---|-------|--------|-------------|--------|
| 1 | #4 | DevEx | Add maintainer wrappers for quick/full gate | ✅ Merged #51 |
| 2 | SCN-XK-WORKFLOW-005 | Workflow | Activate alpha gate scenario | ✅ Merged #53 |
| 3 | #5 | Infra | Post-merge validator summary | ✅ Merged #61 |

### Wave 2: Technical Debt ✅ COMPLETE

| # | Issue | Stream | Description | Status |
|---|-------|--------|-------------|--------|
| 4 | #55 | Taxonomy | HTTP fetching for taxonomy-loader | ✅ Merged #63 |
| 5 | #56 | Dimensions | Typed member handling | ✅ Merged #65 |
| 6 | #57 | Validation | Typed value validation | ✅ Merged #67 |

### Wave 3: Documentation 🔄 IN PROGRESS

| # | Issue | Stream | Description | Status |
|---|-------|--------|-------------|--------|
| 7 | #2 | Docs | Document maintainer command surface | 📋 Ready |
| 8 | #6 | Docs | Document bundle + impact workflow | 📋 Ready |

---

## Success Criteria

- [x] 21+ @alpha-active scenarios passing (21 achieved)
- [x] `make quick` and `make full` commands work locally
- [x] CI emits machine-readable validation receipts
- [ ] All code TODOs have issues or are resolved
- [ ] Maintainer docs complete and accurate

---

*Phase 2 Waves 1-2 complete. Wave 3 documentation in progress.*
