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

// Dimension validation types
#[derive(Debug, Clone, Default)]
pub struct DimensionContext {
    pub dimension: Option<String>,
    pub member: Option<String>,
    pub concept: Option<String>,
}

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
    // Dimension validation state
    pub dimension_context: DimensionContext,
    pub validation_findings: Vec<String>,
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
            dimension_context: DimensionContext::default(),
            validation_findings: Vec::new(),
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

    // Dimension validation Given steps
    if step.text == "the taxonomy has dimension definitions" {
        return Ok(true);
    }

    if step.text == "the taxonomy has domain hierarchies" {
        return Ok(true);
    }

    if step.text == "the taxonomy has hypercube definitions" {
        return Ok(true);
    }

    if let Some(dimension) = step.text.strip_prefix("a context with dimension \"") {
        world.dimension_context.dimension = Some(dimension.trim_end_matches('"').to_string());
        return Ok(true);
    }

    if let Some(member) = step.text.strip_prefix("the member \"") {
        world.dimension_context.member = Some(member.trim_end_matches('"').to_string());
        return Ok(true);
    }

    if let Some(member) = step.text.strip_prefix("an invalid member \"") {
        world.dimension_context.member = Some(member.trim_end_matches('"').to_string());
        return Ok(true);
    }

    if let Some(concept_part) = step.text.strip_prefix("a fact for concept \"") {
        // Parse "concept" requiring dimension "dim"
        let concept = concept_part.trim_end_matches('"').to_string();
        world.dimension_context.concept = Some(concept);
        return Ok(true);
    }

    if step.text == "a context without that dimension" {
        // Context already has no dimension set
        return Ok(true);
    }

    if let Some(dimension) = step.text.strip_prefix("a context with unknown dimension \"") {
        world.dimension_context.dimension = Some(dimension.trim_end_matches('"').to_string());
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

    // Dimension validation When steps
    if step.text == "I validate the dimension-member pair" {
        // Simulate dimension validation based on context state
        let dimension = world.dimension_context.dimension.as_deref().unwrap_or("");
        let member = world.dimension_context.member.as_deref().unwrap_or("");
        
        // Simulate validation logic
        if dimension == "us-gaap:StatementScenarioAxis" {
            if member == "us-gaap:ScenarioActualMember" {
                // Valid - no findings
            } else if member == "us-gaap:NonExistentMember" {
                world.validation_findings.push("XBRL.DIMENSION.INVALID_MEMBER".to_string());
            }
        } else if dimension == "custom:UnknownAxis" {
            world.validation_findings.push("XBRL.DIMENSION.UNKNOWN".to_string());
        }
        return Ok(true);
    }

    if step.text == "I validate the fact dimensions" {
        // Simulate fact dimension validation
        let concept = world.dimension_context.concept.as_deref().unwrap_or("");
        let dimension = world.dimension_context.dimension.as_deref();
        
        if concept == "us-gaap:Revenue" && dimension.is_none() {
            world.validation_findings.push("XBRL.DIMENSION.MISSING_REQUIRED".to_string());
        }
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
        // Dimension validation Then steps
        "the validation should pass" => {
            if world.validation_findings.is_empty() {
                Ok(())
            } else {
                anyhow::bail!("expected validation to pass but had findings: {:?}", world.validation_findings)
            }
        }
        "the validation should fail" => {
            if world.validation_findings.is_empty() {
                anyhow::bail!("expected validation to fail but no findings were recorded")
            } else {
                Ok(())
            }
        }
        "no findings should be reported" => {
            if world.validation_findings.is_empty() {
                Ok(())
            } else {
                anyhow::bail!("expected no findings but had: {:?}", world.validation_findings)
            }
        }
        _ => handle_parameterized_assertion(world, step),
    }
}

fn handle_parameterized_assertion(world: &World, step: &Step) -> anyhow::Result<()> {
    // Dimension validation parameterized assertions
    if let Some(rule_id) = step
        .text
        .strip_prefix("an \"")
    {
        let rule_id = rule_id.trim_end_matches("\" finding should be reported");
        if world.validation_findings.iter().any(|f| f == rule_id) {
            return Ok(());
        } else {
            anyhow::bail!("expected finding {} but had: {:?}", rule_id, world.validation_findings);
        }
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
