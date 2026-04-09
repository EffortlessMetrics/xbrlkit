//! Utility functions for BDD step execution.

use scenario_contract::{FeatureGrid, ScenarioRecord};
use taxonomy_dimensions::{Dimension, DimensionTaxonomy, Domain, DomainMember};

/// Create a synthetic taxonomy for testing when fixture files don't exist.
pub fn create_synthetic_taxonomy() -> DimensionTaxonomy {
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

    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:CustomerAxis".to_string(),
        value_type: "xs:string".to_string(),
        required: false,
    });

    taxonomy
}

/// Parse a count suffix from a step text.
/// Example: "the report contains 5 facts" -> Some(5)
pub fn parse_count_suffix(step: &str, prefix: &str, noun_stem: &str) -> Option<usize> {
    let remainder = step.strip_prefix(prefix)?;
    let count = remainder.split_whitespace().next()?.parse::<usize>().ok()?;
    let noun = remainder
        .split_whitespace()
        .nth(1)
        .unwrap_or_default()
        .trim_end_matches('s');
    if noun == noun_stem { Some(count) } else { None }
}

/// Select scenarios matching a selector (`scenario_id`, `ac_id`, `req_id`, or tag).
pub fn select_matching_scenarios(grid: &FeatureGrid, selector: &str) -> Vec<ScenarioRecord> {
    grid.scenarios
        .iter()
        .filter(|scenario| selector_matches(scenario, selector))
        .cloned()
        .collect()
}

/// Check if a scenario matches the given selector.
fn selector_matches(scenario: &ScenarioRecord, selector: &str) -> bool {
    scenario.scenario_id == selector
        || scenario.ac_id.as_deref() == Some(selector)
        || scenario.req_id.as_deref() == Some(selector)
        || format!("@{}", scenario.scenario_id) == selector
        || scenario
            .ac_id
            .as_ref()
            .is_some_and(|ac| format!("@{ac}") == selector)
}
