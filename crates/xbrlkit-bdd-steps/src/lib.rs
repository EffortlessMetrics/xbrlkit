//! Minimal step execution for the active BDD slices.
//!
//! Phase-specific logic lives in [`given`], [`when`], and [`then`] submodules.

mod given;
mod then;
mod when;

pub mod world;

pub use world::{Step, World};

use anyhow::Context;
use scenario_contract::ScenarioRecord;
use scenario_runner::{ScenarioExecution, assert_scenario_outcome};

pub fn run_scenario(
    world: &mut World,
    scenario: &ScenarioRecord,
    steps: &[Step],
) -> anyhow::Result<()> {
    if !world
        .execution
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

    if let Some(execution) = world.execution.execution.as_ref() {
        assert_scenario_outcome(scenario, execution)?;
    }

    Ok(())
}

fn run_step(world: &mut World, scenario: &ScenarioRecord, step: &Step) -> anyhow::Result<()> {
    if given::handle(world, scenario, step)? {
        return Ok(());
    }
    if when::handle(world, scenario, step)? {
        return Ok(());
    }
    then::handle(world, step)
}

pub(crate) fn execution(world: &World) -> anyhow::Result<&ScenarioExecution> {
    world
        .execution
        .execution
        .as_ref()
        .context("scenario step requires a prior execution")
}

pub(crate) fn assert_declared_inputs_match(
    world: &World,
    scenario: &ScenarioRecord,
) -> anyhow::Result<()> {
    if let Some(profile_id) = &world.execution.profile_id
        && scenario.profile_pack.as_deref() != Some(profile_id.as_str())
    {
        anyhow::bail!("declared profile pack does not match scenario metadata");
    }

    if !world.execution.fixture_dirs.is_empty() {
        let declared = world
            .execution
            .fixture_dirs
            .iter()
            .map(|path| {
                path.strip_prefix(world.execution.repo_root.join("fixtures"))
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
