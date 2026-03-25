//! Unit validation logic

use crate::{
    patterns::{ConceptUnitPatterns, ExpectedUnitType},
    unit_matches_type,
};
use sec_profile_types::UnitRules;
use xbrl_report_types::{Fact, ValidationFinding};

/// Validates unit consistency for XBRL facts
pub struct UnitValidator {
    patterns: ConceptUnitPatterns,
}

impl UnitValidator {
    /// Create a new validator with default patterns
    pub fn new() -> Self {
        Self {
            patterns: ConceptUnitPatterns::new(),
        }
    }

    /// Create a validator with custom rules from profile
    pub fn with_rules(rules: &UnitRules) -> Self {
        let mut patterns = ConceptUnitPatterns::new();

        // Add explicit mappings from profile rules
        for concept in &rules.monetary_concepts {
            patterns.add_explicit(concept.clone(), ExpectedUnitType::Monetary);
        }
        for concept in &rules.share_concepts {
            patterns.add_explicit(concept.clone(), ExpectedUnitType::Shares);
        }
        for concept in &rules.pure_concepts {
            patterns.add_explicit(concept.clone(), ExpectedUnitType::Pure);
        }
        for concept in &rules.per_share_concepts {
            patterns.add_explicit(concept.clone(), ExpectedUnitType::PerShare);
        }

        Self { patterns }
    }

    /// Validate a single fact's unit consistency
    pub fn validate_fact(
        &self,
        fact: &Fact,
        units: &[(String, String)], // (unit_id, unit_measure)
    ) -> Option<ValidationFinding> {
        // Determine expected unit type
        let expected = if let Some(unit_type) = self.patterns.expected_type(&fact.concept) {
            unit_type
        } else if self.patterns.is_likely_monetary(&fact.concept) {
            ExpectedUnitType::Monetary
        } else {
            // No expected type determined - skip validation
            return None;
        };

        // Get the fact's unit reference
        let unit_ref = fact.unit_ref.as_ref()?;

        // Find the fact's unit measure
        let unit_measure = units
            .iter()
            .find(|(id, _)| id == unit_ref)
            .map(|(_, measure)| measure.as_str())?;

        // Check if unit matches expected type
        let valid = unit_matches_type(unit_measure, &expected);

        if valid {
            None
        } else {
            Some(ValidationFinding {
                rule_id: "SEC-UNIT-001".to_string(),
                severity: "error".to_string(),
                message: format!(
                    "Unit inconsistency: concept '{}' expects {:?} unit but found '{}'",
                    fact.concept, expected, unit_measure
                ),
                member: Some(fact.member.clone()),
                subject: Some(fact.concept.clone()),
            })
        }
    }

    /// Validate all facts in a report
    pub fn validate_facts(
        &self,
        facts: &[Fact],
        units: &[(String, String)],
    ) -> Vec<ValidationFinding> {
        let mut findings = Vec::new();

        for fact in facts {
            if let Some(finding) = self.validate_fact(fact, units) {
                findings.push(finding);
            }
        }

        findings
    }
}

impl Default for UnitValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience function for validation-run integration
pub fn validate_unit_consistency(
    facts: &[Fact],
    units: &[(String, String)],
    unit_rules: Option<&UnitRules>,
) -> Vec<ValidationFinding> {
    let validator = match unit_rules {
        Some(rules) => UnitValidator::with_rules(rules),
        None => UnitValidator::new(),
    };

    validator.validate_facts(facts, units)
}

#[cfg(test)]
mod tests {
    use super::*;
    use xbrl_report_types::Fact;

    fn create_test_fact(concept: &str, unit_ref: Option<&str>) -> Fact {
        Fact {
            concept: concept.to_string(),
            context_ref: "ctx-1".to_string(),
            unit_ref: unit_ref.map(|s| s.to_string()),
            decimals: Some("2".to_string()),
            value: "1000".to_string(),
            member: String::new(),
        }
    }

    #[test]
    fn test_monetary_with_currency() {
        let validator = UnitValidator::new();
        let fact = create_test_fact("us-gaap:Revenue", Some("u-1"));
        let units = vec![("u-1".to_string(), "iso4217:USD".to_string())];

        let result = validator.validate_fact(&fact, &units);
        assert!(result.is_none()); // No finding = valid
    }

    #[test]
    fn test_monetary_with_wrong_unit() {
        let validator = UnitValidator::new();
        let fact = create_test_fact("us-gaap:Revenue", Some("u-1"));
        let units = vec![("u-1".to_string(), "xbrli:shares".to_string())];

        let result = validator.validate_fact(&fact, &units);
        assert!(result.is_some()); // Finding = invalid
    }

    #[test]
    fn test_shares_with_correct_unit() {
        let validator = UnitValidator::new();
        let fact = create_test_fact("us-gaap:CommonStockSharesOutstanding", Some("u-1"));
        let units = vec![("u-1".to_string(), "xbrli:shares".to_string())];

        let result = validator.validate_fact(&fact, &units);
        assert!(result.is_none()); // No finding = valid
    }

    #[test]
    fn test_shares_with_wrong_unit() {
        let validator = UnitValidator::new();
        let fact = create_test_fact("us-gaap:CommonStockSharesOutstanding", Some("u-1"));
        let units = vec![("u-1".to_string(), "iso4217:USD".to_string())];

        let result = validator.validate_fact(&fact, &units);
        assert!(result.is_some()); // Finding = invalid
    }

    #[test]
    fn test_no_unit_ref() {
        let validator = UnitValidator::new();
        let fact = create_test_fact("us-gaap:Revenue", None);
        let units = vec![];

        let result = validator.validate_fact(&fact, &units);
        assert!(result.is_none()); // No unit = skip validation
    }
}
