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
      "artifacts": receipt.artifacts,
      "notes": receipt.notes,
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
    fn projects_base_fields_and_empty_compatibility_collections() -> Result<(), String> {
        let receipt = Receipt::new(
            "validation.report",
            "workspace-validation",
            RunResult::Success,
        );

        let report = to_sensor_report("xbrlkit", &receipt);

        require(
            &report,
            &json!({
                "kind": "sensor.report",
                "version": "v1",
                "subject": "workspace-validation",
                "result": "success",
                "sensor_id": "xbrlkit",
                "artifacts": [],
                "notes": [],
                "inner_receipt": {
                    "kind": "validation.report",
                    "version": "v1",
                    "subject": "workspace-validation",
                    "result": "success",
                    "artifacts": [],
                    "notes": []
                }
            }),
            "sensor report",
        )
    }

    #[test]
    fn projects_artifacts_and_notes_at_the_top_level_and_in_inner_receipt() -> Result<(), String> {
        let mut receipt = Receipt::new("validation.report", "subject", RunResult::Warning);
        receipt.artifacts.push(ArtifactRef {
            path: "artifacts/report.json".to_string(),
            sha256: Some("abc123".to_string()),
        });
        receipt.notes.push("review required".to_string());

        let report = to_sensor_report("sensor", &receipt);
        let expected_artifacts = json!([{"path": "artifacts/report.json", "sha256": "abc123"}]);
        let expected_notes = json!(["review required"]);

        require(
            &report["artifacts"],
            &expected_artifacts,
            "top-level artifacts",
        )?;
        require(&report["notes"], &expected_notes, "top-level notes")?;
        require(
            &report["inner_receipt"]["artifacts"],
            &expected_artifacts,
            "inner artifacts",
        )?;
        require(
            &report["inner_receipt"]["notes"],
            &expected_notes,
            "inner notes",
        )
    }

    #[test]
    fn preserves_warning_and_error_result_mapping() -> Result<(), String> {
        for (result, expected) in [(RunResult::Warning, "warning"), (RunResult::Error, "error")] {
            let receipt = Receipt::new("validation.report", "subject", result);
            let report = to_sensor_report("sensor", &receipt);

            require(&report["result"], &json!(expected), "top-level result")?;
            require(
                &report["inner_receipt"]["result"],
                &json!(expected),
                "inner receipt result",
            )?;
        }
        Ok(())
    }
}
