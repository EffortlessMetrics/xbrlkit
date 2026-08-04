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

#[cfg(test)]
mod tests {
    use super::parse_identity;

    #[test]
    fn extracts_accession_and_preserves_identity_defaults() -> Result<(), String> {
        let identity = parse_identity(
            "<SEC-HEADER>\nSUBMISSION TYPE: 10-K\nACCESSION NUMBER: 0000123456-24-000001\n</SEC-HEADER>",
        );

        if identity.accession != "0000123456-24-000001" {
            return Err(format!(
                "accession was not extracted: {}",
                identity.accession
            ));
        }
        if identity.cik != "0000000000" || identity.form != "10-K" {
            return Err(format!(
                "identity defaults changed: cik={}, form={}",
                identity.cik, identity.form
            ));
        }

        Ok(())
    }

    #[test]
    fn uses_unknown_accession_when_header_is_absent() -> Result<(), String> {
        let identity = parse_identity("<SEC-HEADER>\nSUBMISSION TYPE: 10-K\n</SEC-HEADER>");

        if identity.accession != "UNKNOWN" {
            return Err(format!(
                "missing accession fallback changed: {}",
                identity.accession
            ));
        }
        if identity.cik != "0000000000" || identity.form != "10-K" {
            return Err(format!(
                "identity defaults changed: cik={}, form={}",
                identity.cik, identity.form
            ));
        }

        Ok(())
    }
}
