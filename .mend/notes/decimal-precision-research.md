# Decimal Precision Validation Research — Issue #81

**Date:** 2026-03-25  
**Researcher:** Mend  
**Status:** Complete → Ready for Planning

---

## EFM Specification Reference

**Rule:** EFM § 6.5.37 — "Nonzero Digits Truncated"  
**Severity:** Error (E)

> A non-nil numeric fact value is not truncated by the decimals attribute.

---

## How Decimals Attribute Works

The `decimals` attribute on an XBRL numeric fact indicates the precision of the reported value:

| Decimals Value | Meaning | Example Value | Interpreted As |
|---------------|---------|---------------|----------------|
| `INF` | Exact value | 1234.56 | 1234.56 |
| `2` | Rounded to hundredths | 1234.56 | 1234.56 |
| `0` | Rounded to units | 1234 | 1234.00 |
| `-2` | Rounded to hundreds | 1200 | 1200.00 |
| `-3` | Rounded to thousands | 1000 | 1000.00 |

**Key Insight:** Any value other than `INF` implies rounding. The decimals attribute determines which digits are considered "significant" vs "rounded away."

---

## Validation Rule

**Error Condition:** A fact's value contains non-zero digits in positions that would be truncated (rounded to zero) based on its `decimals` attribute.

### Correct Usage Examples

| Fact Text | Decimals | Interpreted Value | Valid? |
|-----------|----------|-------------------|--------|
| -2345.67 | INF | -2345.67 | ✅ Yes |
| -2345.67 | 2 | -2345.67 | ✅ Yes |
| 1000000 | -5 | 1000000 | ✅ Yes (zeros in truncated positions) |

### Error Examples (EFM § 6.5.37)

| Fact Text | Decimals | Interpreted Value | Valid? |
|-----------|----------|-------------------|--------|
| -2345.67 | 0 | -2345.00 | ❌ No (0.67 truncated) |
| -2345.67 | -2 | -2300.00 | ❌ No (45.67 truncated) |
| -2345.67 | -3 | -2000.00 | ❌ No (345.67 truncated) |
| -2345.67 | -6 | 0000.00 | ❌ No (all digits truncated) |

---

## Implementation Approach

### Algorithm

1. Parse the numeric fact value as a decimal
2. Apply the decimals truncation logic
3. Compare original significant digits with truncated representation
4. If any non-zero digits are lost → validation error

### Pseudocode

```rust
fn validate_decimal_precision(fact: &Fact) -> Option<ValidationFinding> {
    let decimals = fact.decimals?;
    if decimals == "INF" {
        return None; // Exact values are always valid
    }
    
    let precision: i32 = decimals.parse().ok()?;
    let value = parse_decimal(&fact.value)?;
    
    // Calculate what value would be after truncation
    let truncated = truncate_to_precision(value, precision);
    
    // Check if non-zero digits were lost
    if has_nonzero_digits_truncated(value, truncated, precision) {
        return Some(ValidationFinding {
            rule_id: "SEC.DECIMAL_PRECISION.TRUNCATED".to_string(),
            severity: "error".to_string(),
            message: format!(
                "Value '{}' has non-zero digits truncated by decimals='{}'",
                fact.value, decimals
            ),
            // ...
        });
    }
    
    None
}
```

### Edge Cases

1. **Zero values:** `0` with any decimals should be valid
2. **Scientific notation:** XBRL allows `1.23E+4` format — need to normalize
3. **Negative zero:** `-0` should be treated as `0`
4. **Very large/small numbers:** Handle without floating-point precision loss

---

## Files to Create/Modify

### New Files
- `crates/numeric-rules/src/decimal_precision.rs` — Core validation logic
- `bdd/features/decimal_precision.feature` — BDD scenarios
- `scenarios/SCN-XK-SEC-DECIMAL-001.feature` — Alpha-check scenario

### Modified Files
- `crates/numeric-rules/src/lib.rs` — Add module export
- `crates/numeric-rules/Cargo.toml` — Add dependencies if needed
- `xtask/src/alpha_check.rs` — Add AC-XK-SEC-DECIMAL-001

---

## Related Rules

| Rule | Description | Relation |
|------|-------------|----------|
| EFM § 6.5.17 | Decimals Not Precision | Use `decimals`, not deprecated `precision` attr |
| EFM § 6.6.4 | Selecting decimals attribute | Guidance on choosing correct decimals value |
| EFM § 6.5.37 | Nonzero Digits Truncated | **This rule** — validation logic |

---

## Acceptance Criteria

- AC-XK-SEC-DECIMAL-001: Detect non-zero digits truncated by decimals attribute
- AC-XK-SEC-DECIMAL-002: Allow INF decimals (exact values)
- AC-XK-SEC-DECIMAL-003: Allow valid rounding (zero digits in truncated positions)

---

## Research Sources

1. [SEC EDGAR XBRL Guide (2024-07-08)](https://www.sec.gov/files/edgar/filer-information/specifications/xbrl-guide-2024-07-08.pdf) — Section 8.5 "Decimals"
2. [Certent XBRL Documentation](https://dm.certent.com/help/Content/I_XBRL/numeric_units/precision.htm) — EFM § 6.6.32 reference

---

## Next Steps

1. Create implementation plan in `.mend/notes/decimal-precision-plan.md`
2. Implement `decimal_precision` module in `numeric-rules` crate
3. Add BDD scenarios
4. Wire into validation-run pipeline
5. Add alpha-check acceptance criteria
