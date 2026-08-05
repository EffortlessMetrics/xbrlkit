use super::*;
use taxonomy_dimensions::{Dimension, Domain, DomainMember, Hypercube};
use xbrl_contexts::{Context, DimensionMember, EntityIdentifier, Period};
use xbrl_report_types::ValidationFinding;

fn create_test_taxonomy() -> DimensionTaxonomy {
    let mut taxonomy = DimensionTaxonomy::new();

    // Add explicit dimension
    taxonomy.add_dimension(Dimension::Explicit {
        qname: "us-gaap:StatementScenarioAxis".to_string(),
        default_domain: Some("us-gaap:StatementScenarioDomain".to_string()),
        required: true,
    });

    // Add domain with members
    let mut domain = Domain::new("us-gaap:StatementScenarioDomain");
    domain.add_member(DomainMember {
        qname: "us-gaap:ScenarioActualMember".to_string(),
        parent: None,
        order: 1,
        label: Some("Actual".to_string()),
    });
    domain.add_member(DomainMember {
        qname: "us-gaap:ScenarioBudgetMember".to_string(),
        parent: None,
        order: 2,
        label: Some("Budget".to_string()),
    });
    taxonomy.add_domain(domain);

    // Link dimension to domain
    taxonomy.link_dimension_domain(
        "us-gaap:StatementScenarioAxis",
        "us-gaap:StatementScenarioDomain",
    );

    // Add hypercube
    let mut hypercube = Hypercube::new("us-gaap:StatementTable");
    hypercube.add_dimension("us-gaap:StatementScenarioAxis", true);
    taxonomy.add_hypercube(hypercube);

    // Associate concept with hypercube
    taxonomy.associate_concept_hypercube("us-gaap:Revenue", "us-gaap:StatementTable", true);

    taxonomy
}

use xbrl_contexts::DimensionalContainer;

fn create_test_context_with_dims(id: &str, dims: Vec<(&str, &str)>) -> Context {
    Context {
        id: id.to_string(),
        entity: EntityIdentifier {
            scheme: "http://www.sec.gov/CIK".to_string(),
            value: "0000320193".to_string(),
        },
        entity_segment: None,
        period: Period::Instant("2024-12-31".to_string()),
        scenario: Some(DimensionalContainer {
            dimensions: dims
                .into_iter()
                .map(|(d, m)| DimensionMember {
                    dimension: d.to_string(),
                    member: m.to_string(),
                    is_typed: false,
                    typed_value: None,
                })
                .collect(),
            raw_xml: None,
        }),
    }
}

#[test]
fn test_validate_context_with_valid_dimensions() {
    let taxonomy = create_test_taxonomy();
    let context = create_test_context_with_dims(
        "ctx-1",
        vec![(
            "us-gaap:StatementScenarioAxis",
            "us-gaap:ScenarioActualMember",
        )],
    );

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        result.findings.is_empty(),
        "Expected no findings for valid dimensions"
    );
    assert_eq!(result.present_dimensions.len(), 1);
    assert!(result.missing_dimensions.is_empty());
}

#[test]
fn test_validate_context_with_missing_required_dimension() {
    let taxonomy = create_test_taxonomy();
    let context = create_test_context_with_dims("ctx-1", vec![]);

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        !result.findings.is_empty(),
        "Expected findings for missing dimension"
    );
    assert_eq!(result.missing_dimensions.len(), 1);
    assert_eq!(
        result.missing_dimensions[0],
        "us-gaap:StatementScenarioAxis"
    );
}

#[test]
fn test_validate_context_with_invalid_member() {
    let taxonomy = create_test_taxonomy();
    let context = create_test_context_with_dims(
        "ctx-1",
        vec![("us-gaap:StatementScenarioAxis", "us-gaap:InvalidMember")],
    );

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        !result.findings.is_empty(),
        "Expected findings for invalid member"
    );
    let has_invalid_member = result
        .findings
        .iter()
        .any(|f| f.rule_id == "XBRL.DIMENSION.INVALID_MEMBER");
    assert!(has_invalid_member, "Expected INVALID_MEMBER finding");
}

#[test]
fn test_summarize_results() {
    let results = vec![
        DimensionalValidationResult {
            context_id: "ctx-1".to_string(),
            findings: vec![ValidationFinding {
                rule_id: "TEST".to_string(),
                severity: "error".to_string(),
                message: "Test error".to_string(),
                member: None,
                subject: None,
            }],
            present_dimensions: vec![],
            missing_dimensions: vec!["dim1".to_string()],
        },
        DimensionalValidationResult {
            context_id: "ctx-2".to_string(),
            findings: vec![],
            present_dimensions: vec!["dim2".to_string()],
            missing_dimensions: vec![],
        },
    ];

    let summary = summarize_results(&results);

    assert_eq!(summary.total_contexts, 2);
    assert_eq!(summary.contexts_with_errors, 1);
    assert_eq!(summary.contexts_with_missing_dims, 1);
    assert_eq!(summary.total_findings, 1);
    assert!(summary.unique_missing_dimensions.contains("dim1"));
}

fn create_test_typed_taxonomy(value_type: &str) -> DimensionTaxonomy {
    let mut taxonomy = DimensionTaxonomy::new();

    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:TypedAxis".to_string(),
        value_type: value_type.to_string(),
        required: true,
    });

    taxonomy
}

