# Research: Issue #7 - Synthetic Fixture for Inconsistent Duplicates

## Current State Analysis

### Active Alpha ACs (from alpha_check.rs)
- AC-XK-SEC-INLINE-001/002 (ix-fraction, ix-tuple) ✓ fixtures exist
- AC-XK-SEC-REQUIRED-001/002 (DEI facts) ✓ fixtures exist
- AC-XK-TAXONOMY-001/002 (taxonomy resolution) ✓ fixtures exist
- AC-XK-DUPLICATES-001 (consistent duplicates) ✓ fixture exists
- AC-XK-IXDS-001/002 (single/multi-file IXDS) ✓ fixtures exist
- AC-XK-EXPORT-001 (OIM JSON export) ✓ reuses existing fixture

### Gap Identified
The duplicate facts validation path in `crates/validation-run/src/lib.rs` handles two rule IDs:
- `XBRL.DUPLICATE_FACT.CONSISTENT` - tested by AC-XK-DUPLICATES-001
- `XBRL.DUPLICATE_FACT.INCONSISTENT` - NOT tested by any active alpha scenario

### Proposed Solution
Add a synthetic fixture for **inconsistent duplicates** that exercises the `XBRL.DUPLICATE_FACT.INCONSISTENT` rule path.

## Implementation Plan

### Option A: Extend AC-XK-DUPLICATES-001
Add a second scenario to the existing AC that tests inconsistent duplicates.

### Option B: Create AC-XK-DUPLICATES-002 (Recommended)
Create a new focused AC for inconsistent duplicates. This keeps scenarios atomic and follows the existing pattern.

### Changes Required
1. **Create fixture**: `fixtures/synthetic/facts/inconsistent-duplicates-01/report.yaml`
   - Two facts with same concept+context but different values
2. **Add scenario**: Extend `specs/features/foundation/duplicate_facts.feature`
   - New @alpha-active scenario for inconsistent duplicates
   - Add AC-XK-DUPLICATES-002 to ACTIVE_ALPHA_ACS
3. **Update alpha_check.rs**: Add "AC-XK-DUPLICATES-002" to ACTIVE_ALPHA_ACS array

### Fixture Structure (YAML)
```yaml
facts:
  - concept: us-gaap:Assets
    context: c1
    value: "100"
  - concept: us-gaap:Assets
    context: c1
    value: "200"  # Different value = inconsistent
```

### Scenario Definition
```gherkin
@alpha-active
@AC-XK-DUPLICATES-002
@SCN-XK-DUPLICATES-002
@speed.fast
Scenario: Detect and flag inconsistent duplicates
  Given the fixture directory "synthetic/facts/inconsistent-duplicates-01"
  When I validate duplicate facts
  Then the validation report contains rule "XBRL.DUPLICATE_FACT.INCONSISTENT"
```

## Estimation
- Fixture creation: 10 minutes
- Feature update: 10 minutes
- Alpha check update: 5 minutes
- Testing/verification: 15 minutes
- **Total: ~40 minutes**

## Confidence: High (85%)
The path is clear, existing patterns to follow, and the validation logic already exists in the codebase.
