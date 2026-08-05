use xbrl_report_types::ValidationFinding;

pub(super) fn missing_context(context_ref: &str, concept: &str) -> ValidationFinding {
    ValidationFinding {
        rule_id: "XBRL.DIMENSION.MISSING_CONTEXT".to_string(),
        severity: "error".to_string(),
        message: format!("Context {context_ref} not found for fact {concept}"),
        member: Some(concept.to_string()),
        subject: Some(context_ref.to_string()),
    }
}

pub(super) fn missing_required_dimension(
    concept_qname: &str,
    required_dimension: &str,
    context_id: &str,
) -> ValidationFinding {
    ValidationFinding {
        rule_id: "XBRL.DIMENSION.MISSING_REQUIRED".to_string(),
        severity: "error".to_string(),
        message: format!(
            "Concept {concept_qname} requires dimension {required_dimension} which is missing in context {context_id}"
        ),
        member: Some(concept_qname.to_string()),
        subject: Some(context_id.to_string()),
    }
}

pub(super) fn unknown_dimension(dimension: &str, member: &str) -> ValidationFinding {
    ValidationFinding {
        rule_id: "XBRL.DIMENSION.UNKNOWN".to_string(),
        severity: "error".to_string(),
        message: format!("Unknown dimension: {dimension}"),
        member: Some(dimension.to_string()),
        subject: Some(member.to_string()),
    }
}

pub(super) fn invalid_member(
    member: &str,
    dimension: &str,
    domain_qname: &str,
) -> ValidationFinding {
    ValidationFinding {
        rule_id: "XBRL.DIMENSION.INVALID_MEMBER".to_string(),
        severity: "error".to_string(),
        message: format!(
            "Member {member} is not valid for dimension {dimension} in domain {domain_qname}"
        ),
        member: Some(member.to_string()),
        subject: Some(dimension.to_string()),
    }
}

pub(super) fn no_domain(dimension: &str, member: &str) -> ValidationFinding {
    ValidationFinding {
        rule_id: "XBRL.DIMENSION.NO_DOMAIN".to_string(),
        severity: "error".to_string(),
        message: format!("Dimension {dimension} has no domain defined"),
        member: Some(dimension.to_string()),
        subject: Some(member.to_string()),
    }
}
