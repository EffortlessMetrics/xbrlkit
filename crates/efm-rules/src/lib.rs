//! SEC profile rule checks.

use ixhtml_scan::scan_inline_fragments;
use sec_profile_types::ProfilePack;
use taxonomy_dts::mixed_taxonomy_years;
use xbrl_report_types::ValidationFinding;

#[must_use]
pub fn validate_inline_restrictions(
    member: &str,
    html: &str,
    profile: &ProfilePack,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    let fragments = scan_inline_fragments(html);
    for fragment in fragments {
        if profile
            .inline_rules
            .banned_elements
            .iter()
            .any(|banned| banned == &fragment.element_name)
        {
            findings.push(ValidationFinding {
                rule_id: inline_element_rule_id(&fragment.element_name),
                severity: "error".to_string(),
                message: format!(
                    "{} is not allowed in the selected SEC profile pack",
                    fragment.element_name
                ),
                member: Some(member.to_string()),
                subject: Some(fragment.element_name.clone()),
            });
        }
        for attribute in fragment.attributes.keys() {
            if profile
                .inline_rules
                .banned_attributes
                .iter()
                .any(|banned| banned == attribute)
            {
                findings.push(ValidationFinding {
                    rule_id: inline_attribute_rule_id(attribute),
                    severity: "error".to_string(),
                    message: format!("{attribute} is not allowed in the selected SEC profile pack"),
                    member: Some(member.to_string()),
                    subject: Some(attribute.clone()),
                });
            }
        }
    }
    findings
}

#[must_use]
pub fn validate_taxonomy_years(
    entry_points: &[String],
    _profile: &ProfilePack,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    if mixed_taxonomy_years(entry_points) {
        findings.push(ValidationFinding {
            rule_id: "SEC.TAXONOMY.SAME_YEAR".to_string(),
            severity: "error".to_string(),
            message: "Taxonomy entry points must resolve to a single accepted taxonomy year"
                .to_string(),
            member: None,
            subject: Some(entry_points.join(", ")),
        });
    }
    findings
}

/// Validates that all required DEI facts are present in the report.
#[must_use]
pub fn validate_required_facts(
    facts: &[xbrl_report_types::Fact],
    profile: &ProfilePack,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();

    // Extract all concept names from facts
    let present_concepts: std::collections::HashSet<String> =
        facts.iter().map(|f| f.concept.clone()).collect();

    // Check each required fact
    for required in &profile.required_facts {
        if !present_concepts.contains(required) {
            findings.push(ValidationFinding {
                rule_id: format!("SEC.REQUIRED_FACT.{}", sanitize_for_rule_id(required)),
                severity: "error".to_string(),
                message: format!("Required fact '{required}' is missing"),
                member: None,
                subject: Some(required.clone()),
            });
        }
    }

    findings
}

fn inline_element_rule_id(element_name: &str) -> String {
    match element_name {
        "ix:fraction" => "SEC.INLINE.NO_IX_FRACTION".to_string(),
        "ix:tuple" => "SEC.INLINE.NO_IX_TUPLE".to_string(),
        _ => format!(
            "SEC.INLINE.BANNED_ELEMENT.{}",
            sanitize_for_rule_id(element_name)
        ),
    }
}

fn inline_attribute_rule_id(attribute: &str) -> String {
    match attribute {
        "xml:base" => "SEC.INLINE.NO_XML_BASE".to_string(),
        "target" => "SEC.INLINE.NO_TARGET".to_string(),
        _ => format!(
            "SEC.INLINE.BANNED_ATTRIBUTE.{}",
            sanitize_for_rule_id(attribute)
        ),
    }
}

