//! Export orchestration.

use oim_normalize::to_json_value;
use receipt_types::{Receipt, RunResult};
use xbrl_report_types::CanonicalReport;

#[must_use = "handle the export result"]
pub fn export_json(report: &CanonicalReport) -> Result<(String, Receipt), serde_json::Error> {
    let json = serde_json::to_string_pretty(&to_json_value(report))?;
    let receipt = Receipt::new("export.report", "canonical-report", RunResult::Success);
    Ok((json, receipt))
}

#[cfg(test)]
mod tests {
    use super::export_json;
    use receipt_types::RunResult;
    use xbrl_report_types::CanonicalReport;

    #[test]
    fn export_json_returns_serialized_report_and_success_receipt() -> Result<(), String> {
        let (json, receipt) =
            export_json(&CanonicalReport::default()).map_err(|error| error.to_string())?;

        if !json.contains("\"members\"") {
            return Err(format!("serialized report omitted members: {json}"));
        }
        if receipt.kind != "export.report" {
            return Err(format!("unexpected receipt kind: {}", receipt.kind));
        }
        if receipt.result != RunResult::Success {
            return Err(format!("unexpected receipt result: {:?}", receipt.result));
        }
        Ok(())
    }
}
