use std::collections::BTreeSet;

use xbrl_report_types::ValidationFinding;

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

/// Get all validation findings from a set of dimensional validation results.
#[must_use]
pub fn collect_findings(results: &[DimensionalValidationResult]) -> Vec<ValidationFinding> {
    results
        .iter()
        .flat_map(|result| result.findings.clone())
        .collect()
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
