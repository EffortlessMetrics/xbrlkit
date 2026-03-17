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
