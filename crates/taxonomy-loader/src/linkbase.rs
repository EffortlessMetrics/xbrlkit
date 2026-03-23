//! Definition linkbase parsing for dimension relationships.
//!
//! Parses arcs:
//! - `hypercube-dimension` → Hypercube → Dimension
//! - `dimension-domain` → Dimension → Domain
//! - `domain-member` → Parent → Child member
//! - `all`/`notAll` → Closed vs open hypercubes

use crate::error::TaxonomyLoaderError;
use roxmltree::{Document, Node};
use std::collections::HashMap;
use taxonomy_dimensions::{Domain, DomainMember, Hypercube};

/// Arc role URIs for dimension relationships.
pub const ARCROLE_HYPERCUBE_DIMENSION: &str = "http://xbrl.org/int/dim/arcrole/hypercube-dimension";
pub const ARCROLE_DIMENSION_DOMAIN: &str = "http://xbrl.org/int/dim/arcrole/dimension-domain";
pub const ARCROLE_DOMAIN_MEMBER: &str = "http://xbrl.org/int/dim/arcrole/domain-member";
pub const ARCROLE_ALL: &str = "http://xbrl.org/int/dim/arcrole/all";
pub const ARCROLE_NOT_ALL: &str = "http://xbrl.org/int/dim/arcrole/notAll";

/// Parses a definition linkbase file.
pub fn parse_definition_linkbase(
    content: &str,
    taxonomy: &mut taxonomy_dimensions::DimensionTaxonomy,
) -> Result<(), TaxonomyLoaderError> {
    let doc = Document::parse(content)?;

    // Extract namespace prefixes
    let ns_map = extract_namespaces(&doc);

    // Find all linkbaseRef elements to locate definition links
    // Process all link:definitionLink elements
    for node in doc.root_element().descendants() {
        if node.tag_name().name() == "definitionLink" {
            parse_definition_link(node, &ns_map, taxonomy)?;
        }
    }

    Ok(())
}

/// Parses a single definitionLink element.
fn parse_definition_link(
    link_node: Node<'_, '_>,
    ns_map: &HashMap<String, String>,
    taxonomy: &mut taxonomy_dimensions::DimensionTaxonomy,
) -> Result<(), TaxonomyLoaderError> {
    // Collect all locators (xlink:href -> QName mappings)
    let locators = extract_locators(link_node, ns_map);

    // Collect all arcs
    for node in link_node.children() {
        let tag_name = node.tag_name().name();
        if tag_name == "definitionArc" {
            parse_definition_arc(node, &locators, taxonomy)?;
        }
    }

    Ok(())
}

/// Extracts locator elements mapping xlink:label to QName.
fn extract_locators(
    link_node: Node<'_, '_>,
    ns_map: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut locators = HashMap::new();

    for node in link_node.children() {
        if node.tag_name().name() == "loc" {
            // Access attributes by iterating since namespaced attributes aren't accessible by local name only
            let mut label = None;
            let mut href = None;

            for attr in node.attributes() {
                if attr.name().ends_with("label") {
                    label = Some(attr.value());
                } else if attr.name().ends_with("href") {
                    href = Some(attr.value());
                }
            }

            if let (Some(label), Some(href)) = (label, href) {
                let qname = extract_qname_from_href(href, ns_map);
                locators.insert(label.to_string(), qname);
            }
        }
    }

    locators
}

/// Extracts QName from href like "schema.xsd#us-gaap_StatementTable".
fn extract_qname_from_href(href: &str, ns_map: &HashMap<String, String>) -> String {
    // Parse the fragment identifier
    if let Some((_schema, fragment)) = href.split_once('#') {
        // Fragment is typically the element ID, which often contains the QName
        // with underscores instead of colons (e.g., "us-gaap_StatementTable")
        if let Some((prefix, local)) = fragment.split_once('_') {
            // Map prefix to namespace if available
            if let Some(ns) = ns_map.get(prefix) {
                // Extract just the prefix part from the namespace for display
                let ns_prefix = ns.split('/').last().unwrap_or(prefix);
                return format!("{}:{}", ns_prefix, local);
            }
            return format!("{}:{}", prefix, local);
        }
        return fragment.to_string();
    }

    href.to_string()
}