fn create_test_context_with_typed_dim(id: &str, value: &str) -> Context {
    Context {
        id: id.to_string(),
        entity: EntityIdentifier {
            scheme: "http://www.sec.gov/CIK".to_string(),
            value: "0000320193".to_string(),
        },
        entity_segment: None,
        period: Period::Instant("2024-12-31".to_string()),
        scenario: Some(DimensionalContainer {
            dimensions: vec![DimensionMember {
                dimension: "dim:TypedAxis".to_string(),
                member: value.to_string(),
                is_typed: true,
                typed_value: Some(value.to_string()),
            }],
            raw_xml: None,
        }),
    }
}

#[test]
fn test_validate_typed_string_value() {
    let taxonomy = create_test_typed_taxonomy("xs:string");
    let context = create_test_context_with_typed_dim("ctx-1", "Any string value");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        result.findings.is_empty(),
        "Expected no findings for valid string value"
    );
}

#[test]
fn test_validate_typed_decimal_value_valid() {
    let taxonomy = create_test_typed_taxonomy("xs:decimal");
    let context = create_test_context_with_typed_dim("ctx-1", "123.45");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        result.findings.is_empty(),
        "Expected no findings for valid decimal value"
    );
}

#[test]
fn test_validate_typed_decimal_value_invalid() {
    let taxonomy = create_test_typed_taxonomy("xs:decimal");
    let context = create_test_context_with_typed_dim("ctx-1", "not-a-number");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        !result.findings.is_empty(),
        "Expected findings for invalid decimal value"
    );
    let has_invalid_value = result
        .findings
        .iter()
        .any(|f| f.rule_id == "XBRL.DIMENSION.INVALID_TYPED_VALUE");
    assert!(has_invalid_value, "Expected INVALID_TYPED_VALUE finding");
}

#[test]
fn test_validate_typed_integer_value_valid() {
    let taxonomy = create_test_typed_taxonomy("xs:integer");
    let context = create_test_context_with_typed_dim("ctx-1", "-42");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        result.findings.is_empty(),
        "Expected no findings for valid integer value"
    );
}

#[test]
fn test_validate_typed_integer_value_invalid() {
    let taxonomy = create_test_typed_taxonomy("xs:integer");
    let context = create_test_context_with_typed_dim("ctx-1", "3.14");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        !result.findings.is_empty(),
        "Expected findings for invalid integer value"
    );
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.rule_id == "XBRL.DIMENSION.INVALID_TYPED_VALUE")
    );
}

#[test]
fn test_validate_typed_date_value_valid() {
    let taxonomy = create_test_typed_taxonomy("xs:date");
    let context = create_test_context_with_typed_dim("ctx-1", "2024-03-15");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        result.findings.is_empty(),
        "Expected no findings for valid date value"
    );
}

#[test]
fn test_validate_typed_date_value_invalid() {
    let taxonomy = create_test_typed_taxonomy("xs:date");
    let context = create_test_context_with_typed_dim("ctx-1", "15-03-2024");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        !result.findings.is_empty(),
        "Expected findings for invalid date value"
    );
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.rule_id == "XBRL.DIMENSION.INVALID_TYPED_VALUE")
    );
}

#[test]
fn test_validate_typed_boolean_value_valid() {
    let taxonomy = create_test_typed_taxonomy("xs:boolean");

    for value in ["true", "false", "1", "0", "TRUE", "False"] {
        let context = create_test_context_with_typed_dim("ctx-1", value);
        let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);
        assert!(
            result.findings.is_empty(),
            "Expected no findings for valid boolean value '{value}'"
        );
    }
}

#[test]
fn test_validate_typed_boolean_value_invalid() {
    let taxonomy = create_test_typed_taxonomy("xs:boolean");
    let context = create_test_context_with_typed_dim("ctx-1", "yes");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        !result.findings.is_empty(),
        "Expected findings for invalid boolean value"
    );
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.rule_id == "XBRL.DIMENSION.INVALID_TYPED_VALUE")
    );
}

#[test]
fn test_validate_typed_empty_value() {
    let taxonomy = create_test_typed_taxonomy("xs:string");
    let context = create_test_context_with_typed_dim("ctx-1", "   ");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        !result.findings.is_empty(),
        "Expected findings for empty typed value"
    );
    assert!(
        result
            .findings
            .iter()
            .any(|f| f.rule_id == "XBRL.DIMENSION.EMPTY_TYPED_VALUE")
    );
}

#[test]
fn test_validate_typed_unknown_type() {
    // Unknown types should pass validation (extensibility)
    let taxonomy = create_test_typed_taxonomy("custom:CustomType");
    let context = create_test_context_with_typed_dim("ctx-1", "any value");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        result.findings.is_empty(),
        "Expected no findings for unknown type (extensibility)"
    );
}

#[test]
fn test_validate_typed_datetime_value_valid() {
    let taxonomy = create_test_typed_taxonomy("xs:dateTime");
    let context = create_test_context_with_typed_dim("ctx-1", "2024-03-15T10:30:00");

    let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);

    assert!(
        result.findings.is_empty(),
        "Expected no findings for valid datetime value"
    );
}

#[test]
fn test_validate_typed_uri_value_valid() {
    let taxonomy = create_test_typed_taxonomy("xs:anyURI");

    for uri in ["http://example.com", "https://test.org/path", "/local/path"] {
        let context = create_test_context_with_typed_dim("ctx-1", uri);
        let result = validate_context_dimensions(&context, "us-gaap:Revenue", &taxonomy);
        assert!(
            result.findings.is_empty(),
            "Expected no findings for valid URI '{uri}'"
        );
    }
}
