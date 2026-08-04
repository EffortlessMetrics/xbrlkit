//! Shared fallible helpers for the xbrlkit CLI.

use anyhow::{Result, anyhow};
use std::path::Path;

/// Derive the workspace root from the CLI manifest directory.
pub fn workspace_root(manifest_dir: &Path) -> Result<&Path> {
    manifest_dir.parent().and_then(Path::parent).ok_or_else(|| {
        anyhow!(
            "unable to derive workspace root from manifest directory `{}`",
            manifest_dir.display()
        )
    })
}
