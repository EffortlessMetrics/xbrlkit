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

// ─── Pre-compiled regex patterns (compiled once, shared by all instances) ───

static DEFAULT_PATTERNS: LazyLock<Vec<(Regex, ExpectedUnitType)>> = LazyLock::new(|| {
    [
        // Share-related concepts → Shares unit
        (r"(?i).*shares.*", ExpectedUnitType::Shares),
        // Per-share concepts → PerShare unit
        (r"(?i).*pershare.*", ExpectedUnitType::PerShare),
        (r"(?i).*per.*share.*", ExpectedUnitType::PerShare),
        // Employee-related → Pure unit
        (r"(?i).*employees.*", ExpectedUnitType::Pure),
        // Percentage/ratio → Pure unit
        (r"(?i).*percentage.*", ExpectedUnitType::Pure),
        (r"(?i).*ratio.*", ExpectedUnitType::Pure),
    ]
    .into_iter()
    .filter_map(|(pattern, unit_type)| Regex::new(pattern).ok().map(|regex| (regex, unit_type)))
    .collect()
});

// Monetary heuristic patterns
static MONETARY_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    [
        r"(?i).*(revenue|sales|income|profit|loss|expense|cost|asset|liabilit).*",
        r"(?i).*(cash|debt|equity|capital|dividend|payment|price).*",
        r"(?i).*(balance|amount|value|gain|proceed).*",
    ]
    .into_iter()
    .filter_map(|pattern| Regex::new(pattern).ok())
    .collect()
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
        Self {
            explicit: HashMap::new(),
            patterns: DEFAULT_PATTERNS.clone(),
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
        for regex in MONETARY_PATTERNS.iter() {
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
    fn test_percentage_and_ratio_patterns() -> Result<(), String> {
        let patterns = ConceptUnitPatterns::new();
        for (concept, expected) in [
            ("us-gaap:PercentageOfAssets", ExpectedUnitType::Pure),
            ("us-gaap:DebtToEquityRatio", ExpectedUnitType::Pure),
        ] {
            let actual = patterns.expected_type(concept);
            if actual != Some(expected.clone()) {
                return Err(format!(
                    "{concept} matched {actual:?}, expected {expected:?}"
                ));
            }
        }
        Ok(())
    }

    #[test]
    fn test_monetary_heuristic() {
        let patterns = ConceptUnitPatterns::new();
        assert!(patterns.is_likely_monetary("us-gaap:Revenue"));
        assert!(patterns.is_likely_monetary("us-gaap:Assets"));
        assert!(!patterns.is_likely_monetary("us-gaap:CommonStockSharesOutstanding"));
    }

    /// Eagerly access all LazyLock statics to verify every configured pattern
    /// compiled successfully. This catches invalid literals in CI before they
    /// can silently remove a default rule.
    #[test]
    fn test_all_lazy_regexes_compile() -> Result<(), &'static str> {
        if DEFAULT_PATTERNS.len() != 6 {
            return Err("a default concept pattern failed to compile");
        }
        if MONETARY_PATTERNS.len() != 3 {
            return Err("a monetary heuristic pattern failed to compile");
        }
        Ok(())
    }
}
