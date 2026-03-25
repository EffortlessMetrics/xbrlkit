# Research: SEC Required Facts Validation

**Issue:** #9  
**Started:** 2026-03-25  
**Status:** 🔍 Research Phase

---

## Background

The SEC requires certain Document and Entity Information (DEI) facts to be present in operating company filings. These required facts are defined in:
- `profiles/sec/efm-77/opco/required_facts.yaml`
- EFM 6.5 and 6.6 specifications

## Required Facts (from profile)

### Core Required Facts

1. **`dei:EntityRegistrantName`**
   - Legal name of the registrant
   - Type: String
   - Context: Required for all filings

2. **`dei:EntityCentralIndexKey`**
 - CIK number (10-digit identifier)
   - Type: String (format: "000XXXXXXXX")
   - Context: Required for all filings

3. **`dei:CurrentFiscalYearEndDate`**
   - Fiscal year end date
   - Type: Date
   - Context: Required for all filings

### Form-Specific Requirements

Different SEC forms (10-K, 10-Q, 8-K, etc.) may have additional required facts:

| Form | Additional Required Facts |
|------|--------------------------|
| 10-K | DocumentType, DocumentFiscalYearFocus, DocumentPeriodEndDate |
| 10-Q | DocumentType, DocumentFiscalPeriodFocus, DocumentPeriodEndDate |
| 8-K  | DocumentType, DocumentPeriodEndDate |

## Implementation Notes

### Current State

- Profile data exists in `profiles/sec/efm-77/opco/required_facts.yaml`
- No validation logic currently enforces these requirements
- EFM rules crate exists but doesn't include required facts check

### Proposed Implementation

1. **Load required facts from profile**
   - Parse `required_facts.yaml`
   - Support form-specific requirements

2. **Validate fact presence**
   - Scan XBRL instance for required facts
   - Report missing facts with severity

3. **Validate fact values** (optional for first slice)
   - CIK format validation
   - Date format validation

### Files to Create/Modify

| File | Purpose |
|------|---------|
| `crates/efm-rules/src/required_facts.rs` | New module for required facts validation |
| `crates/efm-rules/src/lib.rs` | Export new module |
| `crates/validation-run/src/lib.rs` | Wire into validation pipeline |
| `specs/features/sec/required_facts.feature` | Gherkin scenarios |
| `specs/features/sec/required_facts.meta.yaml` | Sidecar metadata |
| `fixtures/synthetic/sec/required-facts/` | Test fixtures |

### Acceptance Criteria

- [ ] AC-XK-SEC-REQUIRED-001: Missing required fact detected and reported
- [ ] AC-XK-SEC-REQUIRED-002: All required facts present passes validation

## Research Findings

**Status: ✅ ALREADY IMPLEMENTED**

The required facts validation was already completed:

1. **Rule Implementation:** `crates/efm-rules/src/lib.rs` - `validate_required_facts()` function exists
2. **Wired into Pipeline:** `crates/validation-run/src/lib.rs` - Called in `validate_html_members()`
3. **BDD Scenarios:** `specs/features/sec/required_facts.feature` - Both scenarios marked `@alpha-active`
4. **Test Fixtures:** `fixtures/synthetic/sec/required-facts/` - Both valid and invalid fixtures exist
5. **ACs in Alpha-Check:** `xtask/src/alpha_check.rs` - Both AC-XK-SEC-REQUIRED-001/002 listed

**Conclusion:** No implementation work needed. Issue #9 was already completed but not properly tracked as closed.

## Next Steps

- Close Issue #9 as already completed
- Mark AC-XK-SEC-REQUIRED-001/002 as ✅ Complete in documentation
- Move to next Phase 3 item
