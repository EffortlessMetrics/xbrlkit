//! Dimensional validation rules for XBRL.
//!
//! This crate validates dimensional aspects of XBRL reports:
//! - Required dimensions are present for concepts that need them
//! - Dimension-member pairs are valid according to taxonomy
//! - Typed dimension values conform to expected types
//! - Domain-member hierarchies are respected

use std::collections::BTreeSet;
use taxonomy_dimensions::DimensionTaxonomy;
use xbrl_contexts::ContextSet;
use xbrl_report_types::{Fact, ValidationFinding};

mod result;
mod typed_values;
mod validate;

pub use result::{
    DimensionalSummary, DimensionalValidationResult, collect_findings, summarize_results,
};
pub use validate::{is_descendant_member, validate_context_dimensions};

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

#[cfg(test)]
mod tests;
