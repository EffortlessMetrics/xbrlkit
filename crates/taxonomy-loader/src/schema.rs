//! XSD schema parsing for dimension elements.
//!
//! Detects:
//! - `xbrldt:hypercubeItem` elements → Hypercubes
//! - `xbrldt:dimensionItem` elements → Dimensions
//! - `xbrli:domainItemType` type elements → Domain members

use crate::error::TaxonomyLoaderError;
use roxmltree::{Document, Node};
use std::collections::HashMap;
use taxonomy_dimensions::{Dimension, Hypercube};

/// Parses an XSD schema file and extracts dimension elements.
pub fn parse_schema(
    content: &str,
    taxonomy: &mut taxonomy_dimensions::DimensionTaxonomy,
) -> Result<(), TaxonomyLoaderError> {
    let doc = Document::parse(content)?;

    // Extract namespace prefixes
    let ns_map = extract_namespaces(&doc);

    // Find the target namespace
    let target_ns = doc
        .root_element()
        .attribute("targetNamespace")
        .unwrap_or("");

    // Parse elements
    for node in doc.root_element().children() {
        if node.tag_name().name() == "element" {
            parse_element(node, target_ns, &ns_map, taxonomy);
        }
    }

    Ok(())
}

/// Extracts namespace mappings from the schema.
fn extract_namespaces(doc: &Document<'_>) -> HashMap<String, String> {
    let mut ns_map = HashMap::new();

    for ns in doc.root_element().namespaces() {
        let prefix = ns.name().unwrap_or("");
        ns_map.insert(prefix.to_string(), ns.uri().to_string());
    }

    ns_map
}

/// Parses an individual element definition.
fn parse_element(
    node: Node<'_, '_>,
    target_ns: &str,
    ns_map: &HashMap<String, String>,
    taxonomy: &mut taxonomy_dimensions::DimensionTaxonomy,
) {
    let name = node.attribute("name").unwrap_or("");
    let _id = node.attribute("id").unwrap_or(name);
    let substitution_group = node.attribute("substitutionGroup").unwrap_or("");
    let type_attr = node.attribute("type").unwrap_or("");

    // Skip if no name
    if name.is_empty() {
        return;
    }

    // Build full QName
    let qname = if target_ns.is_empty() {
        name.to_string()
    } else {
        format!(
            "{}:{}",
            prefix_for_ns(ns_map, target_ns).unwrap_or(""),
            name
        )
    };

    // Check for hypercubeItem
    if is_hypercube_item(substitution_group, ns_map) {
        let hypercube = Hypercube {
            qname: qname.clone(),
            dimensions: std::collections::BTreeMap::new(),
            label: None,
        };
        taxonomy.add_hypercube(hypercube);
    }

    // Check for dimensionItem
    if is_dimension_item(substitution_group, ns_map) {
        let dimension = Dimension::Explicit {
            qname: qname.clone(),
            default_domain: None,
            required: false,
        };
        taxonomy.add_dimension(dimension);
    }

    // Check for domainItemType
    if is_domain_item_type(type_attr, ns_map) {
        // Domain members are typically discovered via linkbases,
        // but we can note their existence here
        // For now, we don't add them directly - they're discovered through
        // domain-member arcs in definition linkbases
    }
}

/// Checks if substitution group is xbrldt:hypercubeItem.
fn is_hypercube_item(substitution_group: &str, ns_map: &HashMap<String, String>) -> bool {
    resolve_qname(substitution_group, ns_map)
        .is_some_and(|(ns, local)| ns.contains("xbrl.org/2005/xbrldt") && local == "hypercubeItem")
}

/// Checks if substitution group is xbrldt:dimensionItem.
fn is_dimension_item(substitution_group: &str, ns_map: &HashMap<String, String>) -> bool {
    resolve_qname(substitution_group, ns_map)
        .is_some_and(|(ns, local)| ns.contains("xbrl.org/2005/xbrldt") && local == "dimensionItem")
}

/// Checks if type is xbrli:domainItemType.
fn is_domain_item_type(type_attr: &str, ns_map: &HashMap<String, String>) -> bool {
    resolve_qname(type_attr, ns_map).is_some_and(|(ns, local)| {
        ns.contains("xbrl.org/2001/instance") && local == "domainItemType"
    })
}

