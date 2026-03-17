//! Dimension normalization.

#[must_use]
pub fn normalize_dimension(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}
