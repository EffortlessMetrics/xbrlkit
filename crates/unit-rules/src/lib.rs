//! Unit Consistency Validation
//!
//! Validates that XBRL facts use appropriate units for their concept types.
//! For example, monetary values should use currency units, share counts
//! should use shares units, etc.

pub mod patterns;
pub mod validator;

pub use patterns::ExpectedUnitType;
pub use validator::{validate_unit_consistency, UnitValidator};

/// Check if a unit matches the expected type
pub fn unit_matches_type(unit_measure: &str, expected: &ExpectedUnitType) -> bool {
    match expected {
        ExpectedUnitType::Monetary => {
            // Monetary units are iso4217:XXX format
            unit_measure.starts_with("iso4217:")
        }
        ExpectedUnitType::Shares => unit_measure == "xbrli:shares",
        ExpectedUnitType::Pure => unit_measure == "xbrli:pure",
        ExpectedUnitType::PerShare => {
            // Per-share units are typically derived (e.g., USDPerShare)
            unit_measure.contains("PerShare") || unit_measure.ends_with("/share")
        }
        ExpectedUnitType::Custom(pattern) => unit_measure.contains(pattern),
    }
}
