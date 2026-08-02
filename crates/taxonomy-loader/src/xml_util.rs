//! Shared XML helpers for taxonomy parsing.

use roxmltree::Document;
use std::collections::HashMap;

/// Extracts namespace mappings from an XML document's root element.
pub(crate) fn extract_namespaces(doc: &Document<'_>) -> HashMap<String, String> {
    let mut ns_map = HashMap::new();

    for ns in doc.root_element().namespaces() {
        let prefix = ns.name().unwrap_or("");
        ns_map.insert(prefix.to_string(), ns.uri().to_string());
    }

    ns_map
}

#[cfg(test)]
mod tests {
    use super::extract_namespaces;
    use roxmltree::Document;

    #[test]
    fn extracts_default_and_prefixed_namespaces() -> Result<(), String> {
        let document = Document::parse(
            r#"<schema xmlns="http://www.w3.org/2001/XMLSchema"
                xmlns:xbrldt="http://xbrl.org/2005/xbrldt"
                xmlns:us-gaap="http://fasb.org/us-gaap/2024">
            </schema>"#,
        )
        .map_err(|error| format!("failed to parse test XML: {error}"))?;
        let namespaces = extract_namespaces(&document);

        if namespaces.get("").map(String::as_str) != Some("http://www.w3.org/2001/XMLSchema") {
            return Err("default namespace was not extracted".to_string());
        }
        if namespaces.get("xbrldt").map(String::as_str) != Some("http://xbrl.org/2005/xbrldt") {
            return Err("xbrldt namespace was not extracted".to_string());
        }
        if namespaces.get("us-gaap").map(String::as_str) != Some("http://fasb.org/us-gaap/2024") {
            return Err("us-gaap namespace was not extracted".to_string());
        }

        Ok(())
    }
}
