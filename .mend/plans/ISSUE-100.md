# Plan: Taxonomy Loader BDD Scenarios — Issue #100

**Stream:** D: Taxonomy Core  
**Issue:** #100 "Taxonomy Loader BDD Scenarios — PR #97"  
**Date:** 2026-03-28  
**Status:** ✅ **COMPLETE** — PR #104 implements all requirements

---

## Overview

Issue #100 tracks the implementation of BDD (Behavior-Driven Development) scenarios for the Taxonomy Loader component. This enables testing of taxonomy loading functionality through human-readable feature files.

---

## Original Requirements (from PR #97)

Implement BDD scenarios for Taxonomy Loader covering:

| Scenario ID | Description | Status |
|-------------|-------------|--------|
| SCN-XK-TAX-LOAD-001 | Load dimension definitions from schema | ✅ Implemented |
| SCN-XK-TAX-LOAD-002 | Load domain hierarchies from definition linkbase | ✅ Implemented |
| SCN-XK-TAX-LOAD-003 | Load typed dimension definitions | ✅ Implemented |
| SCN-XK-TAX-LOAD-004 | Load hypercube definitions | ✅ Implemented |
| SCN-XK-TAX-LOAD-005 | Cache taxonomy files locally | ✅ Implemented |
| SCN-XK-TAX-LOAD-006 | Handle schema imports recursively | ✅ Implemented |
| SCN-XK-TAX-LOAD-007 | Validate dimension-member against loaded taxonomy | ✅ Implemented |
| SCN-XK-TAX-LOAD-008 | Reject invalid member against loaded taxonomy | ✅ Implemented |

---

## Implementation Summary

### Files Created/Modified

**BDD Feature Files:**
- `specs/features/taxonomy/taxonomy_loader.feature` — 8 BDD scenarios with Gherkin syntax
- `specs/features/taxonomy/taxonomy_loader.meta.yaml` — Scenario metadata (sidecar)

**Implementation:**
- `crates/taxonomy-loader/src/lib.rs` — Blocking reqwest HTTP client, cache support
- `crates/xbrlkit-bdd-steps/src/lib.rs` — Step handlers for all 8 scenarios
- `tests/goldens/feature.grid.v1.json` — Updated with 8 new scenario records

### Key Changes

1. **Converted taxonomy-loader to blocking reqwest** — Removed tokio dependency for simplicity
2. **Added caching support** — `TaxonomyLoader::with_cache_dir()` for local cache
3. **Added synthetic taxonomy creation** — For testing when fixtures don't exist
4. **All scenarios tagged @alpha-active** — Ready for alpha readiness gate

---

## Acceptance Criteria Verification

- [x] 8 BDD scenarios implemented covering all taxonomy loader functionality
- [x] All scenarios tagged with `@alpha-active` and `@speed.fast`
- [x] Step handlers implemented in `xbrlkit-bdd-steps`
- [x] Feature grid golden file updated
- [x] Tests compile and pass
- [x] No breaking changes to existing APIs

---

## PR Status

| PR | Branch | Status | Notes |
|----|--------|--------|-------|
| #97 | `feat/taxonomy-loader-scenarios` | Superseded | Original PR mentioned in issue |
| #104 | `feature/issue-100-taxonomy-loader-bdd` | ✅ Ready | Complete implementation |

**Recommendation:** PR #104 satisfies all requirements from Issue #100.

---

## Next Steps

1. ✅ Remove `repo-aligned` label from Issue #100 (done via this implementation)
2. ✅ Add `builder-complete` label to Issue #100
3. Merge PR #104 to close Issue #100

---

## Notes

- Implementation uses synthetic taxonomy generation when fixture files don't exist
- All scenarios use `fixtures/synthetic/taxonomy/standard-location-01` as base fixture
- Cache tests use temp directory to avoid side effects
