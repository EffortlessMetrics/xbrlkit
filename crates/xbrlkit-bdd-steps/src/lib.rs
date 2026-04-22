//! Minimal step execution for the active BDD slices.

use anyhow::Context;
use scenario_contract::{FeatureGrid, ScenarioRecord};
use std::path::PathBuf;
use taxonomy_dimensions::{Dimension, DimensionTaxonomy, Domain, DomainMember, Hypercube};

mod given_steps;
mod parsing;
mod then_steps;
mod when_steps;

pub use parsing::parse_count_suffix;

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
    pub execution: Option<scenario_runner::ScenarioExecution>,
    pub dimension_context: DimensionContext,
    pub context_completeness_context: ContextCompletenessContext,
    pub streaming_context: StreamingContext,
    pub taxonomy_loader_context: TaxonomyLoaderContext,
    pub bundle_manifest: Option<scenario_contract::BundleManifest>,
    pub validation_receipt: Option<receipt_types::Receipt>,
    pub sensor_report: Option<serde_json::Value>,
    pub filing_manifest: Option<edgar_attachments::FilingManifest>,
    pub filing_receipt: Option<receipt_types::Receipt>,
    pub compiled_grid: Option<FeatureGrid>,
    pub cli_output: Option<String>,
    pub cli_json_output: Option<serde_json::Value>,
    pub cli_exit_code: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct DimensionContext {
    pub dimension: Option<String>,
    pub member: Option<String>,
    pub concept: Option<String>,
    pub required_dimension: Option<String>,
    pub validation_findings: Vec<String>,
    pub typed_value_type: Option<String>,
    pub explicit_dimension: Option<String>,
    pub explicit_member: Option<String>,
    pub typed_dimension: Option<String>,
    pub typed_member: Option<String>,
    pub segment_dimension: Option<String>,
    pub segment_member: Option<String>,
    pub parsed_dimensions: Vec<ParsedDimension>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedDimension {
    pub dimension: String,
    pub member: String,
    pub is_typed: bool,
    pub container: DimensionContainer,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum DimensionContainer {
    #[default]
    Scenario,
    Segment,
}

#[derive(Debug, Clone, Default)]
pub struct ContextCompletenessContext {
    pub contexts: Vec<xbrl_contexts::Context>,
    pub facts: Vec<xbrl_report_types::Fact>,
    pub findings: Vec<xbrl_report_types::ValidationFinding>,
}

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

#[derive(Debug, Clone, Default)]
pub struct TaxonomyLoaderContext {
    pub loader: Option<taxonomy_loader::TaxonomyLoader>,
    pub taxonomy: Option<DimensionTaxonomy>,
    pub cache_dir: Option<PathBuf>,
    pub schema_path: Option<String>,
    pub loaded: bool,
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

    if let Some(execution) = world.execution.as_ref() {
        scenario_runner::assert_scenario_outcome(scenario, execution)?;
    }

    Ok(())
}

fn run_step(world: &mut World, scenario: &ScenarioRecord, step: &Step) -> anyhow::Result<()> {
    if given_steps::handle_given(world, scenario, step)? {
        return Ok(());
    }
    if when_steps::handle_when(world, scenario, step)? {
        return Ok(());
    }
    then_steps::handle_then(world, step)
}

fn execution(world: &World) -> anyhow::Result<&scenario_runner::ScenarioExecution> {
    world
        .execution
        .as_ref()
        .context("scenario step requires a prior execution")
}

fn assert_declared_inputs_match(world: &World, scenario: &ScenarioRecord) -> anyhow::Result<()> {
    if let Some(profile_id) = &world.profile_id
        && scenario.profile_pack.as_deref() != Some(profile_id.as_str())
    {
        anyhow::bail!("declared profile pack does not match scenario metadata");
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

fn create_synthetic_taxonomy() -> DimensionTaxonomy {
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
        parent: Some("us-gaap:ScenarioActualMember".to_string()),
        order: 2,
        label: None,
    });
    scenario_domain.add_member(DomainMember {
        qname: "us-gaap:ScenarioBudgetMember".to_string(),
        parent: Some("us-gaap:ScenarioActualMember".to_string()),
        order: 3,
        label: None,
    });
    taxonomy.add_domain(scenario_domain);

    let mut product_domain = Domain::new("us-gaap:ProductDomain");
    product_domain.add_member(DomainMember {
        qname: "us-gaap:AllProductsMember".to_string(),
        parent: None,
        order: 1,
        label: None,
    });
    taxonomy.add_domain(product_domain);

    taxonomy.add_dimension(Dimension::Explicit {
        qname: "us-gaap:StatementScenarioAxis".to_string(),
        default_domain: Some("us-gaap:ScenarioDomain".to_string()),
        required: false,
    });
    taxonomy.dimension_domains.insert(
        "us-gaap:StatementScenarioAxis".to_string(),
        "us-gaap:ScenarioDomain".to_string(),
    );

    taxonomy.add_dimension(Dimension::Explicit {
        qname: "us-gaap:ProductAxis".to_string(),
        default_domain: Some("us-gaap:ProductDomain".to_string()),
        required: false,
    });
    taxonomy.dimension_domains.insert(
        "us-gaap:ProductAxis".to_string(),
        "us-gaap:ProductDomain".to_string(),
    );

    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:CustomerAxis".to_string(),
        value_type: "xs:string".to_string(),
        required: false,
    });
    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:ProductAxis".to_string(),
        value_type: "xs:string".to_string(),
        required: false,
    });
    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:EntityIdentifierAxis".to_string(),
        value_type: "xs:string".to_string(),
        required: false,
    });
    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:OptionalAxis".to_string(),
        value_type: "xs:string".to_string(),
        required: false,
    });
    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:AmountAxis".to_string(),
        value_type: "xs:decimal".to_string(),
        required: false,
    });
    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:ReportDateAxis".to_string(),
        value_type: "xs:date".to_string(),
        required: false,
    });
    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:IsActiveAxis".to_string(),
        value_type: "xs:boolean".to_string(),
        required: false,
    });
    taxonomy.add_dimension(Dimension::Typed {
        qname: "dim:CountAxis".to_string(),
        value_type: "xs:integer".to_string(),
        required: false,
    });

    let mut hypercube = Hypercube::new("us-gaap:StatementTable");
    hypercube.add_dimension("us-gaap:StatementScenarioAxis", false);
    hypercube.add_dimension("us-gaap:ProductAxis", false);
    taxonomy.add_hypercube(hypercube);

    taxonomy
}

fn select_matching_scenarios(grid: &FeatureGrid, selector: &str) -> Vec<ScenarioRecord> {
    grid.scenarios
        .iter()
        .filter(|scenario| selector_matches(scenario, selector))
        .cloned()
        .collect()
}

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
