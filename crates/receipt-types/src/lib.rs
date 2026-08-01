//! Stable receipt DTOs.

use serde::{Deserialize, Serialize};
use std::time::Duration;

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
    /// Wall-clock time spent executing the selected scenario(s), in milliseconds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub execution_duration_ms: Option<u64>,
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
            execution_duration_ms: None,
        }
    }

    /// Record a measured scenario execution duration without allowing a
    /// platform-sized duration to overflow the receipt contract.
    pub fn set_execution_duration(&mut self, duration: Duration) {
        self.execution_duration_ms = Some(u64::try_from(duration.as_millis()).unwrap_or(u64::MAX));
    }
}

#[cfg(test)]
mod tests {
    use super::{Receipt, RunResult};
    use std::time::Duration;

    #[test]
    fn omits_execution_duration_until_recorded() -> Result<(), String> {
        let receipt = Receipt::new("scenario.run", "@alpha-active", RunResult::Success);
        let json = serde_json::to_value(receipt).map_err(|error| error.to_string())?;
        if json.get("execution_duration_ms").is_some() {
            return Err("unset execution duration should be omitted".to_string());
        }
        Ok(())
    }

    #[test]
    fn round_trips_execution_duration() -> Result<(), String> {
        let mut receipt = Receipt::new("scenario.run", "@alpha-active", RunResult::Success);
        receipt.set_execution_duration(Duration::from_millis(42));
        let json = serde_json::to_value(&receipt).map_err(|error| error.to_string())?;
        let decoded: Receipt = serde_json::from_value(json).map_err(|error| error.to_string())?;
        if decoded.execution_duration_ms != Some(42) {
            return Err(format!(
                "expected 42 ms, got {:?}",
                decoded.execution_duration_ms
            ));
        }
        Ok(())
    }

    #[test]
    fn accepts_receipts_written_before_timing_was_added() -> Result<(), String> {
        let json = serde_json::json!({
            "kind": "scenario.run",
            "version": "v1",
            "subject": "@alpha-active",
            "result": "success",
            "artifacts": [],
            "notes": []
        });
        let decoded: Receipt = serde_json::from_value(json).map_err(|error| error.to_string())?;
        if decoded.execution_duration_ms.is_some() {
            return Err("legacy receipt should decode without a duration".to_string());
        }
        Ok(())
    }
}
