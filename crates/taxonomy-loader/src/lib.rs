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

use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

/// Default timeout for HTTP requests (30 seconds).
const HTTP_TIMEOUT: Duration = Duration::from_secs(30);

/// The response data needed by the loader, kept independent of the HTTP
/// client implementation so tests can exercise remote-loading branches
/// without opening a network connection.
#[derive(Debug, Clone)]
struct HttpResponse {
    status: u16,
    status_text: String,
    body: String,
}

impl HttpResponse {
    fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
}

trait HttpTransport: std::fmt::Debug + Send + Sync {
    fn fetch(&self, url: &str) -> Result<HttpResponse, String>;
}

#[derive(Debug, Clone)]
struct ReqwestTransport {
    client: reqwest::blocking::Client,
}

impl HttpTransport for ReqwestTransport {
    fn fetch(&self, url: &str) -> Result<HttpResponse, String> {
        let response = self
            .client
            .get(url)
            .send()
            .map_err(|error| error.to_string())?;
        let status = response.status();
        let body = if status.is_success() {
            response.text().map_err(|error| error.to_string())?
        } else {
            String::new()
        };

        Ok(HttpResponse {
            status: status.as_u16(),
            status_text: status.to_string(),
            body,
        })
    }
}

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
    cache_hits: std::cell::RefCell<HashSet<String>>,
    loaded_schemas: std::cell::RefCell<HashSet<String>>,
    offline_contents: Option<HashMap<String, String>>,
    http_transport: Option<Arc<dyn HttpTransport>>,
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
            cache_hits: std::cell::RefCell::new(HashSet::new()),
            loaded_schemas: std::cell::RefCell::new(HashSet::new()),
            offline_contents: None,
            http_transport: None,
        }
    }

    /// Creates a new taxonomy loader with a cache directory.
    #[must_use]
    pub fn with_cache_dir(path: impl Into<std::path::PathBuf>) -> Self {
        Self::with_cache_dir_and_offline_contents(path.into(), None)
    }

    /// Creates a loader with a cache and deterministic content for one URL.
    ///
    /// The configured content is used only after a cache miss and is written
    /// to the cache. This provides an offline seam for acceptance tests while
    /// preserving the normal HTTP path for other URLs.
    #[must_use]
    pub fn with_cache_dir_and_offline_content(
        path: impl Into<std::path::PathBuf>,
        url: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        let mut offline_contents = HashMap::new();
        offline_contents.insert(url.into(), content.into());
        Self::with_cache_dir_and_offline_contents(path.into(), Some(offline_contents))
    }

    fn with_cache_dir_and_offline_contents(
        cache_dir: std::path::PathBuf,
        offline_contents: Option<HashMap<String, String>>,
    ) -> Self {
        let http_transport = Self::build_http_transport();
        Self {
            cache_dir: Some(cache_dir),
            visited: std::cell::RefCell::new(HashSet::new()),
            cache_hits: std::cell::RefCell::new(HashSet::new()),
            loaded_schemas: std::cell::RefCell::new(HashSet::new()),
            offline_contents,
            http_transport,
        }
    }

    /// Builds the HTTP client with proper configuration.
    fn build_http_transport() -> Option<Arc<dyn HttpTransport>> {
        reqwest::blocking::Client::builder()
            .timeout(HTTP_TIMEOUT)
            .user_agent(concat!("xbrlkit/", env!("CARGO_PKG_VERSION")))
            .build()
            .ok()
            .map(|client| Arc::new(ReqwestTransport { client }) as Arc<dyn HttpTransport>)
    }

    /// Loads a dimension taxonomy from an entrypoint.
    ///
    /// # Errors
    ///
    /// Returns an error if the taxonomy cannot be loaded or parsed.
    pub fn load(&self, entrypoint: &str) -> Result<DimensionTaxonomy, TaxonomyLoaderError> {
        // `visited` belongs to one resolution pass; a loader can be reused for
        // a later load and should resolve the entrypoint again from its cache.
        self.visited.borrow_mut().clear();
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

        self.loaded_schemas.borrow_mut().insert(path.to_string());

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
                let content = std::fs::read_to_string(&cache_path).map_err(|e| {
                    TaxonomyLoaderError::Io(cache_path.to_string_lossy().to_string(), e)
                })?;
                self.cache_hits.borrow_mut().insert(url.to_string());
                return Ok(content);
            }
        }

        if let Some(content) = self
            .offline_contents
            .as_ref()
            .and_then(|contents| contents.get(url))
        {
            let content = content.clone();
            if let Some(ref cache_dir) = self.cache_dir {
                let cache_path = TaxonomyLoader::url_to_cache_path(url, cache_dir);
                if let Err(error) = Self::write_to_cache(&content, &cache_path) {
                    eprintln!("Warning: Failed to write cache for {url}: {error}");
                }
            }
            return Ok(content);
        }

        // Ensure we have an HTTP transport.
        let transport = self
            .http_transport
            .clone()
            .or_else(Self::build_http_transport)
            .ok_or_else(|| {
                TaxonomyLoaderError::HttpError(
                    url.to_string(),
                    "Failed to build HTTP client".into(),
                )
            })?;

        // Fetch content via the configured transport.
        let response = transport
            .fetch(url)
            .map_err(|error| TaxonomyLoaderError::HttpError(url.to_string(), error))?;

        // Check for HTTP errors
        if !response.is_success() {
            return Err(TaxonomyLoaderError::HttpError(
                url.to_string(),
                format!("HTTP {}", response.status_text),
            ));
        }

        let content = response.body;

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

    /// Returns the URLs served from the local cache by this loader.
    #[must_use]
    pub fn cache_hits(&self) -> HashSet<String> {
        self.cache_hits.borrow().clone()
    }

    /// Returns the schemas successfully resolved by this loader.
    #[must_use]
    pub fn loaded_schemas(&self) -> HashSet<String> {
        self.loaded_schemas.borrow().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    const TEST_URL: &str = "https://example.com/schema.xsd";
    const MINIMAL_SCHEMA: &str = r#"<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
        xmlns:xbrldt="http://xbrl.org/2005/xbrldt"
        xmlns:demo="https://example.com/demo"
        targetNamespace="https://example.com/demo">
        <xsd:element name="Axis" substitutionGroup="xbrldt:dimensionItem"/>
    </xsd:schema>"#;

    #[derive(Debug)]
    struct TestTransport {
        response: Result<HttpResponse, String>,
        requested_urls: Mutex<Vec<String>>,
    }

    impl TestTransport {
        fn success(body: &str) -> Self {
            Self::with_response(HttpResponse {
                status: 200,
                status_text: "200 OK".to_string(),
                body: body.to_string(),
            })
        }

        fn status(status: u16, status_text: &str) -> Self {
            Self::with_response(HttpResponse {
                status,
                status_text: status_text.to_string(),
                body: String::new(),
            })
        }

        fn failure(message: &str) -> Self {
            Self {
                response: Err(message.to_string()),
                requested_urls: Mutex::new(Vec::new()),
            }
        }

        fn with_response(response: HttpResponse) -> Self {
            Self {
                response: Ok(response),
                requested_urls: Mutex::new(Vec::new()),
            }
        }

        fn requested_urls(&self) -> Result<Vec<String>, String> {
            self.requested_urls
                .lock()
                .map(|urls| urls.clone())
                .map_err(|error| format!("transport request log was poisoned: {error}"))
        }
    }

    impl HttpTransport for TestTransport {
        fn fetch(&self, url: &str) -> Result<HttpResponse, String> {
            self.requested_urls
                .lock()
                .map_err(|error| format!("transport request log was poisoned: {error}"))?
                .push(url.to_string());
            self.response.clone()
        }
    }

    fn loader_with_transport(transport: Arc<TestTransport>) -> TaxonomyLoader {
        TaxonomyLoader {
            cache_dir: None,
            visited: std::cell::RefCell::new(HashSet::new()),
            cache_hits: std::cell::RefCell::new(HashSet::new()),
            loaded_schemas: std::cell::RefCell::new(HashSet::new()),
            offline_contents: None,
            http_transport: Some(transport),
        }
    }

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

    #[test]
    fn test_remote_schema_uses_deterministic_transport() -> Result<(), Box<dyn std::error::Error>> {
        let transport = Arc::new(TestTransport::success(MINIMAL_SCHEMA));
        let loader = loader_with_transport(transport.clone());
        let taxonomy = loader.load(TEST_URL)?;

        if !taxonomy.dimensions.contains_key("demo:Axis") {
            return Err("remote schema did not populate its dimension".into());
        }
        if transport.requested_urls()? != vec![TEST_URL.to_string()] {
            return Err("remote schema was not fetched exactly once".into());
        }
        Ok(())
    }

    #[test]
    fn test_remote_status_failure_is_reported() -> Result<(), Box<dyn std::error::Error>> {
        let transport = Arc::new(TestTransport::status(404, "404 Not Found"));
        let loader = loader_with_transport(transport.clone());
        let result = loader.fetch_url(TEST_URL);

        match result {
            Err(TaxonomyLoaderError::HttpError(url, detail))
                if url == TEST_URL && detail == "HTTP 404 Not Found" => {}
            Err(error) => return Err(format!("unexpected taxonomy-loader error: {error}").into()),
            Ok(_) => return Err("a non-success response unexpectedly succeeded".into()),
        }
        if transport.requested_urls()? != vec![TEST_URL.to_string()] {
            return Err("status failure did not reach the transport".into());
        }
        Ok(())
    }

    #[test]
    fn test_remote_transport_failure_is_reported() -> Result<(), Box<dyn std::error::Error>> {
        let transport = Arc::new(TestTransport::failure("synthetic transport failure"));
        let loader = loader_with_transport(transport.clone());
        let result = loader.fetch_url(TEST_URL);

        match result {
            Err(TaxonomyLoaderError::HttpError(url, detail))
                if url == TEST_URL && detail == "synthetic transport failure" => {}
            Err(error) => return Err(format!("unexpected taxonomy-loader error: {error}").into()),
            Ok(_) => return Err("a transport failure unexpectedly succeeded".into()),
        }
        if transport.requested_urls()? != vec![TEST_URL.to_string()] {
            return Err("transport failure was not recorded".into());
        }
        Ok(())
    }

    #[test]
    fn test_unsupported_scheme_is_rejected_before_transport()
    -> Result<(), Box<dyn std::error::Error>> {
        let transport = Arc::new(TestTransport::success(MINIMAL_SCHEMA));
        let loader = loader_with_transport(transport.clone());
        let result = loader.fetch_url("ftp://example.com/test.xsd");

        if !matches!(result, Err(TaxonomyLoaderError::UnsupportedUrl(_))) {
            return Err("unsupported URL scheme was not rejected".into());
        }
        if !transport.requested_urls()?.is_empty() {
            return Err("unsupported URL scheme reached the transport".into());
        }
        Ok(())
    }

    #[test]
    fn test_cache_hit_tracking_records_a_successful_cache_read()
    -> Result<(), Box<dyn std::error::Error>> {
        let cache_dir = tempfile::tempdir()?;
        let url = "https://example.com/schema.xsd";
        let cache_path = TaxonomyLoader::url_to_cache_path(url, cache_dir.path());
        TaxonomyLoader::write_to_cache("cached schema", &cache_path)?;

        let loader = TaxonomyLoader::with_cache_dir(cache_dir.path());
        let content = loader.fetch_url(url)?;
        if content != "cached schema" {
            return Err(std::io::Error::other("unexpected cached content").into());
        }
        if !loader.cache_hits().contains(url) {
            return Err(std::io::Error::other("cache hit was not recorded").into());
        }
        Ok(())
    }

    #[test]
    fn test_loaded_schema_tracking_records_recursive_imports()
    -> Result<(), Box<dyn std::error::Error>> {
        let fixture_dir = tempfile::tempdir()?;
        let root = fixture_dir.path().join("root.xsd");
        let imported = fixture_dir.path().join("imported.xsd");
        std::fs::write(
            &root,
            r#"<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema">
                <xsd:import schemaLocation="imported.xsd"/>
            </xsd:schema>"#,
        )?;
        std::fs::write(
            &imported,
            r#"<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"/>"#,
        )?;

        let loader = TaxonomyLoader::new();
        loader.load(&root.to_string_lossy())?;
        let loaded_paths = loader.loaded_schemas();
        if !loaded_paths.iter().any(|path| path.ends_with("root.xsd")) {
            return Err(std::io::Error::other("root schema was not recorded").into());
        }
        if !loaded_paths
            .iter()
            .any(|path| path.ends_with("imported.xsd"))
        {
            return Err(std::io::Error::other("imported schema was not recorded").into());
        }
        Ok(())
    }

    #[test]
    fn test_reusing_loader_resolves_entrypoint_again() -> Result<(), Box<dyn std::error::Error>> {
        let fixture_dir = tempfile::tempdir()?;
        let schema = fixture_dir.path().join("dimension.xsd");
        std::fs::write(
            &schema,
            r#"<xsd:schema xmlns:xsd="http://www.w3.org/2001/XMLSchema"
                xmlns:xbrldt="http://xbrl.org/2005/xbrldt"
                xmlns:demo="https://example.com/demo"
                targetNamespace="https://example.com/demo">
                <xsd:element name="Axis" substitutionGroup="xbrldt:dimensionItem"/>
            </xsd:schema>"#,
        )?;

        let loader = TaxonomyLoader::new();
        let entrypoint = schema.to_string_lossy();
        if loader.load(&entrypoint)?.dimensions.is_empty() {
            return Err(std::io::Error::other("first taxonomy load was empty").into());
        }
        if loader.load(&entrypoint)?.dimensions.is_empty() {
            return Err(std::io::Error::other("second taxonomy load was empty").into());
        }
        Ok(())
    }
}
