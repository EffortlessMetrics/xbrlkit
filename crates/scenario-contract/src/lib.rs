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
    /// Check whether this scenario matches a selector.
    ///
    /// Selectors may use the scenario ID, acceptance-criteria ID,
    /// requirement ID, or an at-prefixed scenario or acceptance-criteria ID.
    #[must_use]
    pub fn matches_selector(&self, selector: &str) -> bool {
        self.scenario_id == selector
            || self.ac_id.as_deref() == Some(selector)
            || self.req_id.as_deref() == Some(selector)
            || format!("@{}", self.scenario_id) == selector
            || self
                .ac_id
                .as_ref()
                .is_some_and(|ac_id| format!("@{ac_id}") == selector)
    }
}

impl FeatureGrid {
    /// Return all scenarios matching a selector.
    #[must_use]
    pub fn select_by_selector(&self, selector: &str) -> Vec<ScenarioRecord> {
        self.scenarios
            .iter()
            .filter(|scenario| scenario.matches_selector(selector))
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
            ..ScenarioRecord::default()
        }
    }

    #[test]
    fn matches_supported_selector_forms() -> Result<(), String> {
        let scenario = scenario_record();
        for selector in [
            "SCN-XK-WORKFLOW-002",
            "AC-XK-WORKFLOW-002",
            "REQ-XK-WORKFLOW",
            "@SCN-XK-WORKFLOW-002",
            "@AC-XK-WORKFLOW-002",
        ] {
            if !scenario.matches_selector(selector) {
                return Err(format!("selector did not match: {selector}"));
            }
        }
        Ok(())
    }

    #[test]
    fn rejects_unknown_selector() -> Result<(), String> {
        if scenario_record().matches_selector("AC-XK-DOES-NOT-EXIST") {
            return Err("unknown selector unexpectedly matched".to_string());
        }
        Ok(())
    }

    #[test]
    fn selects_all_matching_scenarios() -> Result<(), String> {
        let grid = FeatureGrid {
            scenarios: vec![scenario_record(), scenario_record()],
        };
        let selected = grid.select_by_selector("AC-XK-WORKFLOW-002");
        if selected.len() != 2 {
            return Err(format!("expected 2 scenarios, found {}", selected.len()));
        }
        Ok(())
    }

    #[test]
    fn empty_grid_selects_no_scenarios() -> Result<(), String> {
        if !FeatureGrid::default()
            .select_by_selector("AC-XK-WORKFLOW-002")
            .is_empty()
        {
            return Err("empty grid unexpectedly selected a scenario".to_string());
        }
        Ok(())
    }
}
