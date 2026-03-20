//! XBRL Dimensions taxonomy support.
//!
//! This crate handles:
//! - Dimension definitions (explicit and typed)
//! - Domain hierarchies (parent-child member relationships)
//! - Hypercubes (collections of dimensions for validation)
//! - Arc roles: hypercube-dimension, dimension-domain, domain-member, all, notAll

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// A dimension definition (explicit or typed).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Dimension {
    /// Explicit dimension with a defined domain of members
    Explicit {
        /// Dimension QName (e.g., "us-gaap:StatementScenarioAxis")
        qname: String,
        /// Default domain if no member specified
        default_domain: Option<String>,
        /// Whether this dimension is required
        required: bool,
    },
    /// Typed dimension with a value space (e.g., dates, strings)
    Typed {
        /// Dimension QName
        qname: String,
        /// Type of the dimension value (e.g., "xs:date", "xs:string")
        value_type: String,
        /// Whether this dimension is required
        required: bool,
    },
}

impl Dimension {
    /// Get the QName of this dimension.
    #[must_use]
    pub fn qname(&self) -> &str {
        match self {
            Self::Explicit { qname, .. } | Self::Typed { qname, .. } => qname,
        }
    }

    /// Check if this is a typed dimension.
    #[must_use]
    pub fn is_typed(&self) -> bool {
        matches!(self, Self::Typed { .. })
    }

    /// Check if this dimension is required.
    #[must_use]
    pub fn is_required(&self) -> bool {
        match self {
            Self::Explicit { required, .. } | Self::Typed { required, .. } => *required,
        }
    }
}

/// A member in a domain hierarchy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DomainMember {
    /// Member QName (e.g., "us-gaap:ScenarioActualMember")
    pub qname: String,
    /// Parent member QName (None for root members)
    pub parent: Option<String>,
    /// Order in the presentation hierarchy
    pub order: i32,
    /// Human-readable label
    pub label: Option<String>,
}

/// A domain containing a hierarchy of members.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Domain {
    /// Domain QName
    pub qname: String,
    /// Members in this domain (indexed by QName)
    pub members: BTreeMap<String, DomainMember>,
    /// Root members (those without parents)
    pub roots: Vec<String>,
}

impl Domain {
    /// Create a new empty domain.
    #[must_use]
    pub fn new(qname: impl Into<String>) -> Self {
        Self {
            qname: qname.into(),
            members: BTreeMap::new(),
            roots: Vec::new(),
        }
    }

    /// Add a member to the domain.
    pub fn add_member(&mut self, member: DomainMember) {
        if member.parent.is_none() {
            self.roots.push(member.qname.clone());
        }
        self.members.insert(member.qname.clone(), member);
    }

    /// Check if a QName is a valid member of this domain.
    #[must_use]
    pub fn contains(&self, qname: &str) -> bool {
        self.members.contains_key(qname)
    }

    /// Get all descendants of a member (children, grandchildren, etc.).
    #[must_use]
    pub fn descendants(&self, qname: &str) -> Vec<String> {
        let mut result = Vec::new();
        for (member_qname, member) in &self.members {
            if member.parent.as_deref() == Some(qname) {
                result.push(member_qname.clone());
                result.extend(self.descendants(member_qname));
            }
        }
        result
    }

    /// Get the path from root to this member.
    #[must_use]
    pub fn path_to(&self, qname: &str) -> Vec<String> {
        let mut path = Vec::new();
        let mut current = qname;
        
        while let Some(member) = self.members.get(current) {
            path.push(current.to_string());
            match &member.parent {
                Some(parent) => current = parent,
                None => break,
            }
        }
        
        path.reverse();
        path
    }
}

/// A hypercube defines which dimensions apply to a concept.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct Hypercube {
    /// Hypercube QName
    pub qname: String,
    /// Dimensions in this hypercube (QName -> is_required)
    pub dimensions: BTreeMap<String, bool>,
    /// Human-readable label
    pub label: Option<String>,
}

impl Hypercube {
    /// Create a new empty hypercube.
    #[must_use]
    pub fn new(qname: impl Into<String>) -> Self {
        Self {
            qname: qname.into(),
            dimensions: BTreeMap::new(),
            label: None,
        }
    }

