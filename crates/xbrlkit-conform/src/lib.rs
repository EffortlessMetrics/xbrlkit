//! Lightweight conformance helpers.

use anyhow::Result;
use std::path::Path;

pub fn schema_exists(path: &Path) -> Result<bool> {
    Ok(path.exists())
}

#[cfg(test)]
mod tests {
    use super::schema_exists;
    use std::fs::{self, OpenOptions};
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path() -> Result<PathBuf, String> {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|error| format!("system clock is before UNIX epoch: {error}"))?
            .as_nanos();
        Ok(std::env::temp_dir().join(format!(
            "xbrlkit-conform-schema-{}-{nonce}.xsd",
            std::process::id()
        )))
    }

    #[test]
    fn reports_true_for_an_existing_schema_path() -> Result<(), String> {
        let path = unique_temp_path()?;
        let file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|error| format!("create temporary schema file: {error}"))?;
        drop(file);

        let result = schema_exists(&path).map_err(|error| error.to_string())?;
        fs::remove_file(&path).map_err(|error| format!("remove temporary schema file: {error}"))?;

        if !result {
            return Err(format!(
                "expected existing schema path {path:?} to be found"
            ));
        }

        Ok(())
    }

    #[test]
    fn reports_false_for_a_missing_schema_path() -> Result<(), String> {
        let path = unique_temp_path()?;
        if schema_exists(&path).map_err(|error| error.to_string())? {
            return Err(format!(
                "temporary schema path unexpectedly exists: {path:?}"
            ));
        }

        Ok(())
    }
}
