//! Compile feature sidecars into a searchable grid.

use anyhow::Context;
use scenario_contract::{FeatureGrid, ScenarioRecord};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug, Deserialize)]
struct Sidecar {
    feature_id: String,
    layer: String,
    module: String,
    scenarios: BTreeMap<String, SidecarScenario>,
}

#[derive(Debug, Deserialize)]
struct SidecarScenario {
    ac_id: Option<String>,
    req_id: Option<String>,
    #[serde(default)]
    crates: Vec<String>,
    #[serde(default)]
    fixtures: Vec<String>,
    profile_pack: Option<String>,
    #[serde(default)]
    receipts: Vec<String>,
    #[serde(default)]
    allowed_edit_roots: Vec<String>,
    suite: Option<String>,
    speed: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SpecLedger {
    stories: Vec<SpecStory>,
}

#[derive(Debug, Deserialize)]
struct SpecStory {
    requirements: Vec<SpecRequirement>,
}

#[derive(Debug, Deserialize)]
struct SpecRequirement {
    acceptance_criteria: Vec<SpecAcceptanceCriterion>,
}

#[derive(Debug, Deserialize)]
struct SpecAcceptanceCriterion {
    id: String,
    #[serde(default)]
    tests: Vec<SpecTest>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
struct SpecTest {
    #[serde(rename = "type")]
    test_type: String,
    tag: String,
    file: String,
}

pub fn compile(root: &Path) -> anyhow::Result<FeatureGrid> {
    let features_root = root.join("specs/features");
    let test_declarations = load_test_declarations(root)?;
    let mut scenarios = Vec::new();
    for entry in WalkDir::new(&features_root)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let is_sidecar = path.extension().is_some_and(|ext| ext == "yaml")
            && path
                .file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.ends_with(".meta.yaml"));
        if !is_sidecar {
            continue;
        }
        let content =
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let sidecar: Sidecar = serde_yaml::from_str(&content)
            .with_context(|| format!("parsing {}", path.display()))?;
        let feature_file = sibling_feature(path);
        for (scenario_id, meta) in sidecar.scenarios {
            let test = declared_test_for(
                &test_declarations,
                &scenario_id,
                meta.ac_id.as_deref(),
                &repo_relative(root, &feature_file)?,
            )?;
            scenarios.push(ScenarioRecord {
                scenario_id,
                ac_id: meta.ac_id,
                req_id: meta.req_id,
                feature_file: repo_relative(root, &feature_file)?,
                sidecar_file: repo_relative(root, path)?,
                layer: sidecar.layer.clone(),
                module: format!("{}:{}", sidecar.feature_id, sidecar.module),
                crates: meta.crates,
                fixtures: meta.fixtures,
                profile_pack: meta.profile_pack,
                receipts: meta.receipts,
                allowed_edit_roots: meta.allowed_edit_roots,
                suite: meta.suite,
                speed: meta.speed,
                test_type: test.as_ref().map(|test| test.test_type.clone()),
                test_tag: test.as_ref().map(|test| test.tag.clone()),
            });
        }
    }
    scenarios.sort_by(|a, b| a.scenario_id.cmp(&b.scenario_id));
    Ok(FeatureGrid { scenarios })
}

fn load_test_declarations(root: &Path) -> anyhow::Result<BTreeMap<String, Vec<SpecTest>>> {
    let path = root.join("specs/spec_ledger.yaml");
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("reading specification ledger {}", path.display()))?;
    let ledger: SpecLedger = serde_yaml::from_str(&content)
        .with_context(|| format!("parsing specification ledger {}", path.display()))?;
    let mut declarations = BTreeMap::<String, Vec<SpecTest>>::new();
    for story in ledger.stories {
        for requirement in story.requirements {
            for criterion in requirement.acceptance_criteria {
                for test in criterion.tests {
                    declarations
                        .entry(criterion.id.clone())
                        .or_default()
                        .push(test);
                }
            }
        }
    }
    Ok(declarations)
}

fn declared_test_for(
    declarations: &BTreeMap<String, Vec<SpecTest>>,
    scenario_id: &str,
    ac_id: Option<&str>,
    feature_file: &str,
) -> anyhow::Result<Option<SpecTest>> {
    let scenario_tag = format!("@{scenario_id}");
    let selected = ac_id.and_then(|id| declarations.get(id)).and_then(|tests| {
        tests
            .iter()
            .find(|test| test.tag == scenario_tag)
            .or_else(|| tests.first())
    });
    let Some(test) = selected else {
        return Ok(None);
    };
    if test.file != feature_file {
        anyhow::bail!(
            "scenario {scenario_id} feature {feature_file} does not match declared test file {}",
            test.file
        );
    }
    Ok(Some(test.clone()))
}

fn sibling_feature(sidecar: &Path) -> PathBuf {
    let file_name = sidecar
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default()
        .replace(".meta.yaml", ".feature");
    sidecar.with_file_name(file_name)
}

fn repo_relative(root: &Path, path: &Path) -> anyhow::Result<String> {
    path.strip_prefix(root)
        .with_context(|| format!("stripping repo root from {}", path.display()))
        .map(|relative| relative.to_string_lossy().replace('\\', "/"))
}

#[cfg(test)]
mod tests {
    use super::{SpecTest, declared_test_for};
    use std::collections::BTreeMap;

    fn declaration(test_type: &str, tag: &str, file: &str) -> SpecTest {
        SpecTest {
            test_type: test_type.to_string(),
            tag: tag.to_string(),
            file: file.to_string(),
        }
    }

    #[test]
    fn scenario_tag_selects_the_matching_declaration() -> anyhow::Result<()> {
        let mut declarations = BTreeMap::new();
        declarations.insert(
            "AC-XK-WORKFLOW-003".to_string(),
            vec![
                declaration(
                    "bdd",
                    "@AC-XK-WORKFLOW-003",
                    "specs/features/workflow/alpha_check.feature",
                ),
                declaration(
                    "bdd",
                    "@SCN-XK-WORKFLOW-003",
                    "specs/features/workflow/alpha_check.feature",
                ),
            ],
        );

        let Some(selected) = declared_test_for(
            &declarations,
            "SCN-XK-WORKFLOW-003",
            Some("AC-XK-WORKFLOW-003"),
            "specs/features/workflow/alpha_check.feature",
        )?
        else {
            return Err(anyhow::anyhow!("expected a declared test"));
        };
        if selected.tag != "@SCN-XK-WORKFLOW-003" {
            return Err(anyhow::anyhow!("selected the wrong declaration"));
        }
        Ok(())
    }

    #[test]
    fn declaration_file_mismatch_fails_closed() -> anyhow::Result<()> {
        let mut declarations = BTreeMap::new();
        declarations.insert(
            "AC-XK-WORKFLOW-002".to_string(),
            vec![declaration(
                "bdd",
                "@AC-XK-WORKFLOW-002",
                "specs/features/workflow/bundle.feature",
            )],
        );
        let Err(error) = declared_test_for(
            &declarations,
            "SCN-XK-WORKFLOW-002",
            Some("AC-XK-WORKFLOW-002"),
            "specs/features/workflow/feature_grid.feature",
        ) else {
            return Err(anyhow::anyhow!("file mismatch unexpectedly succeeded"));
        };
        if !error
            .to_string()
            .contains("does not match declared test file")
        {
            return Err(anyhow::anyhow!("unexpected error: {error}"));
        }
        Ok(())
    }
}
