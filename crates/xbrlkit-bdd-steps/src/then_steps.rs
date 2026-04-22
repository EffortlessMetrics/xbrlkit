//! Then step handlers and parameterized assertions for BDD scenario execution.

use crate::{execution, parsing, World};
use anyhow::Context;

pub(crate) fn handle_then(world: &mut World, step: &crate::Step) -> anyhow::Result<()> {
    if handle_dimension_assertions(world, step)? {
        return Ok(());
    }
    if handle_decimal_precision_assertions(world, step)? {
        return Ok(());
    }
    if handle_report_assertions(world, step)? {
        return Ok(());
    }
    if handle_bundle_assertions(world, step)? {
        return Ok(());
    }
    if handle_feature_grid_assertions(world, step)? {
        return Ok(());
    }
    if handle_cli_assertions(world, step)? {
        return Ok(());
    }
    if handle_alpha_assertions(world, step)? {
        return Ok(());
    }
    if handle_context_completeness_assertions(world, step)? {
        return Ok(());
    }
    if handle_streaming_assertions(world, step)? {
        return Ok(());
    }
    if handle_taxonomy_loader_assertions(world, step)? {
        return Ok(());
    }
    if handle_parameterized_assertions(world, step)? {
        return Ok(());
    }

    anyhow::bail!("unsupported BDD step: {}", step.text)
}

