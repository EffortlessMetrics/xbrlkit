//! Pattern matching for concept → expected unit type mapping

use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

/// Pre-compiled monetary regex patterns, initialized lazily on first access.
/// Using `LazyLock` avoids recompilation on every call to `is_likely_monetary()`.
static MONETARY_PATTERNS: LazyLock<Vec<Regex>> = LazyLock::new(|| {
    vec![
        Regex::new(r"(?i).*(revenue|sales|income|profit|loss|expense|cost|asset|liabilit).*").unwrap(),
        Regex::new(r"(?i).*(cash|debt|equity|capital|dividend|payment|price).*").unwrap(),
        Regex::new(r"(?i).*(balance|amount|value|gain|proceed).*").unwrap(),
    ]
});

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
            (
                Regex::new(r"(?i).*shares.*").unwrap(),
                ExpectedUnitType::Shares,
            ),
            // Per-share concepts → PerShare unit
            (
                Regex::new(r"(?i).*pershare.*").unwrap(),
                ExpectedUnitType::PerShare,
            ),
            (
                Regex::new(r"(?i).*per.*share.*").unwrap(),
                ExpectedUnitType::PerShare,
            ),
            // Employee-related → Pure unit
            (
                Regex::new(r"(?i).*employees.*").unwrap(),
                ExpectedUnitType::Pure,
            ),
            // Percentage/ratio → Pure unit
            (
                Regex::new(r"(?i).*percentage.*").unwrap(),
                ExpectedUnitType::Pure,
            ),
            (
                Regex::new(r"(?i).*ratio.*").unwrap(),
                ExpectedUnitType::Pure,
            ),
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
    fn test_monetary_heuristic() {
        let patterns = ConceptUnitPatterns::new();
        assert!(patterns.is_likely_monetary("us-gaap:Revenue"));
        assert!(patterns.is_likely_monetary("us-gaap:Assets"));
        assert!(!patterns.is_likely_monetary("us-gaap:CommonStockSharesOutstanding"));
    }

    /// Regression guard: verify `MONETARY_PATTERNS` static initializes safely
    /// under concurrent access and repeated calls do not panic.
    #[test]
    fn test_monetary_patterns_concurrent_initialization() {
        use std::thread;

        let handles: Vec<_> = (0..20)
            .map(|_| {
                thread::spawn(|| {
                    let patterns = ConceptUnitPatterns::new();
                    for i in 0..100 {
                        let concept = if i % 2 == 0 {
                            "us-gaap:Revenue"
                        } else {
                            "us-gaap:CommonStockSharesOutstanding"
                        };
                        // Must not panic
                        let _ = patterns.is_likely_monetary(concept);
                    }
                })
            })
            .collect();

        for handle in handles {
            handle.join().expect("thread should not panic");
        }
    }
}
