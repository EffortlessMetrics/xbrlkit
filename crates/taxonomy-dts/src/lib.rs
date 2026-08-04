//! DTS construction and profile-aware checks.

use sec_profile_types::ProfilePack;
use taxonomy_types::DtsDescriptor;

#[must_use]
pub fn build_dts(profile: &ProfilePack, entry_points: Vec<String>) -> DtsDescriptor {
    let namespaces = profile
        .accepted_taxonomies
        .namespaces
        .iter()
        .filter(|namespace| {
            entry_points
                .iter()
                .any(|entry_point| entry_point == &namespace.uri)
        })
        .cloned()
        .collect::<Vec<_>>();
    DtsDescriptor {
        entry_points,
        namespaces,
    }
}

#[must_use]
pub fn mixed_taxonomy_years(entry_points: &[String]) -> bool {
    let years = entry_points
        .iter()
        .filter_map(|entry| {
            entry
                .split('/')
                .find(|segment| segment.len() == 4 && segment.chars().all(|ch| ch.is_ascii_digit()))
        })
        .collect::<std::collections::BTreeSet<_>>();
    years.len() > 1
}

#[must_use]
pub fn nonstandard_entry_points(dts: &DtsDescriptor, profile: &ProfilePack) -> Vec<String> {
    dts.entry_points
        .iter()
        .filter(|entry_point| {
            !profile
                .standard_taxonomy_uris
                .iter()
                .any(|standard| standard == *entry_point)
        })
        .cloned()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{build_dts, mixed_taxonomy_years, nonstandard_entry_points};
    use sec_profile_types::{AcceptedTaxonomies, ProfilePack};
    use taxonomy_types::{DtsDescriptor, NamespaceMapping};

    #[test]
    fn projects_only_accepted_namespaces_present_in_entry_points() -> Result<(), String> {
        let entry_points = vec![
            "https://example.test/us-gaap/2025.xsd".to_string(),
            "https://example.test/custom.xsd".to_string(),
        ];
        let profile = ProfilePack {
            accepted_taxonomies: AcceptedTaxonomies {
                namespaces: vec![
                    NamespaceMapping {
                        prefix: "us-gaap".to_string(),
                        uri: entry_points[0].clone(),
                    },
                    NamespaceMapping {
                        prefix: "dei".to_string(),
                        uri: "https://example.test/dei/2025.xsd".to_string(),
                    },
                ],
                ..AcceptedTaxonomies::default()
            },
            ..ProfilePack::default()
        };

        let dts = build_dts(&profile, entry_points.clone());
        if dts.entry_points != entry_points {
            return Err("DTS entry points were not preserved".to_string());
        }
        if dts.namespaces
            != vec![NamespaceMapping {
                prefix: "us-gaap".to_string(),
                uri: "https://example.test/us-gaap/2025.xsd".to_string(),
            }]
        {
            return Err("DTS projected an unreferenced namespace".to_string());
        }
        Ok(())
    }

    #[test]
    fn detects_mixed_years_but_not_same_or_missing_years() -> Result<(), String> {
        let mixed = vec![
            "https://example.test/us-gaap/2024/us-gaap.xsd".to_string(),
            "https://example.test/us-gaap/2025/us-gaap.xsd".to_string(),
        ];
        if !mixed_taxonomy_years(&mixed) {
            return Err("different taxonomy years were not detected".to_string());
        }

        let same = vec![
            "https://example.test/us-gaap/2025/us-gaap.xsd".to_string(),
            "https://example.test/dei/2025/dei.xsd".to_string(),
        ];
        if mixed_taxonomy_years(&same) {
            return Err("same taxonomy year was reported as mixed".to_string());
        }

        let missing = vec!["https://example.test/custom/current.xsd".to_string()];
        if mixed_taxonomy_years(&missing) {
            return Err("entries without year segments were reported as mixed".to_string());
        }
        Ok(())
    }

    #[test]
    fn returns_only_nonstandard_entry_points() -> Result<(), String> {
        let standard = "https://example.test/us-gaap/2025.xsd".to_string();
        let custom = "https://example.test/custom.xsd".to_string();
        let dts = DtsDescriptor {
            entry_points: vec![standard.clone(), custom.clone()],
            ..DtsDescriptor::default()
        };
        let profile = ProfilePack {
            standard_taxonomy_uris: vec![standard],
            ..ProfilePack::default()
        };

        let nonstandard = nonstandard_entry_points(&dts, &profile);
        if nonstandard != vec![custom] {
            return Err("standard entry points were not filtered".to_string());
        }
        Ok(())
    }
}
