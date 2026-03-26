# Decimal Precision Validation — Implementation Plan

**Issue:** #81  
**Research Date:** 2026-03-26  
**Confidence:** High (85%)

---

## Research Findings

### 1. SEC EFM Requirements

**EFM 6.6.32 / 6.6.4** — Decimals Attribute Selection:
The value of the `decimals` attribute must correspond to the accuracy of the amount as reported in the official HTML/ASCII document.

| Accuracy in Official Document | decimals Value |
|------------------------------|----------------|
| Exact amount | INF |
| Rounded to billions | -9 |
| Rounded to millions | -6 |
| Rounded to thousands | -3 |
| Rounded to units | 0 |
| Rounded to cents (hundredths) | 2 |
| Rounded to whole percentage | 2 |
| Rounded to basis points | 4 |

**EFM 6.5.37** — Nonzero Digits Truncated (Error):
> "A non-nil numeric fact value is not truncated by the decimals attribute."

This is the core validation rule we must implement. Examples of violations:

| Fact Text | decimals | Interpreted Value | Result |
|-----------|----------|-------------------|--------|
| -2345.67 | INF | -2,345.67 | ✓ Valid |
| -2345.67 | 2 | -2,345.67 | ✓ Valid |
| -2345.67 | 0 | -2,345.00 | ✗ Error (truncates .67) |
| -2345.67 | -2 | -2,300.00 | ✗ Error (truncates 45.67) |
| -2345.67 | -3 | -2,000.00 | ✗ Error (truncates 345.67) |

**EFM 6.5.17** — Decimals Not Precision:
SEC requires the `decimals` attribute, not `precision`. Using `precision` is an error.

### 2. XBRL 2.1 Specification

The XBRL 2.1 spec defines how to infer precision from decimals:

```
precision = n + e + d

where:
- n = number of non-zero digits to left of decimal point (or -zeros to right if no left digits)
- e = exponent value (if present)
- d = decimals attribute value
```

Example: `123.4567` with `decimals="2"` → n=3, e=0, d=2 → precision=5

### 3. Key Insight for Implementation

The validation is **not symmetric**:
- A value like `1,000,000` may have decimals > -6 (e.g., -5, -4)
- But a value like `-2345.67` must NOT have decimals that truncate non-zero digits

The check: After applying the decimals rounding, no non-zero digits from the original value should become zero.

---

## Implementation Design

### Location
Extend `crates/numeric-rules/` (shared with negative-values validation)

### New Module
`crates/numeric-rules/src/decimal_precision.rs`

### Core Function
```rust
pub fn validate_decimal_precision(fact: &NumericFact) -> Vec<ValidationError> {
    // Check if decimals attribute truncates non-zero digits
    // Return error if EFM 6.5.37 violated
}
```

### Validation Logic

1. **Parse the numeric value** (handle scientific notation, decimals)
2. **Get decimals attribute** (INF, or integer)
3. **Apply rounding logic** per XBRL spec
4. **Check for truncation** of non-zero digits
5. **Report error** if truncation detected

### Error Code
`fs-0637-Nonzero-Digits-Truncated` (per SEC EDGAR error codes)

---

## BDD Scenarios

File: `specs/features/sec/decimal_precision.feature`

```gherkin
Feature: Decimal Precision Validation (EFM 6.5.37)

  Background:
    Given the system has loaded the SEC validation rules

  @alpha-candidate @ac-xk-sec-decimal-001
  Scenario: Valid exact value with INF decimals
    Given a numeric fact with value "1234.56" and decimals "INF"
    When decimal precision validation is performed
    Then no validation errors are reported

  @alpha-candidate @ac-xk-sec-decimal-001
  Scenario: Valid rounded value with appropriate decimals
    Given a numeric fact with value "1234.00" and decimals "0"
    When decimal precision validation is performed
    Then no validation errors are reported

  @alpha-candidate @ac-xk-sec-decimal-001
  Scenario: Invalid truncation of fractional digits
    Given a numeric fact with value "1234.56" and decimals "0"
    When decimal precision validation is performed
    Then validation error "NonzeroDigitsTruncated" is reported

  @alpha-candidate @ac-xk-sec-decimal-001
  Scenario: Invalid truncation of significant digits
    Given a numeric fact with value "1234" and decimals "-3"
    When decimal precision validation is performed
    Then validation error "NonzeroDigitsTruncated" is reported

  @alpha-candidate @ac-xk-sec-decimal-002
  Scenario: Valid high-magnitude rounding
    Given a numeric fact with value "1000000" and decimals "-5"
    When decimal precision validation is performed
    Then no validation errors are reported
```

---

## Integration Points

### 1. Wire into validation-run
Add to `crates/validation-run/src/pipeline.rs`:
```rust
// After unit consistency validation
.pipeline(numeric_rules::validate_decimal_precision)
```

### 2. Add Alpha Check ACs
Update `xtask/src/alpha_check.rs`:
```rust
pub const ACTIVE_ALPHA_ACS: &[&str] = &[
    // ... existing ACs ...
    "AC-XK-SEC-DECIMAL-001",
    "AC-XK-SEC-DECIMAL-002",
];
```

---

## Test Fixtures

Create in `crates/numeric-rules/tests/fixtures/`:

**valid_precision.xbrl** — Various valid decimals combinations
**invalid_truncation.xbrl** — Cases that violate EFM 6.5.37

---

## Estimates

| Task | Estimate |
|------|----------|
| Core validation logic | 4h |
| Unit tests | 2h |
| BDD scenarios | 2h |
| Integration (validation-run) | 1h |
| Alpha-check wiring | 1h |
| **Total** | **10h (~1.5 days)** |

---

## Open Questions

1. Should we also validate that `precision` attribute is NOT used (EFM 6.5.17)?
2. Do we need to handle scientific notation (e.g., `1.23e5`)?
3. Should we validate specific decimal values per fact type (some facts require INF)?

**Recommendation:** Start with core EFM 6.5.37 validation (nonzero truncation). Add precision-attribute check as separate rule if needed.

---

## Dependencies

- `numeric-rules` crate (exists)
- `xbrl-types` crate for NumericFact types
- No external dependencies

---

*Plan created by: Mend autonomous queue check*  
*Ready for build phase transition*
