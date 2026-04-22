//! Common quote/string parsing utilities for BDD step handlers.

/// Extract a quoted string from a step text after a given prefix.
///
/// # Example
/// ```ignore
/// let step = r#"the profile pack "sec/efm-77/opco""#;
/// assert_eq!(extract_quoted(step, "the profile pack \""), Some("sec/efm-77/opco".to_string()));
/// ```
#[allow(dead_code)]
pub fn extract_quoted(step: &str, prefix: &str) -> Option<String> {
    step.strip_prefix(prefix).map(|s| s.trim_end_matches('"').to_string())
}

/// Extract all quoted substrings from a step text.
///
/// Splits on double-quote characters and returns every odd-indexed fragment.
#[allow(dead_code)]
pub fn extract_all_quoted(step: &str) -> Vec<String> {
    step.split('"')
        .enumerate()
        .filter(|(i, _)| i % 2 == 1)
        .map(|(_, s)| s.to_string())
        .collect()
}

/// Parse a count from a step text with a prefix and noun stem.
///
/// # Example
/// ```ignore
/// use xbrlkit_bdd_steps::parse_count_suffix;
/// let step = "the report contains 42 facts";
/// assert_eq!(parse_count_suffix(step, "the report contains ", "fact"), Some(42));
/// ```
#[must_use]
pub fn parse_count_suffix(step: &str, prefix: &str, noun_stem: &str) -> Option<usize> {
    let remainder = step.strip_prefix(prefix)?;
    let count = remainder.split_whitespace().next()?.parse::<usize>().ok()?;
    let noun = remainder
        .split_whitespace()
        .nth(1)
        .unwrap_or_default()
        .trim_end_matches('s');
    if noun == noun_stem { Some(count) } else { None }
}
