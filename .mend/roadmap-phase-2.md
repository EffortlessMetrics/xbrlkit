# xbrlkit Roadmap — Phase 2

**Date:** 2026-03-24
**Status:** Planning complete, ready for execution
**Current State:** 20 @alpha-active scenarios passing, queue empty

## Goals for Phase 2

1. **Complete workflow infrastructure** — Self-testing alpha gate
2. **Improve developer experience** — Stable maintainer command surface
3. **Close technical debt** — Typed dimensions, HTTP fetching
4. **Enhance observability** — Machine-readable CI summaries

---

## Planned Work Queue

### Wave 1: Infrastructure & DevEx

| # | Issue | Stream | Description | Est |
|---|-------|--------|-------------|-----|
| 1 | #4 | DevEx | Add maintainer wrappers for quick/full gate | 2h |
| 2 | SCN-XK-WORKFLOW-005 | Workflow | Activate alpha gate scenario | 2h |
| 3 | #5 | Infra | Post-merge validator summary | 3h |

### Wave 2: Technical Debt

| # | Issue | Stream | Description | Est |
|---|-------|--------|-------------|-----|
| 4 | TODO | Taxonomy | HTTP fetching for taxonomy-loader | 4h |
| 5 | TODO | Dimensions | Typed member handling | 6h |
| 6 | TODO | Validation | Typed value validation | 4h |

### Wave 3: Documentation

| # | Issue | Stream | Description | Est |
|---|-------|--------|-------------|-----|
| 7 | #2 | Docs | Document maintainer command surface | 2h |
| 8 | #6 | Docs | Document bundle + impact workflow | 2h |

---

## Dependencies

- Wave 1 items are independent, can parallelize
- Wave 2 items build on existing taxonomy/dimension crates
- Wave 3 should follow Wave 1 (wrappers must exist before documenting)

---

## Success Criteria

- [ ] 23+ @alpha-active scenarios passing
- [ ] `make quick` and `make full` commands work locally
- [ ] CI emits machine-readable validation receipts
- [ ] All code TODOs have issues or are resolved
- [ ] Maintainer docs complete and accurate

---

*Next action: Populate queue with Wave 1 items and begin execution.*
