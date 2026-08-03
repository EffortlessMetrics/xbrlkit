//! Export orchestration.

use oim_normalize::to_json_value;
use receipt_types::{Receipt, RunResult};
use xbrl_report_types::CanonicalReport;

#[must_use = "handle the serialization result and use the exported JSON and receipt"]
pub fn export_json(report: &CanonicalReport) -> Result<(String, Receipt), serde_json::Error> {
    let json = serde_json::to_string_pretty(&to_json_value(report))?;
    let receipt = Receipt::new("export.report", "canonical-report", RunResult::Success);
    Ok((json, receipt))
}

#[cfg(test)]
mod tests {
    use super::export_json;
    use receipt_types::{Receipt, RunResult};
    use xbrl_report_types::CanonicalReport;

    #[test]
    fn export_json_preserves_report_and_receipt_contract() -> Result<(), String> {
        let (json, receipt) = export_json(&CanonicalReport::default())
            .map_err(|error| format!("serializing default report: {error}"))?;
        let value: serde_json::Value = serde_json::from_str(&json)
            .map_err(|error| format!("parsing exported report: {error}"))?;
        let expected = serde_json::json!({
            "members": [],
            "facts": [],
            "findings": [],
        });
        if value != expected {
            return Err(format!("unexpected exported report: {value}"));
        }

        let expected_receipt =
            Receipt::new("export.report", "canonical-report", RunResult::Success);
        if receipt != expected_receipt {
            return Err(format!("unexpected export receipt: {receipt:?}"));
        }
        Ok(())
    }
}
