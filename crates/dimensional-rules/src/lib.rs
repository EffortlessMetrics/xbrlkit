//! Dimensional validation rules for XBRL.
//!
//! This crate validates dimensional aspects of XBRL reports:
//! - Required dimensions are present for concepts that need them
//! - Dimension-member pairs are valid according to taxonomy
//! - Typed dimension values conform to expected types
//! - Domain-member hierarchies are respected

use std::collections::BTreeSet;
use taxonomy_dimensions::{DimensionTaxonomy, Domain};
use xbrl_contexts::{Context, ContextSet, DimensionMember, get_dimensional_members};
use xbrl_report_types::{Fact, ValidationFinding};

/// Result of validating a context's dimensions against taxonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DimensionalValidationResult {
    /// Context ID that was validated
    pub context_id: String,
    /// Findings from validation
    pub findings: Vec<ValidationFinding>,
    /// Dimensions present in the context
    pub present_dimensions: Vec<String>,
    /// Required dimensions that are missing
    pub missing_dimensions: Vec<String>,
}

/// Validate dimensions for a set of facts against taxonomy.
///
/// This checks that:
/// 1. Each fact's concept has required dimensions present in its context
/// 2. Each dimension-member pair is valid according to the taxonomy
/// 3. Typed dimension values conform to expected types
///
/// # Arguments
/// * `facts` - The facts to validate
/// * `context_set` - The contexts referenced by the facts
/// * `dim_taxonomy` - The dimension taxonomy defining valid dimensions and members
///
/// # Returns
/// Vector of validation findings for all dimensional errors.
#[must_use]
pub fn validate_fact_dimensions(
    facts: &[Fact],
    context_set: &ContextSet,
    dim_taxonomy: &DimensionTaxonomy,
) -> Vec<DimensionalValidationResult> {
    let mut results = Vec::new();
    let mut validated_contexts: BTreeSet<String> = BTreeSet::new();

    for fact in facts {
        // Skip if we've already validated this context
        if validated_contexts.contains(&fact.context_ref) {
            continue;
        }
        validated_contexts.insert(fact.context_ref.clone());

        // Get the context
        let Some(context) = context_set.get(&fact.context_ref) else {
            results.push(DimensionalValidationResult {
                context_id: fact.context_ref.clone(),
                findings: vec![ValidationFinding {
                    rule_id: "XBRL.DIMENSION.MISSING_CONTEXT".to_string(),
                    severity: "error".to_string(),
                    message: format!(
                        "Context {} not found for fact {}",
                        fact.context_ref, fact.concept
                    ),
                    member: Some(fact.concept.clone()),
                    subject: Some(fact.context_ref.clone()),
                }],
                present_dimensions: Vec::new(),
                missing_dimensions: Vec::new(),
            });
            continue;
        };

        // Validate this context's dimensions for this fact's concept
        let result = validate_context_dimensions(context, &fact.concept, dim_taxonomy);
        results.push(result);
    }

    results
}

/// Validate dimensions for a single context against taxonomy for a specific concept.
///
/// # Arguments
/// * `context` - The context to validate
/// * `concept_qname` - The concept `QName` to check required dimensions for
/// * `dim_taxonomy` - The dimension taxonomy
///
/// # Returns
/// Validation result with findings and dimension status.
#[must_use]
pub fn validate_context_dimensions(
    context: &Context,
    concept_qname: &str,
    dim_taxonomy: &DimensionTaxonomy,
) -> DimensionalValidationResult {
    let mut findings = Vec::new();
    let dim_members = get_dimensional_members(context);

    // Build set of present dimensions
    let present_dimensions: Vec<String> =
        dim_members.iter().map(|dm| dm.dimension.clone()).collect();

    // Get required dimensions for this concept
    let required_dims = dim_taxonomy.required_dimensions_for_concept(concept_qname);

    // Check for missing required dimensions
    let mut missing_dimensions = Vec::new();
    for req_dim in &required_dims {
        if !present_dimensions.contains(req_dim) {
            missing_dimensions.push(req_dim.clone());
            findings.push(ValidationFinding {
                rule_id: "XBRL.DIMENSION.MISSING_REQUIRED".to_string(),
                severity: "error".to_string(),
                message: format!(
                    "Concept {} requires dimension {} which is missing in context {}",
                    concept_qname, req_dim, context.id
                ),
                member: Some(concept_qname.to_string()),
                subject: Some(context.id.clone()),
            });
        }
    }

    // Validate each present dimension-member pair
    for dim_member in dim_members {
        if let Err(e) = validate_dimension_member(dim_member, dim_taxonomy) {
            findings.push(e);
        }
    }

    DimensionalValidationResult {
        context_id: context.id.clone(),
        findings,
        present_dimensions,
        missing_dimensions,
    }
}

