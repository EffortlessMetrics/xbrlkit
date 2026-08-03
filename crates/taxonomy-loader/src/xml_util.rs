//! Shared XML parsing and path-resolution utilities for taxonomy loaders.

use roxmltree::Document;
use std::collections::HashMap;

/// Extracts namespace prefix-to-URI mappings from the root element of a document.
pub(crate) fn extract_namespaces(doc: &Document<'_>) -> HashMap<String, String> {
    let mut ns_map = HashMap::new();
    for ns in doc.root_element().namespaces() {
        let prefix = ns.name().unwrap_or("");
        ns_map.insert(prefix.to_string(), ns.uri().to_string());
    }
    ns_map
}

/// Resolves a relative path against a base directory.
///
/// - Absolute URLs (`http://`, `https://`) are returned as-is.
/// - If `base_dir` is empty, the relative path is returned as-is.
/// - Otherwise, joins with `base_dir` and `relative`, inserting a separator
///   only when the base does not already end with one.
pub(crate) fn resolve_path(base_dir: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") || base_dir.is_empty() {
        relative.to_string()
    } else {
        let separator = if base_dir.ends_with('/') || base_dir.ends_with('\\') {
            ""
        } else {
            "/"
        };
        format!("{base_dir}{separator}{relative}")
    }
}

/// Extracts the parent directory of `base_path` as a lossy String.
/// Returns an empty string if there is no parent.
pub(crate) fn base_dir_from_path(base_path: &str) -> String {
    std::path::Path::new(base_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_path_http_url_passthrough() -> Result<(), String> {
        let actual = resolve_path("/some/dir", "http://example.com/file.xsd");
        let expected = "http://example.com/file.xsd";
        if actual != expected {
            return Err(format!(
                "http URL passthrough: expected {expected}, got {actual}"
            ));
        }
        Ok(())
    }

    #[test]
    fn test_resolve_path_https_url_passthrough() -> Result<(), String> {
        let actual = resolve_path("/some/dir", "https://example.com/file.xsd");
        let expected = "https://example.com/file.xsd";
        if actual != expected {
            return Err(format!(
                "https URL passthrough: expected {expected}, got {actual}"
            ));
        }
        Ok(())
    }

    #[test]
    fn test_resolve_path_empty_base_dir() -> Result<(), String> {
        let actual = resolve_path("", "file.xsd");
        let expected = "file.xsd";
        if actual != expected {
            return Err(format!(
                "empty base directory: expected {expected}, got {actual}"
            ));
        }
        Ok(())
    }

    #[test]
    fn test_resolve_path_relative() -> Result<(), String> {
        let actual = resolve_path("/taxonomies/2024", "imports/xbrli.xsd");
        let expected = "/taxonomies/2024/imports/xbrli.xsd";
        if actual != expected {
            return Err(format!("relative path: expected {expected}, got {actual}"));
        }
        Ok(())
    }

    #[test]
    fn test_resolve_path_trailing_slash_base() -> Result<(), String> {
        let actual = resolve_path("/taxonomies/2024/", "imports/xbrli.xsd");
        let expected = "/taxonomies/2024/imports/xbrli.xsd";
        if actual != expected {
            return Err(format!(
                "trailing slash base directory: expected {expected}, got {actual}"
            ));
        }
        Ok(())
    }

    #[test]
    fn test_base_dir_from_path_with_parent() -> Result<(), String> {
        let actual = base_dir_from_path("/taxonomies/2024/main.xsd");
        let expected = "/taxonomies/2024";
        if actual != expected {
            return Err(format!(
                "parent directory: expected {expected}, got {actual}"
            ));
        }
        Ok(())
    }

    #[test]
    fn test_base_dir_from_path_no_parent() -> Result<(), String> {
        let actual = base_dir_from_path("main.xsd");
        if !actual.is_empty() {
            return Err(format!("path without parent: expected empty, got {actual}"));
        }
        Ok(())
    }

    #[test]
    fn test_extract_namespaces_basic() -> Result<(), String> {
        let xml =
            r#"<root xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://default.ns"/>"#;
        let doc =
            Document::parse(xml).map_err(|error| format!("parse namespace fixture: {error}"))?;
        let ns = extract_namespaces(&doc);
        let xsd_uri = ns.get("xsd").map(String::as_str);
        if xsd_uri != Some("http://www.w3.org/2001/XMLSchema") {
            return Err(format!("xsd namespace: got {xsd_uri:?}"));
        }
        let default_uri = ns.get("").map(String::as_str);
        if default_uri != Some("http://default.ns") {
            return Err(format!("default namespace: got {default_uri:?}"));
        }
        Ok(())
    }
}