fn sanitize_for_rule_id(value: &str) -> String {
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
    use super::{validate_inline_restrictions, validate_required_facts, validate_taxonomy_years};
    use sec_profile_types::{AcceptedTaxonomies, InlineRules, ProfilePack};
    use xbrl_report_types::Fact;

    fn profile() -> ProfilePack {
        ProfilePack {
            id: "sec/efm-77/opco".to_string(),
            label: "SEC EFM 77 operating companies".to_string(),
            forms: Vec::new(),
            enabled_rule_families: vec!["inline_restrictions".to_string()],
            inline_rules: InlineRules {
                banned_elements: vec!["ix:fraction".to_string()],
                banned_attributes: vec!["xml:base".to_string()],
            },
            accepted_taxonomies: AcceptedTaxonomies::default(),
            standard_taxonomy_uris: Vec::new(),
            required_facts: Vec::new(),
        }
    }

    fn profile_with_required_facts() -> ProfilePack {
        ProfilePack {
            id: "sec/efm-77/opco".to_string(),
            label: "SEC EFM 77 operating companies".to_string(),
            forms: Vec::new(),
            enabled_rule_families: vec!["required_facts".to_string()],
            inline_rules: InlineRules {
                banned_elements: Vec::new(),
                banned_attributes: Vec::new(),
            },
            accepted_taxonomies: AcceptedTaxonomies::default(),
            standard_taxonomy_uris: Vec::new(),
            required_facts: vec![
                "dei:EntityRegistrantName".to_string(),
                "dei:EntityCentralIndexKey".to_string(),
                "dei:DocumentType".to_string(),
            ],
        }
    }

    #[test]
    fn flags_banned_inline_elements_and_attributes() {
        let html = r#"<html><body><ix:fraction xml:base="https://example.com">1/2</ix:fraction></body></html>"#;
        let findings = validate_inline_restrictions("member-a.html", html, &profile());

        assert!(
            findings
                .iter()
                .any(|finding| finding.rule_id == "SEC.INLINE.NO_IX_FRACTION")
        );
        assert!(
            findings
                .iter()
                .any(|finding| finding.rule_id == "SEC.INLINE.NO_XML_BASE")
        );
    }

    #[test]
    fn flags_mixed_year_taxonomy_sets() {
        let entry_points = vec![
            "https://xbrl.sec.gov/dei/2024/dei-2024.xsd".to_string(),
            "https://xbrl.fasb.org/us-gaap/2025/elts/us-gaap-2025.xsd".to_string(),
        ];

        let findings = validate_taxonomy_years(&entry_points, &profile());

        assert!(
            findings
                .iter()
                .any(|finding| finding.rule_id == "SEC.TAXONOMY.SAME_YEAR")
        );
    }

    #[test]
    fn flags_missing_required_facts() {
        let facts = vec![
            Fact {
                concept: "dei:EntityRegistrantName".to_string(),
                value: "Example Corp".to_string(),
                unit_ref: None,
                context_ref: "ctx-1".to_string(),
                decimals: None,
                member: String::new(),
            },
            // Missing: dei:EntityCentralIndexKey, dei:DocumentType
        ];

        let findings = validate_required_facts(&facts, &profile_with_required_facts());

        assert!(
            findings
                .iter()
                .any(|f| f.rule_id == "SEC.REQUIRED_FACT.DEI_ENTITYCENTRALINDEXKEY")
        );
        assert!(
            findings
                .iter()
                .any(|f| f.rule_id == "SEC.REQUIRED_FACT.DEI_DOCUMENTTYPE")
        );
        assert!(
            !findings
                .iter()
                .any(|f| f.rule_id == "SEC.REQUIRED_FACT.DEI_ENTITYREGISTRANTNAME")
        );
    }

    #[test]
    fn passes_with_all_required_facts() {
        let facts = vec![
            Fact {
                concept: "dei:EntityRegistrantName".to_string(),
                value: "Example Corp".to_string(),
                unit_ref: None,
                context_ref: "ctx-1".to_string(),
                decimals: None,
                member: String::new(),
            },
            Fact {
                concept: "dei:EntityCentralIndexKey".to_string(),
                value: "0001234567".to_string(),
                unit_ref: None,
                context_ref: "ctx-1".to_string(),
                decimals: None,
                member: String::new(),
            },
            Fact {
                concept: "dei:DocumentType".to_string(),
                value: "10-K".to_string(),
                unit_ref: None,
                context_ref: "ctx-1".to_string(),
                decimals: None,
                member: String::new(),
            },
        ];

        let findings = validate_required_facts(&facts, &profile_with_required_facts());

        assert!(findings.is_empty());
    }
}