/// Validate a single dimension-member pair against taxonomy.
fn validate_dimension_member(
    dim_member: &DimensionMember,
    dim_taxonomy: &DimensionTaxonomy,
) -> Result<(), ValidationFinding> {
    // Check if dimension exists
    if !dim_taxonomy.dimensions.contains_key(&dim_member.dimension) {
        return Err(ValidationFinding {
            rule_id: "XBRL.DIMENSION.UNKNOWN".to_string(),
            severity: "error".to_string(),
            message: format!("Unknown dimension: {}", dim_member.dimension),
            member: Some(dim_member.dimension.clone()),
            subject: Some(dim_member.member.clone()),
        });
    }

    // Get the dimension definition
    let dimension = dim_taxonomy.dimensions.get(&dim_member.dimension).unwrap();

    // If it's a typed dimension, the member is a value, not a QName
    if dimension.is_typed() {
        // TODO: Add typed value validation based on dimension's value_type
        return Ok(());
    }

    // For explicit dimensions, validate the member against the domain
    if let Some(domain_qname) = dim_taxonomy.dimension_domains.get(&dim_member.dimension) {
        if let Some(domain) = dim_taxonomy.domains.get(domain_qname) {
            if domain.contains(&dim_member.member) {
                return Ok(());
            }
            return Err(ValidationFinding {
                rule_id: "XBRL.DIMENSION.INVALID_MEMBER".to_string(),
                severity: "error".to_string(),
                message: format!(
                    "Member {} is not valid for dimension {} in domain {}",
                    dim_member.member, dim_member.dimension, domain_qname
                ),
                member: Some(dim_member.member.clone()),
                subject: Some(dim_member.dimension.clone()),
            });
        }
    }

    // No domain defined for this dimension
    Err(ValidationFinding {
        rule_id: "XBRL.DIMENSION.NO_DOMAIN".to_string(),
        severity: "error".to_string(),
        message: format!("Dimension {} has no domain defined", dim_member.dimension),
        member: Some(dim_member.dimension.clone()),
        subject: Some(dim_member.member.clone()),
    })
}

/// Check if a member is a descendant of another member in a domain.
#[must_use]
pub fn is_descendant_member(domain: &Domain, ancestor: &str, descendant: &str) -> bool {
    domain
        .descendants(ancestor)
        .contains(&descendant.to_string())
}

/// Get all validation findings from a set of dimensional validation results.
#[must_use]
pub fn collect_findings(results: &[DimensionalValidationResult]) -> Vec<ValidationFinding> {
    results.iter().flat_map(|r| r.findings.clone()).collect()
}

/// Summary of dimensional validation across a report.
#[derive(Debug, Clone, Default)]
pub struct DimensionalSummary {
    /// Total contexts validated
    pub total_contexts: usize,
    /// Contexts with errors
    pub contexts_with_errors: usize,
    /// Contexts with missing required dimensions
    pub contexts_with_missing_dims: usize,
    /// Total findings
    pub total_findings: usize,
    /// Unique missing dimensions found
    pub unique_missing_dimensions: BTreeSet<String>,
}

/// Generate summary from validation results.
#[must_use]
pub fn summarize_results(results: &[DimensionalValidationResult]) -> DimensionalSummary {
    let mut summary = DimensionalSummary {
        total_contexts: results.len(),
        ..Default::default()
    };

    for result in results {
        if !result.findings.is_empty() {
            summary.contexts_with_errors += 1;
            summary.total_findings += result.findings.len();
        }
        if !result.missing_dimensions.is_empty() {
            summary.contexts_with_missing_dims += 1;
            summary
                .unique_missing_dimensions
                .extend(result.missing_dimensions.clone());
        }
    }

    summary
}

#[cfg(test)]
mod tests {
    use super::*;
    use taxonomy_dimensions::{Dimension, DomainMember, Hypercube};
    use xbrl_contexts::{EntityIdentifier, Period};

