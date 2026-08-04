//! Filing load use case.

use edgar_attachments::{Attachment, FilingManifest, build_manifest};
use edgar_sgml::parse_identity;
use receipt_types::{Receipt, RunResult};

#[must_use]
pub fn load_from_submission(input: &str) -> (FilingManifest, Receipt) {
    let filing = parse_identity(input);
    let manifest = build_manifest(
        filing.clone(),
        vec![Attachment {
            name: "primary.html".to_string(),
            kind: "primary".to_string(),
        }],
    );
    let receipt = Receipt::new("filing.manifest", filing.accession, RunResult::Success);
    (manifest, receipt)
}

#[cfg(test)]
mod tests {
    use super::load_from_submission;
    use receipt_types::RunResult;

    #[test]
    fn projects_accession_manifest_attachment_and_receipt() -> Result<(), String> {
        let (manifest, receipt) =
            load_from_submission("ACCESSION NUMBER: 0000123456-24-000001\nDOCUMENT: primary");

        if manifest.filing.accession != "0000123456-24-000001" {
            return Err(format!(
                "manifest accession was not preserved: {}",
                manifest.filing.accession
            ));
        }

        let attachment = manifest
            .attachments
            .first()
            .ok_or_else(|| "primary attachment was not emitted".to_string())?;
        if attachment.name != "primary.html" || attachment.kind != "primary" {
            return Err(format!(
                "unexpected primary attachment: name={}, kind={}",
                attachment.name, attachment.kind
            ));
        }

        if receipt.kind != "filing.manifest"
            || receipt.subject != "0000123456-24-000001"
            || receipt.result != RunResult::Success
        {
            return Err(format!("unexpected manifest receipt: {receipt:?}"));
        }

        Ok(())
    }

    #[test]
    fn uses_deterministic_unknown_accession_fallback() -> Result<(), String> {
        let (manifest, receipt) = load_from_submission("DOCUMENT: primary");

        if manifest.filing.accession != "UNKNOWN" {
            return Err(format!(
                "missing accession fallback was not preserved: {}",
                manifest.filing.accession
            ));
        }
        if receipt.subject != "UNKNOWN" {
            return Err(format!(
                "receipt subject did not use missing accession fallback: {}",
                receipt.subject
            ));
        }

        Ok(())
    }
}
