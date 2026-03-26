//! Numeric validation rules for XBRL reports.
//!
//! This crate provides validation rules for numeric facts in XBRL reports,
//! including negative value detection where prohibited by taxonomy
//! and decimal precision validation per SEC EFM 6.5.37.

use xbrl_report_types::{Fact, ValidationFinding};

pub mod decimal_precision;

pub use decimal_precision::validate_decimal_precision;

/// Validates that numeric facts don't have negative values where prohibited.
///
/// A negative value is considered prohibited if:
/// 1. The concept name matches known non-negative patterns (Shares, Count, NumberOf)
/// 2. The concept is explicitly listed in the prohibited concepts list
///
/// # Arguments
/// * `facts` - The facts to validate
/// * `prohibited_concepts` - List of concept names that explicitly prohibit negative values
///
/// # Returns
/// Vector of validation findings for negative value errors.
#[must_use]
pub fn validate_negative_values(
    facts: &[Fact],
    prohibited_concepts: &[String],
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();

    for fact in facts {
        // Check if this is a numeric fact with a negative value
        if let Some(is_negative) = is_negative_numeric(&fact.value) {
            if is_negative {
                // Check if this concept prohibits negative values
                if concept_prohibits_negative(&fact.concept, prohibited_concepts) {
                    findings.push(ValidationFinding {
                        rule_id: format!(
                            "SEC.NEGATIVE_VALUE.{}",
                            sanitize_for_rule_id(&fact.concept)
                        ),
                        severity: "error".to_string(),
                        message: format!(
                            "Concept '{}' has negative value '{}' but does not allow negative values",
                            fact.concept, fact.value
                        ),
                        member: Some(fact.member.clone()),
                        subject: Some(fact.concept.clone()),
                    });
                }
            }
        }
    }

    findings
}

/// Checks if a value represents a negative numeric.
///
/// Returns:
/// - `Some(true)` if the value is a negative number
/// - `Some(false)` if the value is a non-negative number
/// - `None` if the value is not numeric
fn is_negative_numeric(value: &str) -> Option<bool> {
    // Trim whitespace
    let trimmed = value.trim();

    // Check for parentheses notation (accounting negative): (123) means -123
    if trimmed.starts_with('(') && trimmed.ends_with(')') {
        return Some(true);
    }

    // Try to parse as float
    match trimmed.parse::<f64>() {
        Ok(num) => Some(num < 0.0),
        Err(_) => None,
    }
}

/// Determines if a concept prohibits negative values.
///
/// A concept prohibits negative values if:
/// 1. It's in the explicit prohibited list
/// 2. Its name matches non-negative patterns (Shares, Count, NumberOf)
fn concept_prohibits_negative(concept: &str, prohibited_concepts: &[String]) -> bool {
    // Check explicit prohibition list
    if prohibited_concepts.iter().any(|c| c == concept) {
        return true;
    }

    // Check concept name patterns that imply non-negative
    let concept_lower = concept.to_lowercase();

    // Shares-related concepts (share counts can't be negative)
    if concept_lower.contains("shares") {
        return true;
    }

    // Count-related concepts
    if concept_lower.contains("count") {
        return true;
    }

    // NumberOf patterns
    if concept_lower.contains("numberof") || concept_lower.contains("number_of") {
        return true;
    }

    // Employee count
    if concept_lower.contains("employees") && concept_lower.contains("number") {
        return true;
    }

    false
}

/// Sanitizes a concept name for use in a rule ID.
fn sanitize_for_rule_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fact(concept: &str, value: &str) -> Fact {
        Fact {
            concept: concept.to_string(),
            value: value.to_string(),
            unit_ref: None,
            context_ref: "ctx-1".to_string(),
            decimals: None,
            member: String::new(),
        }
    }

    #[test]
    fn detects_negative_shares_outstanding() {
        let facts = vec![fact("dei:EntityCommonStockSharesOutstanding", "-1000")];
        let prohibited: Vec<String> = Vec::new();

        let findings = validate_negative_values(&facts, &prohibited);

        assert_eq!(findings.len(), 1);
        assert!(findings[0]
            .rule_id
            .contains("SEC.NEGATIVE_VALUE.DEI_ENTITYCOMMONSTOCKSHARESOUTSTANDING"));
        assert_eq!(findings[0].severity, "error");
    }

    #[test]
    fn detects_negative_with_parentheses() {
        let facts = vec![fact("dei:EntityCommonStockSharesOutstanding", "(1000)")];
        let prohibited: Vec<String> = Vec::new();

        let findings = validate_negative_values(&facts, &prohibited);

        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn allows_positive_shares() {
        let facts = vec![fact("dei:EntityCommonStockSharesOutstanding", "1000000")];
        let prohibited: Vec<String> = Vec::new();

        let findings = validate_negative_values(&facts, &prohibited);

        assert!(findings.is_empty());
    }

    #[test]
    fn detects_negative_employee_count() {
        let facts = vec![fact("dei:EntityNumberOfEmployees", "-50")];
        let prohibited: Vec<String> = Vec::new();

        let findings = validate_negative_values(&facts, &prohibited);

        assert_eq!(findings.len(), 1);
        assert!(findings[0]
            .rule_id
            .contains("SEC.NEGATIVE_VALUE.DEI_ENTITYNUMBEROFEMPLOYEES"));
    }

    #[test]
    fn detects_explicitly_prohibited_concept() {
        let facts = vec![fact("custom:MyShareCount", "-100")];
        let prohibited = vec!["custom:MyShareCount".to_string()];

        let findings = validate_negative_values(&facts, &prohibited);

        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn ignores_non_numeric_values() {
        let facts = vec![fact("dei:EntityRegistrantName", "Example Corp")];
        let prohibited: Vec<String> = Vec::new();

        let findings = validate_negative_values(&facts, &prohibited);

        assert!(findings.is_empty());
    }

    #[test]
    fn ignores_negative_on_unrelated_concepts() {
        // Net income can be negative (loss)
        let facts = vec![fact("us-gaap:NetIncomeLoss", "-5000000")];
        let prohibited: Vec<String> = Vec::new();

        let findings = validate_negative_values(&facts, &prohibited);

        assert!(findings.is_empty());
    }

    #[test]
    fn is_negative_numeric_handles_various_formats() {
        assert_eq!(is_negative_numeric("-100"), Some(true));
        assert_eq!(is_negative_numeric("100"), Some(false));
        assert_eq!(is_negative_numeric("0"), Some(false));
        assert_eq!(is_negative_numeric("(100)"), Some(true));
        assert_eq!(is_negative_numeric("abc"), None);
        assert_eq!(is_negative_numeric("-100.50"), Some(true));
        assert_eq!(is_negative_numeric("100.50"), Some(false));
    }
}
