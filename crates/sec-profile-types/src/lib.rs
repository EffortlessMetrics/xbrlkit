//! SEC profile pack DTOs.

use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use taxonomy_types::NamespaceMapping;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct InlineRules {
    #[serde(default)]
    pub banned_elements: Vec<String>,
    #[serde(default)]
    pub banned_attributes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct AcceptedTaxonomies {
    #[serde(default)]
    pub years: Vec<u16>,
    #[serde(default)]
    pub namespaces: Vec<NamespaceMapping>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RequiredFactsFile {
    #[serde(default)]
    pub required_facts: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProfilePack {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub forms: Vec<String>,
    #[serde(default)]
    pub enabled_rule_families: Vec<String>,
    #[serde(default)]
    pub inline_rules: InlineRules,
    #[serde(default)]
    pub accepted_taxonomies: AcceptedTaxonomies,
    #[serde(default)]
    pub standard_taxonomy_uris: Vec<String>,
    #[serde(default)]
    pub required_facts: Vec<String>,
    #[serde(default)]
    pub numeric_rules: Option<NumericRules>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NumericRules {
    #[serde(default)]
    pub negative_value_rules: NegativeValueRules,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NegativeValueRules {
    #[serde(default)]
    pub prohibited_concepts: Vec<String>,
}

pub fn load_profile_from_workspace(root: &Path, profile_id: &str) -> anyhow::Result<ProfilePack> {
    let profile_dir = profile_dir(root, profile_id);
    let profile_yaml = std::fs::read_to_string(profile_dir.join("profile.yaml"))
        .with_context(|| format!("reading profile pack {profile_id}"))?;
    let mut profile: ProfilePack =
        serde_yaml::from_str(&profile_yaml).with_context(|| format!("parsing {profile_id}"))?;
    profile.inline_rules = read_yaml(&profile_dir.join("inline_rules.yaml"))?;
    profile.accepted_taxonomies = read_yaml(&profile_dir.join("accepted_taxonomies.yaml"))?;
    profile.standard_taxonomy_uris =
        read_standard_taxonomy_uris(&profile_dir.join("edgartaxonomies.xml"))?;
    profile.required_facts =
        read_yaml::<RequiredFactsFile>(&profile_dir.join("required_facts.yaml"))
            .map(|f| f.required_facts)
            .unwrap_or_default();
    Ok(profile)
}

#[must_use]
pub fn profile_dir(root: &Path, profile_id: &str) -> PathBuf {
    let mut dir = root.join("profiles");
    for component in profile_id.split('/') {
        dir.push(component);
    }
    dir
}

fn read_yaml<T>(path: &Path) -> anyhow::Result<T>
where
    T: for<'de> Deserialize<'de> + Default,
{
    let bytes =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_yaml::from_str(&bytes).with_context(|| format!("parsing {}", path.display()))
}

fn read_standard_taxonomy_uris(path: &Path) -> anyhow::Result<Vec<String>> {
    let xml =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    Ok(extract_attribute_values(&xml, "namespace"))
}

fn extract_attribute_values(xml: &str, attribute: &str) -> Vec<String> {
    let needle = format!("{attribute}=\"");
    let mut values = Vec::new();
    let mut remainder = xml;
    while let Some(start) = remainder.find(&needle) {
        let value_start = start + needle.len();
        remainder = &remainder[value_start..];
        let Some(end) = remainder.find('"') else {
            break;
        };
        values.push(remainder[..end].to_string());
        remainder = &remainder[end + 1..];
    }
    values
}

#[cfg(test)]
mod tests {
    use super::{extract_attribute_values, load_profile_from_workspace};
    use std::path::PathBuf;

    #[test]
    fn load_profile_pack_from_workspace() {
        let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let workspace_root = manifest_dir
            .parent()
            .and_then(|path| path.parent())
            .expect("workspace root");
        let profile = load_profile_from_workspace(workspace_root, "sec/efm-77/opco")
            .expect("profile pack should load");

        assert_eq!(profile.id, "sec/efm-77/opco");
        assert!(
            profile
                .inline_rules
                .banned_elements
                .contains(&"ix:fraction".to_string())
        );
        assert!(
            profile
                .accepted_taxonomies
                .namespaces
                .iter()
                .any(|namespace| namespace.prefix == "dei")
        );
        assert!(
            profile
                .standard_taxonomy_uris
                .iter()
                .any(|uri| uri.contains("/dei/2025/"))
        );
    }

    #[test]
    fn extracts_attribute_values_from_simple_xml() {
        let xml = r#"<root><node namespace="alpha" /><node namespace="beta" /></root>"#;

        assert_eq!(
            extract_attribute_values(xml, "namespace"),
            vec!["alpha".to_string(), "beta".to_string()]
        );
    }
}
