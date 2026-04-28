# XBRLKit Repository Scout Report

**Generated:** 2026-04-01  
**Repository:** xbrlkit  
**Analysis Scope:** Feature completeness, crate health, BDD coverage

---

## Executive Summary

| Finding Type | Count | Severity |
|--------------|-------|----------|
| Unactivated Scenarios | 40 | Medium |
| Placeholder Crates | 23 | Low |
| Orphaned Scenarios (no handlers) | 0 | - |
| Missing Step Handlers | ~45 | High |
| Missing Plans | Multiple | Low |

---

## 1. Unactivated Scenarios 🔕

**Count:** 40 scenarios lack `@alpha-active` tag

Scenarios with `@SCN-*` tags but **WITHOUT** `@alpha-active`:

### Foundation (4 scenarios)
| Scenario ID | File | AC Tag |
|-------------|------|--------|
| SCN-XK-CONTEXT-001 | foundation/context_completeness.feature | AC-XK-CONTEXT-001 |
| SCN-XK-CONTEXT-002 | foundation/context_completeness.feature | AC-XK-CONTEXT-002 |
| SCN-XK-CONTEXT-003 | foundation/context_completeness.feature | AC-XK-CONTEXT-003 |
| SCN-XK-CONTEXT-004 | foundation/context_completeness.feature | AC-XK-CONTEXT-004 |
| SCN-XK-DUPLICATES-001 | foundation/duplicate_facts.feature | AC-XK-DUPLICATES-001 |
| SCN-XK-MANIFEST-001 | foundation/filing_manifest.feature | AC-XK-MANIFEST-001 |

### SEC Rules (17 scenarios)
| Scenario ID | File | AC Tag |
|-------------|------|--------|
| SCN-XK-SEC-DECIMAL-001 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-001 |
| SCN-XK-SEC-DECIMAL-002 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-002 |
| SCN-XK-SEC-DECIMAL-003 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-001 |
| SCN-XK-SEC-DECIMAL-004 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-002 |
| SCN-XK-SEC-DECIMAL-005 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-001 |
| SCN-XK-SEC-DECIMAL-006 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-002 |
| SCN-XK-SEC-DECIMAL-007 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-001 |
| SCN-XK-SEC-DECIMAL-008 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-002 |
| SCN-XK-SEC-DECIMAL-009 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-001 |
| SCN-XK-SEC-DECIMAL-010 | sec/decimal_precision.feature | AC-XK-SEC-DECIMAL-002 |
| SCN-XK-SEC-INLINE-001 | sec/inline_restrictions.feature | AC-XK-SEC-INLINE-001 |
| SCN-XK-SEC-INLINE-002 | sec/inline_restrictions.feature | AC-XK-SEC-INLINE-002 |
| SCN-XK-SEC-NEGATIVE-001 | sec/negative_values.feature | AC-XK-SEC-NEGATIVE-001 |
| SCN-XK-SEC-NEGATIVE-002 | sec/negative_values.feature | AC-XK-SEC-NEGATIVE-002 |
| SCN-XK-SEC-NEGATIVE-003 | sec/negative_values.feature | AC-XK-SEC-NEGATIVE-003 |
| SCN-XK-SEC-NEGATIVE-004 | sec/negative_values.feature | AC-XK-SEC-NEGATIVE-004 |
| SCN-XK-SEC-NEGATIVE-005 | sec/negative_values.feature | AC-XK-SEC-NEGATIVE-005 |
| SCN-XK-SEC-REQUIRED-001 | sec/required_facts.feature | AC-XK-SEC-REQUIRED-001 |
| SCN-XK-SEC-REQUIRED-002 | sec/required_facts.feature | AC-XK-SEC-REQUIRED-002 |

### Workflow (5 scenarios)
| Scenario ID | File | AC Tag |
|-------------|------|--------|
| SCN-XK-WORKFLOW-001 | workflow/feature_grid.feature | AC-XK-WORKFLOW-001 |
| SCN-XK-WORKFLOW-002 | workflow/bundle.feature | AC-XK-WORKFLOW-002 |
| SCN-XK-WORKFLOW-003 | workflow/cockpit_pack.feature | AC-XK-WORKFLOW-003 |
| SCN-XK-WORKFLOW-004 | workflow/bundle.feature | AC-XK-WORKFLOW-002 |
| SCN-XK-WORKFLOW-005 | workflow/alpha_check.feature | AC-XK-WORKFLOW-005 |

