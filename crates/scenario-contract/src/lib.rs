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

impl ScenarioRecord {
    /// Check if this scenario matches a selector string.
    ///
    /// Supported selector patterns:
    /// - Bare `scenario_id` (e.g. `SCN-XK-WORKFLOW-002`)
    /// - `@`-prefixed `scenario_id` (e.g. `@SCN-XK-WORKFLOW-002`)
    /// - Bare `ac_id` (e.g. `AC-XK-WORKFLOW-002`)
    /// - `@`-prefixed `ac_id` (e.g. `@AC-XK-WORKFLOW-002`)
    /// - Bare `req_id` (e.g. `REQ-XK-WORKFLOW`)
    #[must_use] 
    pub fn matches_selector(&self, selector: &str) -> bool {
        self.scenario_id == selector
            || self.ac_id.as_deref() == Some(selector)
            || self.req_id.as_deref() == Some(selector)
            || format!("@{}", self.scenario_id) == selector
            || self
                .ac_id
                .as_ref()
                .is_some_and(|ac| format!("@{ac}") == selector)
    }
}

impl FeatureGrid {
    /// Select all scenarios matching the given selector.
    #[must_use] 
    pub fn select_by_selector(&self, selector: &str) -> Vec<ScenarioRecord> {
        self.scenarios
            .iter()
            .filter(|s| s.matches_selector(selector))
            .cloned()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{FeatureGrid, ScenarioRecord};

    fn scenario_record() -> ScenarioRecord {
        ScenarioRecord {
            scenario_id: "SCN-XK-WORKFLOW-002".to_string(),
            ac_id: Some("AC-XK-WORKFLOW-002".to_string()),
            req_id: Some("REQ-XK-WORKFLOW".to_string()),
            feature_file: "specs/features/workflow/bundle.feature".to_string(),
            sidecar_file: "specs/features/workflow/bundle.meta.yaml".to_string(),
            layer: "workflow".to_string(),
            module: "bundle".to_string(),
            crates: vec!["xtask".to_string()],
            fixtures: Vec::new(),
            profile_pack: None,
            receipts: vec!["bundle.manifest.v1".to_string()],
            allowed_edit_roots: vec![
                "specs/features/workflow".to_string(),
                "xtask".to_string(),
            ],
            suite: Some("synthetic".to_string()),
            speed: Some("fast".to_string()),
        }
    }

    #[test]
    fn matches_selector_by_scenario_id() {
        let scenario = scenario_record();
        assert!(scenario.matches_selector("SCN-XK-WORKFLOW-002"));
    }

    #[test]
    fn matches_selector_by_ac_id() {
        let scenario = scenario_record();
        assert!(scenario.matches_selector("AC-XK-WORKFLOW-002"));
    }

    #[test]
    fn matches_selector_by_req_id() {
        let scenario = scenario_record();
        assert!(scenario.matches_selector("REQ-XK-WORKFLOW"));
    }

    #[test]
    fn matches_selector_by_tag_style_scenario_id() {
        let scenario = scenario_record();
        assert!(scenario.matches_selector("@SCN-XK-WORKFLOW-002"));
    }

    #[test]
    fn matches_selector_by_tag_style_ac_id() {
        let scenario = scenario_record();
        assert!(scenario.matches_selector("@AC-XK-WORKFLOW-002"));
    }

    #[test]
    fn matches_selector_no_match() {
        let scenario = scenario_record();
        assert!(!scenario.matches_selector("AC-XK-DOES-NOT-EXIST"));
    }

    #[test]
    fn select_by_selector_returns_all_matching() {
        let grid = FeatureGrid {
            scenarios: vec![scenario_record(), scenario_record()],
        };
        let matched = grid.select_by_selector("AC-XK-WORKFLOW-002");
        assert_eq!(matched.len(), 2);
    }

    #[test]
    fn select_by_selector_empty_grid() {
        let grid = FeatureGrid::default();
        assert!(grid.select_by_selector("AC-XK-WORKFLOW-002").is_empty());
    }
}
