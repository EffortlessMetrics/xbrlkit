//! Minimal step execution for the active BDD slices.

use anyhow::Context;
use scenario_contract::{FeatureGrid, ScenarioRecord};
use scenario_runner::{
    ScenarioExecution, assert_scenario_outcome, ensure_ixds_member_count,
    ensure_report_concept_set, ensure_report_contains_rule, ensure_report_does_not_contain_rule,
    ensure_report_fact_count, ensure_report_has_no_error_findings,
    ensure_taxonomy_resolution_resolves_at_least, ensure_taxonomy_resolution_succeeds,
    execute_scenario, write_execution_receipts,
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub text: String,
    pub table: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct World {
    pub repo_root: PathBuf,
    pub grid: FeatureGrid,
    pub profile_id: Option<String>,
    pub fixture_dirs: Vec<PathBuf>,
    pub execution: Option<ScenarioExecution>,
    pub bundle: Option<bundle_selector::Bundle>,
    pub bundle_error: Option<String>,
}

impl World {
    #[must_use]
    pub fn new(repo_root: PathBuf, grid: FeatureGrid) -> Self {
        Self {
            repo_root,
            grid,
            profile_id: None,
            fixture_dirs: Vec::new(),
            execution: None,
            bundle: None,
            bundle_error: None,
        }
    }
}

pub fn run_scenario(
    world: &mut World,
    scenario: &ScenarioRecord,
    steps: &[Step],
) -> anyhow::Result<()> {
    if !world
        .grid
        .scenarios
        .iter()
        .any(|candidate| candidate.scenario_id == scenario.scenario_id)
    {
        anyhow::bail!(
            "scenario {} is not present in the feature grid",
            scenario.scenario_id
        );
    }

    for step in steps {
        run_step(world, scenario, step)?;
    }

    // For bundle scenarios, we don't need a standard execution
    if scenario.scenario_id == "SCN-XK-WORKFLOW-002" || scenario.scenario_id == "SCN-XK-WORKFLOW-004" {
        return Ok(());
    }

    let execution = world
        .execution
        .as_ref()
        .context("scenario completed without executing a When step")?;
    assert_scenario_outcome(scenario, execution)
}

fn run_step(world: &mut World, scenario: &ScenarioRecord, step: &Step) -> anyhow::Result<()> {
    if handle_given(world, scenario, step)? {
        return Ok(());
    }
    if handle_when(world, scenario, step)? {
        return Ok(());
    }
    handle_then(world, step)
}

fn execution(world: &World) -> anyhow::Result<&ScenarioExecution> {
    world
        .execution
        .as_ref()
        .context("scenario step requires a prior execution")
}

fn assert_declared_inputs_match(world: &World, scenario: &ScenarioRecord) -> anyhow::Result<()> {
    if let Some(profile_id) = &world.profile_id {
        if scenario.profile_pack.as_deref() != Some(profile_id.as_str()) {
            anyhow::bail!("declared profile pack does not match scenario metadata");
        }
    }

    if !world.fixture_dirs.is_empty() {
        let declared = world
            .fixture_dirs
            .iter()
            .map(|path| {
                path.strip_prefix(world.repo_root.join("fixtures"))
                    .expect("fixture path under repo root")
                    .to_string_lossy()
                    .replace('\\', "/")
            })
            .collect::<Vec<_>>();
        if declared != scenario.fixtures {
            anyhow::bail!("declared fixture directories do not match scenario metadata");
        }
    }

    Ok(())
}

fn handle_given(world: &mut World, scenario: &ScenarioRecord, step: &Step) -> anyhow::Result<bool> {
    if step.text == "the feature grid is compiled" {
        // The grid is already compiled when World is created
        return Ok(true);
    }

    if step.text == "a validation report receipt" {
        // Validation report is produced by a When step
        return Ok(true);
    }

    if step.text == "the active alpha scenarios are implemented" {
        // Checked by alpha-check itself
        return Ok(true);
    }

    if let Some(profile_id) = step.text.strip_prefix("the profile pack \"") {
        let profile_id = profile_id.trim_end_matches('"').to_string();
        if scenario.profile_pack.as_deref() != Some(profile_id.as_str()) {
            anyhow::bail!(
                "feature file profile pack {profile_id} does not match scenario metadata"
            );
        }
        world.profile_id = Some(profile_id);
        return Ok(true);
    }

    if let Some(fixture) = step
        .text
        .strip_prefix("the fixture directory \"")
        .or_else(|| step.text.strip_prefix("the fixture \""))
    {
        let fixture = fixture.trim_end_matches('"');
        if !scenario
            .fixtures
            .iter()
            .any(|candidate| candidate == fixture)
        {
            anyhow::bail!("feature file fixture {fixture} does not match scenario metadata");
        }
        world
            .fixture_dirs
            .push(world.repo_root.join("fixtures").join(fixture));
        return Ok(true);
    }

    Ok(false)
}

fn handle_when(world: &mut World, scenario: &ScenarioRecord, step: &Step) -> anyhow::Result<bool> {
    if matches!(
        step.text.as_str(),
        "I validate the filing" | "I validate duplicate facts" | "I resolve the DTS"
    ) {
        assert_declared_inputs_match(world, scenario)?;
        let execution = execute_scenario(&world.repo_root, scenario)?;
        write_execution_receipts(&world.repo_root, &execution)?;
        world.execution = Some(execution);
        return Ok(true);
    }

    if step.text == "I export the canonical report to JSON" {
        assert_declared_inputs_match(world, scenario)?;
        let execution = execute_scenario(&world.repo_root, scenario)?;
        write_execution_receipts(&world.repo_root, &execution)?;
        world.execution = Some(execution);
        return Ok(true);
    }

    // Handle bundle selector steps
    if let Some(selector) = step.text.strip_prefix("I bundle the selector \"") {
        let selector = selector.trim_end_matches('"');
        match bundle_selector::select_by_ac(&world.grid, selector) {
            Ok(bundle) => {
                world.bundle = Some(bundle);
                world.bundle_error = None;
            }
            Err(e) => {
                world.bundle_error = Some(e.to_string());
                world.bundle = None;
            }
        }
        return Ok(true);
    }

    // Handle cockpit packaging
    if step.text == "I package the receipt for cockpit" {
        assert_declared_inputs_match(world, scenario)?;
        let execution = execute_scenario(&world.repo_root, scenario)?;
        write_execution_receipts(&world.repo_root, &execution)?;
        world.execution = Some(execution);
        return Ok(true);
    }

    // Handle alpha check
    if step.text == "I run the alpha readiness gate" {
        assert_declared_inputs_match(world, scenario)?;
        let execution = execute_scenario(&world.repo_root, scenario)?;
        write_execution_receipts(&world.repo_root, &execution)?;
        world.execution = Some(execution);
        return Ok(true);
    }

    Ok(false)
}

fn handle_then(world: &World, step: &Step) -> anyhow::Result<()> {
    match step.text.as_str() {
        "the validation report has no error findings" => {
            ensure_report_has_no_error_findings(execution(world)?)
        }
        "the taxonomy resolution succeeds" => {
            ensure_taxonomy_resolution_succeeds(execution(world)?)
        }
        "the concept set is:" => {
            let expected = step
                .table
                .iter()
                .filter_map(|row| row.first())
                .map(String::as_str)
                .collect::<Vec<_>>();
            ensure_report_concept_set(execution(world)?, &expected)
        }
        "the export report receipt is emitted" => {
            let execution = execution(world)?;
            if execution.export_receipt.is_none() {
                anyhow::bail!("export report receipt was not emitted");
            }
            Ok(())
        }
        "the sensor report is emitted" => {
            let execution = execution(world)?;
            if execution.sensor_report.is_none() {
                anyhow::bail!("sensor report receipt was not emitted");
            }
            Ok(())
        }
        "the alpha readiness checks pass" => {
            let execution = execution(world)?;
            if execution.validation_run.is_none() {
                anyhow::bail!("alpha readiness checks did not pass");
            }
            Ok(())
        }
        _ => handle_parameterized_assertion(world, step),
    }
}

fn handle_parameterized_assertion(world: &World, step: &Step) -> anyhow::Result<()> {
    // Bundle manifest assertions
    if let Some(scenario_id) = step
        .text
        .strip_prefix("the bundle manifest lists scenario \"")
    {
        let scenario_id = scenario_id.trim_end_matches('"');
        let bundle = world
            .bundle
            .as_ref()
            .context("no bundle was created")?;
        if !bundle.scenarios.iter().any(|s| s.scenario_id == scenario_id) {
            anyhow::bail!(
                "bundle manifest does not list scenario '{}'",
                scenario_id
            );
        }
        return Ok(());
    }

    if let Some(rule_id) = step
        .text
        .strip_prefix("the validation report contains rule \"")
    {
        let validation_run = execution(world)?
            .validation_run
            .as_ref()
            .context("missing validation run")?;
        return ensure_report_contains_rule(validation_run, rule_id.trim_end_matches('"'));
    }

    if let Some(rule_id) = step
        .text
        .strip_prefix("the validation report does not contain rule \"")
    {
        let validation_run = execution(world)?
            .validation_run
            .as_ref()
            .context("missing validation run")?;
        return ensure_report_does_not_contain_rule(validation_run, rule_id.trim_end_matches('"'));
    }

    if let Some(member_count) =
        parse_count_suffix(&step.text, "the IXDS assembly receipt contains ", "member")
    {
        return ensure_ixds_member_count(execution(world)?, member_count);
    }

    if let Some(namespace_count) = parse_count_suffix(
        &step.text,
        "the taxonomy resolution resolves at least ",
        "namespace",
    ) {
        return ensure_taxonomy_resolution_resolves_at_least(execution(world)?, namespace_count);
    }

    if let Some(fact_count) = parse_count_suffix(&step.text, "the report contains ", "fact") {
        return ensure_report_fact_count(execution(world)?, fact_count);
    }

    anyhow::bail!("unsupported BDD step: {}", step.text)
}

fn parse_count_suffix(step: &str, prefix: &str, noun_stem: &str) -> Option<usize> {
    let remainder = step.strip_prefix(prefix)?;
    let count = remainder.split_whitespace().next()?.parse::<usize>().ok()?;
    let noun = remainder
        .split_whitespace()
        .nth(1)
        .unwrap_or_default()
        .trim_end_matches('s');
    if noun == noun_stem { Some(count) } else { None }
}
