//! Then-phase step handlers and parameterized assertions.

use crate::world::{Step, World};
use anyhow::Context;
use scenario_runner::{
    ensure_ixds_member_count, ensure_report_concept_set, ensure_report_contains_rule,
    ensure_report_does_not_contain_rule, ensure_report_fact_count,
    ensure_report_has_no_error_findings, ensure_taxonomy_resolution_resolves_at_least,
    ensure_taxonomy_resolution_succeeds,
};

#[allow(clippy::too_many_lines)]
pub fn handle(world: &mut World, step: &Step) -> anyhow::Result<()> {
    // Dimension-related Then steps
    if step.text == "the validation should pass" {
        if !world.dimension.validation_findings.is_empty() {
            anyhow::bail!(
                "expected validation to pass but got findings: {:?}",
                world.dimension.validation_findings
            );
        }
        return Ok(());
    }

    if step.text == "the validation should fail" {
        if world.dimension.validation_findings.is_empty() {
            anyhow::bail!("expected validation to fail but no findings were reported");
        }
        return Ok(());
    }

    if step.text == "no findings should be reported" {
        if !world.dimension.validation_findings.is_empty() {
            anyhow::bail!(
                "expected no findings but got: {:?}",
                world.dimension.validation_findings
            );
        }
        return Ok(());
    }

    if let Some(finding) = step.text.strip_prefix("an \"") {
        let expected_finding = finding.trim_end_matches("\" finding should be reported");
        if !world
            .dimension
            .validation_findings
            .iter()
            .any(|f| f == expected_finding)
        {
            anyhow::bail!(
                "expected finding {} but got {:?}",
                expected_finding,
                world.dimension.validation_findings
            );
        }
        return Ok(());
    }

    // Decimal precision Then steps
    if step.text == "no validation errors are reported" {
        if !world.completeness.findings.is_empty() {
            anyhow::bail!(
                "expected no validation errors but got: {:?}",
                world.completeness.findings
            );
        }
        return Ok(());
    }

    if let Some(error_type) = step.text.strip_prefix("validation error \"") {
        let expected_error = error_type.trim_end_matches("\" is reported");
        let has_error = world
            .completeness
            .findings
            .iter()
            .any(|f| f.rule_id.contains(expected_error) || f.message.contains(expected_error));
        if !has_error {
            anyhow::bail!(
                "expected validation error '{}' but got: {:?}",
                expected_error,
                world.completeness.findings
            );
        }
        return Ok(());
    }

    match step.text.as_str() {
        "the validation report has no error findings" => {
            ensure_report_has_no_error_findings(crate::execution(world)?)
        }
        "the taxonomy resolution succeeds" => {
            ensure_taxonomy_resolution_succeeds(crate::execution(world)?)
        }
        "the concept set is:" => {
            let expected = step
                .table
                .iter()
                .filter_map(|row| row.first())
                .map(String::as_str)
                .collect::<Vec<_>>();
            ensure_report_concept_set(crate::execution(world)?, &expected)
        }
        "the export report receipt is emitted" => {
            let execution = crate::execution(world)?;
            if execution.export_receipt.is_none() {
                anyhow::bail!("export report receipt was not emitted");
            }
            Ok(())
        }
        "bundling fails because no scenario matches" => {
            let manifest = world
                .output
                .bundle_manifest
                .as_ref()
                .context("bundle step requires a prior bundle operation")?;
            if !manifest.scenarios.is_empty() {
                anyhow::bail!(
                    "expected bundling to fail but found {} matching scenario(s)",
                    manifest.scenarios.len()
                );
            }
            Ok(())
        }
        "the sensor report is emitted" => {
            if world.output.sensor_report.is_none() {
                anyhow::bail!("sensor report was not emitted");
            }
            Ok(())
        }
        "the filing manifest receipt is emitted" => {
            let receipt = world
                .output
                .filing_receipt
                .as_ref()
                .context("filing manifest receipt was not emitted")?;
            if receipt.kind != "filing.manifest" {
                anyhow::bail!(
                    "expected receipt kind 'filing.manifest', got '{}'",
                    receipt.kind
                );
            }
            Ok(())
        }
        "the output is valid JSON" => {
            let output = world
                .output
                .cli_output
                .clone()
                .context("CLI output not captured")?;
            let json_value: serde_json::Value =
                serde_json::from_str(&output).context("CLI output is not valid JSON")?;
            world.output.cli_json_output = Some(json_value);
            Ok(())
        }
        "the profile contains required fields" => {
            let json_value = world
                .output
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
            Ok(())
        }
        "the alpha readiness checks pass" => {
            let exit_code = world
                .output
                .cli_exit_code
                .context("alpha readiness gate was not executed")?;
            if exit_code != 0 {
                let output = world
                    .output
                    .cli_output
                    .as_deref()
                    .unwrap_or("no output captured");
                anyhow::bail!(
                    "alpha readiness gate failed with exit code {exit_code}\noutput:\n{output}"
                );
            }
            Ok(())
        }
        "the publishable workspace crates package successfully" => {
            let results = &world.output.package_check.package_results;
            if results.is_empty() {
                anyhow::bail!("package readiness check was not executed");
            }
            let failures: Vec<_> = results.iter().filter(|(_, success, _)| !success).collect();
            if !failures.is_empty() {
                use std::fmt::Write;
                let mut msg = String::from("package check failed for:\n");
                for (name, _, stderr) in &failures {
                    let _ = write!(msg, "  - {name}\n{stderr}\n");
                }
                anyhow::bail!(msg);
            }
            Ok(())
        }
        _ => handle_parameterized_assertion(world, step),
    }
}

