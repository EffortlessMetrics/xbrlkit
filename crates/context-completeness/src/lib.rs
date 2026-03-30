//! Context Completeness Validation
//!
//! Validates that all XBRL facts reference valid, defined contexts.
//! A fact's context_ref must resolve to an existing context in the report.

use xbrl_contexts::ContextSet;
use xbrl_report_types::{Fact, ValidationFinding};

/// Validates that all facts reference existing contexts.
///
/// For each fact, checks if the `context_ref` exists in the provided `ContextSet`.
/// Context IDs are matched case-insensitively per XBRL 2.1 specification.
///
/// # Arguments
/// * `facts` - The facts to validate
/// * `contexts` - The set of valid contexts in the report
///
/// # Returns
/// Vector of validation findings for facts with missing context references.
#[must_use]
pub fn validate_context_completeness(
    facts: &[Fact],
    contexts: &ContextSet,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();

    for fact in facts {
        let context_ref = &fact.context_ref;

        // Check if context exists (case-insensitive lookup)
        if contexts.get(context_ref).is_none() {
            findings.push(ValidationFinding {
                rule_id: "SEC-CONTEXT-001".to_string(),
                severity: "error".to_string(),
                message: format!("Fact references undefined context: '{}'", context_ref),
                member: Some(fact.member.clone()),
                subject: Some(fact.concept.clone()),
            });
        }
    }

    findings
}

/// Check if a specific context reference is valid.
///
/// # Arguments
/// * `context_ref` - The context ID to check
/// * `contexts` - The set of valid contexts
///
/// # Returns
/// `true` if the context exists, `false` otherwise.
#[must_use]
pub fn is_valid_context_ref(context_ref: &str, contexts: &ContextSet) -> bool {
    contexts.get(context_ref).is_some()
}

/// Count facts with missing context references.
///
/// # Arguments
/// * `facts` - The facts to check
/// * `contexts` - The set of valid contexts
///
/// # Returns
/// The number of facts referencing undefined contexts.
#[must_use]
pub fn count_missing_contexts(facts: &[Fact], contexts: &ContextSet) -> usize {
    facts
        .iter()
        .filter(|f| contexts.get(&f.context_ref).is_none())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use xbrl_contexts::{Context, EntityIdentifier, Period, normalize_context_id};

    fn create_test_fact(concept: &str, context_ref: &str) -> Fact {
        Fact {
            concept: concept.to_string(),
            context_ref: context_ref.to_string(),
            unit_ref: None,
            decimals: None,
            value: "1000".to_string(),
            member: String::new(),
        }
    }

    fn create_test_context(id: &str) -> Context {
        Context {
            id: normalize_context_id(id), // Normalize the ID for consistent lookup
            entity: EntityIdentifier {
                scheme: "http://www.sec.gov/CIK".to_string(),
                value: "0000320193".to_string(),
            },
            entity_segment: None,
            period: Period::Instant("2024-12-31".to_string()),
            scenario: None,
        }
    }

    #[test]
    fn test_valid_context_reference() {
        let mut contexts = ContextSet::new();
        contexts.insert(create_test_context("ctx-1"));

        let facts = vec![create_test_fact("us-gaap:Revenue", "ctx-1")];
        let findings = validate_context_completeness(&facts, &contexts);

        assert!(findings.is_empty());
    }

    #[test]
    fn test_missing_context_reference() {
        let contexts = ContextSet::new();
        let facts = vec![create_test_fact("us-gaap:Revenue", "ctx-missing")];
        let findings = validate_context_completeness(&facts, &contexts);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].rule_id, "SEC-CONTEXT-001");
        assert!(findings[0].message.contains("ctx-missing"));
    }

    #[test]
    fn test_case_insensitive_matching() {
        let mut contexts = ContextSet::new();
        contexts.insert(create_test_context("CTX-1")); // uppercase (normalized to lowercase)

        let facts = vec![create_test_fact("us-gaap:Revenue", "ctx-1")]; // lowercase
        let findings = validate_context_completeness(&facts, &contexts);

        assert!(findings.is_empty());
    }

    #[test]
    fn test_multiple_missing_contexts() {
        let mut contexts = ContextSet::new();
        contexts.insert(create_test_context("ctx-1"));

        let facts = vec![
            create_test_fact("us-gaap:Revenue", "ctx-missing-1"),
            create_test_fact("us-gaap:Assets", "ctx-1"), // valid
            create_test_fact("us-gaap:Liabilities", "ctx-missing-2"),
        ];
        let findings = validate_context_completeness(&facts, &contexts);

        assert_eq!(findings.len(), 2);
    }

    #[test]
    fn test_empty_facts() {
        let mut contexts = ContextSet::new();
        contexts.insert(create_test_context("ctx-1"));

        let facts: Vec<Fact> = vec![];
        let findings = validate_context_completeness(&facts, &contexts);

        assert!(findings.is_empty());
    }

    #[test]
    fn test_empty_contexts() {
        let contexts = ContextSet::new();
        let facts = vec![create_test_fact("us-gaap:Revenue", "ctx-1")];
        let findings = validate_context_completeness(&facts, &contexts);

        assert_eq!(findings.len(), 1);
    }

    #[test]
    fn test_is_valid_context_ref() {
        let mut contexts = ContextSet::new();
        contexts.insert(create_test_context("ctx-1"));

        assert!(is_valid_context_ref("ctx-1", &contexts));
        assert!(is_valid_context_ref("CTX-1", &contexts)); // case-insensitive
        assert!(!is_valid_context_ref("ctx-missing", &contexts));
    }

    #[test]
    fn test_count_missing_contexts() {
        let mut contexts = ContextSet::new();
        contexts.insert(create_test_context("ctx-1"));

        let facts = vec![
            create_test_fact("us-gaap:Revenue", "ctx-missing-1"),
            create_test_fact("us-gaap:Assets", "ctx-1"), // valid
            create_test_fact("us-gaap:Liabilities", "ctx-missing-2"),
        ];

        assert_eq!(count_missing_contexts(&facts, &contexts), 2);
    }
}
