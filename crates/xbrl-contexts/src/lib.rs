//! XBRL Context parsing with XBRL Dimensions support.
//!
//! Contexts in XBRL provide:
//! - Entity identifier (who)
//! - Period (when)
//! - Dimensional information (what slice of data)
//!
//! XBRL Dimensions adds explicit dimensions via segment/scenario containers.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Normalized context identifier.
#[must_use]
pub fn normalize_context_id(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}

/// Entity identifier with scheme and value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct EntityIdentifier {
    pub scheme: String,
    pub value: String,
}

/// Time period for a context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Period {
    /// A specific instant in time (e.g., 2024-12-31)
    Instant(String),
    /// A duration with start and end dates
    Duration { start: String, end: String },
    /// Forever (rarely used)
    Forever,
}

impl Default for Period {
    fn default() -> Self {
        Self::Forever
    }
}

/// A dimensional member reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DimensionMember {
    /// The dimension QName (e.g., "us-gaap:StatementScenarioAxis")
    pub dimension: String,
    /// The member QName (e.g., "us-gaap:ScenarioActualMember")
    pub member: String,
    /// Whether this is a typed dimension value vs explicit member
    pub is_typed: bool,
    /// For typed dimensions, the actual value
    pub typed_value: Option<String>,
}

/// Dimensional container (segment or scenario).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DimensionalContainer {
    /// Explicit dimension-member pairs
    pub dimensions: Vec<DimensionMember>,
    /// Raw XML content for complex cases
    pub raw_xml: Option<String>,
}

/// A complete XBRL context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Context {
    /// Context ID (normalized)
    pub id: String,
    /// Entity identifier
    pub entity: EntityIdentifier,
    /// Optional entity segment (dimensional)
    pub entity_segment: Option<DimensionalContainer>,
    /// Time period
    pub period: Period,
    /// Optional scenario (dimensional or other)
    pub scenario: Option<DimensionalContainer>,
}

/// Collection of contexts indexed by ID.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ContextSet {
    contexts: BTreeMap<String, Context>,
}

impl ContextSet {
    /// Create an empty context set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a context to the set.
    pub fn insert(&mut self, context: Context) {
        self.contexts.insert(context.id.clone(), context);
    }

    /// Get a context by ID.
    #[must_use]
    pub fn get(&self, id: &str) -> Option<&Context> {
        self.contexts.get(&normalize_context_id(id))
    }

    /// Iterate over all contexts.
    pub fn iter(&self) -> impl Iterator<Item = &Context> {
        self.contexts.values()
    }

    /// Number of contexts.
    #[must_use]
    pub fn len(&self) -> usize {
        self.contexts.len()
    }

    /// Check if empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.contexts.is_empty()
    }
}

/// Errors that can occur during context parsing.
#[derive(Debug, thiserror::Error)]
pub enum ContextError {
    #[error("XML parsing error: {0}")]
    Xml(String),
    #[error("Missing required element: {0}")]
    MissingElement(String),
    #[error("Invalid period format: {0}")]
    InvalidPeriod(String),
    #[error("Invalid entity identifier: {0}")]
    InvalidEntity(String),
}

/// Parse contexts from an XBRL instance document XML.
pub fn parse_contexts(xml: &str) -> Result<ContextSet, ContextError> {
    let mut set = ContextSet::new();

    if xml.trim().is_empty() {
        return Ok(set);
    }

    let doc = roxmltree::Document::parse(xml)
        .map_err(|e| ContextError::Xml(e.to_string()))?;

    // Find all context elements anywhere in the document
    for node in doc.descendants() {
        if node.is_element() && node.tag_name().name() == "context" {
            if let Some(id_attr) = node.attribute("id") {
                let context = parse_context_element(&node, id_attr)?;
                set.insert(context);
            }
        }
    }

    Ok(set)
}

