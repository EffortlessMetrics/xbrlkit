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
