//! OIM-aligned export helpers.

use serde_json::json;
use xbrl_report_types::CanonicalReport;

#[must_use]
pub fn to_json_value(report: &CanonicalReport) -> serde_json::Value {
    json!({
      "members": report.members,
      "facts": report.facts,
      "findings": report.findings,
    })
}
