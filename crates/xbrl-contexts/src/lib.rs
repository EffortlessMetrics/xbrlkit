//! Context normalization.

#[must_use]
pub fn normalize_context_id(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}
