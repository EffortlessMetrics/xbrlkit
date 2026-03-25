# Decimal Precision Validation - Implementation Plan

**Issue:** #81  
**Acceptance Criteria:** AC-XK-SEC-DECIMAL-001, AC-XK-SEC-DECIMAL-002  
**Estimated Effort:** 2 days  

## Research Summary

From SEC EFM § 6.6.4 and § 6.5.17:
- Numeric facts MUST use `@decimals` (not `@precision`)
- Decimals value indicates accuracy: INF (exact), 0 (units), 2 (cents), -3 (thousands), etc.
- Value must correspond to accuracy in official HTML/ASCII document

## Implementation Approach

### 1. Extend `numeric-rules` Crate

Add to `crates/numeric-rules/src/lib.rs`:

```rust
/// Validates decimal precision on numeric facts.
pub fn validate_decimal_precision(
    facts: &[Fact],
    config: &DecimalPrecisionConfig,
) -> Vec<ValidationFinding>
```

**Validation checks:**
1. **Missing decimals**: Numeric fact without `@decimals` attribute → Error
2. **Precision attribute used**: Fact has `@precision` → Error (SEC requirement)
3. **Excessive precision**: Decimals value > threshold for data type → Warning

### 2. Add Configuration

Extend `sec_profile_types` to include:

```rust
pub struct DecimalPrecisionRules {
    pub max_monetary_decimals: i32,      // e.g., 2 (cents)
    pub max_percentage_decimals: i32,    // e.g., 4 (basis points)
    pub prohibited_concepts: Vec<String>, // Concepts requiring exact precision (INF)
}
```

### 3. Wire into `validation-run`

In `crates/validation-run/src/lib.rs`, add:

```rust
use numeric_rules::validate_decimal_precision;

// After negative value validation
if let Some(rules) = &profile.numeric_rules {
    report.findings.extend(validate_decimal_precision(
        &report.facts,
        &rules.decimal_precision_rules,
    ));
}
```

### 4. BDD Scenarios

Create `specs/features/sec/decimal_precision.feature`:

```gherkin
Feature: Decimal Precision Validation (AC-XK-SEC-DECIMAL-001)

  Scenario: Missing decimals attribute on numeric fact
    Given a report with a numeric fact without decimals attribute
    When decimal precision validation runs
    Then finding SEC.DECIMAL_PRECISION.MISSING is produced

  Scenario: Precision attribute used instead of decimals
    Given a report with a numeric fact using precision attribute
    When decimal precision validation runs
    Then finding SEC.DECIMAL_PRECISION.PRECISION_USED is produced

  Scenario: Valid decimals attribute
    Given a report with numeric facts having valid decimals
    When decimal precision validation runs
    Then no decimal precision findings are produced
```

### 5. Test Fixtures

Create `fixtures/synthetic/sec/decimal-precision/`:
- `valid-decimals.xml` - Numeric facts with correct decimals
- `missing-decimals.xml` - Numeric fact without decimals
- `precision-used.xml` - Numeric fact using precision attribute

## File Changes

| File | Change |
|------|--------|
| `crates/numeric-rules/src/lib.rs` | Add `validate_decimal_precision()` function |
| `crates/sec-profile-types/src/lib.rs` | Add `DecimalPrecisionRules` struct |
| `crates/validation-run/src/lib.rs` | Wire validation into pipeline |
| `specs/features/sec/decimal_precision.feature` | New BDD scenarios |
| `specs/features/sec/decimal_precision.meta.yaml` | Sidecar metadata |
| `fixtures/synthetic/sec/decimal-precision/` | Test fixtures |
| `profiles/sec/efm-77/numeric-rules.yaml` | Add decimal precision rules |

## Acceptance Criteria Mapping

- **AC-XK-SEC-DECIMAL-001**: Detect invalid decimal precision
  - Missing decimals attribute → Error
  - Using precision instead of decimals → Error
- **AC-XK-SEC-DECIMAL-002**: Valid precision values pass
  - All numeric facts with proper decimals → No findings

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Breaking existing tests | Run full test suite before PR |
| Profile config missing | Use defaults if config absent |
| Non-numeric facts | Skip validation for non-numeric types |

## Next Steps

1. Create branch: `mend/issue-81-decimal-precision`
2. Implement `validate_decimal_precision()`
3. Add configuration structs
4. Create BDD scenarios
5. Add test fixtures
6. Wire into validation-run
7. Run alpha-check
8. Create PR