/// Parses a definitionArc element.
fn parse_definition_arc(
    arc_node: Node<'_, '_>,
    locators: &HashMap<String, String>,
    taxonomy: &mut taxonomy_dimensions::DimensionTaxonomy,
) -> Result<(), TaxonomyLoaderError> {
    // Try to get arcrole with namespace first, then without
    let arcrole = arc_node
        .attribute("{http://www.w3.org/1999/xlink}arcrole")
        .or_else(|| arc_node.attribute("arcrole"))
        .or_else(|| {
            arc_node
                .attributes()
                .find(|a| a.name().ends_with("arcrole"))
                .map(|a| a.value())
        })
        .unwrap_or("");

    let from_label = arc_node
        .attribute("{http://www.w3.org/1999/xlink}from")
        .or_else(|| arc_node.attribute("from"))
        .or_else(|| {
            arc_node
                .attributes()
                .find(|a| a.name().ends_with("from"))
                .map(|a| a.value())
        })
        .unwrap_or("");

    let to_label = arc_node
        .attribute("{http://www.w3.org/1999/xlink}to")
        .or_else(|| arc_node.attribute("to"))
        .or_else(|| {
            arc_node
                .attributes()
                .find(|a| a.name().ends_with("to"))
                .map(|a| a.value())
        })
        .unwrap_or("");

    let order_str = arc_node.attribute("order").unwrap_or("0");
    let order: i32 = order_str.parse().unwrap_or(0);

    let from_qname = locators.get(from_label).cloned().unwrap_or_default();
    let to_qname = locators.get(to_label).cloned().unwrap_or_default();

    if from_qname.is_empty() || to_qname.is_empty() {
        return Ok(());
    }

    match arcrole {
        ARCROLE_HYPERCUBE_DIMENSION => {
            // Link hypercube to dimension
            link_hypercube_dimension(taxonomy, &from_qname, &to_qname);
        }
        ARCROLE_DIMENSION_DOMAIN => {
            // Link dimension to domain
            taxonomy.link_dimension_domain(from_qname.clone(), to_qname.clone());

            // Ensure domain exists
            if !taxonomy.domains.contains_key(&to_qname) {
                taxonomy.add_domain(Domain::new(&to_qname));
            }
        }
        ARCROLE_DOMAIN_MEMBER => {
            // Add member to domain
            add_domain_member(taxonomy, &from_qname, &to_qname, order);
        }
        ARCROLE_ALL => {
            // Closed hypercube - all dimensions required
            link_concept_hypercube(taxonomy, &from_qname, &to_qname, true);
        }
        ARCROLE_NOT_ALL => {
            // Open hypercube - dimensions optional
            link_concept_hypercube(taxonomy, &from_qname, &to_qname, false);
        }
        _ => {
            // Unknown arcrole, ignore
        }
    }

    Ok(())
}

/// Links a hypercube to a dimension.
fn link_hypercube_dimension(
    taxonomy: &mut taxonomy_dimensions::DimensionTaxonomy,
    hypercube_qname: &str,
    dimension_qname: &str,
) {
    // Get or create the hypercube
    let hypercube = taxonomy
        .hypercubes
        .entry(hypercube_qname.to_string())
        .or_insert_with(|| Hypercube::new(hypercube_qname));

    hypercube.add_dimension(dimension_qname.to_string(), false);
}

/// Links a concept to a hypercube.
fn link_concept_hypercube(
    taxonomy: &mut taxonomy_dimensions::DimensionTaxonomy,
    concept_qname: &str,
    hypercube_qname: &str,
    is_all: bool,
) {
    taxonomy.associate_concept_hypercube(concept_qname, hypercube_qname, is_all);
}

