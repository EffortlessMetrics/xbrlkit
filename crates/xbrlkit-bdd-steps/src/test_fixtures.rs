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

#[cfg(test)]
mod tests {
    use super::{scenario_taxonomy, test_context};
    use taxonomy_dimensions::Dimension;

    #[test]
    fn scenario_taxonomy_contains_the_shared_axis_and_members() -> Result<(), String> {
        let taxonomy = scenario_taxonomy();
        let axis = taxonomy
            .dimensions
            .get("us-gaap:StatementScenarioAxis")
            .ok_or_else(|| "shared scenario axis is missing".to_string())?;
        match axis {
            Dimension::Explicit {
                default_domain,
                required,
                ..
            } if default_domain.as_deref() == Some("us-gaap:ScenarioDomain") && !required => {}
            _ => return Err("shared scenario axis has unexpected definition".to_string()),
        }

        let domain = taxonomy
            .domains
            .get("us-gaap:ScenarioDomain")
            .ok_or_else(|| "shared scenario domain is missing".to_string())?;
        if domain.members.len() != 2 {
            return Err(format!(
                "expected two shared scenario members, got {}",
                domain.members.len()
            ));
        }
        Ok(())
    }

    #[test]
    fn test_context_contains_the_shared_entity_and_period() -> Result<(), String> {
        let context = test_context();
        if context.id != "test-context" {
            return Err(format!("unexpected context id: {}", context.id));
        }
        if context.entity.value != "0001234567" {
            return Err(format!(
                "unexpected context entity: {}",
                context.entity.value
            ));
        }
        if !matches!(
            context.period,
            xbrl_contexts::Period::Duration {
                start,
                end
            } if start == "2024-01-01" && end == "2024-12-31"
        ) {
            return Err("shared context period is not the expected duration".to_string());
        }
        Ok(())
    }
}
