//! Local corpus adapter.

use anyhow::Context;
use std::path::Path;

pub fn read_to_string(path: &Path) -> anyhow::Result<String> {
    std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::read_to_string;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    static NEXT_TEMP_FILE: AtomicU64 = AtomicU64::new(0);

    fn unique_temp_path() -> Result<PathBuf, String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("clock before Unix epoch: {error}"))?;
        let sequence = NEXT_TEMP_FILE.fetch_add(1, Ordering::Relaxed);
        Ok(std::env::temp_dir().join(format!(
            "xbrlkit-corpus-fs-{}-{}-{}.txt",
            std::process::id(),
            timestamp.as_nanos(),
            sequence
        )))
    }

    fn remove_if_present(path: &Path) -> Result<(), String> {
        match std::fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(format!("removing {}: {error}", path.display())),
        }
    }

    fn require<T: PartialEq + std::fmt::Debug>(
        actual: &T,
        expected: &T,
        label: &str,
    ) -> Result<(), String> {
        if actual == expected {
            Ok(())
        } else {
            Err(format!("{label}: expected {expected:?}, got {actual:?}"))
        }
    }

    #[test]
    fn reads_exact_utf8_file_contents() -> Result<(), String> {
        let path = unique_temp_path()?;
        remove_if_present(&path)?;
        if let Err(error) = std::fs::write(&path, "éclair\n世界\n") {
            remove_if_present(&path)?;
            return Err(format!("writing test file: {error}"));
        }

        let result = read_to_string(&path);
        let cleanup = remove_if_present(&path);
        cleanup?;
        let contents = result.map_err(|error| format!("reading test file: {error:#}"))?;

        require(&contents, &"éclair\n世界\n".to_string(), "file contents")
    }

    #[test]
    fn adds_path_context_when_file_is_missing() -> Result<(), String> {
        let path = unique_temp_path()?;
        remove_if_present(&path)?;

        let result = read_to_string(&path);
        let cleanup = remove_if_present(&path);
        cleanup?;
        let Err(error) = result else {
            return Err("missing file unexpectedly read successfully".to_string());
        };

        let message = format!("{error:#}");
        let expected_context = format!("reading {}", path.display());
        if message.contains(&expected_context) {
            Ok(())
        } else {
            Err(format!("missing path context: {message:?}"))
        }
    }
}
