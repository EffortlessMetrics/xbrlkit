//! Filing diff use case.

use receipt_types::{Receipt, RunResult};
use xbrl_report_types::CanonicalReport;

#[must_use]
pub fn diff_reports(left: &CanonicalReport, right: &CanonicalReport) -> (Vec<String>, Receipt) {
    let left_set = left
        .facts
        .iter()
        .map(|fact| format!("{}:{}:{}", fact.concept, fact.context_ref, fact.value))
        .collect::<std::collections::BTreeSet<_>>();
    let right_set = right
        .facts
        .iter()
        .map(|fact| format!("{}:{}:{}", fact.concept, fact.context_ref, fact.value))
        .collect::<std::collections::BTreeSet<_>>();
    let changes = left_set
        .symmetric_difference(&right_set)
        .cloned()
        .collect::<Vec<_>>();
    let receipt = Receipt::new(
        "diff.report",
        "canonical-report",
        if changes.is_empty() {
            RunResult::Success
        } else {
            RunResult::Warning
        },
    );
    (changes, receipt)
}

#[cfg(test)]
mod tests {
    use super::*;
    use xbrl_report_types::Fact;

    fn fact(concept: &str, context_ref: &str, value: &str) -> Fact {
        Fact {
            concept: concept.to_string(),
            context_ref: context_ref.to_string(),
            value: value.to_string(),
            ..Fact::default()
        }
    }

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
    fn identical_reports_return_no_changes_and_success_receipt() -> Result<(), String> {
        let report = CanonicalReport {
            facts: vec![fact("us-gaap:Assets", "ctx-1", "100")],
            ..CanonicalReport::default()
        };

        let (changes, receipt) = diff_reports(&report, &report);

        require(&changes, &Vec::<String>::new(), "changes")?;
        require(&receipt.kind, &"diff.report".to_string(), "receipt kind")?;
        require(
            &receipt.result,
            &RunResult::Success,
            "receipt result for identical reports",
        )
    }

    #[test]
    fn changed_fact_returns_sorted_symmetric_difference_and_warning_receipt() -> Result<(), String>
    {
        let left = CanonicalReport {
            facts: vec![
                fact("us-gaap:Assets", "ctx-1", "100"),
                fact("us-gaap:Liabilities", "ctx-1", "40"),
            ],
            ..CanonicalReport::default()
        };
        let right = CanonicalReport {
            facts: vec![
                fact("us-gaap:Assets", "ctx-1", "120"),
                fact("us-gaap:Equity", "ctx-1", "40"),
            ],
            ..CanonicalReport::default()
        };

        let (changes, receipt) = diff_reports(&left, &right);

        require(
            &changes,
            &vec![
                "us-gaap:Assets:ctx-1:100".to_string(),
                "us-gaap:Assets:ctx-1:120".to_string(),
                "us-gaap:Equity:ctx-1:40".to_string(),
                "us-gaap:Liabilities:ctx-1:40".to_string(),
            ],
            "sorted changes",
        )?;
        require(
            &receipt.result,
            &RunResult::Warning,
            "receipt result for changed reports",
        )
    }

    #[test]
    fn comparison_identity_uses_concept_context_and_value_only() -> Result<(), String> {
        let left = CanonicalReport {
            facts: vec![Fact {
                concept: "us-gaap:Assets".to_string(),
                context_ref: "ctx-1".to_string(),
                unit_ref: Some("iso4217:USD".to_string()),
                decimals: Some("-3".to_string()),
                value: "100".to_string(),
                member: "member-a".to_string(),
            }],
            ..CanonicalReport::default()
        };
        let right = CanonicalReport {
            facts: vec![Fact {
                concept: "us-gaap:Assets".to_string(),
                context_ref: "ctx-1".to_string(),
                unit_ref: Some("iso4217:EUR".to_string()),
                decimals: Some("0".to_string()),
                value: "100".to_string(),
                member: "member-b".to_string(),
            }],
            ..CanonicalReport::default()
        };

        let (changes, receipt) = diff_reports(&left, &right);

        require(&changes, &Vec::<String>::new(), "changes")?;
        require(
            &receipt.result,
            &RunResult::Success,
            "receipt result for equal comparison keys",
        )
    }
}
