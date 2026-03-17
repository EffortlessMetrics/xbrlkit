//! Unit normalization.

#[must_use]
pub fn normalize_unit(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}
