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
