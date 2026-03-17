//! Minimal taxonomy cache adapter.

use anyhow::Result;
use std::path::Path;

pub fn ensure_cache_dir(path: &Path) -> Result<()> {
    std::fs::create_dir_all(path)?;
    Ok(())
}
