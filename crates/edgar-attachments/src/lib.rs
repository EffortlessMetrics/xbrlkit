//! Filing manifest and attachment inventory.

use edgar_identity::FilingIdentity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Attachment {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilingManifest {
    pub filing: FilingIdentity,
    #[serde(default)]
    pub attachments: Vec<Attachment>,
}

#[must_use]
pub fn build_manifest(filing: FilingIdentity, attachments: Vec<Attachment>) -> FilingManifest {
    FilingManifest {
        filing,
        attachments,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_filing_identity_and_attachment_inventory() -> Result<(), String> {
        let filing = FilingIdentity {
            accession: "0000000000-25-000001".to_string(),
            cik: "0000000000".to_string(),
            form: "10-K".to_string(),
        };
        let attachments = vec![
            Attachment {
                name: "primary.html".to_string(),
                kind: "primary".to_string(),
            },
            Attachment {
                name: "notes.xml".to_string(),
                kind: "supporting".to_string(),
            },
        ];

        let manifest = build_manifest(filing.clone(), attachments.clone());

        if manifest.filing != filing {
            return Err("manifest changed the filing identity".to_string());
        }
        if manifest.attachments != attachments {
            return Err("manifest changed the attachment inventory".to_string());
        }
        Ok(())
    }

    #[test]
    fn preserves_empty_attachment_inventory() -> Result<(), String> {
        let manifest = build_manifest(FilingIdentity::default(), Vec::new());

        if !manifest.attachments.is_empty() {
            return Err("empty attachment inventory was not preserved".to_string());
        }
        Ok(())
    }
}
