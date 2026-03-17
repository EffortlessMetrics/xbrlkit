//! Interop test lane helpers.

use receipt_types::{Receipt, RunResult};

#[must_use]
pub fn interop_receipt(subject: &str) -> Receipt {
    Receipt::new("oracle.compare", subject, RunResult::Warning)
}
