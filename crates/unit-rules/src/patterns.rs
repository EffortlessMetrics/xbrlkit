//! Pattern matching for concept → expected unit type mapping

use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Expected unit type for a concept
#[derive(Debug, Clone, PartialEq)]
pub enum ExpectedUnitType {
    /// Monetary/currency unit (iso4217:XXX)
    Monetary,
    /// Share count unit (xbrli:shares)
    Shares,
    /// Pure/dimensionless unit (xbrli:pure)
    Pure,
    /// Per-share derived unit
    PerShare,
    /// Custom pattern match
    Custom(String),
}

#[allow(clippy::unwrap_used)]
static SHARES_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*shares.*").expect("statically verified regex pattern")
});
#[allow(clippy::unwrap_used)]
static PER_SHARE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*pershare.*").expect("statically verified regex pattern")
});
#[allow(clippy::unwrap_used)]
static PER_SPACE_SHARE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*per.*share.*").expect("statically verified regex pattern")
});
#[allow(clippy::unwrap_used)]
static EMPLOYEES_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*employees.*").expect("statically verified regex pattern")
});
#[allow(clippy::unwrap_used)]
static PERCENTAGE_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*percentage.*").expect("statically verified regex pattern")
});
#[allow(clippy::unwrap_used)]
static RATIO_RE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*ratio.*").expect("statically verified regex pattern")
});

#[allow(clippy::unwrap_used)]
static MONETARY_RE_1: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*(revenue|sales|income|profit|loss|expense|cost|asset|liabilit).*")
        .expect("statically verified regex pattern")
});
#[allow(clippy::unwrap_used)]
static MONETARY_RE_2: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*(cash|debt|equity|capital|dividend|payment|price).*")
        .expect("statically verified regex pattern")
});
#[allow(clippy::unwrap_used)]
static MONETARY_RE_3: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?i).*(balance|amount|value|gain|proceed).*")
        .expect("statically verified regex pattern")
});

/// Pattern-based concept matcher for unit type determination
pub struct ConceptUnitPatterns {
    /// Explicit concept name → expected unit type
    explicit: HashMap<String, ExpectedUnitType>,
    /// Regex patterns → expected unit type
    patterns: Vec<(Regex, ExpectedUnitType)>,
}

impl ConceptUnitPatterns {
    /// Create a new pattern matcher with default patterns
    pub fn new() -> Self {
        let patterns = vec![
            // Share-related concepts → Shares unit
            ((*SHARES_RE).clone(), ExpectedUnitType::Shares),
            // Per-share concepts → PerShare unit
            ((*PER_SHARE_RE).clone(), ExpectedUnitType::PerShare),
            ((*PER_SPACE_SHARE_RE).clone(), ExpectedUnitType::PerShare),
            // Employee-related → Pure unit
            ((*EMPLOYEES_RE).clone(), ExpectedUnitType::Pure),
            // Percentage/ratio → Pure unit
            ((*PERCENTAGE_RE).clone(), ExpectedUnitType::Pure),
            ((*RATIO_RE).clone(), ExpectedUnitType::Pure),
        ];

        Self {
            explicit: HashMap::new(),
            patterns,
        }
    }

    /// Add an explicit concept mapping
    pub fn add_explicit(&mut self, concept: impl Into<String>, unit_type: ExpectedUnitType) {
        self.explicit.insert(concept.into(), unit_type);
    }

    /// Add a custom regex pattern
    pub fn add_pattern(
        &mut self,
        pattern: &str,
        unit_type: ExpectedUnitType,
    ) -> Result<(), regex::Error> {
        let regex = Regex::new(pattern)?;
        self.patterns.push((regex, unit_type));
        Ok(())
    }

    /// Determine expected unit type for a concept
    pub fn expected_type(&self, concept: &str) -> Option<ExpectedUnitType> {
        // Check explicit mappings first
        if let Some(unit_type) = self.explicit.get(concept) {
            return Some(unit_type.clone());
        }

        // Check patterns in order
        for (regex, unit_type) in &self.patterns {
            if regex.is_match(concept) {
                return Some(unit_type.clone());
            }
        }

        None
    }

    /// Check if concept appears to be monetary
    ///
    /// This is a heuristic based on common naming patterns.
    /// For more accuracy, use explicit configuration or taxonomy type info.
    pub fn is_likely_monetary(&self, concept: &str) -> bool {
        for regex in [&*MONETARY_RE_1, &*MONETARY_RE_2, &*MONETARY_RE_3] {
            if regex.is_match(concept) {
                // But exclude share-related concepts
                if !concept.to_lowercase().contains("share") {
                    return true;
                }
            }
        }

        false
    }
}

impl Default for ConceptUnitPatterns {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shares_pattern() {
        let patterns = ConceptUnitPatterns::new();
        assert_eq!(
            patterns.expected_type("us-gaap:CommonStockSharesOutstanding"),
            Some(ExpectedUnitType::Shares)
        );
    }

    #[test]
    fn test_pershare_pattern() {
        let patterns = ConceptUnitPatterns::new();
        assert_eq!(
            patterns.expected_type("us-gaap:EarningsPerShare"),
            Some(ExpectedUnitType::PerShare)
        );
    }

    #[test]
    fn test_employees_pattern() {
        let patterns = ConceptUnitPatterns::new();
        assert_eq!(
            patterns.expected_type("us-gaap:NumberOfEmployees"),
            Some(ExpectedUnitType::Pure)
        );
    }

    #[test]
    fn test_monetary_heuristic() {
        let patterns = ConceptUnitPatterns::new();
        assert!(patterns.is_likely_monetary("us-gaap:Revenue"));
        assert!(patterns.is_likely_monetary("us-gaap:Assets"));
        assert!(!patterns.is_likely_monetary("us-gaap:CommonStockSharesOutstanding"));
    }
}
