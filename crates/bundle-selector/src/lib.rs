//! Bundle selector for AC-based scenario grouping.

use anyhow::bail;
use scenario_contract::{FeatureGrid, ScenarioRecord};

/// A bundle of scenarios matching a selector.
#[derive(Debug, Clone)]
pub struct Bundle {
    pub selector: String,
    pub scenarios: Vec<ScenarioRecord>,
}

/// Select scenarios by AC ID.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_empty_grid_fails() {
        let grid = FeatureGrid { scenarios: vec![] };
        let result = select_by_ac(&grid, "AC-XK-DOES-NOT-EXIST");
        assert!(result.is_err());
    }
}
