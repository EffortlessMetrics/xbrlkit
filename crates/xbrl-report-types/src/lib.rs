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
    use std::fmt::Debug;

    fn check_equal<T: Debug + PartialEq>(
        actual: &T,
        expected: &T,
        field: &str,
    ) -> Result<(), String> {
        if actual != expected {
            return Err(format!("{field}: expected {expected:?}, got {actual:?}"));
        }

        Ok(())
    }

    #[test]
    fn new_sets_rule_id_severity_message_and_none_fields() -> Result<(), String> {
        let finding = ValidationFinding::new("RULE-1", "error", "Something went wrong");

        check_equal(&finding.rule_id, &String::from("RULE-1"), "rule_id")?;
        check_equal(&finding.severity, &String::from("error"), "severity")?;
        check_equal(
            &finding.message,
            &String::from("Something went wrong"),
            "message",
        )?;
        check_equal(&finding.member, &None, "member")?;
        check_equal(&finding.subject, &None, "subject")?;

        Ok(())
    }

    #[test]
    fn for_fact_populates_member_and_subject_from_fact() -> Result<(), String> {
        let fact = Fact {
            concept: "us-gaap:Revenue".to_string(),
            context_ref: "ctx-1".to_string(),
            unit_ref: None,
            decimals: None,
            value: "1000".to_string(),
            member: "member-a".to_string(),
        };

        let finding = ValidationFinding::for_fact("SEC.RULE", "error", &fact, "Negative value");

        check_equal(&finding.rule_id, &String::from("SEC.RULE"), "rule_id")?;
        check_equal(&finding.severity, &String::from("error"), "severity")?;
        check_equal(&finding.message, &String::from("Negative value"), "message")?;
        check_equal(&finding.member, &Some(String::from("member-a")), "member")?;
        check_equal(
            &finding.subject,
            &Some(String::from("us-gaap:Revenue")),
            "subject",
        )?;

        Ok(())
    }

    #[test]
    fn for_context_sets_subject_to_context_id_and_none_member() -> Result<(), String> {
        let finding =
            ValidationFinding::for_context("CTX.RULE", "warning", "ctx-1", "Missing entity");

        check_equal(&finding.rule_id, &String::from("CTX.RULE"), "rule_id")?;
        check_equal(&finding.severity, &String::from("warning"), "severity")?;
        check_equal(&finding.message, &String::from("Missing entity"), "message")?;
        check_equal(&finding.member, &None, "member")?;
        check_equal(&finding.subject, &Some(String::from("ctx-1")), "subject")?;

        Ok(())
    }

    #[test]
    fn for_dimension_member_maps_dimension_to_member_and_member_to_subject() -> Result<(), String> {
        let finding = ValidationFinding::for_dimension_member(
            "DIM.RULE",
            "error",
            "us-gaap:ScenarioAxis",
            "us-gaap:ActualMember",
            "Invalid member",
        );

        check_equal(&finding.rule_id, &String::from("DIM.RULE"), "rule_id")?;
        check_equal(&finding.severity, &String::from("error"), "severity")?;
        check_equal(&finding.message, &String::from("Invalid member"), "message")?;
        check_equal(
            &finding.member,
            &Some(String::from("us-gaap:ScenarioAxis")),
            "member",
        )?;
        check_equal(
            &finding.subject,
            &Some(String::from("us-gaap:ActualMember")),
            "subject",
        )?;

        Ok(())
    }

    #[test]
    fn with_member_adds_member_to_new_finding() -> Result<(), String> {
        let finding = ValidationFinding::new("RULE", "error", "msg").with_member("member-a");

        check_equal(&finding.member, &Some(String::from("member-a")), "member")?;
        check_equal(&finding.subject, &None, "subject")?;

        Ok(())
    }

    #[test]
    fn with_subject_adds_subject_to_new_finding() -> Result<(), String> {
        let finding = ValidationFinding::new("RULE", "error", "msg").with_subject("ctx-1");

        check_equal(&finding.member, &None, "member")?;
        check_equal(&finding.subject, &Some(String::from("ctx-1")), "subject")?;

        Ok(())
    }

    #[test]
    fn with_member_and_subject_chain_on_new() -> Result<(), String> {
        let finding = ValidationFinding::new("RULE", "error", "msg")
            .with_member("dim")
            .with_subject("subj");

        check_equal(&finding.member, &Some(String::from("dim")), "member")?;
        check_equal(&finding.subject, &Some(String::from("subj")), "subject")?;

        Ok(())
    }

    #[test]
    fn for_fact_with_empty_member_gives_some_empty_string() -> Result<(), String> {
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
        check_equal(&finding.member, &Some(String::new()), "member")?;

        Ok(())
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
