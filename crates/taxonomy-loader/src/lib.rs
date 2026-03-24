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
use std::time::Duration;

/// Default timeout for HTTP requests (30 seconds).
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
    cache_dir: Option<std::path::PathBuf>,
    visited: std::cell::RefCell<HashSet<String>>,
    http_client: Option<reqwest::Client>,
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
            http_client: None,
        }
    }

    /// Creates a new taxonomy loader with a cache directory.
    #[must_use]
    pub fn with_cache_dir(path: impl Into<std::path::PathBuf>) -> Self {
        let cache_dir = Some(path.into());
        let http_client = Self::build_http_client();
        Self {
            cache_dir,
            visited: std::cell::RefCell::new(HashSet::new()),
            http_client,
        }
    }

    /// Builds the HTTP client with proper configuration.
    fn build_http_client() -> Option<reqwest::Client> {
        reqwest::Client::builder()
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
        // Check if it's a URL or local path
        if path.starts_with("http://") || path.starts_with("https://") {
            self.fetch_url(path)
        } else {
            TaxonomyLoader::fetch_file(path)
        }
    }

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

        // Fetch content via HTTP (blocking for sync API compatibility)
        let content = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                let response =
                    client.get(url).send().await.map_err(|e| {
                        TaxonomyLoaderError::HttpError(url.to_string(), e.to_string())
                    })?;

                // Check for HTTP errors
                if !response.status().is_success() {
                    return Err(TaxonomyLoaderError::HttpError(
                        url.to_string(),
                        format!("HTTP {}", response.status()),
                    ));
                }

                response
                    .text()
                    .await
                    .map_err(|e| TaxonomyLoaderError::HttpError(url.to_string(), e.to_string()))
            })
        })?;

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

    fn url_to_cache_path(url: &str, cache_dir: &Path) -> std::path::PathBuf {
        // Simple cache path generation based on URL
        let filename = url.replace(['/', ':', '?', '&', '='], "_");
        cache_dir.join(filename)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

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

    #[test]
    fn test_url_to_cache_path() {
        let cache_dir = Path::new("/tmp/cache");
        let url = "https://xbrl.fasb.org/us-gaap/2024/entire/us-gaap-2024.xsd";
        let path = TaxonomyLoader::url_to_cache_path(url, cache_dir);
        // URL chars / : are replaced with _, https:// becomes https___
        assert_eq!(
            path,
            Path::new("/tmp/cache/https___xbrl.fasb.org_us-gaap_2024_entire_us-gaap-2024.xsd")
        );
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_fetch_url_success() {
        // Start mock server
        let mock_server = MockServer::start().await;

        // Create test content
        let test_content = r#"<?xml version="1.0"?>
<schema xmlns="http://www.w3.org/2001/XMLSchema">
    <element name="test"/>
</schema>"#;

        // Set up mock response
        Mock::given(method("GET"))
            .and(path("/test.xsd"))
            .respond_with(ResponseTemplate::new(200).set_body_string(test_content))
            .mount(&mock_server)
            .await;

        // Create temp cache directory
        let temp_dir = tempfile::tempdir().unwrap();
        let loader = TaxonomyLoader::with_cache_dir(temp_dir.path());

        // Fetch URL
        let url = format!("{}/test.xsd", mock_server.uri());
        let result = loader.fetch_url(&url);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);

        // Verify it was cached
        let cache_path = TaxonomyLoader::url_to_cache_path(&url, temp_dir.path());
        assert!(cache_path.exists());
        let cached_content = std::fs::read_to_string(&cache_path).unwrap();
        assert_eq!(cached_content, test_content);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_fetch_url_uses_cache() {
        let mock_server = MockServer::start().await;
        let test_content = "cached content";

        Mock::given(method("GET"))
            .and(path("/cached.xsd"))
            .respond_with(ResponseTemplate::new(200).set_body_string("fresh content"))
            .mount(&mock_server)
            .await;

        let temp_dir = tempfile::tempdir().unwrap();
        let loader = TaxonomyLoader::with_cache_dir(temp_dir.path());

        // Pre-populate cache
        let url = format!("{}/cached.xsd", mock_server.uri());
        let cache_path = TaxonomyLoader::url_to_cache_path(&url, temp_dir.path());
        std::fs::create_dir_all(temp_dir.path()).unwrap();
        std::fs::write(&cache_path, test_content).unwrap();

        // Fetch should use cache, not make HTTP request
        let result = loader.fetch_url(&url);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn test_fetch_url_http_error() {
        let mock_server = MockServer::start().await;

        Mock::given(method("GET"))
            .and(path("/error.xsd"))
            .respond_with(ResponseTemplate::new(404))
            .mount(&mock_server)
            .await;

        let temp_dir = tempfile::tempdir().unwrap();
        let loader = TaxonomyLoader::with_cache_dir(temp_dir.path());

        let url = format!("{}/error.xsd", mock_server.uri());
        let result = loader.fetch_url(&url);

        assert!(result.is_err());
        let err_str = format!("{}", result.unwrap_err());
        assert!(err_str.contains("HTTP 404"));
    }

    #[test]
    fn test_fetch_url_invalid_scheme() {
        let loader = TaxonomyLoader::new();
        let result = loader.fetch_url("ftp://example.com/test.xsd");

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            TaxonomyLoaderError::UnsupportedUrl(_)
        ));
    }
}
