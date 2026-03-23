//! XBRL taxonomy loader.
//!
//! Loads dimension taxonomies from XSD schema files and definition linkbases.
//!
//! # Example
//!
//! ```
//! use taxonomy_loader::load_taxonomy;
//!
//! // Load from an entrypoint URL or local path
//! // let taxonomy = load_taxonomy("https://xbrl.fasb.org/us-gaap/2024/entire/us-gaap-2024.xsd")?;
//! ```

mod error;
mod linkbase;
mod schema;

pub use error::TaxonomyLoaderError;

// Re-export taxonomy_dimensions types for CLI and other consumers
pub use taxonomy_dimensions::DimensionTaxonomy;
pub use taxonomy_dimensions::{Dimension, Domain, Hypercube};

use std::collections::HashSet;
use std::path::Path;

/// Loads a dimension taxonomy from an entrypoint URL or local path.
///
/// # Errors
///
/// Returns an error if the taxonomy cannot be loaded or parsed.
pub fn load_taxonomy(entrypoint: &str) -> Result<DimensionTaxonomy, TaxonomyLoaderError> {
    let loader = TaxonomyLoader::new();
    loader.load(entrypoint)
}

/// XBRL taxonomy loader with optional caching support.
#[derive(Debug, Clone)]
pub struct TaxonomyLoader {
    cache_dir: Option<std::path::PathBuf>,
    visited: std::cell::RefCell<HashSet<String>>,
}

impl Default for TaxonomyLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl TaxonomyLoader {
    /// Creates a new taxonomy loader without caching.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache_dir: None,
            visited: std::cell::RefCell::new(HashSet::new()),
        }
    }

    /// Creates a new taxonomy loader with a cache directory.
    #[must_use]
    pub fn with_cache_dir(path: impl Into<std::path::PathBuf>) -> Self {
        Self {
            cache_dir: Some(path.into()),
            visited: std::cell::RefCell::new(HashSet::new()),
        }
    }

    /// Loads a dimension taxonomy from an entrypoint.
    ///
    /// # Errors
    ///
    /// Returns an error if the taxonomy cannot be loaded or parsed.
    pub fn load(&self, entrypoint: &str) -> Result<DimensionTaxonomy, TaxonomyLoaderError> {
        let mut taxonomy = DimensionTaxonomy::new();

        // Load the entrypoint schema
        self.load_schema_recursive(entrypoint, &mut taxonomy)?;

        Ok(taxonomy)
    }

    fn load_schema_recursive(
        &self,
        path: &str,
        taxonomy: &mut DimensionTaxonomy,
    ) -> Result<(), TaxonomyLoaderError> {
        // Prevent circular imports
        if self.visited.borrow().contains(path) {
            return Ok(());
        }
        self.visited.borrow_mut().insert(path.to_string());

        // Read schema content
        let content = self.fetch_content(path)?;

        // Parse schema for dimension elements
        schema::parse_schema(&content, taxonomy)?;

        // Find and load linked linkbases
        let linkbase_refs = linkbase::extract_linkbase_refs(&content, path)?;
        for linkbase_ref in linkbase_refs {
            self.load_linkbase(&linkbase_ref, taxonomy)?;
        }

        // Find and process schema imports/includes
        let import_refs = schema::extract_import_refs(&content, path)?;
        for import_ref in import_refs {
            self.load_schema_recursive(&import_ref, taxonomy)?;
        }

        Ok(())
    }

    fn load_linkbase(
        &self,
        path: &str,
        taxonomy: &mut DimensionTaxonomy,
    ) -> Result<(), TaxonomyLoaderError> {
        let content = self.fetch_content(path)?;
        linkbase::parse_definition_linkbase(&content, taxonomy)?;
        Ok(())
    }

    fn fetch_content(&self, path: &str) -> Result<String, TaxonomyLoaderError> {
        // Check if it's a URL or local path
        if path.starts_with("http://") || path.starts_with("https://") {
            self.fetch_url(path)
        } else {
            self.fetch_file(path)
        }
    }

    fn fetch_url(&self, url: &str) -> Result<String, TaxonomyLoaderError> {
        // Try cache first
        if let Some(ref cache_dir) = self.cache_dir {
            let cache_path = self.url_to_cache_path(url, cache_dir);
            if cache_path.exists() {
                return std::fs::read_to_string(&cache_path).map_err(|e| {
                    TaxonomyLoaderError::Io(cache_path.to_string_lossy().to_string(), e)
                });
            }
        }

        // For now, return an error since we don't have HTTP client yet
        // TODO: Add reqwest or similar for HTTP fetching
        Err(TaxonomyLoaderError::UnsupportedUrl(url.to_string()))
    }

    fn fetch_file(&self, path: &str) -> Result<String, TaxonomyLoaderError> {
        std::fs::read_to_string(path).map_err(|e| TaxonomyLoaderError::Io(path.to_string(), e))
    }

    fn url_to_cache_path(&self, url: &str, cache_dir: &Path) -> std::path::PathBuf {
        // Simple cache path generation based on URL
        let filename = url.replace(['/', ':', '?', '&', '='], "_");
        cache_dir.join(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_new() {
        let loader = TaxonomyLoader::new();
        assert!(loader.cache_dir.is_none());
    }

    #[test]
    fn test_loader_with_cache() {
        let loader = TaxonomyLoader::with_cache_dir("/tmp/cache");
        assert!(loader.cache_dir.is_some());
    }
}
