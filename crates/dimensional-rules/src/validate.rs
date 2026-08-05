use taxonomy_dimensions::{DimensionTaxonomy, Domain};
use xbrl_contexts::{Context, DimensionMember, get_dimensional_members};
use xbrl_report_types::ValidationFinding;

use crate::findings::{invalid_member, missing_required_dimension, no_domain, unknown_dimension};
use crate::result::DimensionalValidationResult;
use crate::typed_values::validate_typed_dimension_value;

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
            findings.push(missing_required_dimension(
                concept_qname,
                req_dim,
                &context.id,
            ));
        }
    }

    // Validate each present dimension-member pair
    for dim_member in dim_members {
        if let Err(error) = validate_dimension_member(dim_member, dim_taxonomy) {
            findings.push(error);
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
        return Err(unknown_dimension(&dim_member.dimension, &dim_member.member));
    }

    // Get the dimension definition
    let dimension = dim_taxonomy.dimensions.get(&dim_member.dimension).unwrap();

    // If it's a typed dimension, validate the value against the expected type
    if dimension.is_typed() {
        return validate_typed_dimension_value(dim_member, dimension);
    }

    // For explicit dimensions, validate the member against the domain
    if let Some(domain_qname) = dim_taxonomy.dimension_domains.get(&dim_member.dimension)
        && let Some(domain) = dim_taxonomy.domains.get(domain_qname)
    {
        if domain.contains(&dim_member.member) {
            return Ok(());
        }
        return Err(invalid_member(
            &dim_member.member,
            &dim_member.dimension,
            domain_qname,
        ));
    }

    // No domain defined for this dimension
    Err(no_domain(&dim_member.dimension, &dim_member.member))
}

/// Check if a member is a descendant of another member in a domain.
#[must_use]
pub fn is_descendant_member(domain: &Domain, ancestor: &str, descendant: &str) -> bool {
    domain
        .descendants(ancestor)
        .contains(&descendant.to_string())
}
