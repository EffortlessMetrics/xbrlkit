//! Shared XML helpers for taxonomy parsing.

use roxmltree::Document;
use std::collections::HashMap;
use std::path::Path;

/// Extracts namespace mappings from an XML document's root element.
pub(crate) fn extract_namespaces(doc: &Document<'_>) -> HashMap<String, String> {
    let mut ns_map = HashMap::new();

    for ns in doc.root_element().namespaces() {
        let prefix = ns.name().unwrap_or("");
        ns_map.insert(prefix.to_string(), ns.uri().to_string());
    }

    ns_map
}

/// Returns the directory containing a source document path.
pub(crate) fn base_directory(base_path: &str) -> String {
    Path::new(base_path)
        .parent()
        .map(|path| path.to_string_lossy().to_string())
        .unwrap_or_default()
}

/// Resolves a relative reference against a source document directory.
pub(crate) fn resolve_path(base_dir: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") || base_dir.is_empty() {
        relative.to_string()
    } else {
        format!("{base_dir}/{relative}")
    }
}

#[cfg(test)]
mod tests {
    use super::{base_directory, extract_namespaces, resolve_path};
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

    #[test]
    fn resolves_relative_and_absolute_references() -> Result<(), String> {
        if base_directory("taxonomy/us-gaap.xsd") != "taxonomy" {
            return Err("source directory was not extracted".to_string());
        }
        if !base_directory("us-gaap.xsd").is_empty() {
            return Err("root-level source should have an empty directory".to_string());
        }
        if resolve_path("taxonomy", "linkbase_def.xml") != "taxonomy/linkbase_def.xml" {
            return Err("relative reference was not resolved".to_string());
        }
        if resolve_path("taxonomy", "https://example.com/linkbase_def.xml")
            != "https://example.com/linkbase_def.xml"
        {
            return Err("absolute URL was unexpectedly prefixed".to_string());
        }
        Ok(())
    }
}
