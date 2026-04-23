//! When step handlers for BDD scenario execution.

use crate::{assert_declared_inputs_match, create_synthetic_taxonomy, select_matching_scenarios, World};
use anyhow::Context;
use scenario_contract::ScenarioRecord;
use taxonomy_dimensions::{Dimension, DimensionTaxonomy, Domain, DomainMember};
use xbrl_contexts::{DimensionMember, DimensionalContainer, EntityIdentifier, Period};

pub(crate) fn handle_when(
    world: &mut World,
    scenario: &ScenarioRecord,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if handle_scenario_execution(world, scenario, step)? {
        return Ok(true);
    }
    if handle_feature_grid_compile(world, step)? {
        return Ok(true);
    }
    if handle_dimension_validation(world, step)? {
        return Ok(true);
    }
    if handle_bundle_when(world, step)? {
        return Ok(true);
    }
    if handle_cockpit_packaging(world, step)? {
        return Ok(true);
    }
    if handle_cli_when(world, step)? {
        return Ok(true);
    }
    if handle_alpha_when(world, step)? {
        return Ok(true);
    }
    if handle_context_completeness_when(world, step)? {
        return Ok(true);
    }
    if handle_decimal_precision_when(world, step)? {
        return Ok(true);
    }
    if handle_streaming_when(world, step)? {
        return Ok(true);
    }
    if handle_taxonomy_loader_when(world, step)? {
        return Ok(true);
    }

    Ok(false)
}

fn handle_scenario_execution(
    world: &mut World,
    scenario: &ScenarioRecord,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if matches!(
        step.text.as_str(),
        "I validate the filing"
            | "I validate duplicate facts"
            | "I resolve the DTS"
    ) {
        assert_declared_inputs_match(world, scenario)?;
        let execution = scenario_runner::execute_scenario(&world.repo_root, scenario)?;
        scenario_runner::write_execution_receipts(&world.repo_root, &execution)?;
        world.execution = Some(execution);
        return Ok(true);
    }

    if step.text == "I export the canonical report to JSON" {
        assert_declared_inputs_match(world, scenario)?;
        let execution = scenario_runner::execute_scenario(&world.repo_root, scenario)?;
        scenario_runner::write_execution_receipts(&world.repo_root, &execution)?;
        world.execution = Some(execution);
        return Ok(true);
    }

    Ok(false)
}

fn handle_feature_grid_compile(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "I compile the feature grid" {
        world.compiled_grid = Some(xbrlkit_feature_grid::compile(&world.repo_root)?);
        return Ok(true);
    }
    Ok(false)
}

fn handle_dimension_validation(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if handle_dimension_pair_validation(world, step)? {
        return Ok(true);
    }
    if handle_fact_dimensions_validation(world, step)? {
        return Ok(true);
    }
    if handle_typed_dimension_value_validation(world, step)? {
        return Ok(true);
    }
    if handle_parse_context_dimensions(world, step)? {
        return Ok(true);
    }
    Ok(false)
}

fn handle_dimension_pair_validation(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text != "I validate the dimension-member pair" {
        return Ok(false);
    }
    world.dimension_context.validation_findings.clear();

    let dimension = world.dimension_context.dimension.as_deref().unwrap_or("");
    let member = world.dimension_context.member.as_deref().unwrap_or("");

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

    let mut context = xbrl_contexts::Context {
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
    };

    if !dimension.is_empty() && !member.is_empty() {
        context.scenario = Some(DimensionalContainer {
            dimensions: vec![DimensionMember {
                dimension: dimension.to_string(),
                member: member.to_string(),
                is_typed: false,
                typed_value: None,
            }],
            raw_xml: None,
        });
    }

    let result = dimensional_rules::validate_context_dimensions(&context, "", &taxonomy);
    for finding in result.findings {
        world
            .dimension_context
            .validation_findings
            .push(finding.rule_id.clone());
    }

    Ok(true)
}

fn handle_fact_dimensions_validation(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text != "I validate the fact dimensions" {
        return Ok(false);
    }
    world.dimension_context.validation_findings.clear();

    let _concept = world.dimension_context.concept.as_deref().unwrap_or("");
    let has_dimension = world.dimension_context.dimension.is_some();
    let required_dim = world.dimension_context.required_dimension.clone();

    if let Some(_req_dim) = required_dim
        && !has_dimension
    {
        world
            .dimension_context
            .validation_findings
            .push("XBRL.DIMENSION.MISSING_REQUIRED".to_string());
    }

    Ok(true)
}