### Inline XBRL (2 scenarios)
| Scenario ID | File | AC Tag |
|-------------|------|--------|
| SCN-XK-IXDS-001 | inline/ixds_assembly.feature | AC-XK-IXDS-001 |
| SCN-XK-IXDS-002 | inline/ixds_assembly.feature | AC-XK-IXDS-002 |

### Taxonomy (2 scenarios)
| Scenario ID | File | AC Tag |
|-------------|------|--------|
| SCN-XK-TAXONOMY-001 | taxonomy/standard_locations.feature | AC-XK-TAXONOMY-001 |
| SCN-XK-TAXONOMY-002 | sec/taxonomy_years.feature | AC-XK-TAXONOMY-002 |

### Performance/Streaming (4 scenarios)
| Scenario ID | File | AC Tag |
|-------------|------|--------|
| SCN-XK-STREAM-001 | performance/streaming_parser.feature | @streaming @memory |
| SCN-XK-STREAM-002 | performance/streaming_parser.feature | @streaming @fallback |
| SCN-XK-STREAM-003 | performance/streaming_parser.feature | @streaming @context |
| SCN-XK-STREAM-004 | performance/streaming_parser.feature | @streaming @handler |

### Export (1 scenario)
| Scenario ID | File | AC Tag |
|-------------|------|--------|
| SCN-XK-EXPORT-001 | export/oim_json.feature | AC-XK-EXPORT-001 |

### CLI (1 scenario)
| Scenario ID | File | AC Tag |
|-------------|------|--------|
| SCN-XK-CLI-001 | cli/describe_profile.feature | AC-XK-CLI-001 |

### Auto-Fix Suggestions

1. **Batch activate scenarios with existing handlers:**
   - SCN-XK-WORKFLOW-001, SCN-XK-WORKFLOW-002 (handlers exist)
   - SCN-XK-STREAM-* scenarios (streaming infrastructure ready)

2. **Create missing handlers first for:**
   - Decimal precision validation steps
   - SEC negative value validation steps
   - Context completeness validation steps

---

## 2. Placeholder Crates 📦

**Count:** 23 crates with `src/lib.rs` < 20 lines

These crates have minimal implementations (likely stubs/placeholders):

| Lines | Crate | Current Implementation |
|-------|-------|------------------------|
| 6 | calc11 | `calculate_ready() -> false` |
| 6 | render-json | (placeholder) |
| 6 | xbrl-dimensions | `normalize_dimension() -> lowercase` |
| 6 | xbrl-linkbases | `has_linkbase_support() -> false` |
| 6 | xbrl-units | (placeholder) |
| 7 | archive-zip | (placeholder) |
| 8 | corpus-fs | (placeholder) |
| 8 | oracle-compare | (placeholder) |
| 8 | xbrlkit-conform | (placeholder) |
| 8 | xbrlkit-interop-tests | (placeholder) |
| 8 | xbrlkit-test-grid | (placeholder) |
| 9 | sec-http | (placeholder) |
| 9 | taxonomy-cache | (placeholder) |
| 9 | xbrlkit-core | (placeholder) |
| 10 | edgar-identity | (placeholder) |
| 11 | render-md | (placeholder) |
| 11 | taxonomy-package | (placeholder) |
| 13 | export-run | (placeholder) |
| 13 | oim-normalize | (placeholder) |
| 15 | cockpit-export | (placeholder) |
| 17 | edgar-sgml | (placeholder) |
| 17 | taxonomy-types | (placeholder) |
| 19 | filing-load | (placeholder) |

### Auto-Fix Suggestions

- **Low priority:** These are likely intentional stub crates awaiting implementation
- **Action:** Add README.md to each explaining intended purpose
- **Consider:** Removing from workspace if not needed in near-term

---

## 3. Orphaned Scenarios 🔍

**Count:** 0 confirmed orphaned scenarios

All 65 `@SCN-*` tagged scenarios appear in the feature grid. No scenarios were found that completely lack handler infrastructure.

**Note:** While scenarios aren't technically orphaned, many have **incomplete step handler coverage** (see Section 5).

---

## 4. Broken/Missing Plans 📝

**Status:** Plans directory underutilized

| Plan File | Status |
|-----------|--------|
| ISSUE-113.md | ✅ Complete (SCN-XK-WORKFLOW-002 activation plan) |