/// Parse a single context element.
fn parse_context_element(node: &roxmltree::Node<'_, '_>, id: &str) -> Result<Context, ContextError> {
    let entity_node = find_child(node, "entity")
        .ok_or_else(|| ContextError::MissingElement("entity".to_string()))?;

    let entity = parse_entity(&entity_node)?;
    let entity_segment = find_child(&entity_node, "segment")
        .map(|n| parse_dimensional_container(&n))
        .transpose()?;

    let period_node = find_child(node, "period")
        .ok_or_else(|| ContextError::MissingElement("period".to_string()))?;
    let period = parse_period(&period_node)?;

    let scenario = find_child(node, "scenario")
        .map(|n| parse_dimensional_container(&n))
        .transpose()?;

    Ok(Context {
        id: normalize_context_id(id),
        entity,
        entity_segment,
        period,
        scenario,
    })
}

/// Parse entity identifier.
fn parse_entity(node: &roxmltree::Node<'_, '_>) -> Result<EntityIdentifier, ContextError> {
    let identifier_node = find_child(node, "identifier")
        .ok_or_else(|| ContextError::MissingElement("identifier".to_string()))?;

    let scheme = identifier_node
        .attribute("scheme")
        .unwrap_or("http://www.sec.gov/CIK")
        .to_string();

    let value = identifier_node
        .text()
        .map(|t| t.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| ContextError::InvalidEntity("empty identifier".to_string()))?;

    Ok(EntityIdentifier { scheme, value })
}

/// Parse period element.
fn parse_period(node: &roxmltree::Node<'_, '_>) -> Result<Period, ContextError> {
    if let Some(instant) = find_child(node, "instant") {
        let date = instant
            .text()
            .map(|t| t.trim().to_string())
            .filter(|s| !s.is_empty())
            .ok_or_else(|| ContextError::InvalidPeriod("empty instant".to_string()))?;
        return Ok(Period::Instant(date));
    }

    let start_date = find_child(node, "startDate")
        .and_then(|n| n.text())
        .map(|t| t.trim().to_string())
        .filter(|s| !s.is_empty());

    let end_date = find_child(node, "endDate")
        .and_then(|n| n.text())
        .map(|t| t.trim().to_string())
        .filter(|s| !s.is_empty());

    match (start_date, end_date) {
        (Some(start), Some(end)) => Ok(Period::Duration { start, end }),
        (None, None) => Ok(Period::Forever),
        _ => Err(ContextError::InvalidPeriod(
            "partial duration (only start or end date)".to_string(),
        )),
    }
}

/// Parse dimensional container (segment or scenario).
fn parse_dimensional_container(node: &roxmltree::Node<'_, '_>) -> Result<DimensionalContainer, ContextError> {
    let mut dimensions = Vec::new();

    for child in node.children().filter(|n| n.is_element()) {
        // Look for explicitMember elements (XBRL Dimensions)
        if child.tag_name().name() == "explicitMember" {
            if let Some(dim_attr) = child.attribute("dimension") {
                let member = child
                    .text()
                    .map(|t| t.trim().to_string())
                    .unwrap_or_default();

                dimensions.push(DimensionMember {
                    dimension: dim_attr.to_string(),
                    member,
                    is_typed: false,
                    typed_value: None,
                });
            }
        }
        // TODO: Handle typedMember for typed dimensions
    }

    Ok(DimensionalContainer {
        dimensions,
        raw_xml: None,
    })
}

/// Find a child element by name (local name only).
fn find_child<'a, 'b>(node: &roxmltree::Node<'a, 'b>, name: &str) -> Option<roxmltree::Node<'a, 'b>> {
    node.children()
        .find(|n| n.is_element() && n.tag_name().name() == name)
}

/// Get dimensional members from a context (combines segment and scenario).
#[must_use]
pub fn get_dimensional_members(context: &Context) -> Vec<&DimensionMember> {
    let mut members = Vec::new();
    
    if let Some(segment) = &context.entity_segment {
        members.extend(segment.dimensions.iter());
    }
    
    if let Some(scenario) = &context.scenario {
        members.extend(scenario.dimensions.iter());
    }
    
    members
}