/// Resolves a `QName` to (namespace, `local_name`).
fn resolve_qname(qname: &str, ns_map: &HashMap<String, String>) -> Option<(String, String)> {
    if let Some((prefix, local)) = qname.split_once(':') {
        ns_map.get(prefix).map(|ns| (ns.clone(), local.to_string()))
    } else {
        // No prefix - use default namespace
        ns_map.get("").map(|ns| (ns.clone(), qname.to_string()))
    }
}

/// Finds a prefix for a given namespace URI.
fn prefix_for_ns<'a>(ns_map: &'a HashMap<String, String>, ns_uri: &str) -> Option<&'a str> {
    ns_map
        .iter()
        .find(|(_, v)| *v == ns_uri)
        .map(|(k, _)| k.as_str())
}

/// Extracts schema import/include references from the schema.
pub fn extract_import_refs(
    content: &str,
    base_path: &str,
) -> Result<Vec<String>, TaxonomyLoaderError> {
    let doc = Document::parse(content)?;
    let mut refs = Vec::new();

    let base_dir = std::path::Path::new(base_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    for node in doc.root_element().children() {
        let tag = node.tag_name().name();
        if tag == "import" || tag == "include" {
            if let Some(schema_location) = node.attribute("schemaLocation") {
                let resolved = resolve_path(&base_dir, schema_location);
                refs.push(resolved);
            }
        }
    }

    Ok(refs)
}

/// Resolves a relative path against a base directory.
/// Resolves a relative path against a base directory.
fn resolve_path(base_dir: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") || base_dir.is_empty() {
        relative.to_string()
    } else {
        format!("{base_dir}/{relative}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_SCHEMA: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:xbrli="http://www.xbrl.org/2001/instance"
            xmlns:xbrldt="http://xbrl.org/2005/xbrldt"
            xmlns:us-gaap="http://fasb.org/us-gaap/2024"
            targetNamespace="http://fasb.org/us-gaap/2024"
            elementFormDefault="qualified">

    <xsd:import namespace="http://www.xbrl.org/2001/instance"
              schemaLocation="http://www.xbrl.org/2001/xbrl-instance-2001-12-31.xsd"/>
    <xsd:import namespace="http://xbrl.org/2005/xbrldt"
              schemaLocation="http://www.xbrl.org/2005/xbrldt-2005.xsd"/>

    <xsd:element id="us-gaap_StatementTable"
                 name="StatementTable"
                 substitutionGroup="xbrldt:hypercubeItem"
                 type="xbrli:stringItemType"
                 xbrli:periodType="duration"
                 abstract="true"/>

    <xsd:element id="us-gaap_StatementScenarioAxis"
                 name="StatementScenarioAxis"
                 substitutionGroup="xbrldt:dimensionItem"
                 type="xbrli:stringItemType"
                 xbrli:periodType="duration"
                 abstract="true"/>

</xsd:schema>
"##;

    #[test]
    fn test_parse_schema_detects_hypercube() {
        let mut taxonomy = taxonomy_dimensions::DimensionTaxonomy::new();
        parse_schema(TEST_SCHEMA, &mut taxonomy).unwrap();

        // Debug output
        // eprintln!("Hypercubes: {:?}", taxonomy.hypercubes.keys().collect::<Vec<_>>());

        assert!(taxonomy.hypercubes.contains_key("us-gaap:StatementTable"));
        assert_eq!(taxonomy.hypercubes.len(), 1);
    }

    #[test]
    fn test_parse_schema_detects_dimension() {
        let mut taxonomy = taxonomy_dimensions::DimensionTaxonomy::new();
        parse_schema(TEST_SCHEMA, &mut taxonomy).unwrap();

        assert!(
            taxonomy
                .dimensions
                .contains_key("us-gaap:StatementScenarioAxis")
        );
        assert_eq!(taxonomy.dimensions.len(), 1);
    }

    #[test]
    fn test_extract_import_refs() {
        let refs = extract_import_refs(TEST_SCHEMA, "test.xsd").unwrap();
        assert_eq!(refs.len(), 2);
        assert!(refs.iter().any(|r| r.contains("xbrl-instance")));
        assert!(refs.iter().any(|r| r.contains("xbrldt-2005")));
    }
}
