//! Lightweight conformance helpers.

use anyhow::Result;
use std::path::Path;

pub fn schema_exists(path: &Path) -> Result<bool> {
    Ok(path.exists())
}
