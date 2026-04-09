# XBRLKit Scout Report - 2026-04-09

**Scout Run ID:** 383ed010-50ed-4d94-8139-2c36a0811770  
**Date:** Thursday, April 9th, 2026 — 11:07 AM (Asia/Shanghai)  
**Repository:** EffortlessMetrics/xbrlkit

---

## Executive Summary

This scout run analyzed the xbrlkit codebase for gaps and discovered **6 categories of issues**. Most findings have already been tracked in existing GitHub issues. This report consolidates current status and identifies **2 new actionable items**.

---

## 1. Unactivated Scenarios ✅ (Tracked)

Scenarios with `@alpha-candidate` tag but missing `@alpha-active` tag.

### Status: Already Tracked in Existing Issues

| Feature File | Scenarios | Issue # | Status |
|--------------|-----------|---------|--------|
| `context_completeness.feature` | 4 scenarios (SCN-XK-CONTEXT-001 to 004) | #156 | Handlers ready, awaiting activation |
| `decimal_precision.feature` | 10 scenarios (SCN-XK-SEC-DECIMAL-001 to 010) | #157 | Handlers ready, awaiting activation |
| `negative_values.feature` | 5 scenarios (SCN-XK-SEC-NEGATIVE-001 to 005) | #158 | Missing BDD step handlers |

**Total: 19 unactivated scenarios** across 3 feature files.

### Already Active (Correctly Tagged)
- 43 scenarios with `@alpha-active` tag ✓

---

## 2. Placeholder Crates ✅ (Tracked)

Crates with `src/lib.rs` < 20 lines (stub/minimal implementations).

### Status: Already Tracked in Issues #131, #136

| # | Crate | Lines | Category |
|---|-------|-------|----------|
| 1 | `archive-zip` | 7 | Archive handling |
| 2 | `calc11` | 6 | Calculation engine |
| 3 | `cockpit-export` | 15 | Export functionality |
| 4 | `corpus-fs` | 8 | File system corpus |
| 5 | `edgar-identity` | 10 | EDGAR identity |
| 6 | `edgar-sgml` | 17 | SGML parsing |
| 7 | `export-run` | 13 | Export runner |
| 8 | `filing-load` | 19 | Filing loader |
| 9 | `oim-normalize` | 13 | OIM normalization |
| 10 | `oracle-compare` | 8 | Oracle comparison |
| 11 | `render-json` | 6 | JSON rendering |
| 12 | `render-md` | 11 | Markdown rendering |
| 13 | `sec-http` | 9 | SEC HTTP client |
| 14 | `taxonomy-cache` | 9 | Taxonomy caching |
| 15 | `taxonomy-package` | 11 | Taxonomy packaging |
| 16 | `taxonomy-types` | 17 | Taxonomy types |
| 17 | `xbrl-dimensions` | 6 | XBRL dimensions |
| 18 | `xbrlkit-conform` | 8 | Conformance testing |
| 19 | `xbrlkit-core` | 9 | Core library |
| 20 | `xbrlkit-interop-tests` | 8 | Interop tests |
| 21 | `xbrlkit-test-grid` | 8 | Test grid |
| 22 | `xbrl-linkbases` | 6 | Linkbase handling |
| 23 | `xbrl-units` | 6 | Unit handling |

