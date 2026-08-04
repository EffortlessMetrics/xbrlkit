//! Canonical internal report model.

use serde::{Deserialize, Serialize};

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

impl ValidationFinding {
    /// Builds a finding whose member and subject identify a source fact.
    #[must_use]
    pub fn for_fact(
        fact: &Fact,
        rule_id: impl Into<String>,
        severity: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: severity.into(),
            message: message.into(),
            member: Some(fact.member.clone()),
            subject: Some(fact.concept.clone()),
        }
    }
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
    use super::{Fact, ValidationFinding};

    #[test]
    fn for_fact_preserves_fact_identity_and_metadata() -> Result<(), String> {
        let fact = Fact {
            concept: "us-gaap:Revenue".to_string(),
            member: "us-gaap:ScenarioActualMember".to_string(),
            ..Fact::default()
        };

        let finding = ValidationFinding::for_fact(&fact, "RULE-001", "error", "invalid fact");

        if finding.rule_id != "RULE-001"
            || finding.severity != "error"
            || finding.message != "invalid fact"
            || finding.member.as_deref() != Some("us-gaap:ScenarioActualMember")
            || finding.subject.as_deref() != Some("us-gaap:Revenue")
        {
            return Err(format!("unexpected fact finding: {finding:?}"));
        }
        Ok(())
    }
}
