//! Oracle comparison surface.

use receipt_types::{Receipt, RunResult};

#[must_use]
pub fn comparison_receipt(subject: &str) -> Receipt {
    Receipt::new("oracle.compare", subject, RunResult::Warning)
}
