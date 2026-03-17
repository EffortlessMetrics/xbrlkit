//! Taxonomy DTOs.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NamespaceMapping {
    pub prefix: String,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DtsDescriptor {
    #[serde(default)]
    pub entry_points: Vec<String>,
    #[serde(default)]
    pub namespaces: Vec<NamespaceMapping>,
}
