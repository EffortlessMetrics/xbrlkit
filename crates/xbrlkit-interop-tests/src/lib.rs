//! Interop test lane helpers.

/// Creates the warning receipt used by the oracle comparison interop lane.
pub use receipt_types::oracle_comparison_receipt as interop_receipt;

#[cfg(test)]
mod tests {
    use super::interop_receipt;

    #[test]
    fn interop_receipt_preserves_the_public_contract() -> Result<(), String> {
        let actual = interop_receipt("filing-001");
        let expected = receipt_types::Receipt::new(
            "oracle.compare",
            "filing-001",
            receipt_types::RunResult::Warning,
        );

        if actual != expected {
            return Err(format!("unexpected interop receipt: {actual:?}"));
        }
        Ok(())
    }
}