/// Check if a context has any dimensional information.
#[must_use]
pub fn has_dimensions(context: &Context) -> bool {
    !get_dimensional_members(context).is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_context_id() {
        assert_eq!(normalize_context_id("C-2024"), "c-2024");
        assert_eq!(normalize_context_id("  Context1  "), "context1");
    }

    #[test]
    fn test_context_set() {
        let mut set = ContextSet::new();
        assert!(set.is_empty());

        let ctx = Context {
            id: "ctx-1".to_string(),
            entity: EntityIdentifier {
                scheme: "http://www.sec.gov/CIK".to_string(),
                value: "0000320193".to_string(),
            },
            period: Period::Instant("2024-12-31".to_string()),
            ..Context::default()
        };

        set.insert(ctx);
        assert_eq!(set.len(), 1);
        assert!(set.get("ctx-1").is_some());
        assert!(set.get("CTX-1").is_some()); // case insensitive
    }

    #[test]
    fn test_parse_instant_context() {
        let xml = r#"
            <xbrl xmlns="http://www.xbrl.org/2003/instance"
                  xmlns:dei="http://xbrl.sec.gov/dei/2024"
                  xmlns:xbrldi="http://xbrl.org/2006/xbrldi">
                <context id="ctx-2024" xmlns="http://www.xbrl.org/2003/instance">
                    <entity>
                        <identifier scheme="http://www.sec.gov/CIK">0000320193</identifier>
                    </entity>
                    <period>
                        <instant>2024-12-31</instant>
                    </period>
                </context>
            </xbrl>
        "#;

        let set = parse_contexts(xml).unwrap();
        assert_eq!(set.len(), 1);

        let ctx = set.get("ctx-2024").unwrap();
        assert_eq!(ctx.entity.value, "0000320193");
        assert!(!has_dimensions(ctx));
        
        match &ctx.period {
            Period::Instant(date) => assert_eq!(date, "2024-12-31"),
            _ => panic!("Expected instant period"),
        }
    }

    #[test]
    fn test_parse_duration_context() {
        let xml = r#"
            <xbrl xmlns="http://www.xbrl.org/2003/instance">
                <context id="ctx-duration" xmlns="http://www.xbrl.org/2003/instance">
                    <entity>
                        <identifier scheme="http://www.sec.gov/CIK">0000320193</identifier>
                    </entity>
                    <period>
                        <startDate>2024-01-01</startDate>
                        <endDate>2024-12-31</endDate>
                    </period>
                </context>
            </xbrl>
        "#;

        let set = parse_contexts(xml).unwrap();
        let ctx = set.get("ctx-duration").unwrap();
        
        match &ctx.period {
            Period::Duration { start, end } => {
                assert_eq!(start, "2024-01-01");
                assert_eq!(end, "2024-12-31");
            }
            _ => panic!("Expected duration period"),
        }
    }

    #[test]
    fn test_parse_dimensional_context() {
        let xml = r#"
            <xbrl xmlns="http://www.xbrl.org/2003/instance"
                  xmlns:xbrldi="http://xbrl.org/2006/xbrldi">
                <context id="ctx-dim" xmlns="http://www.xbrl.org/2003/instance">
                    <entity>
                        <identifier scheme="http://www.sec.gov/CIK">0000320193</identifier>
                        <segment>
                            <xbrldi:explicitMember dimension="us-gaap:StatementScenarioAxis">
                                us-gaap:ScenarioActualMember
                            </xbrldi:explicitMember>
                        </segment>
                    </entity>
                    <period>
                        <instant>2024-12-31</instant>
                    </period>
                </context>
            </xbrl>
        "#;

        let set = parse_contexts(xml).unwrap();
        let ctx = set.get("ctx-dim").unwrap();
        
        assert!(has_dimensions(ctx));
        
        let members = get_dimensional_members(ctx);
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].dimension, "us-gaap:StatementScenarioAxis");
        assert!(members[0].member.contains("ScenarioActualMember"));
    }

    #[test]
    fn test_empty_xml() {
        let set = parse_contexts("").unwrap();
        assert!(set.is_empty());
    }
}
