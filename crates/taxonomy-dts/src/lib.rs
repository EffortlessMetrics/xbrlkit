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
