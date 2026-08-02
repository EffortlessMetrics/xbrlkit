//! Compile feature sidecars into a searchable grid.

use anyhow::Context;
use scenario_contract::{FeatureGrid, ScenarioRecord};
use serde::Deserialize;
use std::collections::BTreeMap;
use std::fs;
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

pub fn compile(root: &Path) -> anyhow::Result<FeatureGrid> {
    let features_root = root.join("specs/features");
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
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let sidecar: Sidecar = serde_yaml::from_str(&content)
            .with_context(|| format!("parsing {}", path.display()))?;
        let feature_file = sibling_feature(path);
        let feature_tags = parse_feature_tags(&feature_file)?;
        for (scenario_id, meta) in sidecar.scenarios {
            let tags = feature_tags.get(&scenario_id).with_context(|| {
                format!(
                    "scenario {} from {} is missing from {}",
                    scenario_id,
                    path.display(),
                    feature_file.display()
                )
            })?;
            scenarios.push(ScenarioRecord {
                scenario_id,
                ac_id: meta.ac_id,
                req_id: meta.req_id,
                feature_file: repo_relative(root, &feature_file)?,
                sidecar_file: repo_relative(root, path)?,
                layer: sidecar.layer.clone(),
                module: format!("{}:{}", sidecar.feature_id, sidecar.module),
                tags: tags.clone(),
                crates: meta.crates,
                fixtures: meta.fixtures,
                profile_pack: meta.profile_pack,
                receipts: meta.receipts,
                allowed_edit_roots: meta.allowed_edit_roots,
                suite: meta.suite,
                speed: meta.speed,
            });
        }
    }
    scenarios.sort_by(|a, b| a.scenario_id.cmp(&b.scenario_id));
    Ok(FeatureGrid { scenarios })
}

