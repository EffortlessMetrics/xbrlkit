//! Export orchestration.

use oim_normalize::to_json_value;
use receipt_types::{Receipt, RunResult};
use xbrl_report_types::CanonicalReport;

/// Errors that can occur while exporting a canonical report as JSON.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("serializing canonical report: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Serializes a canonical report and returns its successful export receipt.
///
/// # Errors
///
/// Returns [`ExportError::Serialization`] when the report cannot be encoded as JSON.
pub fn export_json(report: &CanonicalReport) -> Result<(String, Receipt), ExportError> {
    let json = serde_json::to_string_pretty(&to_json_value(report))?;
    let receipt = Receipt::new("export.report", "canonical-report", RunResult::Success);
    Ok((json, receipt))
}

#[cfg(test)]
mod tests {
    use super::export_json;
    use xbrl_report_types::CanonicalReport;

    #[test]
    fn exports_report_and_receipt() -> Result<(), String> {
        let (json, receipt) = export_json(&CanonicalReport::default())
            .map_err(|error| format!("export failed: {error}"))?;

        if !json.contains('\n') {
            return Err("expected pretty-printed JSON output".to_string());
        }
        if receipt.kind != "export.report" {
            return Err(format!("unexpected receipt kind: {}", receipt.kind));
        }
        if receipt.subject != "canonical-report" {
            return Err(format!("unexpected receipt subject: {}", receipt.subject));
        }

        Ok(())
    }
}
