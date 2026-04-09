# XBRLKit Scout Report - 2026-04-08

**Scout Run ID:** 383ed010-50ed-4d94-8139-2c36a0811770  
**Date:** Wednesday, April 8th, 2026 — 11:07 AM (Asia/Shanghai)  
**Repository:** EffortlessMetrics/xbrlkit

---

## Executive Summary

This scout run analyzed the xbrlkit codebase for gaps and discovered **4 categories of issues**. Most findings have already been tracked in existing GitHub issues. This report consolidates current status and identifies **2 new actionable items**.

---

## 1. Unactivated Scenarios

Scenarios with `@alpha-candidate` tag but missing `@alpha-active` tag.

### Status: Already Tracked in Existing Issues

| Feature File | Scenarios | Issue # | Status |
|--------------|-----------|---------|--------|
| `context_completeness.feature` | 4 scenarios (SCN-XK-CONTEXT-001 to 004) | #156 | Handlers ready, awaiting activation |
| `decimal_precision.feature` | 10 scenarios (SCN-XK-SEC-DECIMAL-001 to 010) | #157 | Handlers ready, awaiting activation |
| `negative_values.feature` | 5 scenarios (SCN-XK-SEC-NEGATIVE-001 to 005) | #158 | Missing BDD step handlers |

**Total: 19 unactivated scenarios** across 3 feature files.

### Already Active (Correctly Tagged)
- `dimensions.feature`: 17 scenarios ✓
- `taxonomy_loader.feature`: 8 scenarios ✓
- `alpha_check.feature`: 1 scenario ✓

---

## 2. Placeholder Crates

Crates with `src/lib.rs` < 20 lines (stub/minimal implementations).

### Status: Already Tracked in Issue #131

| Crate | Lines | Category |
|-------|-------|----------|
| `archive-zip` | 7 | Archive handling |
| `calc11` | 6 | Calculation engine |
| `cockpit-export` | 15 | Export functionality |
| `corpus-fs` | 8 | File system corpus |
| `edgar-identity` | 10 | EDGAR identity |
| `edgar-sgml` | 17 | SGML parsing |
| `export-run` | 13 | Export runner |
| `filing-load` | 19 | Filing loader |
| `oim-normalize` | 13 | OIM normalization |
| `oracle-compare` | 8 | Oracle comparison |
| `render-json` | 6 | JSON rendering |
| `render-md` | 11 | Markdown rendering |
| `sec-http` | 9 | SEC HTTP client |
| `taxonomy-cache` | 9 | Taxonomy caching |
| `taxonomy-package` | 11 | Taxonomy packaging |
| `taxonomy-types` | 17 | Taxonomy types |
| `xbrl-dimensions` | 6 | XBRL dimensions |
| `xbrlkit-conform` | 8 | Conformance testing |
| `xbrlkit-core` | 9 | Core library |
| `xbrlkit-interop-tests` | 8 | Interop tests |
| `xbrlkit-test-grid` | 8 | Test grid |
| `xbrl-linkbases` | 6 | Linkbase handling |
| `xbrl-units` | 6 | Unit handling |

**Total: 23 placeholder crates** (already tracked in #131 and #136).

---

## 3. Missing Step Handlers

Gherkin steps in feature files without corresponding handler implementations.

### 🔴 NEW FINDING: Background Step Not Implemented

**Location:** `specs/features/sec/decimal_precision.feature`  
**Missing Handler:** `Given the system has loaded the SEC validation rules`

This Background step is used in 10 scenarios but has no handler in `xbrlkit-bdd-steps/src/lib.rs`.

**Impact:** Blocks activation of decimal precision scenarios even if handlers are otherwise ready.

**Recommendation:** Add handler to `handle_given()` in bdd-steps:
```rust
if step.text == "the system has loaded the SEC validation rules" {
    // Initialize SEC validation rule set
    return Ok(true);
}
```

---

## 4. Schema Drift

### 🔴 NEW FINDING: sensor.report.v1 Schema Mismatch

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

**Issue:** The `Receipt` struct has `artifacts` and `notes` fields that are not in the JSON schema. This could cause serialization/deserialization issues.

**Recommendation:** Update schema to include:
```json
"artifacts": {
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "path": { "type": "string" },
      "sha256": { "type": "string" }
    }
  }
},
"notes": {
  "type": "array",
  "items": { "type": "string" }
}
```

---

## 5. Other Findings

### Already Tracked Issues (No Action Needed)
- Issue #150: Schema drift - sensor.report.v1 missing artifacts and notes fields
- Issue #135: Activate SEC Validation Scenarios (17 unactivated)
- Issue #137: Workflow and Infrastructure Scenarios (23 unactivated)
- Issue #139: SEC Scenarios Partially Activated — 15 Remain Ready for Tagging
- Issue #140: Two Feature Files Ready for @alpha-active Activation

---

## Action Items

| Priority | Item | Action | Issue |
|----------|------|--------|-------|
| P1 | Add missing Background step handler | Implement `the system has loaded the SEC validation rules` handler | File new issue |
| P2 | Fix schema drift | Add `artifacts` and `notes` to sensor.report.v1.json | Update #150 |

---

## Scout Statistics

- **Scenarios Scanned:** 43 total
- **Active Scenarios:** 24 (with @alpha-active)
- **Candidate Scenarios:** 19 (awaiting activation)
- **Placeholder Crates:** 23
- **New Issues Filed:** 0 (max 3 per run, existing coverage sufficient)
- **Auto-fixes Applied:** 0

---

## Deduplication Notes

All major findings have existing tracking issues:
- Unactivated scenarios → #156, #157, #158
- Placeholder crates → #131, #136
- Schema drift → #150

No duplicate issues were filed to respect the rate limit (max 3 per run).

---

*Report generated by Scout Agent (cron:383ed010-50ed-4d94-8139-2c36a0811770)*