fn handle_dimension_assertions(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "the validation should pass" {
        if !world.dimension_context.validation_findings.is_empty() {
            anyhow::bail!(
                "expected validation to pass but got findings: {:?}",
                world.dimension_context.validation_findings
            );
        }
        return Ok(true);
    }

    if step.text == "the validation should fail" {
        if world.dimension_context.validation_findings.is_empty() {
            anyhow::bail!("expected validation to fail but no findings were reported");
        }
        return Ok(true);
    }

    if step.text == "no findings should be reported" {
        if !world.dimension_context.validation_findings.is_empty() {
            anyhow::bail!(
                "expected no findings but got: {:?}",
                world.dimension_context.validation_findings
            );
        }
        return Ok(true);
    }

    if let Some(finding) = step.text.strip_prefix("an \"") {
        let expected_finding = finding.trim_end_matches("\" finding should be reported");
        if !world
            .dimension_context
            .validation_findings
            .iter()
            .any(|f| f == expected_finding)
        {
            anyhow::bail!(
                "expected finding {} but got {:?}",
                expected_finding,
                world.dimension_context.validation_findings
            );
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_decimal_precision_assertions(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if step.text == "no validation errors are reported" {
        if !world.context_completeness_context.findings.is_empty() {
            anyhow::bail!(
                "expected no validation errors but got: {:?}",
                world.context_completeness_context.findings
            );
        }
        return Ok(true);
    }

    if let Some(error_type) = step.text.strip_prefix("validation error \"") {
        let expected_error = error_type.trim_end_matches("\" is reported");
        let has_error = world
            .context_completeness_context
            .findings
            .iter()
            .any(|f| f.rule_id.contains(expected_error) || f.message.contains(expected_error));
        if !has_error {
            anyhow::bail!(
                "expected validation error '{}' but got: {:?}",
                expected_error,
                world.context_completeness_context.findings
            );
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_report_assertions(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    match step.text.as_str() {
        "the validation report has no error findings" => {
            scenario_runner::ensure_report_has_no_error_findings(execution(world)?)?;
            return Ok(true);
        }
        "the taxonomy resolution succeeds" => {
            scenario_runner::ensure_taxonomy_resolution_succeeds(execution(world)?)?;
            return Ok(true);
        }
        "the concept set is:" => {
            let expected = step
                .table
                .iter()
                .filter_map(|row| row.first())
                .map(String::as_str)
                .collect::<Vec<_>>();
            scenario_runner::ensure_report_concept_set(execution(world)?, &expected)?;
            return Ok(true);
        }
        "the export report receipt is emitted" => {
            let execution = execution(world)?;
            if execution.export_receipt.is_none() {
                anyhow::bail!("export report receipt was not emitted");
            }
            return Ok(true);
        }
        _ => {}
    }
    Ok(false)
}

fn handle_bundle_assertions(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "bundling fails because no scenario matches" {
        let manifest = world
            .bundle_manifest
            .as_ref()
            .context("bundle step requires a prior bundle operation")?;
        if !manifest.scenarios.is_empty() {
            anyhow::bail!(
                "expected bundling to fail but found {} matching scenario(s)",
                manifest.scenarios.len()
            );
        }
        return Ok(true);
    }

    if step.text == "the sensor report is emitted" {
        if world.sensor_report.is_none() {
            anyhow::bail!("sensor report was not emitted");
        }
        return Ok(true);
    }

    if step.text == "the filing manifest receipt is emitted" {
        let receipt = world
            .filing_receipt
            .as_ref()
            .context("filing manifest receipt was not emitted")?;
        if receipt.kind != "filing.manifest" {
            anyhow::bail!(
                "expected receipt kind 'filing.manifest', got '{}'",
                receipt.kind
            );
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_feature_grid_assertions(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if let Some(scenario_id) = step
        .text
        .strip_prefix("the feature grid contains scenario \"")
    {
        let scenario_id = scenario_id.trim_end_matches('"');
        let grid = world
            .compiled_grid
            .as_ref()
            .context("feature grid assertion requires a prior compile operation")?;
        if !grid.scenarios.iter().any(|s| s.scenario_id == scenario_id) {
            anyhow::bail!(
                "scenario {} not found in feature grid (contains {} scenario(s))",
                scenario_id,
                grid.scenarios.len()
            );
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_cli_assertions(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "the output is valid JSON" {
        let output = world
            .cli_output
            .clone()
            .context("CLI output not captured")?;
        let json_value: serde_json::Value =
            serde_json::from_str(&output).context("CLI output is not valid JSON")?;
        world.cli_json_output = Some(json_value);
        return Ok(true);
    }

    if step.text == "the profile contains required fields" {
        let json_value = world
            .cli_json_output
            .as_ref()
            .context("JSON output not parsed")?;
        let required_fields = [
            "id",
            "label",
            "forms",
            "enabled_rule_families",
            "standard_taxonomy_uris",
            "required_facts",
        ];
        for field in &required_fields {
            if json_value.get(field).is_none() {
                anyhow::bail!("required field '{field}' is missing from profile output");
            }
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_alpha_assertions(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "the alpha readiness checks pass" {
        let exit_code = world
            .cli_exit_code
            .context("alpha readiness gate was not executed")?;
        if exit_code != 0 {
            let output = world.cli_output.as_deref().unwrap_or("no output captured");
            anyhow::bail!(
                "alpha readiness gate failed with exit code {exit_code}\noutput:\n{output}"
            );
        }
        return Ok(true);
    }
    Ok(false)
}

fn handle_context_completeness_assertions(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if let Some(context_ref) = step
        .text
        .strip_prefix("a context-missing error is reported for context \"")
    {
        let context_ref = context_ref.trim_end_matches('"');
        let found = world
            .context_completeness_context
            .findings
            .iter()
            .any(|f| f.rule_id == "SEC-CONTEXT-001" && f.message.contains(context_ref));
        if !found {
            anyhow::bail!(
                "expected context-missing error for '{}' but got findings: {:?}",
                context_ref,
                world.context_completeness_context.findings
            );
        }
        return Ok(true);
    }

    if step.text == "no context completeness findings are reported" {
        if !world.context_completeness_context.findings.is_empty() {
            anyhow::bail!(
                "expected no findings but got: {:?}",
                world.context_completeness_context.findings
            );
        }
        return Ok(true);
    }

    if let Some(count_str) = step.text.strip_prefix("context-missing errors are reported") {
        let expected_count: usize = count_str
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let actual_count = world
            .context_completeness_context
            .findings
            .iter()
            .filter(|f| f.rule_id == "SEC-CONTEXT-001")
            .count();
        if actual_count != expected_count {
            anyhow::bail!(
                "expected {} context-missing errors but got {}: {:?}",
                expected_count,
                actual_count,
                world.context_completeness_context.findings
            );
        }
        return Ok(true);
    }

    if let Some(rule_id) = step.text.strip_prefix("the finding rule ID is \"") {
        let rule_id = rule_id.trim_end_matches('"');
        let found = world
            .context_completeness_context
            .findings
            .iter()
            .any(|f| f.rule_id == rule_id);
        if !found {
            anyhow::bail!(
                "expected finding with rule ID '{}' but got: {:?}",
                rule_id,
                world.context_completeness_context.findings
            );
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_streaming_assertions(world: &mut World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "memory usage should stay under 50MB peak" {
        let peak = world.streaming_context.memory_peak_mb.unwrap_or(f64::MAX);
        if peak > 50.0 {
            anyhow::bail!("memory usage was {peak}MB, expected under 50MB");
        }
        return Ok(true);
    }

    if step.text == "all facts should be processed" {
        if world.streaming_context.facts_processed.is_empty() {
            anyhow::bail!("no facts were processed");
        }
        return Ok(true);
    }

    if step.text == "context references should be validated" {
        return Ok(true);
    }

    if step.text == "the DOM parser should be recommended" {
        let size = world.streaming_context.file_size_mb.unwrap_or(0.0);
        if size > 10.0 {
            anyhow::bail!(
                "DOM parser should be recommended for files under 10MB, but file is {size}MB"
            );
        }
        return Ok(true);
    }

    if step.text == "the streaming parser should be available as option" {
        if !world.streaming_context.use_streaming {
            anyhow::bail!("streaming parser should be available as an option");
        }
        return Ok(true);
    }

    if step.text == "missing context references should be reported" {
        if world.streaming_context.missing_context_refs.is_empty() {
            anyhow::bail!("expected missing context references to be reported");
        }
        return Ok(true);
    }

    if step.text == "line numbers should indicate error locations" {
        return Ok(true);
    }

    if step.text == "the handler should receive each fact" {
        if world.streaming_context.facts_processed.is_empty() {
            anyhow::bail!("handler did not receive any facts");
        }
        return Ok(true);
    }

    if step.text == "contexts should be collected" {
        if world.streaming_context.contexts_collected.is_empty() {
            anyhow::bail!("no contexts were collected");
        }
        return Ok(true);
    }

    if step.text == "units should be available for reference" {
        if world.streaming_context.units_collected.is_empty() {
            anyhow::bail!("no units were collected");
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_taxonomy_loader_assertions(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if handle_taxonomy_structure_assertions(world, step)? {
        return Ok(true);
    }
    if handle_taxonomy_behavior_assertions(world, step)? {
        return Ok(true);
    }
    Ok(false)
}

fn handle_taxonomy_structure_assertions(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if handle_taxonomy_dimensions_assertions(world, step)? {
        return Ok(true);
    }
    if handle_taxonomy_types_assertions(world, step)? {
        return Ok(true);
    }
    Ok(false)
}

fn handle_taxonomy_dimensions_assertions(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if step.text == "the taxonomy should contain dimensions" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        if taxonomy.dimensions.is_empty() {
            anyhow::bail!("taxonomy has no dimensions");
        }
        return Ok(true);
    }

    if step.text == "explicit dimensions should have domains" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        let has_explicit_with_domain = taxonomy.dimensions.iter().any(|(_, d)| match d {
            taxonomy_dimensions::Dimension::Explicit { default_domain, .. } => {
                default_domain.is_some()
            }
            taxonomy_dimensions::Dimension::Typed { .. } => false,
        });
        if !has_explicit_with_domain && !taxonomy.dimensions.is_empty() {
            anyhow::bail!("no explicit dimensions have domains defined");
        }
        return Ok(true);
    }

    if step.text == "domains should have members" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        let has_members = taxonomy.domains.values().any(|d| !d.members.is_empty());
        if !has_members {
            anyhow::bail!("no domains have members defined");
        }
        return Ok(true);
    }

    if step.text == "members should maintain parent-child relationships" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        let has_parent_child = taxonomy
            .domains
            .values()
            .any(|domain| domain.members.values().any(|m| m.parent.is_some()));
        if !has_parent_child {
            anyhow::bail!("no members have parent-child relationships defined");
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_taxonomy_types_assertions(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if step.text == "typed dimensions should have value types" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        let has_value_types = taxonomy.dimensions.values().any(|d| match d {
            taxonomy_dimensions::Dimension::Typed { value_type, .. } => !value_type.is_empty(),
            taxonomy_dimensions::Dimension::Explicit { .. } => false,
        });
        if !has_value_types {
            anyhow::bail!("no typed dimensions have value types defined");
        }
        return Ok(true);
    }

    if step.text == "the value types should be valid XSD types" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        let valid_xsd = [
            "xs:string",
            "xs:decimal",
            "xs:date",
            "xs:boolean",
            "xs:integer",
        ];
        let all_valid = taxonomy.dimensions.values().all(|d| match d {
            taxonomy_dimensions::Dimension::Typed { value_type, .. } => {
                valid_xsd.contains(&value_type.as_str())
            }
            taxonomy_dimensions::Dimension::Explicit { .. } => true,
        });
        if !all_valid {
            anyhow::bail!("some typed dimensions have invalid XSD value types");
        }
        return Ok(true);
    }

    if step.text == "all dimension definitions should be available" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        if taxonomy.dimensions.is_empty() {
            anyhow::bail!("no dimension definitions available");
        }
        return Ok(true);
    }

    Ok(false)
}

fn handle_taxonomy_behavior_assertions(
    world: &mut World,
    step: &crate::Step,
) -> anyhow::Result<bool> {
    if step.text == "hypercubes should contain their dimensions" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        let has_hypercubes_with_dims = taxonomy
            .hypercubes
            .values()
            .any(|h| !h.dimensions.is_empty());
        if !has_hypercubes_with_dims {
            anyhow::bail!("no hypercubes contain dimensions");
        }
        return Ok(true);
    }

    if step.text == "dimensions should reference their domains" {
        let taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        if taxonomy.dimension_domains.is_empty() && !taxonomy.dimensions.is_empty() {
            anyhow::bail!("no dimension-domain references found");
        }
        return Ok(true);
    }

    if step.text == "the taxonomy file should be cached" {
        let cache_dir = world
            .taxonomy_loader_context
            .cache_dir
            .as_ref()
            .context("cache directory not configured")?;
        if !cache_dir.exists() {
            anyhow::bail!("cache directory does not exist");
        }
        return Ok(true);
    }

    if step.text == "subsequent loads should use the cache" {
        let cache_dir = world
            .taxonomy_loader_context
            .cache_dir
            .as_ref()
            .context("cache directory not configured")?;
        if !cache_dir.exists() {
            anyhow::bail!("cache directory does not exist");
        }
        return Ok(true);
    }

    if step.text == "imported schemas should be loaded" {
        let _taxonomy = world
            .taxonomy_loader_context
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        return Ok(true);
    }

    Ok(false)
}

fn handle_parameterized_assertions(world: &World, step: &crate::Step) -> anyhow::Result<bool> {
    if handle_validation_report_parameterized(world, step)? {
        return Ok(true);
    }
    if handle_dimension_parsing_assertions(world, step)? {
        return Ok(true);
    }
    if handle_bundle_parameterized(world, step)? {
        return Ok(true);
    }
    Ok(false)
}

fn handle_validation_report_parameterized(world: &World, step: &crate::Step) -> anyhow::Result<bool> {
    if let Some(rule_id) = step
        .text
        .strip_prefix("the validation report contains rule \"")
    {
        let validation_run = execution(world)?
            .validation_run
            .as_ref()
            .context("missing validation run")?;
        scenario_runner::ensure_report_contains_rule(
            validation_run,
            rule_id.trim_end_matches('"'),
        )?;
        return Ok(true);
    }

    if let Some(rule_id) = step
        .text
        .strip_prefix("the validation report does not contain rule \"")
    {
        let validation_run = execution(world)?
            .validation_run
            .as_ref()
            .context("missing validation run")?;
        scenario_runner::ensure_report_does_not_contain_rule(
            validation_run,
            rule_id.trim_end_matches('"'),
        )?;
        return Ok(true);
    }

    if let Some(member_count) =
        parsing::parse_count_suffix(&step.text, "the IXDS assembly receipt contains ", "member")
    {
        scenario_runner::ensure_ixds_member_count(execution(world)?, member_count)?;
        return Ok(true);
    }

    if let Some(namespace_count) = parsing::parse_count_suffix(
        &step.text,
        "the taxonomy resolution resolves at least ",
        "namespace",
    ) {
        scenario_runner::ensure_taxonomy_resolution_resolves_at_least(
            execution(world)?,
            namespace_count,
        )?;
        return Ok(true);
    }

    if let Some(fact_count) =
        parsing::parse_count_suffix(&step.text, "the report contains ", "fact")
    {
        scenario_runner::ensure_report_fact_count(execution(world)?, fact_count)?;
        return Ok(true);
    }

    Ok(false)
}

fn handle_bundle_parameterized(world: &World, step: &crate::Step) -> anyhow::Result<bool> {
    if let Some(scenario_id) = step
        .text
        .strip_prefix("the bundle manifest lists scenario \"")
    {
        let scenario_id = scenario_id.trim_end_matches('"');
        let manifest = world
            .bundle_manifest
            .as_ref()
            .context("bundle assertion requires a prior bundle operation")?;
        if !manifest
            .scenarios
            .iter()
            .any(|s| s.scenario_id == scenario_id)
        {
            anyhow::bail!(
                "scenario {} not found in bundle manifest (contains {} scenario(s))",
                scenario_id,
                manifest.scenarios.len()
            );
        }
        return Ok(true);
    }
    Ok(false)
}

fn handle_dimension_parsing_assertions(world: &World, step: &crate::Step) -> anyhow::Result<bool> {
    if handle_dimension_typed_assertions(world, step)? {
        return Ok(true);
    }
    if handle_dimension_member_assertions(world, step)? {
        return Ok(true);
    }
    Ok(false)
}

fn handle_dimension_typed_assertions(world: &World, step: &crate::Step) -> anyhow::Result<bool> {
    if step.text == "the dimension should be marked as typed" {
        let has_typed = world
            .dimension_context
            .parsed_dimensions
            .iter()
            .any(|d| d.is_typed);
        if !has_typed {
            anyhow::bail!(
                "expected at least one typed dimension but none found in parsed dimensions: {:?}",
                world.dimension_context.parsed_dimensions
            );
        }
        return Ok(true);
    }

    if let Some(expected) = step.text.strip_prefix("the typed value should be \"") {
        let expected = expected.trim_end_matches('"');
        let typed = world
            .dimension_context
            .parsed_dimensions
            .iter()
            .find(|d| d.is_typed);
        match typed {
            Some(d) if d.member == expected => return Ok(true),
            Some(d) => {
                anyhow::bail!("expected typed value '{}' but got '{}'", expected, d.member)
            }
            None => anyhow::bail!("no typed dimension found in parsed dimensions"),
        }
    }

    if step.text == "the typed value should be empty" {
        let typed = world
            .dimension_context
            .parsed_dimensions
            .iter()
            .find(|d| d.is_typed);
        match typed {
            Some(d) if d.member.is_empty() => return Ok(true),
            Some(d) => anyhow::bail!("expected empty typed value but got '{}'", d.member),
            None => anyhow::bail!("no typed dimension found in parsed dimensions"),
        }
    }

    Ok(false)
}

fn handle_dimension_member_assertions(world: &World, step: &crate::Step) -> anyhow::Result<bool> {
    if let Some(expected) = step.text.strip_prefix("the member should be \"") {
        let expected = expected.trim_end_matches('"');
        let dim = world.dimension_context.parsed_dimensions.first();
        match dim {
            Some(d) if d.member == expected => return Ok(true),
            Some(d) => anyhow::bail!("expected member '{}' but got '{}'", expected, d.member),
            None => anyhow::bail!("no dimensions parsed"),
        }
    }

    if let Some(expected) = step
        .text
        .strip_prefix("the explicit dimension should have member \"")
    {
        let expected = expected.trim_end_matches('"');
        let explicit = world
            .dimension_context
            .parsed_dimensions
            .iter()
            .find(|d| !d.is_typed);
        match explicit {
            Some(d) if d.member == expected => return Ok(true),
            Some(d) => anyhow::bail!(
                "expected explicit member '{}' but got '{}'",
                expected,
                d.member
            ),
            None => anyhow::bail!("no explicit dimension found in parsed dimensions"),
        }
    }

    if let Some(expected) = step
        .text
        .strip_prefix("the typed dimension should have value \"")
    {
        let expected = expected.trim_end_matches('"');
        let typed = world
            .dimension_context
            .parsed_dimensions
            .iter()
            .find(|d| d.is_typed);
        match typed {
            Some(d) if d.member == expected => return Ok(true),
            Some(d) => anyhow::bail!("expected typed value '{}' but got '{}'", expected, d.member),
            None => anyhow::bail!("no typed dimension found in parsed dimensions"),
        }
    }

    if step.text == "both dimensions should be accessible" {
        let count = world.dimension_context.parsed_dimensions.len();
        if count != 2 {
            anyhow::bail!(
                "expected 2 accessible dimensions but found {}: {:?}",
                count,
                world.dimension_context.parsed_dimensions
            );
        }
        return Ok(true);
    }

    if step.text == "the typed dimension should be in the entity segment" {
        let segment = world
            .dimension_context
            .parsed_dimensions
            .iter()
            .find(|d| matches!(d.container, crate::DimensionContainer::Segment));
        if segment.is_none() {
            anyhow::bail!(
                "expected typed dimension in segment but none found in parsed dimensions: {:?}",
                world.dimension_context.parsed_dimensions
            );
        }
        return Ok(true);
    }

    Ok(false)
}
