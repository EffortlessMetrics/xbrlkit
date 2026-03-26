//! Main validation orchestrator.

use duplicate_facts::{DuplicateDisposition, classify};
use efm_rules::{validate_inline_restrictions, validate_required_facts, validate_taxonomy_years};
use ixds_assemble::assemble;
use numeric_rules::validate_negative_values;
use receipt_types::{Receipt, RunResult};
use sec_profile_types::ProfilePack;
use taxonomy_dimensions::DimensionTaxonomy;
use taxonomy_dts::{build_dts, nonstandard_entry_points};
use taxonomy_types::DtsDescriptor;
use xbrl_contexts::{ContextSet, parse_contexts};
use xbrl_report_types::{CanonicalReport, Fact, ValidationFinding};

#[derive(Debug, Clone)]
pub struct ValidationRun {
    pub report: CanonicalReport,
    pub receipt: Receipt,
}

#[derive(Debug, Clone)]
pub struct TaxonomyResolutionRun {
    pub dts: DtsDescriptor,
    pub receipt: Receipt,
}

#[must_use]
pub fn validate_html_members(members: &[(&str, &str)], profile: &ProfilePack) -> ValidationRun {
    let mut report = assemble(members);
    for (member, html) in members {
        report
            .findings
            .extend(validate_inline_restrictions(member, html, profile));
    }
    // Validate required facts after assembly
    report
        .findings
        .extend(validate_required_facts(&report.facts, profile));
    // Validate negative values where prohibited
    let prohibited_concepts = profile
        .numeric_rules
        .as_ref()
        .map(|nr| nr.negative_value_rules.prohibited_concepts.clone())
        .unwrap_or_default();
    report.findings.extend(validate_negative_values(
        &report.facts,
        &prohibited_concepts,
    ));
    let subject = format!("{} members", report.members.len());
    finalize_validation(report, subject)
}

#[must_use]
pub fn validate_duplicate_report(report: CanonicalReport) -> ValidationRun {
    let subject = format!("{} members", report.members.len());
    finalize_validation(report, subject)
}

#[must_use]
pub fn validate_taxonomy_entry_points(
    entry_points: &[String],
    profile: &ProfilePack,
) -> ValidationRun {
    let report = CanonicalReport {
        members: entry_points.to_vec(),
        facts: Vec::new(),
        findings: validate_taxonomy_years(entry_points, profile),
    };
    finalize_validation(report, format!("{} entry points", entry_points.len()))
}

#[must_use]
pub fn resolve_taxonomy_entry_points(
    entry_points: &[String],
    profile: &ProfilePack,
) -> TaxonomyResolutionRun {
    let dts = build_dts(profile, entry_points.to_vec());
    let missing = nonstandard_entry_points(&dts, profile);
    let result = if missing.is_empty() {
        RunResult::Success
    } else {
        RunResult::Error
    };
    let mut receipt = Receipt::new(
        "taxonomy.resolve",
        format!("{} entry points", dts.entry_points.len()),
        result,
    );
    if !missing.is_empty() {
        receipt.notes.push(format!(
            "non-standard taxonomy locations: {}",
            missing.join(", ")
        ));
    }
    TaxonomyResolutionRun { dts, receipt }
}

fn finalize_validation(mut report: CanonicalReport, subject: String) -> ValidationRun {
    apply_duplicate_fact_findings(&mut report);
    let mut receipt = Receipt::new("validation.report", subject, run_result(&report));
    if !report.findings.is_empty() {
        receipt
            .notes
            .push(format!("{} finding(s)", report.findings.len()));
    }
    ValidationRun { report, receipt }
}

fn apply_duplicate_fact_findings(report: &mut CanonicalReport) {
    match classify(report) {
        DuplicateDisposition::None => {}
        DuplicateDisposition::Consistent => {
            report.findings.push(ValidationFinding {
                rule_id: "XBRL.DUPLICATE_FACT.CONSISTENT".to_string(),
                severity: "info".to_string(),
                message: "Consistent duplicate facts detected".to_string(),
                member: None,
                subject: None,
            });
        }
        DuplicateDisposition::Inconsistent => {
            report.findings.push(ValidationFinding {
                rule_id: "XBRL.DUPLICATE_FACT.INCONSISTENT".to_string(),
                severity: "error".to_string(),
                message: "Inconsistent duplicate facts detected".to_string(),
                member: None,
                subject: None,
            });
        }
    }
}

fn run_result(report: &CanonicalReport) -> RunResult {
    if report
        .findings
        .iter()
        .any(|finding| finding.severity == "error")
    {
        RunResult::Error
    } else if report
        .findings
        .iter()
        .any(|finding| finding.severity == "warning")
    {
        RunResult::Warning
    } else {
        RunResult::Success
    }
}

