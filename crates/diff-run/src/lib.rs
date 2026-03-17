//! Filing diff use case.

use receipt_types::{Receipt, RunResult};
use xbrl_report_types::CanonicalReport;

#[must_use]
pub fn diff_reports(left: &CanonicalReport, right: &CanonicalReport) -> (Vec<String>, Receipt) {
    let left_set = left
        .facts
        .iter()
        .map(|fact| format!("{}:{}:{}", fact.concept, fact.context_ref, fact.value))
        .collect::<std::collections::BTreeSet<_>>();
    let right_set = right
        .facts
        .iter()
        .map(|fact| format!("{}:{}:{}", fact.concept, fact.context_ref, fact.value))
        .collect::<std::collections::BTreeSet<_>>();
    let changes = left_set
        .symmetric_difference(&right_set)
        .cloned()
        .collect::<Vec<_>>();
    let receipt = Receipt::new(
        "diff.report",
        "canonical-report",
        if changes.is_empty() {
            RunResult::Success
        } else {
            RunResult::Warning
        },
    );
    (changes, receipt)
}
