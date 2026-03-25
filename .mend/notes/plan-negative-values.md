# Plan: Negative Value Validation Implementation

**Issue:** #80  
**Status:** 📐 Planning Phase  
**Research:** `.mend/notes/research-negative-values.md`

---

## Overview

Implement validation to detect negative values where prohibited by taxonomy concepts.

## Architecture

### New Crate: `crates/numeric-rules/`

```
crates/numeric-rules/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Public API
│   ├── negative_values.rs  # Negative value validation
│   └── types.rs        # Shared types
└── tests/
    └── negative_values_tests.rs
```

### Dependencies

```toml
[dependencies]
xbrl-report-types = { path = "../xbrl-report-types" }
sec-profile-types = { path = "../sec-profile-types" }
taxonomy-loader = { path = "../taxonomy-loader" }
```

## Public API

```rust
/// Validates that numeric facts don't have negative values where prohibited.
#[must_use]
pub fn validate_negative_values(
    facts: &[Fact],
    taxonomy: &TaxonomyLoader,
    profile: &ProfilePack,
) -> Vec<ValidationFinding>;

/// Determines if a concept allows negative values based on taxonomy metadata.
fn concept_allows_negative(
    concept: &str,
    taxonomy: &TaxonomyLoader,
) -> bool;

/// Checks if concept is in the profile's explicit prohibition list.
fn is_explicitly_prohibited(
    concept: &str,
    profile: &ProfilePack,
) -> bool;
```

## Rule Logic

### Finding Generation

```rust
ValidationFinding {
    rule_id: "SEC.NEGATIVE_VALUE.{concept}",
    severity: "error",
    message: "Concept '{concept}' with value {value} is negative but does not allow negative values",
    member: Some(fact.member.clone()),
    subject: Some(fact.concept.clone()),
}
```

### Detection Criteria

A negative value is **prohibited** if:
1. Concept type is `xbrli:nonNegativeInteger` or `xbrli:nonPositiveInteger` (wrong sign)
2. Profile explicitly lists concept in `prohibited_negative_concepts`
3. Concept name matches patterns:
   - `*Shares*` (share counts)
   - `*Count*` (entity counts)
   - `*NumberOf*` (number of items)

## Profile Pack Extension

```yaml
# profiles/sec/efm-77/opco/numeric_rules.yaml
negative_value_rules:
  prohibited_concepts:
    - dei:EntityCommonStockSharesOutstanding
    - dei:EntityNumberOfEmployees
    - dei:CommonStockSharesAuthorized
    - dei:CommonStockSharesIssued
    - dei:CommonStockSharesOutstanding
```

## BDD Scenarios

### Feature File: `specs/features/sec/negative_values.feature`

```gherkin
@REQ-XK-SEC-NEGATIVE
@layer.sec
@suite.synthetic
Feature: SEC negative value validation

  @alpha-candidate
  @AC-XK-SEC-NEGATIVE-001
  @SCN-XK-SEC-NEGATIVE-001
  @speed.fast
  Scenario: Negative share count detected as error
    Given the profile pack "sec/efm-77/opco"
    And the filing contains a fact "dei:EntityCommonStockSharesOutstanding" with value "-1000"
    When I validate the filing
    Then the validation report contains rule "SEC.NEGATIVE_VALUE.DEI_ENTITYCOMMONSTOCKSHARESOUTSTANDING"
    And the finding severity is "error"

  @alpha-candidate
  @AC-XK-SEC-NEGATIVE-002
  @SCN-XK-SEC-NEGATIVE-002
  @speed.fast
  Scenario: Valid non-negative share count passes validation
    Given the profile pack "sec/efm-77/opco"
    And the filing contains a fact "dei:EntityCommonStockSharesOutstanding" with value "1000000"
    When I validate the filing
    Then the validation report has no error findings

  @alpha-candidate
  @AC-XK-SEC-NEGATIVE-003
  @SCN-XK-SEC-NEGATIVE-003
  @speed.fast
  Scenario: Negative employee count detected as error
    Given the profile pack "sec/efm-77/opco"
    And the filing contains a fact "dei:EntityNumberOfEmployees" with value "-50"
    When I validate the filing
    Then the validation report contains rule "SEC.NEGATIVE_VALUE.DEI_ENTITYNUMBEROFEMPLOYEES"
```

## Test Fixtures

```
fixtures/synthetic/sec/negative-values/
├── invalid-shares-negative/
│   └── member-a.html      # Contains negative shares outstanding
├── invalid-employees-negative/
│   └── member-a.html      # Contains negative employee count
└── valid-all-nonnegative/
    └── member-a.html      # All values properly non-negative
```

## Integration Points

### Wiring into validation-run

```rust
// In crates/validation-run/src/lib.rs
use numeric_rules::validate_negative_values;

pub fn validate_html_members(members: &[(&str, &str)], profile: &ProfilePack) -> ValidationRun {
    // ... existing validation ...
    
    // Add negative value validation
    let taxonomy = load_taxonomy_for_profile(profile);
    report.findings.extend(validate_negative_values(&report.facts, &taxonomy, profile));
    
    finalize_validation(report, subject)
}
```

## Implementation Steps

### Step 1: Create Crate Structure
- [ ] Create `crates/numeric-rules/` directory
- [ ] Create `Cargo.toml` with dependencies
- [ ] Create `src/lib.rs` with module structure

### Step 2: Implement Core Logic
- [ ] Implement `validate_negative_values()`
- [ ] Implement `concept_allows_negative()`
- [ ] Implement `is_explicitly_prohibited()`

### Step 3: Add Profile Support
- [ ] Add `numeric_rules.yaml` to profile pack
- [ ] Extend `ProfilePack` struct to include numeric rules

### Step 4: Wire into Pipeline
- [ ] Import in `validation-run`
- [ ] Call validation function

### Step 5: BDD Scenarios
- [ ] Create feature file
- [ ] Create sidecar metadata
- [ ] Create test fixtures

### Step 6: Tests and Documentation
- [ ] Unit tests for rule logic
- [ ] Integration tests
- [ ] Update documentation

## Acceptance Criteria

- [ ] AC-XK-SEC-NEGATIVE-001: Negative prohibited value detected
- [ ] AC-XK-SEC-NEGATIVE-002: Valid non-negative values pass
- [ ] All unit tests pass
- [ ] Alpha-check passes with new scenarios

## Risks and Mitigations

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Taxonomy lookup performance | Medium | Medium | Cache concept metadata |
| False positives | Low | High | Profile-based allowlist |
| Edge cases in concept patterns | Medium | Low | Extensive test fixtures |

## Est. Effort

3-4 days
