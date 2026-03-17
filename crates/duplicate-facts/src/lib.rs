//! Duplicate fact policy.

use std::collections::{BTreeMap, BTreeSet};
use xbrl_report_types::CanonicalReport;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DuplicateDisposition {
    None,
    Consistent,
    Inconsistent,
}

#[must_use]
pub fn classify(report: &CanonicalReport) -> DuplicateDisposition {
    let mut values = BTreeMap::<(String, String, Option<String>), BTreeSet<String>>::new();
    for fact in &report.facts {
        values
            .entry((
                fact.concept.clone(),
                fact.context_ref.clone(),
                fact.unit_ref.clone(),
            ))
            .or_default()
            .insert(fact.value.clone());
    }

    let mut saw_duplicate = false;
    for ((concept, context_ref, unit_ref), distinct_values) in &values {
        let matching_facts = report
            .facts
            .iter()
            .filter(|fact| {
                fact.concept == *concept
                    && fact.context_ref == *context_ref
                    && fact.unit_ref == *unit_ref
            })
            .count();

        if matching_facts > 1 {
            saw_duplicate = true;
            if distinct_values.len() > 1 {
                return DuplicateDisposition::Inconsistent;
            }
        }
    }

    if saw_duplicate {
        DuplicateDisposition::Consistent
    } else {
        DuplicateDisposition::None
    }
}

#[cfg(test)]
mod tests {
    use super::{DuplicateDisposition, classify};
    use xbrl_report_types::{CanonicalReport, Fact};

    fn fact(value: &str) -> Fact {
        Fact {
            concept: "us-gaap:Assets".to_string(),
            context_ref: "c1".to_string(),
            unit_ref: Some("u1".to_string()),
            decimals: None,
            value: value.to_string(),
            member: "member-a.html".to_string(),
        }
    }

    #[test]
    fn classify_returns_none_when_no_duplicates_exist() {
        let report = CanonicalReport {
            members: vec!["member-a.html".to_string()],
            facts: vec![fact("100")],
            findings: Vec::new(),
        };

        assert_eq!(classify(&report), DuplicateDisposition::None);
    }

    #[test]
    fn classify_returns_consistent_when_duplicate_values_match() {
        let report = CanonicalReport {
            members: vec!["member-a.html".to_string()],
            facts: vec![fact("100"), fact("100")],
            findings: Vec::new(),
        };

        assert_eq!(classify(&report), DuplicateDisposition::Consistent);
    }

    #[test]
    fn classify_returns_inconsistent_when_duplicate_values_differ() {
        let report = CanonicalReport {
            members: vec!["member-a.html".to_string()],
            facts: vec![fact("100"), fact("101")],
            findings: Vec::new(),
        };

        assert_eq!(classify(&report), DuplicateDisposition::Inconsistent);
    }
}
