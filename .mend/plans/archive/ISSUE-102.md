# Plan: ADR-008 - Taxonomy-loader HTTP Client Architecture

**Stream:** Architecture Decision Records  
**Issue:** #102  
**Date:** 2026-03-28  
**Status:** 📝 Plan Draft

---

## Overview

This plan documents the creation of **ADR-008** to formally record the architectural decision to use **blocking reqwest** instead of **async tokio** for the taxonomy-loader component's HTTP client.

The decision has already been made and implemented; this work focuses on properly documenting it in the project's ADR registry for future reference and institutional knowledge preservation.

---

## Acceptance Criteria Breakdown

| # | Criterion | Status |
|---|-----------|--------|
| 1 | Create `adr/ADR-008-taxonomy-loader-http-client.md` following project ADR format | Pending |
| 2 | Document the decision context (why this choice was needed) | Pending |
| 3 | Document the decision (blocking reqwest) | Pending |
| 4 | Document the rationale (simplicity, testing, overhead) | Pending |
| 5 | Document consequences (positive and negative) | Pending |
| 6 | Reference related issues/PRs (#97 BDD scenarios) | Pending |
| 7 | Update any relevant code comments to reference ADR-008 | Pending |

---

## Proposed Approach

### 1. ADR Document Structure

Following the established ADR format (see ADR-007 as reference):

```
adr/ADR-008-taxonomy-loader-http-client.md
├── Title and Decision Statement
├── Context (problem statement)
├── Decision (blocking reqwest)
├── Rationale (detailed reasoning)
├── Consequences (trade-offs table)
├── Related ADRs/Issues/PRs
└── Status
```

### 2. Key Content Points

**Context:**
- Taxonomy-loader needs to fetch XBRL taxonomy files from remote sources (SEC, XBRL International)
- Decision needed on sync vs async HTTP client architecture
- Trade-off between simplicity and concurrency performance

**Decision:**
- Use `reqwest` with **blocking** client API
- Explicitly avoid async/tokio for this component

**Rationale:**
- Taxonomy loading is typically sequential (schemas depend on each other)
- Async overhead not justified for expected use case
- Blocking code is easier to reason about and test
- Reduces dependency tree (no tokio runtime needed)

**Consequences:**

| Positive | Negative |
|----------|----------|
| Simpler implementation | Less efficient for high-concurrency scenarios |
| Easier unit testing | Cannot leverage async ecosystem |
| Reduced cognitive overhead | Blocking I/O in async contexts requires care |
| Smaller dependency footprint | |

---

## Files to Modify/Create

### New Files
| Path | Purpose |
|------|---------|
| `adr/ADR-008-taxonomy-loader-http-client.md` | The ADR document itself |

### Files to Review (potential updates)
| Path | Purpose |
|------|---------|
| `crates/taxonomy-loader/src/lib.rs` | Add ADR reference comment if HTTP client is visible |
| `crates/taxonomy-loader/Cargo.toml` | Verify reqwest dependency is blocking-only |

---

## Test Strategy

This is a documentation task; no code tests required.

**Verification:**
- [ ] ADR markdown renders correctly
- [ ] All links in ADR resolve
- [ ] ADR follows project formatting conventions
- [ ] ADR is discoverable from main ADR index (if one exists)

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| ADR format inconsistency | Low | Low | Follow ADR-007 template exactly |
| Missing related references | Medium | Low | Cross-check with issue #102 and PR #97 |
| Incorrect consequence analysis | Low | Medium | Review decision with implementation |

**Overall Risk Level:** 🟢 Low

---

## Estimated Effort

| Task | Estimate |
|------|----------|
| Draft ADR content | 30 min |
| Review against existing ADRs | 15 min |
| Cross-reference related issues | 15 min |
| Final review and formatting | 15 min |
| **Total** | **~1.25 hours** |

---

## Related Work

- **Issue #102**: This tracking issue
- **PR #97**: BDD scenarios (related to taxonomy-loader)
- **ADR-007**: Reference for ADR format
- **taxonomy-loader crate**: Implementation location

---

## Notes

- The decision is already implemented; this is purely documentation
- Consider whether this ADR should be linked from the taxonomy-loader crate documentation
- May need to update `adr/README.md` or similar index if one exists

---

## Plan Author

*planner-initial agent*  
*Created: 2026-03-28*
