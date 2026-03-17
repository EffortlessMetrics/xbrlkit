//! Taxonomy package helpers.

use taxonomy_types::DtsDescriptor;

#[must_use]
pub fn load_entry_points(entry_points: Vec<String>) -> DtsDescriptor {
    DtsDescriptor {
        entry_points,
        namespaces: Vec::new(),
    }
}