/// Adds a member to a domain.
fn add_domain_member(
    taxonomy: &mut taxonomy_dimensions::DimensionTaxonomy,
    parent_qname: &str,
    member_qname: &str,
    order: i32,
) {
    // Find the domain for this parent
    // The parent is either a domain or another member
    // We need to find which domain this belongs to

    // For simplicity, we'll look for a domain that matches the parent
    // or create one if needed
    let domain_qname = if taxonomy.domains.contains_key(parent_qname) {
        parent_qname.to_string()
    } else {
        // Try to find a domain that contains this parent as a member
        // For now, create a new domain
        parent_qname.to_string()
    };

    let domain = taxonomy
        .domains
        .entry(domain_qname.clone())
        .or_insert_with(|| Domain::new(&domain_qname));

    let parent = if parent_qname == domain_qname {
        None
    } else {
        Some(parent_qname.to_string())
    };

    domain.add_member(DomainMember {
        qname: member_qname.to_string(),
        parent,
        order,
        label: None,
    });
}

/// Extracts namespace mappings from the document.
fn extract_namespaces(doc: &Document<'_>) -> HashMap<String, String> {
    let mut ns_map = HashMap::new();

    for ns in doc.root_element().namespaces() {
        let prefix = ns.name().unwrap_or("");
        ns_map.insert(prefix.to_string(), ns.uri().to_string());
    }

    ns_map
}

/// Extracts linkbase references from a schema.
pub fn extract_linkbase_refs(
    content: &str,
    base_path: &str,
) -> Result<Vec<String>, TaxonomyLoaderError> {
    let doc = Document::parse(content)?;
    let mut refs = Vec::new();

    let base_dir = std::path::Path::new(base_path)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    for node in doc.root_element().descendants() {
        // Check for linkbaseRef with or without namespace prefix
        let tag_name = node.tag_name();
        let local_name = tag_name.name();

        let is_linkbase_ref = local_name == "linkbaseRef";

        if is_linkbase_ref {
            // Try namespaced attribute first, then fallback to prefixed name
            let href = node
                .attribute("{http://www.w3.org/1999/xlink}href")
                .or_else(|| node.attribute("href"))
                .or_else(|| {
                    // Look for xlink:href as a prefixed attribute
                    node.attributes()
                        .find(|a| {
                            a.name().ends_with("href")
                                && a.namespace() == Some("http://www.w3.org/1999/xlink")
                        })
                        .map(|a| a.value())
                });

            if let Some(href) = href {
                let resolved = resolve_path(&base_dir, href);
                // Only process definition linkbases
                if resolved.ends_with("_def.xml") || resolved.contains("definition") {
                    refs.push(resolved);
                }
            }
        }
    }

    Ok(refs)
}

