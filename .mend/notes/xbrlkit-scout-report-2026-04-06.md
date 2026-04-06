# XBRLKit Repository Scout Report

**Generated:** 2026-04-06 03:07 UTC  
**Repository:** EffortlessMetrics/xbrlkit  
**Analysis Scope:** Feature completeness, crate health, BDD coverage, schema alignment

---

## Executive Summary

| Finding Type | Count | Severity | Trend* |
|--------------|-------|----------|--------|
| Unactivated Scenarios | 38 | Medium | ↓2 improved |
| Placeholder Crates | 23 | Low | → stable |
| Orphaned Scenarios | 0 | - | → stable |
| Missing Step Handlers | ~40 | High | → stable |
| Schema Drift | 1 | Medium | ↑1 new |
| Broken/Missing Plans | Multiple | Low | → stable |

\* Trend compares to 2026-04-01 baseline

---

## 1. Unactivated Scenarios 🔕

**Count:** 38 scenarios lack `@alpha-active` tag (down from 40)

### Recently Activated (2 scenarios since last run)
| Scenario ID | File | Notes |
|-------------|------|-------|
| SCN-XK-SEC-REQUIRED-001 | sec/required_facts.feature | Now active |
| SCN-XK-SEC-REQUIRED-002 | sec/required_facts.feature | Now active |

### Still Inactive by Category

#### Foundation (4 scenarios)
- SCN-XK-CONTEXT-001 through SCN-XK-CONTEXT-004
- File: `foundation/context_completeness.feature`

#### SEC Rules (15 scenarios)
- **Decimal Precision:** SCN-XK-SEC-DECIMAL-001 through 010 (10 scenarios)
- **Negative Values:** SCN-XK-SEC-NEGATIVE-001 through 005 (5 scenarios)
- File: `sec/decimal_precision.feature`, `sec/negative_values.feature`

#### Performance/Streaming (3 scenarios)
- SCN-XK-STREAM-002, SCN-XK-STREAM-003, SCN-XK-STREAM-004
- File: `performance/streaming_parser.feature`
- Note: SCN-XK-STREAM-001 is now active

#### Workflow (6 scenarios)
- SCN-XK-WORKFLOW-001, 002, 003, 004, 005, 006
- Files: `workflow/*.feature`

#### Inline XBRL (0 scenarios)
- ✅ ALL ACTIVE: SCN-XK-IXDS-001, SCN-XK-IXDS-002

#### Taxonomy (1 scenario)
- SCN-XK-TAXONOMY-001
- Note: SCN-XK-TAXONOMY-002 is now active

#### CLI (1 scenario)
- SCN-XK-CLI-001 - appears active, needs verification

#### Export (1 scenario)
- SCN-XK-EXPORT-001 - appears active, needs verification

---

## 2. Placeholder Crates 📦

**Count:** 23 crates with `src/lib.rs` < 20 lines (unchanged)

| Lines | Crate | Status |
|-------|-------|--------|
| 6 | calc11 | placeholder |
| 6 | render-json | placeholder |
| 6 | xbrl-dimensions | placeholder |
| 6 | xbrl-linkbases | placeholder |
| 6 | xbrl-units | placeholder |
| 7 | archive-zip | placeholder |
| 8 | corpus-fs | placeholder |
| 8 | oracle-compare | placeholder |
| 8 | xbrlkit-conform | placeholder |
| 8 | xbrlkit-interop-tests | placeholder |
| 8 | xbrlkit-test-grid | placeholder |
| 9 | sec-http | placeholder |
| 9 | taxonomy-cache | placeholder |
| 9 | xbrlkit-core | placeholder |
| 10 | edgar-identity | placeholder |
| 11 | render-md | placeholder |
| 11 | taxonomy-package | placeholder |
| 13 | export-run | placeholder |
| 13 | oim-normalize | placeholder |
| 15 | cockpit-export | placeholder |
| 17 | edgar-sgml | placeholder |
| 17 | taxonomy-types | placeholder |
| 19 | filing-load | placeholder |

---

## 3. Orphaned Scenarios 🔍

**Count:** 0 confirmed orphaned scenarios

All scenarios have at least partial handler infrastructure in `crates/xbrlkit-bdd-steps/src/lib.rs`.

---

## 4. Schema Drift 🔄

**Count:** 1 potential drift detected

| Location | Issue | Severity |
|----------|-------|----------|
| `receipt_types::Receipt` | `sensor.report.v1` missing `artifacts` and `notes` fields | Medium |

Already filed as Issue #150.

---

## 5. Missing Step Handlers ⚠️

**Count:** ~40 Gherkin steps without full handler implementations

### Critical Missing (blocks scenario activation):
1. `When the document is validated` (used in SEC scenarios)
2. `When I validate it using the streaming parser` (streaming)
3. `Then memory usage should stay under 50MB peak` (streaming)
4. `When I bundle the selector "..."` (workflow - partial)
5. `When I run describe-profile --json` (CLI)

---

## 6. GitHub Issues Status 📋

### Existing Scout-Discovered Issues

| Issue | Title | Status |
|-------|-------|--------|
| #135 | [Scout] Activate SEC Validation Scenarios (17 unactivated) | Open |
| #136 | [Scout] Placeholder Crates Need Implementation (23 crates) | Open |
| #137 | [Scout] Workflow and Infrastructure Scenarios (23 unactivated) | Open |
| #139 | [Scout] SEC Scenarios Partially Activated — 15 Remain Ready for Tagging | Open |
| #140 | [Scout] Two Feature Files Ready for @alpha-active Activation | Open |
| #150 | [Scout] Schema drift: sensor.report.v1 missing artifacts and notes fields | Open |

### Recommendations

No new issues filed this run (max 3 per run policy; existing issues cover findings).

---

## Auto-Fixes Applied ✅

None applied this run. Trivial fixes identified but deferred:
- SCN-XK-TAXONOMY-001 activation (needs handler verification first)
- Workflow scenario batch activation (pending handler completion)

---

## Recommendations

### Immediate (This Sprint)
1. **Complete SEC activation** - Issues #135, #139 cover this
2. **Verify CLI/Export scenario status** - May already be active

### Short-term (Next 2 Sprints)
1. **Implement missing step handlers** for streaming parser assertions
2. **Document placeholder crates** with READMEs explaining intent

### Long-term
1. **Reduce placeholder crate count** - implement or remove from workspace
2. **Achieve 100% step handler coverage**

---

*Report generated by Scout Agent (cron:383ed010-50ed-4d94-8139-2c36a0811770)*