    /// Add a dimension to the hypercube.
    pub fn add_dimension(&mut self, qname: impl Into<String>, required: bool) {
        self.dimensions.insert(qname.into(), required);
    }

    /// Get all dimension QNames.
    #[must_use]
    pub fn dimension_qnames(&self) -> Vec<String> {
        self.dimensions.keys().cloned().collect()
    }

    /// Get required dimensions.
    #[must_use]
    pub fn required_dimensions(&self) -> Vec<String> {
        self.dimensions
            .iter()
            .filter(|(_, required)| **required)
            .map(|(qname, _)| qname.clone())
            .collect()
    }
}

/// Association between a concept and its hypercubes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ConceptHypercubes {
    /// Concept QName
    pub concept_qname: String,
    /// Hypercubes that apply (QName -> is_all)
    /// is_all=true means all dimensions are required (closed hypercube)
    /// is_all=false means dimensions are optional (open hypercube)
    pub hypercubes: BTreeMap<String, bool>,
}

/// Complete dimension taxonomy for a DTS.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DimensionTaxonomy {
    /// All dimensions (indexed by QName)
    pub dimensions: BTreeMap<String, Dimension>,
    /// All domains (indexed by QName)
    pub domains: BTreeMap<String, Domain>,
    /// All hypercubes (indexed by QName)
    pub hypercubes: BTreeMap<String, Hypercube>,
    /// Concept-to-hypercube associations
    pub concept_hypercubes: BTreeMap<String, ConceptHypercubes>,
    /// Dimension to domain mapping (for explicit dimensions)
    pub dimension_domains: BTreeMap<String, String>,
}

impl DimensionTaxonomy {
    /// Create an empty dimension taxonomy.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a dimension.
    pub fn add_dimension(&mut self, dimension: Dimension) {
        self.dimensions.insert(dimension.qname().to_string(), dimension);
    }

    /// Add a domain.
    pub fn add_domain(&mut self, domain: Domain) {
        self.domains.insert(domain.qname.clone(), domain);
    }

    /// Add a hypercube.
    pub fn add_hypercube(&mut self, hypercube: Hypercube) {
        self.hypercubes.insert(hypercube.qname.clone(), hypercube);
    }

    /// Associate a concept with a hypercube.
    pub fn associate_concept_hypercube(
        &mut self,
        concept_qname: impl Into<String>,
        hypercube_qname: impl Into<String>,
        is_all: bool,
    ) {
        let concept_qname = concept_qname.into();
        let hypercube_qname = hypercube_qname.into();
        
        let entry = self
            .concept_hypercubes
            .entry(concept_qname.clone())
            .or_insert_with(|| ConceptHypercubes {
                concept_qname,
                hypercubes: BTreeMap::new(),
            });
        
        entry.hypercubes.insert(hypercube_qname, is_all);
    }

    /// Link a dimension to its domain.
    pub fn link_dimension_domain(
        &mut self,
        dimension_qname: impl Into<String>,
        domain_qname: impl Into<String>,
    ) {
        self.dimension_domains
            .insert(dimension_qname.into(), domain_qname.into());
    }