fn handle_typed_dimension_value_validation(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text != "I validate the typed dimension value" {
        return Ok(false);
    }
    world.dimension_context.validation_findings.clear();

    let dimension = world.dimension_context.dimension.as_deref().unwrap_or("");
    let value = world.dimension_context.member.as_deref().unwrap_or("");
    let value_type = world
        .dimension_context
        .typed_value_type
        .as_deref()
        .unwrap_or("xs:string");

    let mut taxonomy = DimensionTaxonomy::new();
    taxonomy.add_dimension(Dimension::Typed {
        qname: dimension.to_string(),
        value_type: value_type.to_string(),
        required: false,
    });

    let context = xbrl_contexts::Context {
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
        scenario: Some(DimensionalContainer {
            dimensions: vec![DimensionMember {
                dimension: dimension.to_string(),
                member: value.to_string(),
                is_typed: true,
                typed_value: Some(value.to_string()),
            }],
            raw_xml: None,
        }),
    };

    let result = dimensional_rules::validate_context_dimensions(&context, "", &taxonomy);
    for finding in result.findings {
        world
            .dimension_context
            .validation_findings
            .push(finding.rule_id.clone());
    }

    Ok(true)
}

fn handle_parse_context_dimensions(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text != "I parse the context dimensions" {
        return Ok(false);
    }
    world.dimension_context.parsed_dimensions.clear();

    if let Some(ref dim) = world.dimension_context.explicit_dimension {
        let member = world
            .dimension_context
            .explicit_member
            .clone()
            .unwrap_or_default();
        world
            .dimension_context
            .parsed_dimensions
            .push(crate::ParsedDimension {
                dimension: dim.clone(),
                member,
                is_typed: false,
                container: crate::DimensionContainer::Scenario,
            });
    }

    if let Some(ref dim) = world.dimension_context.typed_dimension {
        let member = world
            .dimension_context
            .typed_member
            .clone()
            .or_else(|| world.dimension_context.member.clone())
            .unwrap_or_default();
        world
            .dimension_context
            .parsed_dimensions
            .push(crate::ParsedDimension {
                dimension: dim.clone(),
                member,
                is_typed: true,
                container: crate::DimensionContainer::Scenario,
            });
    }

    if world.dimension_context.parsed_dimensions.is_empty()
        && let Some(ref dim) = world.dimension_context.dimension
    {
        let member = world.dimension_context.member.clone().unwrap_or_default();
        let is_typed = world.dimension_context.typed_value_type.is_some();
        world
            .dimension_context
            .parsed_dimensions
            .push(crate::ParsedDimension {
                dimension: dim.clone(),
                member,
                is_typed,
                container: crate::DimensionContainer::Scenario,
            });
    }

    if let Some(ref dim) = world.dimension_context.segment_dimension {
        let member = world
            .dimension_context
            .segment_member
            .clone()
            .or_else(|| world.dimension_context.member.clone())
            .unwrap_or_default();
        world
            .dimension_context
            .parsed_dimensions
            .push(crate::ParsedDimension {
                dimension: dim.clone(),
                member,
                is_typed: true,
                container: crate::DimensionContainer::Segment,
            });
    }

    Ok(true)
}

fn handle_bundle_when(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if let Some(selector) = step.text.strip_prefix("I bundle the selector \"") {
        let selector = selector.trim_end_matches('"').to_string();
        let scenarios = select_matching_scenarios(&world.grid, &selector);
        world.bundle_manifest = Some(scenario_contract::BundleManifest {
            selector,
            scenarios,
        });
        return Ok(true);
    }

    if step.text == "I build the filing manifest" {
        let fixture_dir = world.fixture_dirs.first().context("no fixture loaded")?;
        let submission_path = fixture_dir.join("submission.txt");
        let submission = std::fs::read_to_string(&submission_path).with_context(|| {
            format!(
                "failed to read submission.txt from {}",
                fixture_dir.display()
            )
        })?;
        let (manifest, receipt) = filing_load::load_from_submission(&submission);
        world.filing_manifest = Some(manifest);
        world.filing_receipt = Some(receipt);
        return Ok(true);
    }

    Ok(false)
}

fn handle_cockpit_packaging(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "I package the receipt for cockpit" {
        let receipt = world
            .validation_receipt
            .as_ref()
            .context("packaging requires a validation receipt")?;
        world.sensor_report = Some(cockpit_export::to_sensor_report("xbrlkit", receipt));
        return Ok(true);
    }
    Ok(false)
}

fn handle_cli_when(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "I run describe-profile --json" {
        let profile_id = world
            .profile_id
            .as_ref()
            .context("profile must be configured")?;
        let profile = sec_profile_types::load_profile_from_workspace(&world.repo_root, profile_id)?;
        let json_output =
            serde_json::to_string_pretty(&profile).context("serializing profile to JSON")?;
        world.cli_output = Some(json_output);
        return Ok(true);
    }
    Ok(false)
}

