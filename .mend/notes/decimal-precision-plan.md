# Decimal Precision Validation — Implementation Plan

**Issue:** #81  
**Status:** 📐 Plan  
**Research:** `.mend/notes/decimal-precision-research.md`  
**Est. Effort:** 2-3 days

---

## Overview

Implement SEC EFM § 6.5.37 validation: detect when a numeric fact's `decimals` attribute would cause non-zero digits to be truncated.

---

## Architecture

### Module Structure
```
crates/numeric-rules/src/
├── lib.rs                      # Re-export modules
├── negative_values.rs          # Existing
└── decimal_precision.rs        # NEW
```

### Core Types

```rust
/// Validates that numeric facts don't have their decimals attribute
/// truncating non-zero digits.
pub fn validate_decimal_precision(fact: &Fact) -> Option<ValidationFinding>;

/// Internal: applies truncation logic and detects nonzero truncation.
fn truncate_to_precision(value: &BigDecimal, precision: i32) -> BigDecimal;
fn has_nonzero_digits_truncated(original: &BigDecimal, truncated: &BigDecimal, precision: i32) -> bool;
```

---

## Implementation Steps

### Step 1: Add Dependencies (15 min)

Add to `crates/numeric-rules/Cargo.toml`:
```toml
[dependencies]
bigdecimal = "0.4"
```

### Step 2: Core Module (2 hours)

Create `crates/numeric-rules/src/decimal_precision.rs`:

1. **Parse decimals attribute:**
   - Handle "INF" → exact, always valid
   - Handle integers → precision value
   - Return None for invalid/parse errors

2. **Truncate logic:**
   - Positive precision: decimal places (2 = hundredths)
   - Negative precision: powers of 10 (-2 = hundreds)
   - Use `BigDecimal` to avoid floating-point errors

3. **Nonzero truncation detection:**
   - Compare original vs truncated
   - If any non-zero digits were zeroed out → error

### Step 3: Integration (30 min)

Update `crates/numeric-rules/src/lib.rs`:
```rust
pub mod decimal_precision;
pub use decimal_precision::validate_decimal_precision;
```

Wire into `validation-run` pipeline (if not already generic).

### Step 4: Unit Tests (1 hour)

Cover all EFM § 6.5.37 examples:

| Test Case | Value | Decimals | Expected |
|-----------|-------|----------|----------|
| exact_inf | -2345.67 | INF | valid |
| exact_two | -2345.67 | 2 | valid |
| truncate_zero | -2345.67 | 0 | error |
| truncate_neg2 | -2345.67 | -2 | error |
| truncate_neg3 | -2345.67 | -3 | error |
| truncate_neg6 | -2345.67 | -6 | error |
| valid_rounding | 1000000 | -5 | valid (zeros) |
| zero_value | 0 | any | valid |

### Step 5: BDD Scenarios (1 hour)

Create `bdd/features/decimal_precision.feature`:

```gherkin
Feature: Decimal Precision Validation (EFM § 6.5.37)
  As a compliance officer
  I want to detect when decimals attributes truncate nonzero digits
  So that filings meet SEC EDGAR requirements

  Background:
    Given the validation engine is initialized

  Scenario: Exact value with INF decimals is valid
    Given a numeric fact with value "-2345.67" and decimals "INF"
    When decimal precision validation runs
    Then no findings are reported

  Scenario: Rounded value within precision is valid
    Given a numeric fact with value "-2345.67" and decimals "2"
    When decimal precision validation runs
    Then no findings are reported

  Scenario Outline: Truncation of nonzero digits is an error
    Given a numeric fact with value "<value>" and decimals "<decimals>"
    When decimal precision validation runs
    Then a "SEC.DECIMAL_PRECISION.TRUNCATED" error is reported
    
    Examples:
      | value      | decimals |
      | -2345.67   | 0        |
      | -2345.67   | -2       |
      | -2345.67   | -3       |
      | -2345.67   | -6       |

  Scenario: Zero digits in truncated positions is valid
    Given a numeric fact with value "1000000" and decimals "-5"
    When decimal precision validation runs
    Then no findings are reported
```

### Step 6: Alpha-Check Scenario (30 min)

Create `scenarios/SCN-XK-SEC-DECIMAL-001.feature`:

```gherkin
@alpha-active
Feature: SCN-XK-SEC-DECIMAL-001 — Decimal Precision Validation

  Background:
    Given the SEC validation rules are loaded

  Scenario: AC-XK-SEC-DECIMAL-001 — Detect truncated nonzero digits
    Given a filing with a numeric fact
    And the fact value is "1234.56"
    And the decimals attribute is "0"
    When the validation run executes
    Then finding "SEC.DECIMAL_PRECISION.TRUNCATED" is reported

  Scenario: AC-XK-SEC-DECIMAL-002 — INF decimals always valid
    Given a filing with a numeric fact
    And the decimals attribute is "INF"
    When the validation run executes
    Then no decimal precision findings are reported

  Scenario: AC-XK-SEC-DECIMAL-003 — Valid rounding passes
    Given a filing with a numeric fact
    And the fact value is "1000000"
    And the decimals attribute is "-5"
    When the validation run executes
    Then no decimal precision findings are reported
```

Add to `xtask/src/alpha_check.rs`:
```rust
pub const ACTIVE_ALPHA_ACS: &[&str] = &[
    // ... existing ACs ...
    "AC-XK-SEC-DECIMAL-001",
    "AC-XK-SEC-DECIMAL-002",
    "AC-XK-SEC-DECIMAL-003",
];
```

### Step 7: Documentation (30 min)

Add to `crates/numeric-rules/README.md`:
```markdown
## Decimal Precision Validation

Validates that numeric facts don't have non-zero digits truncated by their `decimals` attribute.

**Rule:** SEC EFM § 6.5.37  
**Severity:** Error

### Examples

| Value | Decimals | Valid? | Reason |
|-------|----------|--------|--------|
| -2345.67 | INF | ✅ Yes | Exact value |
| -2345.67 | 2 | ✅ Yes | Rounded to hundredths |
| -2345.67 | 0 | ❌ No | 0.67 truncated |
| 1000000 | -5 | ✅ Yes | Zeros in truncated positions |
```

---

## Quality Gates

Before PR:
- [ ] `cargo fmt --all --check` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo xtask alpha-check` passes

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| BigDecimal precision edge cases | Low | Medium | Extensive unit tests, use established crate |
| Scientific notation handling | Medium | Low | Normalize before truncation check |
| Performance on large filings | Low | Low | O(n) per numeric fact, minimal overhead |

---

## Dependencies

| Dependency | Version | Purpose |
|------------|---------|---------|
| bigdecimal | 0.4 | Arbitrary precision decimal arithmetic |

---

## Next Steps

1. Create branch: `mend/feat-decimal-precision-validation`
2. Implement core module
3. Add tests and BDD scenarios
4. Run quality gates
5. Open PR

---

## References

- Research: `.mend/notes/decimal-precision-research.md`
- EFM Spec: § 6.5.37, § 6.6.4
- Related: #80 (Negative Value Validation — same crate)
