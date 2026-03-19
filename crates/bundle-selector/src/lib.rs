//! Bundle selector for AC-based scenario grouping.

use anyhow::{Context, bail};
use scenario_contract::{FeatureGrid, ScenarioRecord};

/// A bundle of scenarios matching a selector.
#[derive(Debug, Clone)]
pub struct Bundle {
    pub selector: String,
    pub scenarios: Vec<ScenarioRecord>,
}

/// Select scenarios by AC ID prefix.
///
/// Selector format: "AC-XK-IXDS-002" matches exact AC
///                  "AC-XK-IXDS-*" would match all IXDS ACs (not implemented)
pub fn select_by_ac(grid: &FeatureGrid, selector: &str) -> anyhow::Result<Bundle> {
    let scenarios: Vec<_> = grid
        .scenarios
        .iter()
        .filter(|s| s.ac_id.as_deref() == Some(selector))
        .cloned()
        .collect();

    if scenarios.is_empty() {
        bail!("no scenarios match selector '{}'", selector);
    }

    Ok(Bundle {
        selector: selector.to_string(),
        scenarios,
    })
}

/// Generate bundle manifest from scenarios.
pub fn generate_manifest(bundle: &Bundle) -> BundleManifest {
    BundleManifest {
        selector: bundle.selector.clone(),
        scenario_ids: bundle
            .scenarios
            .iter()
            .map(|s| s.scenario_id.clone())
            .collect(),
    }
}

/// Bundle manifest for verification.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BundleManifest {
    pub selector: String,
    pub scenario_ids: Vec<String>,
}

impl BundleManifest {
    /// Check if manifest contains a specific scenario.
    pub fn contains_scenario(&self, scenario_id: &str) -> bool {
        self.scenario_ids.iter().any(|id| id == scenario_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_grid() -> FeatureGrid {
        FeatureGrid { scenarios: vec![] }
    }

    #[test]
    fn test_select_empty_grid_fails() {
        let grid = mock_grid();
        let result = select_by_ac(&grid, "AC-XK-DOES-NOT-EXIST");
        assert!(result.is_err());
    }
}
