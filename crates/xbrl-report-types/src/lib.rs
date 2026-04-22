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
    /// Create an error-level finding.
    ///
    /// # Example
    /// ```
    /// use xbrl_report_types::ValidationFinding;
    ///
    /// let finding = ValidationFinding::error("RULE.001", "Something went wrong");
    /// assert_eq!(finding.severity, "error");
    /// ```
    pub fn error(rule_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: "error".to_string(),
            message: message.into(),
            member: None,
            subject: None,
        }
    }

    /// Create an info-level finding.
    pub fn info(rule_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: "info".to_string(),
            message: message.into(),
            member: None,
            subject: None,
        }
    }

    /// Create a warning-level finding.
    pub fn warning(rule_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: "warning".to_string(),
            message: message.into(),
            member: None,
            subject: None,
        }
    }

    /// Set the member field.
    pub fn with_member(mut self, member: impl Into<String>) -> Self {
        self.member = Some(member.into());
        self
    }

    /// Set the subject field.
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Derive member and subject from a Fact.
    pub fn for_fact(mut self, fact: &Fact) -> Self {
        self.member = Some(fact.member.clone());
        self.subject = Some(fact.concept.clone());
        self
    }
}

/// Sanitize a concept/identifier for use in a rule ID.
///
    /// Non-alphanumeric characters are replaced with `_`, and alphabetic
    /// characters are uppercased.
    ///
    /// # Example
    /// ```
    /// use xbrl_report_types::sanitize_for_rule_id;
    ///
    /// assert_eq!(sanitize_for_rule_id("us-gaap:Revenue"), "US_GAAP_REVENUE");
    /// ```
    pub fn sanitize_for_rule_id(value: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_error_sets_severity() {
        let f = ValidationFinding::error("RULE.001", "msg");
        assert_eq!(f.rule_id, "RULE.001");
        assert_eq!(f.severity, "error");
        assert_eq!(f.message, "msg");
        assert_eq!(f.member, None);
        assert_eq!(f.subject, None);
    }

    #[test]
    fn builder_info_sets_severity() {
        let f = ValidationFinding::info("RULE.002", "msg");
        assert_eq!(f.severity, "info");
    }

    #[test]
    fn builder_warning_sets_severity() {
        let f = ValidationFinding::warning("RULE.003", "msg");
        assert_eq!(f.severity, "warning");
    }

    #[test]
    fn builder_chaining() {
        let f = ValidationFinding::error("RULE.004", "msg")
            .with_member("member-a")
            .with_subject("subject-b");
        assert_eq!(f.member, Some("member-a".to_string()));
        assert_eq!(f.subject, Some("subject-b".to_string()));
    }

    #[test]
    fn builder_for_fact() {
        let fact = Fact {
            concept: "us-gaap:Revenue".to_string(),
            context_ref: "ctx-1".to_string(),
            unit_ref: None,
            decimals: None,
            value: "1000".to_string(),
            member: "member-x".to_string(),
        };
        let f = ValidationFinding::error("RULE.005", "msg").for_fact(&fact);
        assert_eq!(f.member, Some("member-x".to_string()));
        assert_eq!(f.subject, Some("us-gaap:Revenue".to_string()));
    }

    #[test]
    fn builder_chaining_with_for_fact() {
        let fact = Fact {
            concept: "us-gaap:Revenue".to_string(),
            context_ref: "ctx-1".to_string(),
            unit_ref: None,
            decimals: None,
            value: "1000".to_string(),
            member: "member-x".to_string(),
        };
        let f = ValidationFinding::error("RULE.006", "msg")
            .with_member("overridden")
            .for_fact(&fact);
        // for_fact should override with_member
        assert_eq!(f.member, Some("member-x".to_string()));
        assert_eq!(f.subject, Some("us-gaap:Revenue".to_string()));
    }

    #[test]
    fn sanitize_replaces_non_alphanumeric() {
        assert_eq!(sanitize_for_rule_id("us-gaap:Revenue"), "US_GAAP_REVENUE");
        assert_eq!(sanitize_for_rule_id("a-b.c"), "A_B_C");
        assert_eq!(sanitize_for_rule_id("123"), "123");
        assert_eq!(sanitize_for_rule_id(""), "");
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
