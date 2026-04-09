//! Core types for BDD step execution.

use anyhow::Context;
use scenario_contract::{BundleManifest, FeatureGrid, ScenarioRecord};
use scenario_runner::{ScenarioExecution, assert_scenario_outcome};
use std::path::PathBuf;

/// A BDD step with text and optional table data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub text: String,
    pub table: Vec<Vec<String>>,
}

/// The world state shared across BDD steps.
#[derive(Debug, Clone)]
pub struct World {
    pub repo_root: PathBuf,
    pub grid: FeatureGrid,
    pub profile_id: Option<String>,
    pub fixture_dirs: Vec<PathBuf>,
    pub execution: Option<ScenarioExecution>,
    pub dimension_context: DimensionContext,
    pub context_completeness_context: ContextCompletenessContext,
    pub streaming_context: StreamingContext,
    pub taxonomy_loader_context: TaxonomyLoaderContext,
    pub bundle_manifest: Option<BundleManifest>,
    pub validation_receipt: Option<receipt_types::Receipt>,
    pub sensor_report: Option<serde_json::Value>,
    pub filing_manifest: Option<edgar_attachments::FilingManifest>,
    pub filing_receipt: Option<receipt_types::Receipt>,
    pub compiled_grid: Option<FeatureGrid>,
    pub cli_output: Option<String>,
    pub cli_json_output: Option<serde_json::Value>,
    pub cli_exit_code: Option<i32>,
}

/// Context for dimension-related BDD steps.
#[derive(Debug, Clone, Default)]
pub struct DimensionContext {
    pub dimension: Option<String>,
    pub member: Option<String>,
    pub concept: Option<String>,
    pub required_dimension: Option<String>,
    pub validation_findings: Vec<String>,
    pub typed_value_type: Option<String>,
}

/// Context for context completeness validation steps.
#[derive(Debug, Clone, Default)]
pub struct ContextCompletenessContext {
    pub contexts: Vec<xbrl_contexts::Context>,
    pub facts: Vec<xbrl_report_types::Fact>,
    pub findings: Vec<xbrl_report_types::ValidationFinding>,
}

/// Context for streaming parser steps.
#[derive(Debug, Clone, Default)]
pub struct StreamingContext {
    pub file_size_mb: Option<f64>,
    pub fact_count: Option<usize>,
    pub memory_peak_mb: Option<f64>,
    pub facts_processed: Vec<xbrl_stream::StreamingFact>,
    pub contexts_collected: Vec<xbrl_stream::StreamingContext>,
    pub units_collected: Vec<xbrl_stream::StreamingUnit>,
    pub use_streaming: bool,
    pub missing_context_refs: Vec<String>,
}

/// Context for taxonomy loader steps.
#[derive(Debug, Clone, Default)]
pub struct TaxonomyLoaderContext {
    pub loader: Option<taxonomy_loader::TaxonomyLoader>,
    pub taxonomy: Option<taxonomy_dimensions::DimensionTaxonomy>,
    pub cache_dir: Option<PathBuf>,
    pub schema_path: Option<String>,
    pub loaded: bool,
}

impl World {
    /// Create a new World instance.
    #[must_use]
    pub fn new(repo_root: PathBuf, grid: FeatureGrid) -> Self {
        Self {
            repo_root,
            grid,
            profile_id: None,
            fixture_dirs: Vec::new(),
            execution: None,
            dimension_context: DimensionContext::default(),
            context_completeness_context: ContextCompletenessContext::default(),
            streaming_context: StreamingContext::default(),
            taxonomy_loader_context: TaxonomyLoaderContext::default(),
            bundle_manifest: None,
            validation_receipt: None,
            sensor_report: None,
            filing_manifest: None,
            filing_receipt: None,
            compiled_grid: None,
            cli_output: None,
            cli_json_output: None,
            cli_exit_code: None,
        }
    }
}

/// Run a scenario with the given steps.
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

    // Some scenarios (like dimension validation) validate via step assertions
    // and don't set execution. Skip scenario-level outcome check in that case.
    if let Some(execution) = world.execution.as_ref() {
        assert_scenario_outcome(scenario, execution)?;
    }

    Ok(())
}

fn run_step(world: &mut World, scenario: &ScenarioRecord, step: &Step) -> anyhow::Result<()> {
    if crate::given::handle_given(world, scenario, step)? {
        return Ok(());
    }
    if crate::when::handle_when(world, scenario, step)? {
        return Ok(());
    }
    crate::then::handle_then(world, step)
}

/// Get a reference to the current execution.
pub(crate) fn execution(world: &World) -> anyhow::Result<&ScenarioExecution> {
    world
        .execution
        .as_ref()
        .context("scenario step requires a prior execution")
}

/// Assert that declared inputs match scenario metadata.
pub(crate) fn assert_declared_inputs_match(world: &World, scenario: &ScenarioRecord) -> anyhow::Result<()> {
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