**Missing Plans For:**
- 40 unactivated scenarios need activation plans
- 23 placeholder crates need implementation plans
- BDD step handler gaps need completion plans

### Auto-Fix Suggestions

Create plan templates for:
```
.mend/plans/SCN-{ID}-activation.md
.mend/plans/crate-{name}-implementation.md
```

---

## 5. Missing Step Handlers ⚠️

**Count:** ~45 Gherkin steps without handler implementations

The `crates/xbrlkit-bdd-steps/src/lib.rs` has 90 conditional handlers, but many Gherkin steps in feature files lack coverage.

### Top Missing Handlers by Frequency

| Step Pattern | Count in Features | Status |
|--------------|-------------------|--------|
| `Given the profile pack "..."` | 13 | ✅ Implemented |
| `When decimal precision validation is performed` | 10 | ⚠️ Partial |
| `When I validate the typed dimension value` | 9 | ✅ Implemented |
| `Then the validation should fail` | 9 | ✅ Implemented |
| `When I validate the filing` | 7 | ✅ Implemented |
| `When I load the taxonomy` | 6 | ✅ Implemented |
| `Then the validation should pass` | 6 | ✅ Implemented |
| `Then no validation errors are reported` | 6 | ✅ Implemented |
| `When the document is validated` | 5 | ⚠️ Missing |
| `When I validate the dimension-member pair` | 5 | ✅ Implemented |
| `And no findings should be reported` | 5 | ✅ Implemented |

### Critical Missing Handlers

These steps appear in feature files but lack proper handlers:

1. **SEC Validation Steps:**
   - `When the document is validated` (5 occurrences)
   - `Then the validation report contains a finding with rule ID containing "NEGATIVE_VALUE"` (3 occurrences)
   - `Then the validation report contains rule "SEC.TAXONOMY.SAME_YEAR"`
   - `Then the validation report contains rule "SEC.REQUIRED_FACT.DEI_ENTITYREGISTRANTNAME"`
   - `Then the validation report contains rule "SEC.INLINE.NO_IX_TUPLE"`
   - `Then the validation report contains rule "SEC.INLINE.NO_IX_FRACTION"`

2. **Streaming Parser Steps:**
   - `When I validate it using the streaming parser`
   - `Then memory usage should stay under 50MB peak`
   - `Then the handler should receive each fact`

3. **Workflow Steps:**
   - `When I bundle the selector "AC-XK-IXDS-002"` (partially implemented)
   - `Then the bundle manifest lists scenario "SCN-XK-IXDS-002"`
   - `When I bundle the selector "AC-XK-DOES-NOT-EXIST"`
   - `Then bundling fails because no scenario matches`
   - `When I package the receipt for cockpit`
   - `Then the sensor report is emitted`

4. **CLI Steps:**
   - `When I run describe-profile --json`
   - `Then the output is valid JSON`

5. **Taxonomy Steps:**
   - `Then the taxonomy resolution succeeds`
   - `Then the taxonomy resolution resolves at least 1 namespaces`

### Auto-Fix Suggestions

1. **High Priority:** Implement missing SEC validation step handlers
2. **Medium Priority:** Complete workflow bundle handlers
3. **Low Priority:** Add streaming parser assertion handlers

---

## Recommendations

### Immediate Actions (This Sprint)

1. **Activate SCN-XK-WORKFLOW-002** - Plan already exists (ISSUE-113.md)
2. **Implement missing SEC validation handlers** - Blocks 17 scenario activations
3. **Add handler for `When the document is validated`** - Used 5 times

### Short-term (Next 2 Sprints)

1. **Batch activate dimension scenarios** - 17 already have handlers
2. **Complete workflow bundle feature** - SCN-XK-WORKFLOW-002, 004
3. **Document placeholder crates** - Add READMEs explaining intent

### Long-term

1. **Create activation plans** for all 40 unactivated scenarios
2. **Implement placeholder crates** or remove from workspace
3. **Achieve 100% step handler coverage** for all Gherkin steps

---

## Appendix: Data Sources

- Feature files: `specs/features/**/*.feature` (18 files)
- Step handlers: `crates/xbrlkit-bdd-steps/src/lib.rs` (90 conditions, 1625 lines)
- Active alpha ACs: `xtask/src/alpha_check.rs` (14 ACs)
- Plans: `.mend/plans/` (1 plan)
- Crates: `crates/*/` (48 total, 23 placeholder)
