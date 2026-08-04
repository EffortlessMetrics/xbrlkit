//! IXDS assembly from one or more inline members.

use ixhtml_scan::scan_inline_fragments;
use xbrl_report_types::{CanonicalReport, Fact};

#[must_use]
pub fn assemble(members: &[(&str, &str)]) -> CanonicalReport {
    let member_names = members
        .iter()
        .map(|(name, _)| (*name).to_string())
        .collect::<Vec<_>>();
    let facts = members
        .iter()
        .flat_map(|(member, html)| {
            scan_inline_fragments(html)
                .into_iter()
                .filter(|fragment| {
                    matches!(
                        fragment.element_name.as_str(),
                        "ix:nonNumeric" | "ix:nonFraction"
                    )
                })
                .map(|fragment| Fact {
                    concept: fragment
                        .fact_name
                        .unwrap_or_else(|| fragment.element_name.clone()),
                    context_ref: fragment.context_ref.unwrap_or_default(),
                    unit_ref: fragment.unit_ref,
                    decimals: fragment.decimals,
                    value: fragment.value,
                    member: (*member).to_string(),
                })
                .collect::<Vec<_>>()
        })
        .collect();
    CanonicalReport {
        members: member_names,
        facts,
        findings: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::assemble;
    use xbrl_report_types::{CanonicalReport, Fact};

    fn require<T: PartialEq + std::fmt::Debug>(
        actual: &T,
        expected: &T,
        label: &str,
    ) -> Result<(), String> {
        if actual == expected {
            Ok(())
        } else {
            Err(format!("{label}: expected {expected:?}, got {actual:?}"))
        }
    }

    #[test]
    fn projects_supported_fragments_in_member_and_fact_order() -> Result<(), String> {
        let report = assemble(&[
            (
                "member-a",
                r#"<ix:nonNumeric name="dei:DocumentType" contextRef="ctx-a">10-K</ix:nonNumeric>"#,
            ),
            (
                "member-b",
                r#"<ix:nonFraction name="us-gaap:Assets" contextRef="ctx-b" unitRef="iso4217:USD" decimals="-3">100</ix:nonFraction>"#,
            ),
        ]);

        require(
            &report.members,
            &vec!["member-a".to_string(), "member-b".to_string()],
            "member order",
        )?;
        require(
            &report.facts,
            &vec![
                Fact {
                    concept: "dei:DocumentType".to_string(),
                    context_ref: "ctx-a".to_string(),
                    value: "10-K".to_string(),
                    member: "member-a".to_string(),
                    ..Fact::default()
                },
                Fact {
                    concept: "us-gaap:Assets".to_string(),
                    context_ref: "ctx-b".to_string(),
                    unit_ref: Some("iso4217:USD".to_string()),
                    decimals: Some("-3".to_string()),
                    value: "100".to_string(),
                    member: "member-b".to_string(),
                },
            ],
            "fact projection",
        )
    }

    #[test]
    fn filters_unsupported_fragments_and_falls_back_for_missing_names() -> Result<(), String> {
        let report = assemble(&[(
            "member-a",
            r#"<ix:tuple>ignored</ix:tuple><ix:nonNumeric contextRef="ctx-a">Some <b>value</b></ix:nonNumeric><ix:nonFraction name="us-gaap:Assets" unitRef="iso4217:USD">100</ix:nonFraction>"#,
        )]);

        require(
            &report.facts,
            &vec![
                Fact {
                    concept: "ix:nonNumeric".to_string(),
                    context_ref: "ctx-a".to_string(),
                    value: "Some value".to_string(),
                    member: "member-a".to_string(),
                    ..Fact::default()
                },
                Fact {
                    concept: "us-gaap:Assets".to_string(),
                    unit_ref: Some("iso4217:USD".to_string()),
                    value: "100".to_string(),
                    member: "member-a".to_string(),
                    ..Fact::default()
                },
            ],
            "supported fact filtering and fallback",
        )
    }

    #[test]
    fn empty_members_produce_an_empty_canonical_report() -> Result<(), String> {
        require(
            &assemble(&[]),
            &CanonicalReport::default(),
            "empty assembly",
        )
    }
}