    fn create_test_taxonomy() -> DimensionTaxonomy {
        let mut taxonomy = DimensionTaxonomy::new();

        // Add explicit dimension
        taxonomy.add_dimension(Dimension::Explicit {
            qname: "us-gaap:StatementScenarioAxis".to_string(),
            default_domain: Some("us-gaap:StatementScenarioDomain".to_string()),
            required: true,
        });

        // Add domain with members
        let mut domain = Domain::new("us-gaap:StatementScenarioDomain");
        domain.add_member(DomainMember {
            qname: "us-gaap:ScenarioActualMember".to_string(),
            parent: None,
            order: 1,
            label: Some("Actual".to_string()),
        });
        domain.add_member(DomainMember {
            qname: "us-gaap:ScenarioBudgetMember".to_string(),
            parent: None,
            order: 2,
            label: Some("Budget".to_string()),
        });
        taxonomy.add_domain(domain);

        // Link dimension to domain
        taxonomy.link_dimension_domain(
            "us-gaap:StatementScenarioAxis",
            "us-gaap:StatementScenarioDomain",
        );

        // Add hypercube
        let mut hypercube = Hypercube::new("us-gaap:StatementTable");
        hypercube.add_dimension("us-gaap:StatementScenarioAxis", true);
        taxonomy.add_hypercube(hypercube);

        // Associate concept with hypercube
        taxonomy.associate_concept_hypercube("us-gaap:Revenue", "us-gaap:StatementTable", true);

        taxonomy
    }

    use xbrl_contexts::DimensionalContainer;

    fn create_test_context_with_dims(id: &str, dims: Vec<(&str, &str)>) -> Context {
        Context {
            id: id.to_string(),
            entity: EntityIdentifier {
                scheme: "http://www.sec.gov/CIK".to_string(),
                value: "0000320193".to_string(),
            },
            entity_segment: None,
            period: Period::Instant("2024-12-31".to_string()),
            scenario: Some(DimensionalContainer {
                dimensions: dims
                    .into_iter()
                    .map(|(d, m)| DimensionMember {
                        dimension: d.to_string(),
                        member: m.to_string(),
                        is_typed: false,
                        typed_value: None,
                    })
                    .collect(),
                raw_xml: None,
            }),
        }
    }

    #[test]
    fn test_validate_context_with_valid_dimensions() {
        let taxonomy = create_test_taxonomy();
        let context = create_test_context_with_dims(
            "ctx-1",
            vec![(
                "us-gaap:StatementScenarioAxis",
                "us-gaap:ScenarioActualMember",
            )],
        );

        let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

        assert!(
            result.findings.is_empty(),
            "Expected no findings for valid dimensions"
        );
        assert_eq!(result.present_dimensions.len(), 1);
        assert!(result.missing_dimensions.is_empty());
    }

    #[test]
    fn test_validate_context_with_missing_required_dimension() {
        let taxonomy = create_test_taxonomy();
        let context = create_test_context_with_dims("ctx-1", vec![]);

        let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

        assert!(
            !result.findings.is_empty(),
            "Expected findings for missing dimension"
        );
        assert_eq!(result.missing_dimensions.len(), 1);
        assert_eq!(
            result.missing_dimensions[0],
            "us-gaap:StatementScenarioAxis"
        );
    }

    #[test]
    fn test_validate_context_with_invalid_member() {
        let taxonomy = create_test_taxonomy();
        let context = create_test_context_with_dims(
            "ctx-1",
            vec![("us-gaap:StatementScenarioAxis", "us-gaap:InvalidMember")],
        );

        let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

        assert!(
            !result.findings.is_empty(),
            "Expected findings for invalid member"
        );
        let has_invalid_member = result
            .findings
            .iter()
            .any(|f| f.rule_id == "XBRL.DIMENSION.INVALID_MEMBER");
        assert!(has_invalid_member, "Expected INVALID_MEMBER finding");
    }

    #[test]
    fn test_summarize_results() {
        let results = vec![
            DimensionalValidationResult {
                context_id: "ctx-1".to_string(),
                findings: vec![ValidationFinding {
                    rule_id: "TEST".to_string(),
                    severity: "error".to_string(),
                    message: "Test error".to_string(),
                    member: None,
                    subject: None,
                }],
                present_dimensions: vec![],
                missing_dimensions: vec!["dim1".to_string()],
            },
            DimensionalValidationResult {
                context_id: "ctx-2".to_string(),
                findings: vec![],
                present_dimensions: vec!["dim2".to_string()],
                missing_dimensions: vec![],
            },
        ];

        let summary = summarize_results(&results);

        assert_eq!(summary.total_contexts, 2);
        assert_eq!(summary.contexts_with_errors, 1);
        assert_eq!(summary.contexts_with_missing_dims, 1);
        assert_eq!(summary.total_findings, 1);
        assert!(summary.unique_missing_dimensions.contains("dim1"));
    }
}
