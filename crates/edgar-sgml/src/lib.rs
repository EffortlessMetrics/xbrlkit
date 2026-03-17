//! Minimal SGML/header parsing.

use edgar_identity::FilingIdentity;

#[must_use]
pub fn parse_identity(input: &str) -> FilingIdentity {
    let accession = input
        .lines()
        .find_map(|line| line.strip_prefix("ACCESSION NUMBER: "))
        .unwrap_or("UNKNOWN")
        .to_string();
    FilingIdentity {
        accession,
        cik: "0000000000".to_string(),
        form: "10-K".to_string(),
    }
}
