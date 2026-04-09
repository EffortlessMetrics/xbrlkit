# Scout Report — 2026-04-06

**Scout Run ID:** scout-issue-finder-2026-04-06
**Repository:** EffortlessMetrics/xbrlkit
**Date:** Monday, April 6th, 2026

## Summary

| Category | Count | Status |
|----------|-------|--------|
| Unactivated Scenarios (@alpha-candidate) | 24 | Ready for activation |
| Placeholder Crates | 23 | Need implementation |
| Missing Step Handlers | 3+ | Need implementation |
| Broken Plans | 2 | Need review |
| Total Findings | 50+ | See details below |

---

## 1. Unactivated Scenarios (24 scenarios)

These scenarios have `@alpha-candidate` but lack `@alpha-active`. They are ready for activation pending step handler implementation or fixture completion.

### SEC Validation (19 scenarios)

| Feature File | Scenario ID | AC ID | Status |
|--------------|-------------|-------|--------|
| `sec/negative_values.feature` | SCN-XK-SEC-NEGATIVE-001 | AC-XK-SEC-NEGATIVE-001 | Missing handler |
| `sec/negative_values.feature` | SCN-XK-SEC-NEGATIVE-002 | AC-XK-SEC-NEGATIVE-002 | Missing handler |
| `sec/negative_values.feature` | SCN-XK-SEC-NEGATIVE-003 | AC-XK-SEC-NEGATIVE-003 | Missing handler |
| `sec/negative_values.feature` | SCN-XK-SEC-NEGATIVE-004 | AC-XK-SEC-NEGATIVE-004 | Missing handler |
| `sec/negative_values.feature` | SCN-XK-SEC-NEGATIVE-005 | AC-XK-SEC-NEGATIVE-005 | Missing handler |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-001 | AC-XK-SEC-DECIMAL-001 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-002 | AC-XK-SEC-DECIMAL-001 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-003 | AC-XK-SEC-DECIMAL-001 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-004 | AC-XK-SEC-DECIMAL-001 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-005 | AC-XK-SEC-DECIMAL-002 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-006 | AC-XK-SEC-DECIMAL-002 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-007 | AC-XK-SEC-DECIMAL-001 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-008 | AC-XK-SEC-DECIMAL-002 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-009 | AC-XK-SEC-DECIMAL-002 | Handler exists |
| `sec/decimal_precision.feature` | SCN-XK-SEC-DECIMAL-010 | AC-XK-SEC-DECIMAL-001 | Handler exists |

### Foundation (4 scenarios)

| Feature File | Scenario ID | AC ID | Status |
|--------------|-------------|-------|--------|
| `foundation/context_completeness.feature` | SCN-XK-CONTEXT-001 | AC-XK-CONTEXT-001 | Handler exists |
| `foundation/context_completeness.feature` | SCN-XK-CONTEXT-002 | AC-XK-CONTEXT-002 | Handler exists |
| `foundation/context_completeness.feature` | SCN-XK-CONTEXT-003 | AC-XK-CONTEXT-003 | Handler exists |
| `foundation/context_completeness.feature` | SCN-XK-CONTEXT-004 | AC-XK-CONTEXT-004 | Handler exists |

### Workflow (1 scenario)

| Feature File | Scenario ID | AC ID | Status |
|--------------|-------------|-------|--------|
| `workflow/package_check.feature` | SCN-XK-WORKFLOW-006 | AC-XK-WORKFLOW-004 | Needs handler |

---

## 2. Placeholder Crates (23 crates)

These crates have minimal implementations (< 20 lines of actual code) and need full implementation:

| Crate | Lines | Status |
|-------|-------|--------|
| `crates/archive-zip` | 7 | Placeholder - only `open_zip()` stub |
| `crates/calc11` | 6 | Placeholder - only `calculate_ready()` returns false |
| `crates/corpus-fs` | 8 | Placeholder - minimal file reading wrapper |
| `crates/edgar-identity` | 10 | Placeholder |
| `crates/edgar-sgml` | 17 | Placeholder |
| `crates/export-run` | 13 | Placeholder |
| `crates/filing-load` | 19 | Placeholder - minimal manifest loading |
| `crates/oim-normalize` | 13 | Placeholder |
| `crates/oracle-compare` | 8 | Placeholder |
| `crates/render-json` | 6 | Placeholder |
| `crates/render-md` | 11 | Placeholder |
| `crates/sec-http` | 9 | Placeholder |
| `crates/taxonomy-cache` | 9 | Placeholder |
| `crates/taxonomy-package` | 11 | Placeholder |
| `crates/taxonomy-types` | 17 | Placeholder |
| `crates/unit-rules` | 28 | Placeholder - minimal unit validation stubs |
| `crates/xbrl-dimensions` | 6 | **Critical** - Empty placeholder |
| `crates/xbrlkit-conform` | 8 | Placeholder |
| `crates/xbrlkit-core` | 8 | Placeholder |
| `crates/xbrlkit-interop-tests` | 8 | Placeholder |
| `crates/xbrlkit-test-grid` | 8 | Placeholder |
| `crates/xbrl-linkbases` | 6 | Placeholder |
| `crates/xbrl-units` | 6 | Placeholder |

---

## 3. Missing Step Handlers

The following BDD steps are used in feature files but have NO handlers in `crates/xbrlkit-bdd-steps/src/lib.rs`:

### Critical Missing Handlers

| Step Pattern | Used In | Priority |
|--------------|---------|----------|
| `Given an inline XBRL document with fact "..." valued "..."` | negative_values.feature (5x) | **High** |
| `When I parse the context dimensions` | dimensions.feature (6x) | **High** |
| `Given a context with typed dimension "..." in segment` | dimensions.feature (1x) | Medium |

### Partial Handlers (may need expansion)

| Step Pattern | Issue | Notes |
|--------------|-------|-------|
| `Then the explicit dimension should have member "..."` | Not implemented | dimensions.feature |
| `Then the typed dimension should be in the entity segment` | Not implemented | dimensions.feature |

---

## 4. Broken/Stalled Plans

| Issue | Plan File | Status | Problem |
|-------|-----------|--------|---------|
| #102 | `.mend/plans/ISSUE-102.md` | 📝 Plan Draft | ADR-008 creation still pending |
| #118 | `.mend/plans/taxonomy-loader.md` reference | plan-needs-work | Stalled - sensor.report.v1 wrapping incomplete |

---

## 5. Other Gaps

### Schema Drift
- No automated check for receipt struct vs JSON schema alignment
- Issue #150 mentions sensor.report.v1 missing artifacts and notes fields

### Test Coverage
- Some placeholder crates have no tests
- Streaming parser scenarios may need performance benchmarking infrastructure

### Documentation Gaps
- Several placeholder crates lack proper module documentation
- ADR index may be incomplete

---

## Recommendations

### Immediate (This Week)
1. **File GitHub issues** for missing step handlers in negative_values.feature
2. **Activate decimal_precision scenarios** - handlers appear complete
3. **Activate context_completeness scenarios** - handlers appear complete

### Short-term (Next 2 Weeks)
1. **Implement missing BDD step handlers** for dimensions.feature parsing steps
2. **Review and update** plan files for issues #102 and #118
3. **Prioritize placeholder crate implementation** based on roadmap needs

### Medium-term (Next Month)
1. **Create implementation plans** for the 23 placeholder crates
2. **Add automated schema drift detection** to CI
3. **Complete ADR-008** documentation for taxonomy-loader HTTP client

---

## Related Issues

- #119 - [Design Proposal] Create Scout Agent for Autonomous Issue Discovery (this tracking issue)
- #132 - [Tracking] Activate 19 @alpha-candidate Scenarios for Alpha Gate
- #135 - [Scout] Activate SEC Validation Scenarios (17 unactivated)
- #136 - [Scout] Placeholder Crates Need Implementation (23 crates)
- #137 - [Scout] Workflow and Infrastructure Scenarios (23 unactivated)
- #139 - [Scout] SEC Scenarios Partially Activated — 15 Remain Ready for Tagging
- #140 - [Scout] Two Feature Files Ready for @alpha-active Activation
- #150 - [Scout] Schema drift: sensor.report.v1 missing artifacts and notes fields

---

*Report generated by scout-issue-finder agent*
