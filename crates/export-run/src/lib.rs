//! Export orchestration.

use oim_normalize::to_json_value;
use receipt_types::{Receipt, RunResult};
use std::io::Write;
use xbrl_report_types::CanonicalReport;

/// Errors that can occur while exporting a canonical report as JSON.
#[derive(Debug, thiserror::Error)]
pub enum ExportError {
    #[error("serializing canonical report: {0}")]
    Serialization(#[from] serde_json::Error),
}

/// Serializes a canonical report and returns its successful export receipt.
///
/// # Errors
///
/// Returns [`ExportError::Serialization`] when the report cannot be encoded as JSON.
pub fn export_json(report: &CanonicalReport) -> Result<(String, Receipt), ExportError> {
    let json = serde_json::to_string_pretty(&to_json_value(report))?;
    Ok((json, export_receipt()))
}

/// Serializes a canonical report to a caller-provided writer.
///
/// This is the fallible boundary used when export output is streamed to a file,
/// socket, or other external sink.
///
/// # Errors
///
/// Returns [`ExportError::Serialization`] when the report cannot be encoded or
/// the writer rejects output.
pub fn export_json_to<W: Write>(
    report: &CanonicalReport,
    writer: W,
) -> Result<Receipt, ExportError> {
    serde_json::to_writer_pretty(writer, &to_json_value(report))?;
    Ok(export_receipt())
}

fn export_receipt() -> Receipt {
    Receipt::new("export.report", "canonical-report", RunResult::Success)
}

#[cfg(test)]
mod tests {
    use super::{export_json, export_json_to};
    use std::io::{self, Write};
    use xbrl_report_types::CanonicalReport;

    struct FailingWriter;

    impl Write for FailingWriter {
        fn write(&mut self, _buffer: &[u8]) -> io::Result<usize> {
            Err(io::Error::new(
                io::ErrorKind::BrokenPipe,
                "test sink failed",
            ))
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn exports_report_and_receipt() -> Result<(), String> {
        let (json, receipt) = export_json(&CanonicalReport::default())
            .map_err(|error| format!("export failed: {error}"))?;

        if !json.contains('\n') {
            return Err("expected pretty-printed JSON output".to_string());
        }
        if receipt.kind != "export.report" {
            return Err(format!("unexpected receipt kind: {}", receipt.kind));
        }
        if receipt.subject != "canonical-report" {
            return Err(format!("unexpected receipt subject: {}", receipt.subject));
        }

        Ok(())
    }

    #[test]
    fn export_json_to_propagates_writer_failure() -> Result<(), String> {
        let Err(error) = export_json_to(&CanonicalReport::default(), FailingWriter) else {
            return Err("a failing writer should return an export error".to_string());
        };

        if !error.to_string().contains("test sink failed") {
            return Err(format!("unexpected export error: {error}"));
        }
        Ok(())
    }
}
