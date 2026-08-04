//! Cockpit envelope helpers.

use receipt_types::Receipt;

#[must_use]
pub fn to_sensor_report(sensor_id: &str, receipt: &Receipt) -> serde_json::Value {
    serde_json::json!({
      "kind": "sensor.report",
      "version": "v1",
      "subject": receipt.subject,
      "result": format!("{:?}", receipt.result).to_ascii_lowercase(),
      "sensor_id": sensor_id,
      "inner_receipt": receipt,
    })
}

#[cfg(test)]
mod tests {
    use super::to_sensor_report;
    use receipt_types::{ArtifactRef, Receipt, RunResult};
    use serde_json::json;

    fn require<T: PartialEq + std::fmt::Debug>(
        actual: &T,
        expected: &T,
        label: &str,
    ) -> Result<(), String> {
        if actual == expected {
            Ok(())
        } else {
            Err(format!("{label}: expected {expected:?}, got {actual:?}"))
        }
    }

    #[test]
    fn projects_envelope_fields_and_success_receipt() -> Result<(), String> {
        let receipt = Receipt::new(
            "validation.report",
            "workspace-validation",
            RunResult::Success,
        );

        let report = to_sensor_report("xbrlkit", &receipt);

        require(&report["kind"], &json!("sensor.report"), "envelope kind")?;
        require(&report["version"], &json!("v1"), "envelope version")?;
        require(
            &report["subject"],
            &json!("workspace-validation"),
            "envelope subject",
        )?;
        require(&report["result"], &json!("success"), "envelope result")?;
        require(&report["sensor_id"], &json!("xbrlkit"), "sensor id")?;
        require(
            &report["inner_receipt"],
            &json!({
                "kind": "validation.report",
                "version": "v1",
                "subject": "workspace-validation",
                "result": "success",
                "artifacts": [],
                "notes": []
            }),
            "inner receipt",
        )
    }

    #[test]
    fn maps_warning_and_error_results_without_changing_the_envelope() -> Result<(), String> {
        for (result, expected) in [(RunResult::Warning, "warning"), (RunResult::Error, "error")] {
            let receipt = Receipt::new("validation.report", "subject", result);
            let report = to_sensor_report("sensor", &receipt);

            require(&report["result"], &json!(expected), "top-level result")?;
            require(
                &report["inner_receipt"]["result"],
                &json!(expected),
                "inner receipt result",
            )?;
            require(&report["kind"], &json!("sensor.report"), "envelope kind")?;
        }
        Ok(())
    }

    #[test]
    fn preserves_artifacts_and_notes_in_the_inner_receipt() -> Result<(), String> {
        let mut receipt = Receipt::new("validation.report", "subject", RunResult::Warning);
        receipt.artifacts.push(ArtifactRef {
            path: "artifacts/report.json".to_string(),
            sha256: Some("abc123".to_string()),
        });
        receipt.notes.push("review required".to_string());

        let report = to_sensor_report("sensor", &receipt);

        require(
            &report["inner_receipt"]["artifacts"],
            &json!([{"path": "artifacts/report.json", "sha256": "abc123"}]),
            "inner artifacts",
        )?;
        require(
            &report["inner_receipt"]["notes"],
            &json!(["review required"]),
            "inner notes",
        )
    }
}
