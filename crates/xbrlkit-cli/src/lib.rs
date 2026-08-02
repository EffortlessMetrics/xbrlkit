//! Shared CLI behavior that needs direct acceptance coverage.

use serde::Serialize;

/// Serialize an inspect response and return the CLI's success/error contract.
///
/// The binary uses the returned exit code and writes the returned text to
/// stdout on success or stderr on failure.
pub fn serialize_json_or_error<T: Serialize>(value: &T, subject: &str) -> (i32, String) {
    match serde_json::to_string_pretty(value) {
        Ok(json) => (0, json),
        Err(error) => (1, format!("error serializing {subject} as JSON: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use super::serialize_json_or_error;
    use serde::Serializer;

    struct FailingJson;

    impl serde::Serialize for FailingJson {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            Err(serde::ser::Error::custom("synthetic serialization failure"))
        }
    }

    #[test]
    fn reports_serialization_failure_without_panicking() -> Result<(), String> {
        let (exit_code, output) = serialize_json_or_error(&FailingJson, "contexts");
        if exit_code != 1 {
            return Err(format!("expected exit code 1, got {exit_code}"));
        }
        if !output.contains("error serializing contexts as JSON") {
            return Err(format!("unexpected error output: {output}"));
        }
        Ok(())
    }
}