    /// Get hypercubes for a concept.
    #[must_use]
    pub fn hypercubes_for_concept(&self, concept_qname: &str) -> Vec<String> {
        self.concept_hypercubes
            .get(concept_qname)
            .map(|ch| ch.hypercubes.keys().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all required dimensions for a concept.
    #[must_use]
    pub fn required_dimensions_for_concept(&self, concept_qname: &str) -> Vec<String> {
        let mut required = BTreeSet::new();
        
        if let Some(ch) = self.concept_hypercubes.get(concept_qname) {
            for (hypercube_qname, is_all) in &ch.hypercubes {
                if let Some(hypercube) = self.hypercubes.get(hypercube_qname) {
                    if *is_all {
                        // Closed hypercube: all dimensions required
                        required.extend(hypercube.dimension_qnames());
                    } else {
                        // Open hypercube: only explicitly required dimensions
                        required.extend(hypercube.required_dimensions());
                    }
                }
            }
        }
        
        required.into_iter().collect()
    }

    /// Validate a dimension-member pair.
    pub fn validate_member(
        &self,
        dimension_qname: &str,
        member_qname: &str,
    ) -> Result<(), DimensionValidationError> {
        // Check dimension exists
        let dimension = self
            .dimensions
            .get(dimension_qname)
            .ok_or_else(|| DimensionValidationError::UnknownDimension {
                dimension: dimension_qname.to_string(),
            })?;

        // Typed dimensions accept any value of the correct type
        if dimension.is_typed() {
            return Ok(());
        }

        // For explicit dimensions, check the domain
        if let Some(domain_qname) = self.dimension_domains.get(dimension_qname) {
            if let Some(domain) = self.domains.get(domain_qname) {
                if domain.contains(member_qname) {
                    return Ok(());
                }
                return Err(DimensionValidationError::InvalidMember {
                    dimension: dimension_qname.to_string(),
                    member: member_qname.to_string(),
                    domain: domain_qname.clone(),
                });
            }
        }

        Err(DimensionValidationError::NoDomain {
            dimension: dimension_qname.to_string(),
        })
    }
}

/// Errors during dimension validation.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DimensionValidationError {
    #[error("unknown dimension: {dimension}")]
    UnknownDimension { dimension: String },
    
    #[error("dimension {dimension} has no domain defined")]
    NoDomain { dimension: String },
    
    #[error("invalid member {member} for dimension {dimension} in domain {domain}")]
    InvalidMember {
        dimension: String,
        member: String,
        domain: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domain_hierarchy() {
        let mut domain = Domain::new("us-gaap:StatementScenarioDomain");
        
        domain.add_member(DomainMember {
            qname: "us-gaap:ScenarioActualMember".to_string(),
            parent: None,
            order: 1,
            label: Some("Actual".to_string()),
        });
        
        domain.add_member(DomainMember {
            qname: "us-gaap:ScenarioBudgetMember".to_string(),
            parent: None,
            order: 2,
            label: Some("Budget".to_string()),
        });
        
        assert!(domain.contains("us-gaap:ScenarioActualMember"));
        assert!(domain.contains("us-gaap:ScenarioBudgetMember"));
        assert!(!domain.contains("us-gaap:NonExistentMember"));
        
        assert_eq!(domain.roots.len(), 2);
    }

    #[test]
    fn test_hypercube_dimensions() {
        let mut hypercube = Hypercube::new("us-gaap:StatementTable");
        
        hypercube.add_dimension("us-gaap:StatementScenarioAxis", true);
        hypercube.add_dimension("us-gaap:StatementPeriodAxis", false);
        
        assert_eq!(hypercube.dimension_qnames().len(), 2);
        assert_eq!(hypercube.required_dimensions().len(), 1);
        assert!(hypercube.required_dimensions().contains(&"us-gaap:StatementScenarioAxis".to_string()));
    }

    #[test]
    fn test_dimension_taxonomy() {
        let mut taxonomy = DimensionTaxonomy::new();
        
        // Add dimension
        taxonomy.add_dimension(Dimension::Explicit {
            qname: "us-gaap:StatementScenarioAxis".to_string(),
            default_domain: Some("us-gaap:StatementScenarioDomain".to_string()),
            required: true,
        });
        
        // Add domain
        let mut domain = Domain::new("us-gaap:StatementScenarioDomain");
        domain.add_member(DomainMember {
            qname: "us-gaap:ScenarioActualMember".to_string(),
            parent: None,
            order: 1,
            label: None,
        });
        taxonomy.add_domain(domain);
        
        // Link dimension to domain
        taxonomy.link_dimension_domain(
            "us-gaap:StatementScenarioAxis",
            "us-gaap:StatementScenarioDomain",
        );
        
        // Validate valid member
        assert!(taxonomy.validate_member(
            "us-gaap:StatementScenarioAxis",
            "us-gaap:ScenarioActualMember"
        ).is_ok());
        
        // Validate invalid member
        assert!(taxonomy.validate_member(
            "us-gaap:StatementScenarioAxis",
            "us-gaap:InvalidMember"
        ).is_err());
    }
}