fn parse_feature_tags(path: &Path) -> anyhow::Result<BTreeMap<String, Vec<String>>> {
    let content =
        fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let mut feature_tags = Vec::new();
    let mut pending_tags = Vec::new();
    let mut feature_header_seen = false;
    let mut current: Option<(String, Vec<String>)> = None;
    let mut scenarios = BTreeMap::new();

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('@') {
            let tags = line.split_whitespace();
            let target = if !feature_header_seen && current.is_none() {
                &mut feature_tags
            } else {
                &mut pending_tags
            };
            for tag in tags {
                if !tag.starts_with('@') || tag.len() == 1 {
                    anyhow::bail!("invalid tag {tag:?} in {}", path.display());
                }
                let tag = tag.to_string();
                if !target.contains(&tag) {
                    target.push(tag);
                }
            }
            continue;
        }
        if line.starts_with("Feature:") {
            feature_header_seen = true;
            continue;
        }
        if line.starts_with("Scenario:") || line.starts_with("Scenario Outline:") {
            if let Some((scenario_id, tags)) = current.take() {
                scenarios.insert(scenario_id, tags);
            }
            let mut tags = feature_tags.clone();
            for tag in pending_tags.drain(..) {
                if !tags.contains(&tag) {
                    tags.push(tag);
                }
            }
            let scenario_id = tags
                .iter()
                .find_map(|tag| {
                    tag.strip_prefix("@SCN-")
                        .filter(|suffix| !suffix.is_empty())
                        .map(|suffix| format!("SCN-{suffix}"))
                })
                .with_context(|| {
                    format!("scenario in {} is missing an @SCN tag", path.display())
                })?;
            current = Some((scenario_id, tags));
        }
    }

    if let Some((scenario_id, tags)) = current {
        scenarios.insert(scenario_id, tags);
    }
    Ok(scenarios)
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
    use super::compile;
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP_DIR: AtomicU64 = AtomicU64::new(0);

    struct TempRoot(PathBuf);

    impl Drop for TempRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    fn temp_root() -> Result<TempRoot, String> {
        let id = NEXT_TEMP_DIR.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("xbrlkit-feature-grid-{}-{id}", std::process::id()));
        fs::create_dir_all(root.join("specs/features/test")).map_err(|error| error.to_string())?;
        Ok(TempRoot(root))
    }

    fn write_inputs(root: &TempRoot, feature: &str, sidecar: &str) -> Result<(), String> {
        fs::write(root.0.join("specs/features/test/example.feature"), feature)
            .map_err(|error| error.to_string())?;
        fs::write(
            root.0.join("specs/features/test/example.meta.yaml"),
            sidecar,
        )
        .map_err(|error| error.to_string())
    }

    #[test]
    fn compile_preserves_inherited_and_scenario_tags_once() -> Result<(), String> {
        let root = temp_root()?;
        write_inputs(
            &root,
            "@REQ-TEST\n@alpha-active\nFeature: Example\n\n  @alpha-active @SCN-TEST-001\n  Scenario: One\n",
            "feature_id: FEAT-TEST\nlayer: foundation\nmodule: example\nscenarios:\n  SCN-TEST-001:\n    ac_id: AC-TEST-001\n    req_id: REQ-TEST\n",
        )?;

        let grid = compile(&root.0).map_err(|error| error.to_string())?;
        let scenario = grid
            .scenarios
            .first()
            .ok_or_else(|| "compiled grid should contain one scenario".to_string())?;
        if scenario.tags
            != vec![
                "@REQ-TEST".to_string(),
                "@alpha-active".to_string(),
                "@SCN-TEST-001".to_string(),
            ]
        {
            return Err(format!("unexpected tags: {:?}", scenario.tags));
        }
        Ok(())
    }

    #[test]
    fn compile_rejects_scenario_without_id_tag() -> Result<(), String> {
        let root = temp_root()?;
        write_inputs(
            &root,
            "@REQ-TEST\nFeature: Example\n\n  Scenario: Missing ID\n",
            "feature_id: FEAT-TEST\nlayer: foundation\nmodule: example\nscenarios:\n  SCN-TEST-001:\n    ac_id: AC-TEST-001\n    req_id: REQ-TEST\n",
        )?;

        let error = match compile(&root.0) {
            Ok(_) => return Err("feature without scenario ID should fail".to_string()),
            Err(error) => error.to_string(),
        };
        if !error.contains("missing an @SCN tag") {
            return Err(format!("unexpected parse error: {error}"));
        }
        Ok(())
    }

    #[test]
    fn compile_rejects_malformed_tag_token() -> Result<(), String> {
        let root = temp_root()?;
        write_inputs(
            &root,
            "@REQ-TEST invalid\nFeature: Example\n\n  @SCN-TEST-001\n  Scenario: Invalid tag\n",
            "feature_id: FEAT-TEST\nlayer: foundation\nmodule: example\nscenarios:\n  SCN-TEST-001:\n    ac_id: AC-TEST-001\n    req_id: REQ-TEST\n",
        )?;

        let error = match compile(&root.0) {
            Ok(_) => return Err("malformed tag should fail".to_string()),
            Err(error) => error.to_string(),
        };
        if !error.contains("invalid tag \"invalid\"") {
            return Err(format!("unexpected parse error: {error}"));
        }
        Ok(())
    }

    #[test]
    fn compile_rejects_empty_scenario_id_suffix() -> Result<(), String> {
        let root = temp_root()?;
        write_inputs(
            &root,
            "Feature: Example\n\n  @SCN-\n  Scenario: Empty ID\n",
            "feature_id: FEAT-TEST\nlayer: foundation\nmodule: example\nscenarios:\n  SCN-:\n    ac_id: AC-TEST-001\n    req_id: REQ-TEST\n",
        )?;

        let error = match compile(&root.0) {
            Ok(_) => return Err("empty scenario ID suffix should fail".to_string()),
            Err(error) => error.to_string(),
        };
        if !error.contains("missing an @SCN tag") {
            return Err(format!("unexpected parse error: {error}"));
        }
        Ok(())
    }

    #[test]
    fn compile_supports_scenario_outline() -> Result<(), String> {
        let root = temp_root()?;
        write_inputs(
            &root,
            "Feature: Example\n\n  @SCN-TEST-001\n  Scenario Outline: Outline\n    Given a value <value>\n\n    Examples:\n      | value |\n      | one   |\n",
            "feature_id: FEAT-TEST\nlayer: foundation\nmodule: example\nscenarios:\n  SCN-TEST-001:\n    ac_id: AC-TEST-001\n    req_id: REQ-TEST\n",
        )?;

        let grid = compile(&root.0).map_err(|error| error.to_string())?;
        if grid.scenarios.len() != 1 || grid.scenarios[0].scenario_id != "SCN-TEST-001" {
            return Err("scenario outline should compile into its sidecar scenario".to_string());
        }
        Ok(())
    }
}
