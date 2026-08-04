//! Stable receipt DTOs.

use serde::{Deserialize, Serialize};

/// Outcome for a receipt-producing run.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunResult {
    Success,
    Warning,
    Error,
}

/// Reference to an emitted artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ArtifactRef {
    pub path: String,
    pub sha256: Option<String>,
}

/// Base receipt surface used by the workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Receipt {
    pub kind: String,
    pub version: String,
    pub subject: String,
    pub result: RunResult,
    #[serde(default)]
    pub artifacts: Vec<ArtifactRef>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl Receipt {
    #[must_use]
    pub fn new(kind: impl Into<String>, subject: impl Into<String>, result: RunResult) -> Self {
        Self {
            kind: kind.into(),
            version: "v1".to_string(),
            subject: subject.into(),
            result,
            artifacts: Vec::new(),
            notes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ArtifactRef, Receipt, RunResult};

    fn require(condition: bool, message: &str) -> Result<(), String> {
        if condition {
            Ok(())
        } else {
            Err(message.to_string())
        }
    }

    #[test]
    fn constructor_sets_stable_defaults() -> Result<(), String> {
        let receipt = Receipt::new("scenario.run", "AC-XK-WORKFLOW-003", RunResult::Success);

        require(receipt.kind == "scenario.run", "constructor kind mismatch")?;
        require(receipt.version == "v1", "constructor version mismatch")?;
        require(
            receipt.subject == "AC-XK-WORKFLOW-003",
            "constructor subject mismatch",
        )?;
        require(
            receipt.result == RunResult::Success,
            "constructor result mismatch",
        )?;
        require(
            receipt.artifacts.is_empty(),
            "constructor artifacts should be empty",
        )?;
        require(
            receipt.notes.is_empty(),
            "constructor notes should be empty",
        )
    }

    #[test]
    fn run_results_use_snake_case_wire_names() -> Result<(), String> {
        let cases = [
            (RunResult::Success, "\"success\""),
            (RunResult::Warning, "\"warning\""),
            (RunResult::Error, "\"error\""),
        ];

        for (result, expected) in cases {
            let actual = serde_json::to_string(&result).map_err(|error| error.to_string())?;
            require(actual == expected, "run result wire name mismatch")?;
        }

        Ok(())
    }

    #[test]
    fn serializes_populated_receipt_to_stable_json_shape() -> Result<(), String> {
        let mut receipt = Receipt::new("scenario.run", "AC-XK-WORKFLOW-003", RunResult::Warning);
        receipt.artifacts.push(ArtifactRef {
            path: "artifacts/runs/scenario.run.v1.json".to_string(),
            sha256: None,
        });
        receipt.notes.push("completed with warnings".to_string());

        let actual = serde_json::to_string(&receipt).map_err(|error| error.to_string())?;
        let expected = concat!(
            "{\"kind\":\"scenario.run\",",
            "\"version\":\"v1\",",
            "\"subject\":\"AC-XK-WORKFLOW-003\",",
            "\"result\":\"warning\",",
            "\"artifacts\":[{\"path\":\"artifacts/runs/scenario.run.v1.json\",",
            "\"sha256\":null}],",
            "\"notes\":[\"completed with warnings\"]}"
        );

        require(actual == expected, "receipt JSON shape mismatch")
    }

    #[test]
    fn omitted_receipt_collections_default_to_empty_vectors() -> Result<(), String> {
        let receipt: Receipt = serde_json::from_str(
            r#"{
                "kind": "scenario.run",
                "version": "v1",
                "subject": "AC-XK-WORKFLOW-003",
                "result": "success"
            }"#,
        )
        .map_err(|error| error.to_string())?;

        require(
            receipt.artifacts.is_empty(),
            "omitted artifacts should default to empty",
        )?;
        require(
            receipt.notes.is_empty(),
            "omitted notes should default to empty",
        )
    }
}
