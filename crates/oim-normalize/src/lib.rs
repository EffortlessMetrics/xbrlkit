//! OIM-aligned export helpers.

use serde_json::json;
use xbrl_report_types::CanonicalReport;

#[must_use]
pub fn to_json_value(report: &CanonicalReport) -> serde_json::Value {
    json!({
      "members": report.members,
      "facts": report.facts,
      "findings": report.findings,
    })
}

#[cfg(test)]
mod tests {
    use super::to_json_value;
    use serde_json::json;
    use xbrl_report_types::{CanonicalReport, Fact, ValidationFinding};

    #[test]
    fn projects_populated_report_fields_and_nested_values() -> Result<(), String> {
        let report = CanonicalReport {
            members: vec!["member-a".to_string()],
            facts: vec![Fact {
                concept: "us-gaap:Revenue".to_string(),
                context_ref: "ctx-1".to_string(),
                unit_ref: Some("iso4217:USD".to_string()),
                decimals: Some("-3".to_string()),
                value: "1000".to_string(),
                member: "member-a".to_string(),
            }],
            findings: vec![ValidationFinding {
                rule_id: "RULE-1".to_string(),
                severity: "error".to_string(),
                message: "invalid value".to_string(),
                member: None,
                subject: None,
            }],
        };
        let actual = to_json_value(&report);
        let expected = json!({
            "members": ["member-a"],
            "facts": [{
                "concept": "us-gaap:Revenue",
                "context_ref": "ctx-1",
                "unit_ref": "iso4217:USD",
                "decimals": "-3",
                "value": "1000",
                "member": "member-a"
            }],
            "findings": [{
                "rule_id": "RULE-1",
                "severity": "error",
                "message": "invalid value",
                "member": null,
                "subject": null
            }]
        });

        if actual != expected {
            return Err(format!("unexpected populated report projection: {actual}"));
        }

        Ok(())
    }

    #[test]
    fn projects_empty_collections_as_empty_arrays() -> Result<(), String> {
        let actual = to_json_value(&CanonicalReport::default());
        let expected = json!({
            "members": [],
            "facts": [],
            "findings": []
        });

        if actual != expected {
            return Err(format!("unexpected empty report projection: {actual}"));
        }

        Ok(())
    }
}
