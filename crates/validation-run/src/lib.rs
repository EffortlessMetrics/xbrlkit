//! Main validation orchestrator.

use duplicate_facts::{DuplicateDisposition, classify};
use efm_rules::{validate_inline_restrictions, validate_required_facts, validate_taxonomy_years};
use ixds_assemble::assemble;
use receipt_types::{Receipt, RunResult};
use sec_profile_types::ProfilePack;
use taxonomy_dts::{build_dts, nonstandard_entry_points};
use taxonomy_types::DtsDescriptor;
use xbrl_report_types::{CanonicalReport, ValidationFinding};

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
    report.findings.extend(validate_required_facts(&report.facts, profile));
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
