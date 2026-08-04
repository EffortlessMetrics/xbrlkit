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

/// Elapsed execution time for one selected BDD scenario.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioTiming {
    /// Stable scenario identifier from the feature grid.
    pub scenario_id: String,
    /// Monotonic elapsed time converted to milliseconds.
    pub duration_ms: u64,
}

/// The `scenario.run.v1` receipt with optional per-scenario timing evidence.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScenarioRunReceipt {
    /// Common receipt metadata and notes.
    #[serde(flatten)]
    pub receipt: Receipt,
    /// Timing entries for scenarios selected by the run.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub timings: Vec<ScenarioTiming>,
}

impl ScenarioRunReceipt {
    /// Creates an empty scenario-run receipt.
    #[must_use]
    pub fn new(subject: impl Into<String>, result: RunResult) -> Self {
        Self {
            receipt: Receipt::new("scenario.run", subject, result),
            timings: Vec::new(),
        }
    }

    /// Records elapsed time for one selected scenario.
    pub fn push_timing(&mut self, scenario_id: impl Into<String>, duration_ms: u64) {
        self.timings.push(ScenarioTiming {
            scenario_id: scenario_id.into(),
            duration_ms,
        });
    }
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
    use super::{RunResult, ScenarioRunReceipt};
    use serde_json::{Value, json};
    use std::io::Error;

    fn missing(message: &str) -> Error {
        Error::other(message)
    }

    #[test]
    fn scenario_receipt_round_trips_timing_entries() -> Result<(), Box<dyn std::error::Error>> {
        let mut receipt = ScenarioRunReceipt::new("@alpha-active", RunResult::Success);
        receipt.push_timing("SCN-XK-EXAMPLE-001", 17);

        let encoded = serde_json::to_value(&receipt)?;
        let timings = encoded
            .get("timings")
            .and_then(Value::as_array)
            .ok_or_else(|| missing("timings were not serialized"))?;
        let timing = timings
            .first()
            .ok_or_else(|| missing("timing entry was not serialized"))?;
        if timing.get("scenario_id").and_then(Value::as_str) != Some("SCN-XK-EXAMPLE-001")
            || timing.get("duration_ms").and_then(Value::as_u64) != Some(17)
        {
            return Err(missing("serialized timing entry has the wrong shape").into());
        }

        let decoded: ScenarioRunReceipt = serde_json::from_value(encoded)?;
        if decoded != receipt {
            return Err(missing("scenario receipt did not round-trip").into());
        }
        Ok(())
    }

    #[test]
    fn scenario_receipt_accepts_legacy_receipt_shape() -> Result<(), Box<dyn std::error::Error>> {
        let legacy = json!({
            "kind": "scenario.run",
            "version": "v1",
            "subject": "@alpha-active",
            "result": "success",
            "artifacts": [],
            "notes": ["legacy receipt"]
        });
        let decoded: ScenarioRunReceipt = serde_json::from_value(legacy)?;
        if !decoded.timings.is_empty() {
            return Err(missing("legacy receipt unexpectedly gained timing entries").into());
        }
        Ok(())
    }
}
