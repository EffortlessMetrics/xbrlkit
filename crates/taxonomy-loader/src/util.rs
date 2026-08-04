//! Shared helpers for taxonomy document parsing.

use roxmltree::Document;
use std::collections::HashMap;

/// Extract namespace prefix-to-URI mappings from a taxonomy document.
pub(crate) fn extract_namespaces(doc: &Document<'_>) -> HashMap<String, String> {
    let mut ns_map = HashMap::new();

    for ns in doc.root_element().namespaces() {
        let prefix = ns.name().unwrap_or("");
        ns_map.insert(prefix.to_string(), ns.uri().to_string());
    }

    ns_map
}
