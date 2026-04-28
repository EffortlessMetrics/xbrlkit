//! World state and context structs for BDD step handlers.

use scenario_contract::{BundleManifest, FeatureGrid};
use scenario_runner::ScenarioExecution;
use std::path::PathBuf;
use taxonomy_dimensions::{Dimension, DimensionTaxonomy, Domain, DomainMember};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub text: String,
    pub table: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct World {
    pub execution: ExecutionContext,
    pub dimension: DimensionContext,
    pub completeness: ContextCompletenessContext,
    pub processing: ProcessingContext,
    pub output: OutputContext,
}

#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub repo_root: PathBuf,
    pub grid: FeatureGrid,
    pub profile_id: Option<String>,
    pub fixture_dirs: Vec<PathBuf>,
    pub execution: Option<ScenarioExecution>,
    pub compiled_grid: Option<FeatureGrid>,
}

#[derive(Debug, Clone, Default)]
pub struct DimensionContext {
    pub dimension: Option<String>,
    pub member: Option<String>,
    pub concept: Option<String>,
    pub required_dimension: Option<String>,
    pub validation_findings: Vec<String>,
    pub typed_value_type: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ContextCompletenessContext {
    pub contexts: Vec<xbrl_contexts::Context>,
    pub facts: Vec<xbrl_report_types::Fact>,
    pub findings: Vec<xbrl_report_types::ValidationFinding>,
}

#[derive(Debug, Clone, Default)]
pub struct StreamProcessingContext {
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

#[derive(Debug, Clone, Default)]
pub struct ProcessingContext {
    pub streaming: StreamProcessingContext,
    pub taxonomy_loader: TaxonomyLoaderContext,
}

#[derive(Debug, Clone, Default)]
pub struct PackageCheckContext {
    pub publishable_crates: Vec<String>,
    pub package_results: Vec<(String, bool, String)>,
}

#[derive(Debug, Clone, Default)]
pub struct OutputContext {
    pub bundle_manifest: Option<BundleManifest>,
    pub validation_receipt: Option<receipt_types::Receipt>,
    pub sensor_report: Option<serde_json::Value>,
    pub filing_manifest: Option<edgar_attachments::FilingManifest>,
    pub filing_receipt: Option<receipt_types::Receipt>,
    pub cli_output: Option<String>,
    pub cli_json_output: Option<serde_json::Value>,
    pub cli_exit_code: Option<i32>,
    pub package_check: PackageCheckContext,
}

impl World {
    #[must_use]
    pub fn new(repo_root: PathBuf, grid: FeatureGrid) -> Self {
        Self {
            execution: ExecutionContext {
                repo_root,
                grid,
                profile_id: None,
                fixture_dirs: Vec::new(),
                execution: None,
                compiled_grid: None,
            },
            dimension: DimensionContext::default(),
            completeness: ContextCompletenessContext::default(),
            processing: ProcessingContext {
                streaming: StreamProcessingContext::default(),
                taxonomy_loader: TaxonomyLoaderContext::default(),
            },
            output: OutputContext::default(),
        }
    }
}

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

/// Parse a count suffix from a BDD step text.
///
/// Example: `"the report contains 42 facts"` with prefix `"the report contains "`
/// and noun stem `"fact"` returns `Some(42)`.
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
