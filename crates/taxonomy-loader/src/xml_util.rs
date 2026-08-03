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
    if has_http_scheme(relative) || base_dir.is_empty() {
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

fn has_http_scheme(path: &str) -> bool {
    path.get(..7)
        .is_some_and(|scheme| scheme.eq_ignore_ascii_case("http://"))
        || path
            .get(..8)
            .is_some_and(|scheme| scheme.eq_ignore_ascii_case("https://"))
}

/// Extracts the parent directory of `base_path` as a lossy String.
/// Returns an empty string if there is no parent.
pub(crate) fn base_dir_from_path(base_path: &str) -> String {
    let native_parent = std::path::Path::new(base_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    if !native_parent.is_empty() {
        return native_parent;
    }

    // XML references can contain Windows paths even when parsed on another host.
    // Fall back to the other separator when the host path implementation finds no
    // parent, while preserving a drive-root separator such as `C:\`.
    let Some(separator) = base_path.rfind(['/', '\\']) else {
        return String::new();
    };
    if separator == 2 && base_path.as_bytes().get(1) == Some(&b':') {
        base_path[..=separator].to_string()
    } else {
        base_path[..separator].to_string()
    }
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
    fn test_resolve_path_mixed_case_http_url_passthrough() -> Result<(), String> {
        let actual = resolve_path("/some/dir", "HtTp://example.com/file.xsd");
        let expected = "HtTp://example.com/file.xsd";
        if actual != expected {
            return Err(format!(
                "mixed-case http URL passthrough: expected {expected}, got {actual}"
            ));
        }
        Ok(())
    }

    #[test]
    fn test_resolve_path_mixed_case_https_url_passthrough() -> Result<(), String> {
        let actual = resolve_path("/some/dir", "hTtPs://example.com/file.xsd");
        let expected = "hTtPs://example.com/file.xsd";
        if actual != expected {
            return Err(format!(
                "mixed-case https URL passthrough: expected {expected}, got {actual}"
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
    fn test_resolve_path_trailing_backslash_base() -> Result<(), String> {
        let actual = resolve_path(r"C:\taxonomies\2024\", r"imports\xbrli.xsd");
        let expected = r"C:\taxonomies\2024\imports\xbrli.xsd";
        if actual != expected {
            return Err(format!(
                "trailing backslash base directory: expected {expected}, got {actual}"
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
    fn test_base_dir_from_path_with_backslash_parent() -> Result<(), String> {
        let actual = base_dir_from_path(r"C:\taxonomies\2024\main.xsd");
        let expected = r"C:\taxonomies\2024";
        if actual != expected {
            return Err(format!(
                "backslash parent directory: expected {expected}, got {actual}"
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
