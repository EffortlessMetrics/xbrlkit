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

/// Creates the warning receipt emitted by oracle-comparison lanes.
#[must_use]
pub fn oracle_comparison_receipt(subject: &str) -> Receipt {
    Receipt::new("oracle.compare", subject, RunResult::Warning)
}

#[cfg(test)]
mod tests {
    use super::oracle_comparison_receipt;

    #[test]
    fn oracle_comparison_receipt_preserves_the_contract() -> Result<(), String> {
        let receipt = oracle_comparison_receipt("filing-001");

        if receipt.kind != "oracle.compare" {
            return Err(format!("unexpected receipt kind: {}", receipt.kind));
        }
        if receipt.version != "v1" {
            return Err(format!("unexpected receipt version: {}", receipt.version));
        }
        if receipt.subject != "filing-001" {
            return Err(format!("unexpected receipt subject: {}", receipt.subject));
        }
        if receipt.result != super::RunResult::Warning {
            return Err(format!("unexpected receipt result: {:?}", receipt.result));
        }

        Ok(())
    }
}
