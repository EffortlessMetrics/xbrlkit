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
            std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let sidecar: Sidecar = serde_yaml::from_str(&content)
            .with_context(|| format!("parsing {}", path.display()))?;
        let feature_file = sibling_feature(path);
        for (scenario_id, meta) in sidecar.scenarios {
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
            });
        }
    }
    scenarios.sort_by(|a, b| a.scenario_id.cmp(&b.scenario_id));
    Ok(FeatureGrid { scenarios })
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
