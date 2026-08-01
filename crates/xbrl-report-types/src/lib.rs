//! Canonical internal report model.

use serde::{Deserialize, Serialize};

/// Converts a value into the uppercase ASCII component used in rule IDs.
///
/// ASCII letters and digits are retained; every other character becomes `_`.
#[must_use]
pub fn sanitize_rule_id_component(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
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
    use super::sanitize_rule_id_component;

    #[test]
    fn sanitizes_rule_id_components() -> Result<(), String> {
        let actual = sanitize_rule_id_component("dei:Entity-Name_é");
        if actual != "DEI_ENTITY_NAME__" {
            return Err(format!("unexpected sanitized component: {actual}"));
        }

        Ok(())
    }
}
