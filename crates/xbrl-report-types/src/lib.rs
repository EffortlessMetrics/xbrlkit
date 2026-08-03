//! Canonical internal report model.

use serde::{Deserialize, Serialize};

/// Shared XBRL reporting period used by parsed and streaming contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Period {
    /// A specific instant in time (for example, 2024-12-31).
    Instant(String),
    /// A duration with start and end dates.
    Duration { start: String, end: String },
    /// A period that has no end date.
    #[default]
    Forever,
    /// A period that has not been determined or is invalid.
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Fact {
    pub concept: String,
    #[serde(alias = "context")]
    pub context_ref: String,
    pub unit_ref: Option<String>,
    pub decimals: Option<String>,
    pub value: String,
    #[serde(default)]
    pub member: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ValidationFinding {
    pub rule_id: String,
    pub severity: String,
    pub message: String,
    pub member: Option<String>,
    pub subject: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CanonicalReport {
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub facts: Vec<Fact>,
    #[serde(default)]
    pub findings: Vec<ValidationFinding>,
}

#[cfg(test)]
mod tests {
    use super::Period;

    #[test]
    fn period_defaults_to_forever() {
        assert_eq!(Period::default(), Period::Forever);
    }

    #[test]
    fn period_serializes_to_stable_contract_variants() -> Result<(), String> {
        let cases = [
            (
                Period::Instant("2024-12-31".to_string()),
                serde_json::json!({"Instant": "2024-12-31"}),
            ),
            (
                Period::Duration {
                    start: "2024-01-01".to_string(),
                    end: "2024-12-31".to_string(),
                },
                serde_json::json!({
                    "Duration": {"start": "2024-01-01", "end": "2024-12-31"}
                }),
            ),
            (Period::Forever, serde_json::json!("Forever")),
            (Period::Unknown, serde_json::json!("Unknown")),
        ];

        for (period, expected) in cases {
            let actual = serde_json::to_value(period).map_err(|error| error.to_string())?;
            if actual != expected {
                return Err(format!("expected {expected}, got {actual}"));
            }
        }

        Ok(())
    }
}
