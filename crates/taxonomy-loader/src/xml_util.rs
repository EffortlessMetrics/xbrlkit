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
/// - Otherwise, joins with `{base_dir}/{relative}`.
pub(crate) fn resolve_path(base_dir: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") || base_dir.is_empty() {
        relative.to_string()
    } else {
        format!("{base_dir}/{relative}")
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
    fn test_resolve_path_http_url_passthrough() {
        assert_eq!(
            resolve_path("/some/dir", "http://example.com/file.xsd"),
            "http://example.com/file.xsd"
        );
    }

    #[test]
    fn test_resolve_path_https_url_passthrough() {
        assert_eq!(
            resolve_path("/some/dir", "https://example.com/file.xsd"),
            "https://example.com/file.xsd"
        );
    }

    #[test]
    fn test_resolve_path_empty_base_dir() {
        assert_eq!(resolve_path("", "file.xsd"), "file.xsd");
    }

    #[test]
    fn test_resolve_path_relative() {
        assert_eq!(
            resolve_path("/taxonomies/2024", "imports/xbrli.xsd"),
            "/taxonomies/2024/imports/xbrli.xsd"
        );
    }

    #[test]
    fn test_resolve_path_trailing_slash_base() {
        assert_eq!(
            resolve_path("/taxonomies/2024/", "imports/xbrli.xsd"),
            "/taxonomies/2024//imports/xbrli.xsd"
        );
    }

    #[test]
    fn test_base_dir_from_path_with_parent() {
        assert_eq!(
            base_dir_from_path("/taxonomies/2024/main.xsd"),
            "/taxonomies/2024"
        );
    }

    #[test]
    fn test_base_dir_from_path_no_parent() {
        assert_eq!(base_dir_from_path("main.xsd"), "");
    }

    #[test]
    fn test_extract_namespaces_basic() {
        let xml = r#"<root xmlns:xsd="http://www.w3.org/2001/XMLSchema" xmlns="http://default.ns"/>"#;
        let doc = Document::parse(xml).unwrap();
        let ns = extract_namespaces(&doc);
        assert_eq!(ns.get("xsd"), Some(&"http://www.w3.org/2001/XMLSchema".to_string()));
        assert_eq!(ns.get(""), Some(&"http://default.ns".to_string()));
    }
}
