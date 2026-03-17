//! Derived test-grid utilities.

use scenario_contract::FeatureGrid;

#[must_use]
pub fn scenario_count(grid: &FeatureGrid) -> usize {
    grid.scenarios.len()
}
