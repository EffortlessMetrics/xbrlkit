//! Local corpus adapter.

use anyhow::Context;
use std::path::Path;

pub fn read_to_string(path: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}