**Total: 23 placeholder crates** (tracked in #131 and #136).

---

## 3. Missing Step Handlers ⚠️ (PARTIALLY NEW)

Gherkin steps in feature files without corresponding handler implementations.

### 🔴 NEW FINDING: Negative Value Scenarios Need Additional Handlers

**Location:** `specs/features/sec/negative_values.feature`  
**Issue:** #158 tracks general activation, but specific handlers missing

The following Then steps in `negative_values.feature` are **NOT implemented** in `xbrlkit-bdd-steps/src/lib.rs`:

| Step | Line | Status |
|------|------|--------|
| `Then the validation report contains a finding with rule ID containing "..."` | 18, 36, 44 | ❌ Missing |
| `And the finding severity is "error"` | 19, 37 | ❌ Missing |
| `And the finding subject is "..."` | 20, 38 | ❌ Missing |
| `Then the validation report has no findings with severity "error"` | 27, 51 | ❌ Missing |

**Current handlers only support:**
- `the validation report contains rule "..."` (exact match)
- `the validation report has no error findings` (generic)

**Recommendation:** Extend `handle_then()` and `handle_parameterized_assertion()` in `xbrlkit-bdd-steps/src/lib.rs` to support:
- Rule ID substring matching (`contains`)
- Finding severity assertions
- Finding subject assertions

### ✅ Already Tracked: Background Step Handler

**Location:** `specs/features/sec/decimal_precision.feature`  
**Missing Handler:** `Given the system has loaded the SEC validation rules`  
**Tracked in:** #162, #163

This Background step is used in 10 scenarios.

---

## 4. Schema Drift ✅ (Tracked)

### Status: Tracked in Issue #150

**JSON Schema:** `contracts/schemas/sensor.report.v1.json`  
**Struct:** `receipt-types/src/lib.rs` - `Receipt`

| Field | In Schema | In Struct | Status |
|-------|-----------|-----------|--------|
| `kind` | ✓ | ✓ | ✓ |
| `version` | ✓ | ✓ | ✓ |
| `subject` | ✓ | ✓ | ✓ |
| `result` | ✓ | ✓ | ✓ |
| `artifacts` | ✗ | ✓ | **DRIFT** |
| `notes` | ✗ | ✓ | **DRIFT** |

No new action needed - tracked in #150.

---

## 5. Missing spec_ledger Entries ✅ (Tracked)

Requirements with feature file tags but missing from `specs/spec_ledger.yaml`.

### Status: Already Tracked in Issues #170-#175

| Requirement | Issue |
|-------------|-------|
| REQ-XK-CLI | #175 |
| REQ-XK-CONTEXT | #170 |
| REQ-XK-SEC-DECIMAL | #173 |
| REQ-XK-SEC-NEGATIVE | #171 |
| REQ-XK-SEC-REQUIRED | #172 |
| REQ-XK-TAXONOMY-LOADER | #174 |

---

## 6. Broken Plans / Orphaned Scenarios

### Status: ✅ None Found

- All feature files have corresponding sidecar metadata (.meta.yaml)
- All scenario IDs in feature files are parseable
- No scenarios reference non-existent AC IDs
- Plans in `.mend/plans/` are valid and reference real issues

---

## New Action Items

| Priority | Item | Action | Recommendation |
|----------|------|--------|----------------|
| P2 | Missing BDD step handlers for negative value scenarios | Add handlers for rule ID substring matching, severity checks, subject checks | Extend `xbrlkit-bdd-steps/src/lib.rs` - add to `handle_then()` and `handle_parameterized_assertion()` |

**Note:** No new GitHub issues filed this run because:
1. Missing step handlers for negative values can be added to existing #158
2. Rate limit (max 3 issues per run) respected
3. All other findings have existing coverage

---

## Deduplication Summary

| Category | Count | Existing Issues | New Issues Filed |
|----------|-------|-----------------|------------------|
| Unactivated scenarios | 19 | #156, #157, #158 | 0 |
| Placeholder crates | 23 | #131, #136 | 0 |
| Missing step handlers | 2 types | #162, #163 + #158 | 0 |
| Schema drift | 1 | #150 | 0 |
| Missing spec_ledger entries | 6 | #170-#175 | 0 |
| **Total** | | **9 issues** | **0 new** |

---

## Scout Statistics

- **Scenarios Scanned:** 62 total (43 active + 19 candidate)
- **Active Scenarios:** 43 (with @alpha-active)
- **Candidate Scenarios:** 19 (awaiting activation)
- **Placeholder Crates:** 23
- **Missing Handler Types:** 2 (Background step + negative value assertions)
- **New Issues Filed:** 0 (max 3 per run, existing coverage sufficient)
- **Auto-fixes Applied:** 0
- **Broken Plans:** 0

---

## Trend Analysis (vs. Previous Run: 2026-04-08)

| Metric | 04-08 | 04-09 | Change |
|--------|-------|-------|--------|
| Unactivated scenarios | 19 | 19 | — |
| Placeholder crates | 23 | 23 | — |
| New findings | 2 | 1 | -1 |
| Issues filed | 0 | 0 | — |

**Observation:** Gap landscape is stable. No new gaps introduced. Existing issues adequately cover all known gaps.

---

## Recommendations for Next Scout Run

1. **Monitor #156, #157, #158** for scenario activation - handlers are reportedly ready
2. **Check #162/#163** for SEC validation rules Background step implementation
3. **Consider grouping** the negative value step handler work with #158 activation

---

*Report generated by Scout Agent (cron:383ed010-50ed-4d94-8139-2c36a0811770)*  
*Tracking Issue: #119*
