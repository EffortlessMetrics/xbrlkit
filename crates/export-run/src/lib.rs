//! Export orchestration.

use oim_normalize::to_json_value;
use receipt_types::{Receipt, RunResult};
use xbrl_report_types::CanonicalReport;

#[must_use]
pub fn export_json(report: &CanonicalReport) -> (String, Receipt) {
    let json = serde_json::to_string_pretty(&to_json_value(report))
        .expect("canonical report serialization should succeed");
    let receipt = Receipt::new("export.report", "canonical-report", RunResult::Success);
    (json, receipt)
}
