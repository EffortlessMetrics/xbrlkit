//! Small cross-cutting utilities shared by xbrlkit validation crates.

/// Convert a value into an uppercase, ASCII-safe rule-ID suffix.
///
/// ASCII alphanumeric characters are uppercased. Every other character is
/// replaced with an underscore so generated rule IDs remain stable and safe
/// for the repository's rule-ID format.
#[must_use]
pub fn sanitize_for_rule_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::sanitize_for_rule_id;

    fn same<T: std::fmt::Debug + PartialEq>(actual: &T, expected: &T) -> Result<(), String> {
        if actual == expected {
            Ok(())
        } else {
            Err(format!("expected {expected:?}, got {actual:?}"))
        }
    }

    #[test]
    fn preserves_ascii_alphanumeric_and_uppercases_letters() -> Result<(), String> {
        same(
            &sanitize_for_rule_id("dei:EntityCommonStockSharesOutstanding"),
            &"DEI_ENTITYCOMMONSTOCKSHARESOUTSTANDING".to_string(),
        )
    }

    #[test]
    fn replaces_non_ascii_and_separators_with_underscores() -> Result<(), String> {
        same(&sanitize_for_rule_id("é-A/B"), &"__A_B".to_string())
    }

    #[test]
    fn preserves_empty_input_as_empty_output() -> Result<(), String> {
        same(&sanitize_for_rule_id(""), &String::new())
    }
}