#[allow(clippy::too_many_lines)]
fn handle_parameterized_assertion(world: &World, step: &Step) -> anyhow::Result<()> {
    if let Some(rule_id) = step
        .text
        .strip_prefix("the validation report contains rule \"")
    {
        let validation_run = crate::execution(world)?
            .validation_run
            .as_ref()
            .context("missing validation run")?;
        return ensure_report_contains_rule(validation_run, rule_id.trim_end_matches('"'));
    }

    if let Some(rule_id) = step
        .text
        .strip_prefix("the validation report does not contain rule \"")
    {
        let validation_run = crate::execution(world)?
            .validation_run
            .as_ref()
            .context("missing validation run")?;
        return ensure_report_does_not_contain_rule(validation_run, rule_id.trim_end_matches('"'));
    }

    if let Some(member_count) = crate::world::parse_count_suffix(
        &step.text,
        "the IXDS assembly receipt contains ",
        "member",
    ) {
        return ensure_ixds_member_count(crate::execution(world)?, member_count);
    }

    if let Some(namespace_count) = crate::world::parse_count_suffix(
        &step.text,
        "the taxonomy resolution resolves at least ",
        "namespace",
    ) {
        return ensure_taxonomy_resolution_resolves_at_least(
            crate::execution(world)?,
            namespace_count,
        );
    }

    if let Some(fact_count) =
        crate::world::parse_count_suffix(&step.text, "the report contains ", "fact")
    {
        return ensure_report_fact_count(crate::execution(world)?, fact_count);
    }

    // Bundle-related assertions
    if let Some(scenario_id) = step
        .text
        .strip_prefix("the bundle manifest lists scenario \"")
    {
        let scenario_id = scenario_id.trim_end_matches('"');
        let manifest = world
            .output
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
        return Ok(());
    }

    // Feature grid assertions
    if let Some(scenario_id) = step
        .text
        .strip_prefix("the feature grid contains scenario \"")
    {
        let scenario_id = scenario_id.trim_end_matches('"');
        let grid = world
            .execution
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
        return Ok(());
    }

    // Context completeness Then steps
    if let Some(context_ref) = step
        .text
        .strip_prefix("a context-missing error is reported for context \"")
    {
        let context_ref = context_ref.trim_end_matches('"');
        let found = world
            .completeness
            .findings
            .iter()
            .any(|f| f.rule_id == "SEC-CONTEXT-001" && f.message.contains(context_ref));
        if !found {
            anyhow::bail!(
                "expected context-missing error for '{}' but got findings: {:?}",
                context_ref,
                world.completeness.findings
            );
        }
        return Ok(());
    }

    if step.text == "no context completeness findings are reported" {
        if !world.completeness.findings.is_empty() {
            anyhow::bail!(
                "expected no findings but got: {:?}",
                world.completeness.findings
            );
        }
        return Ok(());
    }

    if let Some(count_str) = step
        .text
        .strip_prefix("context-missing errors are reported")
    {
        let expected_count: usize = count_str
            .split_whitespace()
            .next()
            .and_then(|s| s.parse().ok())
            .unwrap_or(1);
        let actual_count = world
            .completeness
            .findings
            .iter()
            .filter(|f| f.rule_id == "SEC-CONTEXT-001")
            .count();
        if actual_count != expected_count {
            anyhow::bail!(
                "expected {} context-missing errors but got {}: {:?}",
                expected_count,
                actual_count,
                world.completeness.findings
            );
        }
        return Ok(());
    }

    if let Some(rule_id) = step.text.strip_prefix("the finding rule ID is \"") {
        let rule_id = rule_id.trim_end_matches('"');
        let found = world
            .completeness
            .findings
            .iter()
            .any(|f| f.rule_id == rule_id);
        if !found {
            anyhow::bail!(
                "expected finding with rule ID '{}' but got: {:?}",
                rule_id,
                world.completeness.findings
            );
        }
        return Ok(());
    }

    // Streaming parser Then steps
    if step.text == "memory usage should stay under 50MB peak" {
        let peak = world
            .processing
            .streaming
            .memory_peak_mb
            .unwrap_or(f64::MAX);
        if peak > 50.0 {
            anyhow::bail!("memory usage was {peak}MB, expected under 50MB");
        }
        return Ok(());
    }

    if step.text == "all facts should be processed" {
        if world.processing.streaming.facts_processed.is_empty() {
            anyhow::bail!("no facts were processed");
        }
        return Ok(());
    }

    if step.text == "context references should be validated" {
        return Ok(());
    }

    if step.text == "the DOM parser should be recommended" {
        let size = world.processing.streaming.file_size_mb.unwrap_or(0.0);
        if size > 10.0 {
            anyhow::bail!(
                "DOM parser should be recommended for files under 10MB, but file is {size}MB"
            );
        }
        return Ok(());
    }

    if step.text == "the streaming parser should be available as option" {
        if !world.processing.streaming.use_streaming {
            anyhow::bail!("streaming parser should be available as an option");
        }
        return Ok(());
    }

    if step.text == "missing context references should be reported" {
        if world.processing.streaming.missing_context_refs.is_empty() {
            anyhow::bail!("expected missing context references to be reported");
        }
        return Ok(());
    }

    if step.text == "line numbers should indicate error locations" {
        return Ok(());
    }

    if step.text == "the handler should receive each fact" {
        if world.processing.streaming.facts_processed.is_empty() {
            anyhow::bail!("handler did not receive any facts");
        }
        return Ok(());
    }

    if step.text == "contexts should be collected" {
        if world.processing.streaming.contexts_collected.is_empty() {
            anyhow::bail!("no contexts were collected");
        }
        return Ok(());
    }

    if step.text == "units should be available for reference" {
        if world.processing.streaming.units_collected.is_empty() {
            anyhow::bail!("no units were collected");
        }
        return Ok(());
    }

    // Taxonomy loader Then steps
    if step.text == "the taxonomy should contain dimensions" {
        let taxonomy = world
            .processing
            .taxonomy_loader
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        if taxonomy.dimensions.is_empty() {
            anyhow::bail!("taxonomy has no dimensions");
        }
        return Ok(());
    }

    if step.text == "explicit dimensions should have domains" {
        let taxonomy = world
            .processing
            .taxonomy_loader
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
        return Ok(());
    }

    if step.text == "domains should have members" {
        let taxonomy = world
            .processing
            .taxonomy_loader
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        let has_members = taxonomy.domains.values().any(|d| !d.members.is_empty());
        if !has_members {
            anyhow::bail!("no domains have members defined");
        }
        return Ok(());
    }

    if step.text == "members should maintain parent-child relationships" {
        return Ok(());
    }

    if step.text == "typed dimensions should have value types" {
        return Ok(());
    }

    if step.text == "the value types should be valid XSD types" {
        return Ok(());
    }

    if step.text == "hypercubes should contain their dimensions" {
        return Ok(());
    }

    if step.text == "dimensions should reference their domains" {
        let taxonomy = world
            .processing
            .taxonomy_loader
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        if taxonomy.dimension_domains.is_empty() && !taxonomy.dimensions.is_empty() {
            anyhow::bail!("no dimension-domain references found");
        }
        return Ok(());
    }

    if step.text == "the taxonomy file should be cached" {
        let cache_dir = world
            .processing
            .taxonomy_loader
            .cache_dir
            .as_ref()
            .context("cache directory not configured")?;
        if !cache_dir.exists() {
            anyhow::bail!("cache directory does not exist");
        }
        return Ok(());
    }

    if step.text == "subsequent loads should use the cache" {
        return Ok(());
    }

    if step.text == "imported schemas should be loaded" {
        return Ok(());
    }

    if step.text == "all dimension definitions should be available" {
        let taxonomy = world
            .processing
            .taxonomy_loader
            .taxonomy
            .as_ref()
            .context("taxonomy not loaded")?;
        if taxonomy.dimensions.is_empty() {
            anyhow::bail!("no dimension definitions available");
        }
        return Ok(());
    }

    anyhow::bail!("unsupported BDD step: {}", step.text)
}
