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
#[cfg(feature = "http")]
use std::path::Path;
#[cfg(feature = "http")]
use std::time::Duration;

/// Default timeout for HTTP requests (30 seconds).
#[cfg(feature = "http")]
const HTTP_TIMEOUT: Duration = Duration::from_secs(30);

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
    #[cfg(feature = "http")]
    cache_dir: Option<std::path::PathBuf>,
    visited: std::cell::RefCell<HashSet<String>>,
    #[cfg(feature = "http")]
    http_client: Option<reqwest::blocking::Client>,
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
            #[cfg(feature = "http")]
            cache_dir: None,
            visited: std::cell::RefCell::new(HashSet::new()),
            #[cfg(feature = "http")]
            http_client: None,
        }
    }

    /// Creates a new taxonomy loader with a cache directory.
    #[must_use]
    pub fn with_cache_dir(path: impl Into<std::path::PathBuf>) -> Self {
        #[cfg(feature = "http")]
        let (cache_dir, http_client) = {
            let cache_dir = Some(path.into());
            let http_client = Self::build_http_client();
            (cache_dir, http_client)
        };
        #[cfg(not(feature = "http"))]
        let _ = path.into();

        Self {
            #[cfg(feature = "http")]
            cache_dir,
            visited: std::cell::RefCell::new(HashSet::new()),
            #[cfg(feature = "http")]
            http_client,
        }
    }

    /// Builds the HTTP client with proper configuration.
    #[cfg(feature = "http")]
    fn build_http_client() -> Option<reqwest::blocking::Client> {
        reqwest::blocking::Client::builder()
            .timeout(HTTP_TIMEOUT)
            .user_agent(concat!("xbrlkit/", env!("CARGO_PKG_VERSION")))
            .build()
            .ok()
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
        #[cfg(not(feature = "http"))]
        let _ = self;

        // Check if it's a URL or local path
        if path.starts_with("http://") || path.starts_with("https://") {
            #[cfg(feature = "http")]
            {
                return self.fetch_url(path);
            }
            #[cfg(not(feature = "http"))]
            {
                return Err(TaxonomyLoaderError::UnsupportedUrl(path.to_string()));
            }
        }

        TaxonomyLoader::fetch_file(path)
    }

    #[cfg(feature = "http")]
    fn fetch_url(&self, url: &str) -> Result<String, TaxonomyLoaderError> {
        // Validate URL format
        let parsed_url: url::Url = url.parse()?;

        // Only allow http and https schemes
        if parsed_url.scheme() != "http" && parsed_url.scheme() != "https" {
            return Err(TaxonomyLoaderError::UnsupportedUrl(url.to_string()));
        }

        // Check cache first
        if let Some(ref cache_dir) = self.cache_dir {
            let cache_path = TaxonomyLoader::url_to_cache_path(url, cache_dir);
            if cache_path.exists() {
                return std::fs::read_to_string(&cache_path).map_err(|e| {
                    TaxonomyLoaderError::Io(cache_path.to_string_lossy().to_string(), e)
                });
            }
        }

        // Ensure we have an HTTP client
        let client = if let Some(ref client) = self.http_client {
            client.clone()
        } else {
            Self::build_http_client().ok_or_else(|| {
                TaxonomyLoaderError::HttpError(
                    url.to_string(),
                    "Failed to build HTTP client".into(),
                )
            })?
        };

        // Fetch content via HTTP (blocking)
        let response = client
            .get(url)
            .send()
            .map_err(|e| TaxonomyLoaderError::HttpError(url.to_string(), e.to_string()))?;

        // Check for HTTP errors
        if !response.status().is_success() {
            return Err(TaxonomyLoaderError::HttpError(
                url.to_string(),
                format!("HTTP {}", response.status()),
            ));
        }

        let content = response
            .text()
            .map_err(|e| TaxonomyLoaderError::HttpError(url.to_string(), e.to_string()))?;

        // Write to cache if configured
        if let Some(ref cache_dir) = self.cache_dir {
            let cache_path = TaxonomyLoader::url_to_cache_path(url, cache_dir);
            if let Err(e) = Self::write_to_cache(&content, &cache_path) {
                // Cache write failure is non-fatal, just log it
                eprintln!("Warning: Failed to write cache for {url}: {e}");
            }
        }

        Ok(content)
    }

    #[cfg(feature = "http")]
    fn write_to_cache(content: &str, cache_path: &Path) -> Result<(), std::io::Error> {
        // Ensure parent directory exists
        if let Some(parent) = cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(cache_path, content)
    }

    fn fetch_file(path: &str) -> Result<String, TaxonomyLoaderError> {
        std::fs::read_to_string(path).map_err(|e| TaxonomyLoaderError::Io(path.to_string(), e))
    }

    #[cfg(feature = "http")]
    fn url_to_cache_path(url: &str, cache_dir: &Path) -> std::path::PathBuf {
        // Simple cache path generation based on URL
        let filename = url.replace(['/', ':', '?', '&', '='], "_");
        cache_dir.join(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "http")]
    #[test]
    fn test_loader_new() -> Result<(), String> {
        let loader = std::hint::black_box(TaxonomyLoader::new());
        if std::hint::black_box(loader.cache_dir.is_some()) {
            return Err("new loader should not have a cache directory".to_string());
        }
        Ok(())
    }

    #[cfg(not(feature = "http"))]
    #[test]
    fn test_loader_new() {
        let _loader = TaxonomyLoader::new();
    }

    #[test]
    #[cfg(feature = "http")]
    fn test_loader_with_cache() -> Result<(), String> {
        let loader = TaxonomyLoader::with_cache_dir("/tmp/cache");
        if loader.cache_dir.is_none() {
            return Err("cache-configured loader should retain its cache directory".to_string());
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "http")]
    fn test_url_to_cache_path() -> Result<(), String> {
        let cache_dir = Path::new("/tmp/cache");
        let url = "https://xbrl.fasb.org/us-gaap/2024/entire/us-gaap-2024.xsd";
        let path = TaxonomyLoader::url_to_cache_path(url, cache_dir);
        // URL chars / : are replaced with _, https:// becomes https___
        let expected =
            Path::new("/tmp/cache/https___xbrl.fasb.org_us-gaap_2024_entire_us-gaap-2024.xsd");
        if path != expected {
            return Err(format!(
                "unexpected cache path: got {}, expected {}",
                path.display(),
                expected.display()
            ));
        }
        Ok(())
    }

    #[test]
    #[cfg(feature = "http")]
    fn test_fetch_url_invalid_scheme() -> Result<(), String> {
        let loader = TaxonomyLoader::new();
        let result = loader.fetch_url("ftp://example.com/test.xsd");

        match result {
            Err(TaxonomyLoaderError::UnsupportedUrl(_)) => Ok(()),
            Ok(_) => Err("unsupported URL scheme unexpectedly succeeded".to_string()),
            Err(error) => Err(format!("unexpected error for unsupported scheme: {error}")),
        }
    }

    #[cfg(not(feature = "http"))]
    #[test]
    fn test_http_url_requires_http_feature() -> Result<(), String> {
        let loader = TaxonomyLoader::new();
        let result = loader.fetch_content("https://example.com/test.xsd");

        match result {
            Err(TaxonomyLoaderError::UnsupportedUrl(url))
                if url == "https://example.com/test.xsd" =>
            {
                Ok(())
            }
            Ok(_) => Err("HTTP URL unexpectedly succeeded without the http feature".to_string()),
            Err(error) => Err(format!("unexpected error for HTTP URL: {error}")),
        }
    }

    #[cfg(not(feature = "http"))]
    #[test]
    fn local_taxonomy_loading_remains_available_without_http_feature()
    -> Result<(), Box<dyn std::error::Error>> {
        let directory = tempfile::tempdir()?;
        let schema_path = directory.path().join("schema.xsd");
        std::fs::write(
            &schema_path,
            r#"<xs:schema xmlns:xs="http://www.w3.org/2001/XMLSchema" targetNamespace="urn:test" />"#,
        )?;
        let entrypoint = schema_path
            .to_str()
            .ok_or_else(|| std::io::Error::other("temporary schema path is not UTF-8"))?;

        load_taxonomy(entrypoint)?;
        Ok(())
    }
}
