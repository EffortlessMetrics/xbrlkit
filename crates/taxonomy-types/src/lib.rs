//! Taxonomy DTOs.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NamespaceMapping {
    pub prefix: String,
    pub uri: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct DtsDescriptor {
    #[serde(default)]
    pub entry_points: Vec<String>,
    #[serde(default)]
    pub namespaces: Vec<NamespaceMapping>,
}

#[cfg(test)]
mod tests {
    use super::{DtsDescriptor, NamespaceMapping};
    use serde_json::json;

    fn require<T: PartialEq + std::fmt::Debug>(
        actual: &T,
        expected: &T,
        label: &str,
    ) -> Result<(), String> {
        if actual == expected {
            Ok(())
        } else {
            Err(format!("{label}: expected {expected:?}, got {actual:?}"))
        }
    }

    #[test]
    fn preserves_populated_descriptor_json_shape_and_values() -> Result<(), String> {
        let descriptor = DtsDescriptor {
            entry_points: vec!["https://example.test/2024/schema.xsd".to_string()],
            namespaces: vec![NamespaceMapping {
                prefix: "us-gaap".to_string(),
                uri: "https://fasb.org/us-gaap/2024".to_string(),
            }],
        };

        let encoded = serde_json::to_value(&descriptor)
            .map_err(|error| format!("serializing descriptor: {error}"))?;
        require(
            &encoded,
            &json!({
                "entry_points": ["https://example.test/2024/schema.xsd"],
                "namespaces": [{
                    "prefix": "us-gaap",
                    "uri": "https://fasb.org/us-gaap/2024"
                }]
            }),
            "descriptor JSON",
        )?;

        let decoded: DtsDescriptor = serde_json::from_value(encoded)
            .map_err(|error| format!("deserializing descriptor: {error}"))?;
        require(&decoded, &descriptor, "descriptor round trip")
    }

    #[test]
    fn defaults_omitted_descriptor_arrays_to_empty_vectors() -> Result<(), String> {
        let decoded: DtsDescriptor = serde_json::from_value(json!({}))
            .map_err(|error| format!("deserializing default descriptor: {error}"))?;

        require(&decoded, &DtsDescriptor::default(), "default descriptor")
    }
}
