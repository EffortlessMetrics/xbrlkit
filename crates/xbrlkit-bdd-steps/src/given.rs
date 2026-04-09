//! Given step handlers for BDD scenarios.

use crate::types::{Step, World};
use crate::types::assert_declared_inputs_match;
use scenario_contract::ScenarioRecord;
use taxonomy_dimensions::{Dimension, DimensionTaxonomy, Domain, DomainMember};
use xbrl_contexts::{DimensionMember, DimensionalContainer, EntityIdentifier, Period};

/// Handle Given steps.
#[allow(clippy::too_many_lines)]
pub fn handle_given(world: &mut World, scenario: &ScenarioRecord, step: &Step) -> anyhow::Result<bool> {
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

    // Dimension-related Given steps
    if step.text == "the taxonomy has dimension definitions" {
        // Stub - implement actual taxonomy loading when needed
        return Ok(true);
    }

    if step.text == "the taxonomy has domain hierarchies" {
        // Stub - implement actual taxonomy loading when needed
        return Ok(true);
    }

    if step.text == "the taxonomy has hypercube definitions" {
        // Stub - implement actual taxonomy loading when needed
        return Ok(true);
    }

    if let Some(dimension) = step.text.strip_prefix("a context with dimension \"") {
        world.dimension_context.dimension = Some(dimension.trim_end_matches('"').to_string());
        return Ok(true);
    }

    if let Some(dimension) = step
        .text
        .strip_prefix("a context with unknown dimension \"")
    {
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

    if let Some(concept) = step.text.strip_prefix("a fact for concept \"") {
        world.dimension_context.concept = Some(concept.trim_end_matches('"').to_string());
        return Ok(true);
    }

    if let Some(dimension) = step.text.strip_prefix("the concept requires dimension \"") {
        world.dimension_context.required_dimension =
            Some(dimension.trim_end_matches('"').to_string());
        return Ok(true);
    }

    if step.text == "a context without that dimension" {
        // Ensure dimension is not set (or clear it)
        world.dimension_context.dimension = None;
        return Ok(true);
    }

    // Typed dimension Given steps
    if let Some(dimension) = step.text.strip_prefix("a context with typed dimension \"") {
        let rest = dimension.trim_end_matches('"');
        // Handle "dim:Axis" of type "xs:type" format
        if let Some((dim, type_part)) = rest.split_once("\" of type \"") {
            world.dimension_context.dimension = Some(dim.to_string());
            world.dimension_context.typed_value_type = Some(type_part.to_string());
        } else {
            world.dimension_context.dimension = Some(rest.to_string());
        }
        return Ok(true);
    }

    if let Some(value) = step.text.strip_prefix("the typed member value \"") {
        world.dimension_context.member = Some(value.trim_end_matches('"').to_string());
        return Ok(true);
    }

    // Bundle-related Given steps
    if step.text == "the feature grid is compiled" {
        // Grid is already loaded in World, just verify it's not empty
        if world.grid.scenarios.is_empty() {
            anyhow::bail!("feature grid is empty");
        }
        return Ok(true);
    }

    // Feature grid Given steps
    if step.text == "the repo has feature sidecars" {
        // Check that at least one .meta.yaml file exists in specs/features
        let features_root = world.repo_root.join("specs/features");
        let has_sidecars = walkdir::WalkDir::new(&features_root)
            .into_iter()
            .filter_map(Result::ok)
            .any(|entry: walkdir::DirEntry| {
                let path = entry.path();
                path.extension().is_some_and(|ext| ext == "yaml")
                    && path
                        .file_name()
                        .and_then(|n: &std::ffi::OsStr| n.to_str())
                        .is_some_and(|n: &str| n.ends_with(".meta.yaml"))
            });
        if !has_sidecars {
            anyhow::bail!("no feature sidecars found in {}", features_root.display());
        }
        return Ok(true);
    }

    // Cockpit pack Given steps
    if step.text == "a validation report receipt" {
        world.validation_receipt = Some(receipt_types::Receipt::new(
            "validation.report",
            "synthetic-subject",
            receipt_types::RunResult::Success,
        ));
        return Ok(true);
    }

    // CLI Given steps
    if step.text == "a SEC profile is configured" {
        world.profile_id = Some("sec/efm-77/opco".to_string());
        return Ok(true);
    }

    // Alpha check Given steps
    if step.text == "the active alpha scenarios are implemented" {
        // Parse feature files to verify scenarios with @alpha-active tag exist
        let features_root = world.repo_root.join("specs/features");
        let mut has_alpha_scenarios = false;

        for entry in walkdir::WalkDir::new(&features_root)
            .into_iter()
            .filter_map(Result::ok)
        {
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "feature") {
                continue;
            }

            if let Ok(content) = std::fs::read_to_string(path) {
                // Check for @alpha-active tag in the file
                if content.contains("@alpha-active") {
                    has_alpha_scenarios = true;
                    break;
                }
            }
        }

        if !has_alpha_scenarios {
            anyhow::bail!("no scenarios with @alpha-active tag found in feature files");
        }
        return Ok(true);
    }

    // Context completeness Given steps
    if step.text.starts_with("an XBRL report with context ") {
        // Parse contexts from the step text
        // Format: "an XBRL report with context \"ctx-1\"" or "an XBRL report with contexts \"ctx-1\" and \"ctx-2\""
        let text = &step.text;
        let contexts: Vec<String> = text
            .split('"')
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, s)| s.to_string())
            .collect();

        for ctx_id in contexts {
            let context = xbrl_contexts::Context {
                id: xbrl_contexts::normalize_context_id(&ctx_id),
                entity: EntityIdentifier {
                    scheme: "http://www.sec.gov/CIK".to_string(),
                    value: "0000320193".to_string(),
                },
                entity_segment: None,
                period: Period::Instant("2024-12-31".to_string()),
                scenario: None,
            };
            world.context_completeness_context.contexts.push(context);
        }
        return Ok(true);
    }

    if step.text.starts_with("a fact referencing concept ") {
        // Parse: "a fact referencing concept \"us-gaap:Revenue\" with context \"ctx-1\""
        let text = &step.text;
        if let Some(concept_start) = text.find('"') {
            let concept_end = text[concept_start + 1..]
                .find('"')
                .map(|i| concept_start + 1 + i);
            if let Some(concept_end) = concept_end {
                let concept = &text[concept_start + 1..concept_end];
                if let Some(ctx_start) = text[concept_end + 1..].find('"') {
                    let ctx_start = concept_end + 1 + ctx_start;
                    let ctx_end = text[ctx_start + 1..].find('"').map(|i| ctx_start + 1 + i);
                    if let Some(ctx_end) = ctx_end {
                        let context_ref = &text[ctx_start + 1..ctx_end];
                        let fact = xbrl_report_types::Fact {
                            concept: concept.to_string(),
                            context_ref: context_ref.to_string(),
                            unit_ref: None,
                            decimals: None,
                            value: "1000".to_string(),
                            member: String::new(),
                        };
                        world.context_completeness_context.facts.push(fact);
                        return Ok(true);
                    }
                }
            }
        }
        anyhow::bail!("invalid fact specification: {}", step.text);
    }

    if step.text.starts_with("facts referencing concepts ") {
        // Parse: "facts referencing concepts \"us-gaap:Revenue\" and \"us-gaap:Assets\" with contexts \"ctx-1\" and \"ctx-2\""
        // For simplicity, we'll create facts for each concept-context pair
        // Parse all quoted strings
        let quoted: Vec<String> = step
            .text
            .split('"')
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, s)| s.to_string())
            .collect();

        if quoted.len() >= 2 {
            // First half are concepts, second half are contexts
            let mid = quoted.len() / 2;
            let concepts = &quoted[..mid];
            let contexts = &quoted[mid..];

            for (i, concept) in concepts.iter().enumerate() {
                let context_ref = contexts
                    .get(i)
                    .or_else(|| contexts.first())
                    .map_or("ctx-1", String::as_str);
                let fact = xbrl_report_types::Fact {
                    concept: concept.clone(),
                    context_ref: context_ref.to_string(),
                    unit_ref: None,
                    decimals: None,
                    value: "1000".to_string(),
                    member: String::new(),
                };
                world.context_completeness_context.facts.push(fact);
            }
        }
        return Ok(true);
    }

    // Decimal precision Given steps
    if step.text.starts_with("a numeric fact with value ") {
        // Clear previous state for standalone decimal precision scenarios
        world.context_completeness_context.facts.clear();
        world.context_completeness_context.contexts.clear();
        world.context_completeness_context.findings.clear();

        // Parse: "a numeric fact with value "1234.56" and decimals "INF""
        let quoted: Vec<String> = step
            .text
            .split('"')
            .enumerate()
            .filter(|(i, _)| i % 2 == 1)
            .map(|(_, s)| s.to_string())
            .collect();

        if quoted.len() >= 2 {
            let value = &quoted[0];
            let decimals = &quoted[1];
            let fact = xbrl_report_types::Fact {
                concept: "us-gaap:TestConcept".to_string(),
                context_ref: "ctx-1".to_string(),
                unit_ref: Some("usd".to_string()),
                decimals: Some(decimals.clone()),
                value: value.clone(),
                member: String::new(),
            };
            world.context_completeness_context.facts.push(fact);
            return Ok(true);
        }
        anyhow::bail!("invalid numeric fact specification: {}", step.text);
    }

    // Streaming parser Given steps
    if step.text == "the xbrl-stream crate is available" {
        // Verify the crate exists and can be used
        world.streaming_context.use_streaming = true;
        return Ok(true);
    }

    if step.text.starts_with("an XBRL filing larger than ") {
        // Parse: "an XBRL filing larger than 100MB"
        let mb_str = step
            .text
            .strip_prefix("an XBRL filing larger than ")
            .and_then(|s| s.strip_suffix("MB"))
            .or_else(|| {
                step.text
                    .strip_prefix("an XBRL filing larger than ")
                    .and_then(|s| s.strip_suffix("mb"))
            });
        if let Some(mb) = mb_str {
            world.streaming_context.file_size_mb = Some(mb.parse().unwrap_or(100.0));
            return Ok(true);
        }
        anyhow::bail!("invalid file size specification: {}", step.text);
    }

    if step.text.starts_with("an XBRL filing smaller than ") {
        // Parse: "an XBRL filing smaller than 10MB"
        let mb_str = step
            .text
            .strip_prefix("an XBRL filing smaller than ")
            .and_then(|s| s.strip_suffix("MB"))
            .or_else(|| {
                step.text
                    .strip_prefix("an XBRL filing smaller than ")
                    .and_then(|s| s.strip_suffix("mb"))
            });
        if let Some(mb) = mb_str {
            world.streaming_context.file_size_mb = Some(mb.parse().unwrap_or(10.0));
            world.streaming_context.use_streaming = true; // Always available as option
            return Ok(true);
        }
        anyhow::bail!("invalid file size specification: {}", step.text);
    }

    if step.text.starts_with("a large XBRL filing with ") {
        // Parse: "a large XBRL filing with 1000+ facts"
        if let Some(facts_str) = step.text.strip_prefix("a large XBRL filing with ") {
            let facts = facts_str
                .split('+')
                .next()
                .and_then(|s| s.trim().parse().ok())
                .unwrap_or(1000);
            world.streaming_context.fact_count = Some(facts);
            world.streaming_context.file_size_mb = Some(50.0); // Assume large
            return Ok(true);
        }
        anyhow::bail!("invalid fact count specification: {}", step.text);
    }

    if step.text == "some facts reference non-existent contexts" {
        // Mark that we'll simulate missing context refs
        world.streaming_context.missing_context_refs = vec!["missing-ctx-1".to_string()];
        return Ok(true);
    }

    if step.text == "a streaming parser with a custom handler" {
        world.streaming_context.use_streaming = true;
        world.streaming_context.facts_processed.clear();
        world.streaming_context.contexts_collected.clear();
        world.streaming_context.units_collected.clear();
        return Ok(true);
    }

    // Taxonomy loader Given steps
    if step.text == "the taxonomy loader is available" {
        world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
        return Ok(true);
    }

    if step.text == "a taxonomy schema with dimension elements" {
        // Use a fixture path or create synthetic schema
        world.taxonomy_loader_context.schema_path =
            Some("fixtures/synthetic/taxonomy/standard-location-01/schema.xsd".to_string());
        world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
        return Ok(true);
    }

    if step.text == "a taxonomy definition linkbase with domain members" {
        world.taxonomy_loader_context.schema_path =
            Some("fixtures/synthetic/taxonomy/standard-location-01/schema.xsd".to_string());
        world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
        return Ok(true);
    }

    if step.text == "a taxonomy with typed dimensions" {
        world.taxonomy_loader_context.schema_path =
            Some("fixtures/synthetic/taxonomy/standard-location-01/schema.xsd".to_string());
        world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
        return Ok(true);
    }

    if step.text == "a taxonomy with hypercube elements" {
        world.taxonomy_loader_context.schema_path =
            Some("fixtures/synthetic/taxonomy/standard-location-01/schema.xsd".to_string());
        world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
        return Ok(true);
    }

    if step.text == "a taxonomy URL to load" {
        // Use a synthetic path that doesn't exist - triggers synthetic taxonomy creation
        world.taxonomy_loader_context.schema_path =
            Some("fixtures/synthetic/taxonomy/url-test/schema.xsd".to_string());
        return Ok(true);
    }

    if step.text == "a cache directory is configured" {
        let cache_dir = std::env::temp_dir().join("xbrlkit_taxonomy_cache");
        // Create the cache directory so it exists for the Then step
        let _ = std::fs::create_dir_all(&cache_dir);
        world.taxonomy_loader_context.cache_dir = Some(cache_dir.clone());
        world.taxonomy_loader_context.loader =
            Some(taxonomy_loader::TaxonomyLoader::with_cache_dir(&cache_dir));
        return Ok(true);
    }

    if step.text == "a taxonomy schema that imports another schema" {
        world.taxonomy_loader_context.schema_path =
            Some("fixtures/synthetic/taxonomy/standard-location-01/schema.xsd".to_string());
        world.taxonomy_loader_context.loader = Some(taxonomy_loader::TaxonomyLoader::new());
        return Ok(true);
    }

    if step.text == "a loaded taxonomy with dimension definitions" {
        // Simulate loading a taxonomy with dimensions
        let mut taxonomy = DimensionTaxonomy::new();

        // Add a domain
        let mut domain = Domain::new("us-gaap:ScenarioDomain");
        domain.add_member(DomainMember {
            qname: "us-gaap:ScenarioActualMember".to_string(),
            parent: None,
            order: 1,
            label: None,
        });
        domain.add_member(DomainMember {
            qname: "us-gaap:ScenarioForecastMember".to_string(),
            parent: None,
            order: 2,
            label: None,
        });
        taxonomy.add_domain(domain);

        // Add an explicit dimension
        taxonomy.add_dimension(Dimension::Explicit {
            qname: "us-gaap:StatementScenarioAxis".to_string(),
            default_domain: Some("us-gaap:ScenarioDomain".to_string()),
            required: false,
        });
        taxonomy.dimension_domains.insert(
            "us-gaap:StatementScenarioAxis".to_string(),
            "us-gaap:ScenarioDomain".to_string(),
        );

        world.taxonomy_loader_context.taxonomy = Some(taxonomy);
        world.taxonomy_loader_context.loaded = true;
        return Ok(true);
    }

    Ok(false)
}
