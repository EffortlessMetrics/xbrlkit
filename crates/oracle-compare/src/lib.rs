//! Oracle comparison surface.

/// Creates the warning receipt used by the oracle comparison surface.
pub use receipt_types::oracle_comparison_receipt as comparison_receipt;

#[cfg(test)]
mod tests {
    use super::comparison_receipt;

    #[test]
    fn comparison_receipt_preserves_the_public_contract() -> Result<(), String> {
        let actual = comparison_receipt("filing-001");
        let expected = receipt_types::Receipt::new(
            "oracle.compare",
            "filing-001",
            receipt_types::RunResult::Warning,
        );

        if actual != expected {
            return Err(format!("unexpected comparison receipt: {actual:?}"));
        }
        Ok(())
    }
}
