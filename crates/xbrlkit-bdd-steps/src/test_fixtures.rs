//! Shared synthetic fixtures used by dimension-related BDD steps.

use taxonomy_dimensions::{Dimension, DimensionTaxonomy, Domain, DomainMember};
use xbrl_contexts::{Context, EntityIdentifier, Period};

/// Build the common explicit scenario taxonomy used by dimension steps.
pub(crate) fn scenario_taxonomy() -> DimensionTaxonomy {
    let mut taxonomy = DimensionTaxonomy::new();

    let mut scenario_domain = Domain::new("us-gaap:ScenarioDomain");
    scenario_domain.add_member(DomainMember {
        qname: "us-gaap:ScenarioActualMember".to_string(),
        parent: None,
        order: 1,
        label: None,
    });
    scenario_domain.add_member(DomainMember {
        qname: "us-gaap:ScenarioForecastMember".to_string(),
        parent: None,
        order: 2,
        label: None,
    });
    taxonomy.add_domain(scenario_domain);

    taxonomy.add_dimension(Dimension::Explicit {
        qname: "us-gaap:StatementScenarioAxis".to_string(),
        default_domain: Some("us-gaap:ScenarioDomain".to_string()),
        required: false,
    });
    taxonomy.dimension_domains.insert(
        "us-gaap:StatementScenarioAxis".to_string(),
        "us-gaap:ScenarioDomain".to_string(),
    );

    taxonomy
}

/// Build the common context used by dimensional validation steps.
pub(crate) fn test_context() -> Context {
    Context {
        id: "test-context".to_string(),
        entity: EntityIdentifier {
            scheme: "http://www.sec.gov/CIK".to_string(),
            value: "0001234567".to_string(),
        },
        period: Period::Duration {
            start: "2024-01-01".to_string(),
            end: "2024-12-31".to_string(),
        },
        entity_segment: None,
        scenario: None,
    }
}
