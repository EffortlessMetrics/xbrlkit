//! Edgar filing identity types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilingIdentity {
    pub accession: String,
    pub cik: String,
    pub form: String,
}

#[cfg(test)]
mod tests {
    use super::FilingIdentity;

    fn require(condition: bool, message: &str) -> Result<(), String> {
        if condition {
            Ok(())
        } else {
            Err(message.to_string())
        }
    }

    #[test]
    fn default_identity_has_empty_fields() -> Result<(), String> {
        let identity = FilingIdentity::default();

        require(
            identity.accession.is_empty(),
            "default accession should be empty",
        )?;
        require(identity.cik.is_empty(), "default CIK should be empty")?;
        require(identity.form.is_empty(), "default form should be empty")
    }

    #[test]
    fn populated_identity_round_trips_with_stable_json_fields() -> Result<(), String> {
        let identity = FilingIdentity {
            accession: "0000123456-24-000001".to_string(),
            cik: "0000123456".to_string(),
            form: "10-K".to_string(),
        };

        let actual = serde_json::to_string(&identity).map_err(|error| error.to_string())?;
        let expected = r#"{"accession":"0000123456-24-000001","cik":"0000123456","form":"10-K"}"#;
        require(actual == expected, "identity JSON field shape mismatch")?;

        let round_trip: FilingIdentity =
            serde_json::from_str(&actual).map_err(|error| error.to_string())?;
        require(round_trip == identity, "identity JSON round-trip mismatch")
    }
}
