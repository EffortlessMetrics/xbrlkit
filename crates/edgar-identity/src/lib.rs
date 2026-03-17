//! Edgar filing identity types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct FilingIdentity {
    pub accession: String,
    pub cik: String,
    pub form: String,
}