/// Resolves a relative path against a base directory.
fn resolve_path(base_dir: &str, relative: &str) -> String {
    if relative.starts_with("http://") || relative.starts_with("https://") {
        relative.to_string()
    } else if base_dir.is_empty() {
        relative.to_string()
    } else {
        format!("{}/{}", base_dir, relative)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_LINKBASE: &str = r##"<?xml version="1.0" encoding="UTF-8"?>
<link:linkbase xmlns:link="http://www.xbrl.org/2003/linkbase"
               xmlns:xlink="http://www.w3.org/1999/xlink"
               xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
               xsi:schemaLocation="http://www.xbrl.org/2003/linkbase http://www.xbrl.org/2003/xbrl-linkbase-2003-12-31.xsd">

    <link:definitionLink xlink:type="extended" xlink:role="http://www.xbrl.org/2003/role/link">
        <link:loc xlink:type="locator"
                  xlink:href="us-gaap-2024.xsd#us-gaap_StatementTable"
                  xlink:label="StatementTable"/>
        <link:loc xlink:type="locator"
                  xlink:href="us-gaap-2024.xsd#us-gaap_StatementScenarioAxis"
                  xlink:label="StatementScenarioAxis"/>
        <link:loc xlink:type="locator"
                  xlink:href="us-gaap-2024.xsd#us-gaap_StatementScenarioDomain"
                  xlink:label="StatementScenarioDomain"/>
        <link:loc xlink:type="locator"
                  xlink:href="us-gaap-2024.xsd#us-gaap_ScenarioActualMember"
                  xlink:label="ScenarioActualMember"/>
        <link:loc xlink:type="locator"
                  xlink:href="us-gaap-2024.xsd#us-gaap_ScenarioBudgetMember"
                  xlink:label="ScenarioBudgetMember"/>

        <!-- Hypercube to Dimension -->
        <link:definitionArc xlink:type="arc"
                           xlink:arcrole="http://xbrl.org/int/dim/arcrole/hypercube-dimension"
                           xlink:from="StatementTable"
                           xlink:to="StatementScenarioAxis"
                           order="1"/>

        <!-- Dimension to Domain -->
        <link:definitionArc xlink:type="arc"
                           xlink:arcrole="http://xbrl.org/int/dim/arcrole/dimension-domain"
                           xlink:from="StatementScenarioAxis"
                           xlink:to="StatementScenarioDomain"
                           order="1"/>

        <!-- Domain Members -->
        <link:definitionArc xlink:type="arc"
                           xlink:arcrole="http://xbrl.org/int/dim/arcrole/domain-member"
                           xlink:from="StatementScenarioDomain"
                           xlink:to="ScenarioActualMember"
                           order="1"/>

        <link:definitionArc xlink:type="arc"
                           xlink:arcrole="http://xbrl.org/int/dim/arcrole/domain-member"
                           xlink:from="StatementScenarioDomain"
                           xlink:to="ScenarioBudgetMember"
                           order="2"/>
    </link:definitionLink>

</link:linkbase>
"##;

    #[test]
    fn test_parse_definition_linkbase() {
        let mut taxonomy = taxonomy_dimensions::DimensionTaxonomy::new();

        // Pre-populate with hypercube and dimension
        taxonomy.add_hypercube(Hypercube::new("us-gaap:StatementTable"));

        parse_definition_linkbase(TEST_LINKBASE, &mut taxonomy).unwrap();

        // Check hypercube-dimension link
        let hypercube = taxonomy.hypercubes.get("us-gaap:StatementTable").unwrap();
        assert!(
            hypercube
                .dimensions
                .contains_key("us-gaap:StatementScenarioAxis")
        );

        // Check dimension-domain link
        assert_eq!(
            taxonomy
                .dimension_domains
                .get("us-gaap:StatementScenarioAxis"),
            Some(&"us-gaap:StatementScenarioDomain".to_string())
        );

        // Check domain members
        let domain = taxonomy
            .domains
            .get("us-gaap:StatementScenarioDomain")
            .unwrap();
        assert!(domain.contains("us-gaap:ScenarioActualMember"));
        assert!(domain.contains("us-gaap:ScenarioBudgetMember"));
        assert_eq!(domain.roots.len(), 2);
    }

    #[test]
    fn test_extract_linkbase_refs() {
        let schema = r##"<?xml version="1.0" encoding="UTF-8"?>
<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
            xmlns:link="http://www.xbrl.org/2003/linkbase"
            xmlns:xlink="http://www.w3.org/1999/xlink"
            targetNamespace="http://test.com">

    <link:linkbaseRef xlink:type="simple"
                      xlink:href="us-gaap-2024_def.xml"
                      xlink:role="http://www.xbrl.org/2003/role/definitionLinkbaseRef"/>

    <link:linkbaseRef xlink:type="simple"
                      xlink:href="us-gaap-2024_pre.xml"
                      xlink:role="http://www.xbrl.org/2003/role/presentationLinkbaseRef"/>

</xsd:schema>
        "##;

        let refs = extract_linkbase_refs(schema, "test.xsd").unwrap();
        assert_eq!(refs.len(), 1);
        assert!(refs[0].contains("_def.xml"));
    }
}