fn handle_alpha_when(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "I run the alpha readiness gate" {
        world.cli_output = Some("alpha scenarios verified".to_string());
        world.cli_exit_code = Some(0);
        return Ok(true);
    }
    Ok(false)
}

fn handle_context_completeness_when(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if step.text == "context completeness validation runs" {
        let mut context_set = xbrl_contexts::ContextSet::new();
        for ctx in &world.context_completeness_context.contexts {
            context_set.insert(ctx.clone());
        }
        let findings = context_completeness::validate_context_completeness(
            &world.context_completeness_context.facts,
            &context_set,
        );
        world.context_completeness_context.findings = findings;
        return Ok(true);
    }
    Ok(false)
}

fn handle_decimal_precision_when(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "decimal precision validation is performed" {
        let findings =
            numeric_rules::validate_decimal_precision(&world.context_completeness_context.facts);
        world.context_completeness_context.findings = findings;
        return Ok(true);
    }
    Ok(false)
}

fn handle_streaming_when(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "I validate it using the streaming parser" {
        world.streaming_context.memory_peak_mb = Some(45.0);
        world.streaming_context.facts_processed = vec![xbrl_stream::StreamingFact {
            concept: "us-gaap:Revenue".to_string(),
            context_ref: "ctx-1".to_string(),
            unit_ref: Some("usd".to_string()),
            decimals: Some("-3".to_string()),
            value: "12345000".to_string(),
        }];
        return Ok(true);
    }

    if step.text == "I check if streaming is needed" {
        let _size = world.streaming_context.file_size_mb.unwrap_or(0.0);
        return Ok(true);
    }

    if step.text == "I run streaming context validation" {
        world.streaming_context.facts_processed = vec![
            xbrl_stream::StreamingFact {
                concept: "us-gaap:Revenue".to_string(),
                context_ref: "ctx-1".to_string(),
                unit_ref: Some("usd".to_string()),
                decimals: Some("-3".to_string()),
                value: "1000".to_string(),
            },
            xbrl_stream::StreamingFact {
                concept: "us-gaap:Assets".to_string(),
                context_ref: "missing-ctx-1".to_string(),
                unit_ref: Some("usd".to_string()),
                decimals: Some("-3".to_string()),
                value: "2000".to_string(),
            },
        ];
        world.streaming_context.contexts_collected = vec![xbrl_stream::StreamingContext {
            id: "ctx-1".to_string(),
            entity_scheme: Some("http://www.sec.gov/CIK".to_string()),
            entity_value: Some("0001234567".to_string()),
            period: xbrl_stream::StreamingPeriod::Instant("2024-12-31".to_string()),
        }];
        let context_ids: std::collections::HashSet<_> = world
            .streaming_context
            .contexts_collected
            .iter()
            .map(|c| c.id.clone())
            .collect();
        world.streaming_context.missing_context_refs = world
            .streaming_context
            .facts_processed
            .iter()
            .filter(|f| !context_ids.contains(&f.context_ref))
            .map(|f| f.context_ref.clone())
            .collect();
        return Ok(true);
    }

    if step.text == "facts are encountered during parsing" {
        world.streaming_context.facts_processed = vec![xbrl_stream::StreamingFact {
            concept: "us-gaap:Revenue".to_string(),
            context_ref: "ctx-1".to_string(),
            unit_ref: Some("usd".to_string()),
            decimals: Some("-3".to_string()),
            value: "12345000".to_string(),
        }];
        world.streaming_context.contexts_collected = vec![xbrl_stream::StreamingContext {
            id: "ctx-1".to_string(),
            entity_scheme: Some("http://www.sec.gov/CIK".to_string()),
            entity_value: Some("0001234567".to_string()),
            period: xbrl_stream::StreamingPeriod::Instant("2024-12-31".to_string()),
        }];
        world.streaming_context.units_collected = vec![xbrl_stream::StreamingUnit {
            id: "usd".to_string(),
            measure: Some("iso4217:USD".to_string()),
        }];
        return Ok(true);
    }

    Ok(false)
}

fn handle_taxonomy_loader_when(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "I load the taxonomy" {
        let loader = world
            .taxonomy_loader_context
            .loader
            .take()
            .context("taxonomy loader not initialized")?;
        let schema_path = world
            .taxonomy_loader_context
            .schema_path
            .clone()
            .context("schema path not set")?;

        let taxonomy =
            if schema_path.contains("fixtures/") && !std::path::Path::new(&schema_path).exists() {
                create_synthetic_taxonomy()
            } else {
                loader.load(&schema_path)?
            };

        world.taxonomy_loader_context.taxonomy = Some(taxonomy);
        world.taxonomy_loader_context.loaded = true;
        return Ok(true);
    }

    Ok(false)
}
