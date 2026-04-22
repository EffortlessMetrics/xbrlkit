//! Decimal precision validation for XBRL numeric facts.
//!
//! Implements SEC EFM 6.5.37: Nonzero digits must not be truncated by the decimals attribute.

use xbrl_report_types::{Fact, ValidationFinding};

/// Validates that numeric facts don't truncate non-zero digits via the decimals attribute.
///
/// Per SEC EFM 6.5.37: "A non-nil numeric fact value is not truncated by the decimals attribute."
///
/// # Examples of violations:
/// - Value `1234.56` with `decimals="0"` truncates `.56` → Error
/// - Value `1234` with `decimals="-3"` truncates `234` → Error
/// - Value `1000000` with `decimals="-6"` is valid (zeros don't count as truncation)
///
/// # Arguments
/// * `facts` - The facts to validate
///
/// # Returns
/// Vector of validation findings for decimal precision errors.
#[must_use]
pub fn validate_decimal_precision(facts: &[Fact]) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();

    for fact in facts {
        if let Some(decimals) = &fact.decimals
            && let Some(error) = check_decimal_truncation(fact, decimals)
        {
            findings.push(error);
        }
    }

    findings
}

/// Checks if a fact's decimals attribute truncates non-zero digits.
///
/// Returns `Some(ValidationFinding)` if truncation is detected, `None` otherwise.
fn check_decimal_truncation(fact: &Fact, decimals: &str) -> Option<ValidationFinding> {
    // Parse the decimals value
    let decimals_val = parse_decimals(decimals)?;

    // Parse the numeric value
    let value = fact.value.trim();
    if value.is_empty() {
        return None;
    }

    // Check for truncation based on decimals value
    if would_truncate_nonzero_digits(value, decimals_val) {
        return Some(
            ValidationFinding::error(
                "fs-0637-Nonzero-Digits-Truncated",
                format!(
                    "Value '{}' has decimals='{}' which truncates non-zero digits (EFM 6.5.37)",
                    fact.value, decimals
                ),
            )
            .for_fact(fact),
        );
    }

    None
}

/// Parses the decimals attribute value.
///
/// Returns:
/// - `Some(i32)` for finite decimal values (e.g., "0" → 0, "-2" → -2)
/// - `None` for INF (infinite precision, always valid)
fn parse_decimals(decimals: &str) -> Option<i32> {
    let trimmed = decimals.trim();

    // INF means infinite precision - always valid
    if trimmed.eq_ignore_ascii_case("INF") {
        return None;
    }

    // Try to parse as integer
    trimmed.parse::<i32>().ok()
}

