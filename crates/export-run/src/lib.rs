//! Export orchestration.

use anyhow::Context;
use oim_normalize::to_json_value;
use receipt_types::{Receipt, RunResult};
use xbrl_report_types::CanonicalReport;

pub fn export_json(report: &CanonicalReport) -> anyhow::Result<(String, Receipt)> {
    let json = serde_json::to_string_pretty(&to_json_value(report))
        .context("serializing canonical report")?;
    let receipt = Receipt::new("export.report", "canonical-report", RunResult::Success);
    Ok((json, receipt))
}

#[cfg(test)]
mod tests {
    use super::export_json;
    use anyhow::Context;
    use receipt_types::RunResult;
    use xbrl_report_types::CanonicalReport;

    #[test]
    fn export_json_returns_serialized_report_and_success_receipt() -> anyhow::Result<()> {
        let report = CanonicalReport {
            members: vec!["member-1".to_string()],
            ..CanonicalReport::default()
        };

        let (json, receipt) = export_json(&report)?;
        let value: serde_json::Value =
            serde_json::from_str(&json).context("parsing exported JSON")?;
        let members = value
            .get("members")
            .context("exported JSON is missing members")?;

        if members != &serde_json::json!(["member-1"]) {
            anyhow::bail!("exported members did not preserve the report");
        }
        if receipt.kind != "export.report"
            || receipt.subject != "canonical-report"
            || receipt.result != RunResult::Success
        {
            anyhow::bail!("export receipt did not preserve the success contract");
        }

        Ok(())
    }
}
