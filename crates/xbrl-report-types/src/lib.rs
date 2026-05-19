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
    /// Create a baseline finding with no member or subject.
    pub fn new(
        rule_id: impl Into<String>,
        severity: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: severity.into(),
            message: message.into(),
            member: None,
            subject: None,
        }
    }

    /// Create a finding from a fact, using `fact.member` and `fact.concept`.
    pub fn for_fact(
        rule_id: impl Into<String>,
        severity: impl Into<String>,
        fact: &Fact,
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

    /// Create a finding tied to a context, using `context_id` as the subject.
    pub fn for_context(
        rule_id: impl Into<String>,
        severity: impl Into<String>,
        context_id: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: severity.into(),
            message: message.into(),
            member: None,
            subject: Some(context_id.into()),
        }
    }

    /// Create a finding from a dimension-member pair.
    ///
    /// `dimension` maps to the `member` field; `member` maps to the `subject` field,
    /// matching the domain convention used in dimensional validation.
    pub fn for_dimension_member(
        rule_id: impl Into<String>,
        severity: impl Into<String>,
        dimension: impl Into<String>,
        member: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: severity.into(),
            message: message.into(),
            member: Some(dimension.into()),
            subject: Some(member.into()),
        }
    }

    /// Set the member field (builder-style).
    #[must_use]
    pub fn with_member(mut self, member: impl Into<String>) -> Self {
        self.member = Some(member.into());
        self
    }

    /// Set the subject field (builder-style).
    #[must_use]
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_sets_rule_id_severity_message_and_none_fields() {
        let finding = ValidationFinding::new("RULE-1", "error", "Something went wrong");

        assert_eq!(finding.rule_id, "RULE-1");
        assert_eq!(finding.severity, "error");
        assert_eq!(finding.message, "Something went wrong");
        assert_eq!(finding.member, None);
        assert_eq!(finding.subject, None);
    }

    #[test]
    fn for_fact_populates_member_and_subject_from_fact() {
        let fact = Fact {
            concept: "us-gaap:Revenue".to_string(),
            context_ref: "ctx-1".to_string(),
            unit_ref: None,
            decimals: None,
            value: "1000".to_string(),
            member: "member-a".to_string(),
        };

        let finding =
            ValidationFinding::for_fact("SEC.RULE", "error", &fact, "Negative value");

        assert_eq!(finding.rule_id, "SEC.RULE");
        assert_eq!(finding.severity, "error");
        assert_eq!(finding.message, "Negative value");
        assert_eq!(finding.member, Some("member-a".to_string()));
        assert_eq!(finding.subject, Some("us-gaap:Revenue".to_string()));
    }

    #[test]
    fn for_context_sets_subject_to_context_id_and_none_member() {
        let finding =
            ValidationFinding::for_context("CTX.RULE", "warning", "ctx-1", "Missing entity");

        assert_eq!(finding.rule_id, "CTX.RULE");
        assert_eq!(finding.severity, "warning");
        assert_eq!(finding.message, "Missing entity");
        assert_eq!(finding.member, None);
        assert_eq!(finding.subject, Some("ctx-1".to_string()));
    }

    #[test]
    fn for_dimension_member_maps_dimension_to_member_and_member_to_subject() {
        let finding = ValidationFinding::for_dimension_member(
            "DIM.RULE",
            "error",
            "us-gaap:ScenarioAxis",
            "us-gaap:ActualMember",
            "Invalid member",
        );

        assert_eq!(finding.rule_id, "DIM.RULE");
        assert_eq!(finding.severity, "error");
        assert_eq!(finding.message, "Invalid member");
        assert_eq!(finding.member, Some("us-gaap:ScenarioAxis".to_string()));
        assert_eq!(finding.subject, Some("us-gaap:ActualMember".to_string()));
    }

    #[test]
    fn with_member_adds_member_to_new_finding() {
        let finding = ValidationFinding::new("RULE", "error", "msg").with_member("member-a");
        assert_eq!(finding.member, Some("member-a".to_string()));
        assert_eq!(finding.subject, None);
    }

    #[test]
    fn with_subject_adds_subject_to_new_finding() {
        let finding = ValidationFinding::new("RULE", "error", "msg").with_subject("ctx-1");
        assert_eq!(finding.member, None);
        assert_eq!(finding.subject, Some("ctx-1".to_string()));
    }

    #[test]
    fn with_member_and_subject_chain_on_new() {
        let finding = ValidationFinding::new("RULE", "error", "msg")
            .with_member("dim")
            .with_subject("subj");
        assert_eq!(finding.member, Some("dim".to_string()));
        assert_eq!(finding.subject, Some("subj".to_string()));
    }

    #[test]
    fn for_fact_with_empty_member_gives_some_empty_string() {
        let fact = Fact {
            concept: "us-gaap:Revenue".to_string(),
            context_ref: "ctx-1".to_string(),
            unit_ref: None,
            decimals: None,
            value: "1000".to_string(),
            member: String::new(),
        };

        let finding = ValidationFinding::for_fact("RULE", "error", &fact, "msg");

        // Empty member is still Some("") because it came from the fact field
        assert_eq!(finding.member, Some("".to_string()));
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