/// Determines if applying the given decimals value would truncate non-zero digits.
///
/// The key insight from EFM 6.5.37: After rounding to the specified precision,
/// no non-zero digits from the original value should become zero.
///
/// # Algorithm
/// 1. Parse the numeric value into its digits
/// 2. Determine what the rounded value would be with the given decimals
/// 3. Check if any non-zero digits are lost in rounding
fn would_truncate_nonzero_digits(value: &str, decimals: i32) -> bool {
    // Remove any whitespace
    let value = value.trim();

    // Handle scientific notation
    if let Some(exp_pos) = value.to_lowercase().find('e') {
        let (mantissa, exp) = value.split_at(exp_pos);
        let exp_val: i32 = exp[1..].parse().unwrap_or(0);
        // Adjust decimals for scientific notation
        return would_truncate_nonzero_digits(mantissa, decimals - exp_val);
    }

    // Parse the numeric value
    let (integer_part, fractional_part) = match value.find('.') {
        Some(pos) => (&value[..pos], &value[pos + 1..]),
        None => (value, ""),
    };

    // Handle negative values
    let integer_part = integer_part.trim_start_matches('-');

    // Get all integer digits (including leading zeros for position calculation)
    let int_digits: Vec<char> = integer_part.chars().collect();

    // Get fractional digits
    let frac_digits: Vec<char> = fractional_part.chars().collect();

    // Check if decimals would truncate non-zero digits
    if decimals >= 0 {
        // Positive decimals: preserve fractional digits
        // Check if we have more significant fractional digits than allowed
        let sig_frac_digits = frac_digits.iter().rev().skip_while(|&&c| c == '0').count() as i32;

        if sig_frac_digits > decimals {
            // We have more significant fractional digits than decimals allows
            // This means non-zero digits would be truncated
            return true;
        }
    } else {
        // Negative decimals: rounding to left of decimal point
        // e.g., decimals="-2" means round to hundreds (keep only digits at position 2 and above)
        // decimals="-3" means round to thousands (keep only digits at position 3 and above)
        // The number of digits to drop from the right is: -decimals
        let digits_to_drop = (-decimals) as usize;

        // Check if we have enough digits that would be rounded away
        if int_digits.len() > digits_to_drop {
            // Check the digits that would be dropped (the rightmost 'digits_to_drop' digits)
            let start_idx = int_digits.len() - digits_to_drop;
            for item in int_digits.iter().skip(start_idx) {
                if *item != '0' {
                    // A non-zero digit would be truncated
                    return true;
                }
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fact_with_decimals(concept: &str, value: &str, decimals: &str) -> Fact {
        Fact {
            concept: concept.to_string(),
            value: value.to_string(),
            unit_ref: None,
            context_ref: "ctx-1".to_string(),
            decimals: Some(decimals.to_string()),
            member: String::new(),
        }
    }

    #[test]
    fn valid_exact_value_with_inf() {
        let facts = vec![fact_with_decimals("us-gaap:Revenue", "1234.56", "INF")];
        let findings = validate_decimal_precision(&facts);
        assert!(findings.is_empty());
    }

    #[test]
    fn valid_rounded_value_with_appropriate_decimals() {
        // Value is exact to units, decimals="0" is valid
        let facts = vec![fact_with_decimals("us-gaap:Revenue", "1234.00", "0")];
        let findings = validate_decimal_precision(&facts);
        assert!(findings.is_empty());
    }

    #[test]
    fn invalid_truncation_of_fractional_digits() {
        // Value has .56 but decimals="0" would truncate it
        let facts = vec![fact_with_decimals("us-gaap:Revenue", "1234.56", "0")];
        let findings = validate_decimal_precision(&facts);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "fs-0637-Nonzero-Digits-Truncated");
    }

    #[test]
    fn invalid_truncation_of_significant_digits() {
        // Value is 1234, decimals="-3" would round to thousands, truncating 234
        let facts = vec![fact_with_decimals("us-gaap:Revenue", "1234", "-3")];
        let findings = validate_decimal_precision(&facts);
        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn valid_high_magnitude_rounding() {
        // Value is exactly 1000000, decimals="-5" is valid (no truncation of non-zero digits)
        let facts = vec![fact_with_decimals("us-gaap:Revenue", "1000000", "-5")];
        let findings = validate_decimal_precision(&facts);
        assert!(findings.is_empty());
    }

    #[test]
    fn valid_two_decimal_places() {
        let facts = vec![fact_with_decimals("us-gaap:EarningsPerShare", "1.23", "2")];
        let findings = validate_decimal_precision(&facts);
        assert!(findings.is_empty());
    }

    #[test]
    fn invalid_three_fractional_digits_with_decimals_2() {
        let facts = vec![fact_with_decimals("us-gaap:EarningsPerShare", "1.234", "2")];
        let findings = validate_decimal_precision(&facts);
        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn valid_negative_value_with_inf() {
        let facts = vec![fact_with_decimals(
            "us-gaap:NetIncomeLoss",
            "-2345.67",
            "INF",
        )];
        let findings = validate_decimal_precision(&facts);
        assert!(findings.is_empty());
    }

    #[test]
    fn invalid_negative_value_truncation() {
        let facts = vec![fact_with_decimals("us-gaap:NetIncomeLoss", "-2345.67", "0")];
        let findings = validate_decimal_precision(&facts);
        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn ignores_non_numeric_values() {
        let facts = vec![fact_with_decimals(
            "dei:EntityRegistrantName",
            "Example Corp",
            "2",
        )];
        let findings = validate_decimal_precision(&facts);
        assert!(findings.is_empty());
    }

    #[test]
    fn ignores_missing_decimals() {
        let facts = vec![Fact {
            concept: "us-gaap:Revenue".to_string(),
            value: "1234.56".to_string(),
            unit_ref: None,
            context_ref: "ctx-1".to_string(),
            decimals: None,
            member: String::new(),
        }];
        let findings = validate_decimal_precision(&facts);
        assert!(findings.is_empty());
    }

    #[test]
    fn valid_thousands_rounding() {
        // 1234000 with decimals="-3" is valid (rounds to thousands, 1234 is preserved)
        let facts = vec![fact_with_decimals("us-gaap:Revenue", "1234000", "-3")];
        let findings = validate_decimal_precision(&facts);
        assert!(findings.is_empty());
    }
}
