//! Scenario grid and bundle contracts.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ScenarioRecord {
    pub scenario_id: String,
    pub ac_id: Option<String>,
    pub req_id: Option<String>,
    pub feature_file: String,
    pub sidecar_file: String,
    pub layer: String,
    pub module: String,
    #[serde(default)]
    pub crates: Vec<String>,
    #[serde(default)]
    pub fixtures: Vec<String>,
    pub profile_pack: Option<String>,
    #[serde(default)]
    pub receipts: Vec<String>,
    #[serde(default)]
    pub allowed_edit_roots: Vec<String>,
    pub suite: Option<String>,
    pub speed: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FeatureGrid {
    #[serde(default)]
    pub scenarios: Vec<ScenarioRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct BundleManifest {
    pub selector: String,
    #[serde(default)]
    pub scenarios: Vec<ScenarioRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ImpactReport {
    #[serde(default)]
    pub changed_paths: Vec<String>,
    #[serde(default)]
    pub impacted_scenarios: Vec<String>,
}
