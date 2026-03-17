//! Offline SEC HTTP adapter boundary.

use anyhow::Result;

pub fn fetch(_url: &str) -> Result<String> {
    Err(anyhow::anyhow!(
        "live SEC HTTP is intentionally unavailable in the default workspace flow"
    ))
}