/// Parse and validate contexts from an XBRL instance.
///
/// # Errors
/// Returns error if XML parsing fails.
pub fn validate_contexts(xbrl_xml: &str) -> Result<(ContextSet, Vec<ValidationFinding>), String> {
    let mut findings = Vec::new();

    let context_set =
        parse_contexts(xbrl_xml).map_err(|e| format!("Failed to parse contexts: {e}"))?;

    // Check for contexts without entity identifiers
    for context in context_set.iter() {
        if context.entity.value.is_empty() {
            findings.push(ValidationFinding {
                rule_id: "XBRL.CONTEXT.MISSING_ENTITY".to_string(),
                severity: "error".to_string(),
                message: format!("Context {} has no entity identifier", context.id),
                member: None,
                subject: Some(context.id.clone()),
            });
        }

        // Check for contexts with dimensional information
        let dim_count = xbrl_contexts::get_dimensional_members(context).len();
        if dim_count > 0 {
            findings.push(ValidationFinding {
                rule_id: "XBRL.CONTEXT.HAS_DIMENSIONS".to_string(),
                severity: "info".to_string(),
                message: format!("Context {} has {dim_count} dimensional members", context.id),
                member: None,
                subject: Some(context.id.clone()),
            });
        }
    }

    Ok((context_set, findings))
}

/// Validate dimensional aspects of facts using dimensional-rules crate.
///
/// # Arguments
/// * `facts` - The facts to validate
/// * `context_set` - The contexts referenced by the facts
/// * `dim_taxonomy` - The dimension taxonomy for validation
///
/// # Returns
/// Vector of validation findings for dimensional errors.
#[must_use]
pub fn validate_dimensions(
    facts: &[Fact],
    context_set: &ContextSet,
    dim_taxonomy: &DimensionTaxonomy,
) -> Vec<ValidationFinding> {
    use dimensional_rules::validate_fact_dimensions;

    let results = validate_fact_dimensions(facts, context_set, dim_taxonomy);
    let mut findings = Vec::new();

    for result in results {
        for finding in result.findings {
            findings.push(finding);
        }
        // Note: MISSING_REQUIRED findings are already included in result.findings
        // by validate_context_dimensions, so we don't need to add them again
    }

    findings
}

/// Validate context completeness using streaming parser for large files.
///
/// For files >100MB, uses SAX-style streaming to avoid DOM memory overhead.
/// Collects facts and validates context references in a single pass.
///
/// # Arguments
/// * `xbrl_xml` - The XBRL XML content
/// * `size_threshold_mb` - Use streaming if file exceeds this size (default: 100)
///
/// # Returns
/// Vector of validation findings for missing context references.
#[must_use]
pub fn validate_context_completeness_streaming(xbrl_xml: &str, _size_threshold_mb: usize) -> Vec<ValidationFinding> {
    use xbrl_stream::{FactHandler, StreamingFact, StreamingContext, XbrlStreamReader};
    use std::collections::HashSet;
    
    struct ContextCompletenessHandler {
        facts: Vec<StreamingFact>,
        contexts: HashSet<String>,
    }
    
    impl FactHandler for ContextCompletenessHandler {
        fn on_fact(&mut self, fact: StreamingFact) -> anyhow::Result<()> {
            self.facts.push(fact);
            Ok(())
        }
        
        fn on_context(&mut self, context: StreamingContext) -> anyhow::Result<()> {
            self.contexts.insert(context.id);
            Ok(())
        }
    }
    
    let handler = ContextCompletenessHandler {
        facts: Vec::new(),
        contexts: HashSet::new(),
    };
    
    let reader = XbrlStreamReader::new(std::io::Cursor::new(xbrl_xml), handler);
    
    match reader.parse() {
        Ok(handler) => {
            let mut findings = Vec::new();
            
            // Check for facts referencing non-existent contexts
            for fact in &handler.facts {
                if !handler.contexts.contains(&fact.context_ref) {
                    findings.push(ValidationFinding {
                        rule_id: "XBRL.CONTEXT.MISSING_REF".to_string(),
                        severity: "error".to_string(),
                        message: format!(
                            "Fact '{}' references undefined context '{}'",
                            fact.concept, fact.context_ref
                        ),
                        member: None,
                        subject: Some(fact.context_ref.clone()),
                    });
                }
            }
            
            findings
        }
        Err(e) => {
            vec![ValidationFinding {
                rule_id: "XBRL.STREAM_PARSE_ERROR".to_string(),
                severity: "error".to_string(),
                message: format!("Streaming parse failed: {}", e),
                member: None,
                subject: None,
            }]
        }
    }
}

/// Returns whether streaming parser should be used for given content size.
///
/// Default threshold is 100MB to avoid excessive memory usage with DOM parsing.
#[must_use]
pub fn should_use_streaming(content_size_bytes: usize, threshold_mb: Option<usize>) -> bool {
    let threshold = threshold_mb.unwrap_or(100);
    content_size_bytes > threshold * 1024 * 1024
}
