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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CanonicalReport {
    #[serde(default)]
    pub members: Vec<String>,
    #[serde(default)]
    pub facts: Vec<Fact>,
    #[serde(default)]
    pub findings: Vec<ValidationFinding>,
}
