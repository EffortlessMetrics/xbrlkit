//! Shared parsing utilities for taxonomy documents.

use roxmltree::Document;
use std::collections::HashMap;

/// Extracts namespace mappings from a parsed XML document.
pub(crate) fn extract_namespaces(doc: &Document<'_>) -> HashMap<String, String> {
    let mut ns_map = HashMap::new();

    for ns in doc.root_element().namespaces() {
        let prefix = ns.name().unwrap_or("");
        ns_map.insert(prefix.to_string(), ns.uri().to_string());
    }

    ns_map
}

/// Resolves a relative path against a base directory.
pub(crate) fn resolve_path(base_dir: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") || base_dir.is_empty() {
        relative.to_string()
    } else {
        format!("{base_dir}/{relative}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_root_namespace_mappings() -> Result<(), String> {
        let document = Document::parse(r#"<root xmlns="urn:default" xmlns:sample="urn:sample" />"#)
            .map_err(|error| format!("test XML should parse: {error}"))?;
        let namespaces = extract_namespaces(&document);

        let default_namespace = namespaces
            .get("")
            .ok_or_else(|| "default namespace should be present".to_owned())?;
        if default_namespace != "urn:default" {
            return Err(format!("unexpected default namespace: {default_namespace}"));
        }

        let sample_namespace = namespaces
            .get("sample")
            .ok_or_else(|| "sample namespace should be present".to_owned())?;
        if sample_namespace != "urn:sample" {
            return Err(format!("unexpected sample namespace: {sample_namespace}"));
        }

        Ok(())
    }

    #[test]
    fn resolves_urls_and_relative_paths_without_normalization() -> Result<(), String> {
        let cases = [
            ("base", "relative.xsd", "base/relative.xsd"),
            ("", "relative.xsd", "relative.xsd"),
            (
                "base",
                "http://example.test/schema.xsd",
                "http://example.test/schema.xsd",
            ),
            (
                "base",
                "https://example.test/schema.xsd",
                "https://example.test/schema.xsd",
            ),
        ];

        for (base_dir, relative, expected) in cases {
            let actual = resolve_path(base_dir, relative);
            if actual != expected {
                return Err(format!(
                    "resolve_path({base_dir:?}, {relative:?}) returned {actual:?}, expected {expected:?}"
                ));
            }
        }

        Ok(())
    }
}
