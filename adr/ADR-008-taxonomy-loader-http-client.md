# ADR-008: Taxonomy-Loader HTTP Client Architecture

## Context

The taxonomy-loader crate needs to fetch XBRL taxonomy files from remote sources (e.g., FASB, SEC, ESMA). These taxonomies can be large (multi-MB XSD and linkbase files) and are frequently referenced by URL in XBRL instance documents. The crate already has local file loading but lacks HTTP support.

Key requirements:
- Fetch taxonomy files via HTTP/HTTPS
- Cache downloaded files locally to avoid repeated downloads
- Handle network errors gracefully
- Maintain synchronous API compatibility (existing consumers expect sync)
- Avoid OpenSSL dependency issues (prefer pure Rust TLS)

## Decision

Use **reqwest** with the blocking client API for HTTP fetching in taxonomy-loader.

### Configuration

```toml
[dependencies]
reqwest = { version = "0.12", features = ["rustls-tls", "blocking"], default-features = false }
url = "2.5"
```

### Pattern

The loader uses a hybrid approach:
- **With cache**: `TaxonomyLoader::with_cache_dir()` enables HTTP fetching + disk caching
- **Without cache**: `TaxonomyLoader::new()` supports local files only (no HTTP)

```rust
pub struct TaxonomyLoader {
    cache_dir: Option<PathBuf>,
    http_client: Option<reqwest::blocking::Client>,
    // ...
}

impl TaxonomyLoader {
    pub fn with_cache_dir(path: impl Into<PathBuf>) -> Self {
        Self {
            cache_dir: Some(path.into()),
            http_client: Self::build_http_client(),
            // ...
        }
    }

    fn build_http_client() -> Option<reqwest::blocking::Client> {
        reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent(concat!("xbrlkit/", env!("CARGO_PKG_VERSION")))
            .build()
            .ok()
    }
}
```

### Cache Strategy

1. Check cache first (if configured)
2. Cache hit → return cached content
3. Cache miss → fetch via HTTP
4. On successful fetch → write to cache, then return content

```rust
fn fetch_url(&self, url: &str) -> Result<String, TaxonomyLoaderError> {
    // Check cache first
    if let Some(ref cache_dir) = self.cache_dir {
        let cache_path = Self::url_to_cache_path(url, cache_dir);
        if cache_path.exists() {
            return std::fs::read_to_string(&cache_path)
                .map_err(|e| TaxonomyLoaderError::Io(...));
        }
    }

    // Fetch via HTTP
    let content = self.http_client
        .get(url)
        .send()?
        .text()?;

    // Write to cache (non-fatal on failure)
    if let Some(ref cache_dir) = self.cache_dir {
        let _ = Self::write_to_cache(&content, &cache_path);
    }

    Ok(content)
}
```

### Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum TaxonomyLoaderError {
    #[error("HTTP request failed for {0}: {1}")]
    HttpError(String, String),

    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    #[error("unsupported URL: {0}")]
    UnsupportedUrl(String),
    // ...
}
```

## Alternatives Considered

| Library | Verdict | Rationale |
|---------|---------|-----------|
| **reqwest** (blocking) | ✅ Selected | Native blocking API, excellent ergonomics, automatic redirect handling, connection pooling |
| ureq | ❌ Rejected | Blocking only, less ergonomic, manual redirect handling |
| hyper | ❌ Rejected | Too low-level, requires significant boilerplate for simple use case |
| async reqwest | ❌ Rejected | Would require async runtime changes throughout the codebase |

## Rationale

- **Blocking API compatibility**: Existing consumers expect sync APIs; async would require breaking changes
- **rustls-tls**: Pure Rust TLS avoids OpenSSL system dependency issues
- **Timeout + User-Agent**: Professional HTTP client behavior out of the box
- **Cache opt-in**: HTTP fetching only enabled when cache is configured (predictable behavior)
- **Cache write failures are non-fatal**: Network fetch succeeds even if disk write fails

## Consequences

### Positive
- Simple, ergonomic HTTP API
- Automatic redirect following
- Connection pooling and reuse
- Professional User-Agent header
- Pure Rust TLS (no OpenSSL)

### Trade-offs
- Blocking I/O in async contexts could cause issues (mitigated: taxonomy loading is typically done once at startup)
- 30-second timeout may be insufficient for very slow connections (can be made configurable in future)
- Cache has no TTL (files never expire; acceptable for stable taxonomy releases)

## Related

- Issue #102: ADR: Taxonomy-loader HTTP client architecture
- PR #97: Taxonomy loader implementation with HTTP fetching
- ADR-007: Validation pattern for SEC rules
- taxonomy-loader crate: `crates/taxonomy-loader/`

## Usage Example

```rust
use taxonomy_loader::TaxonomyLoader;

// With caching (enables HTTP support)
let loader = TaxonomyLoader::with_cache_dir("~/.cache/xbrlkit/taxonomy");
let taxonomy = loader.load("https://xbrl.fasb.org/us-gaap/2024/entire/us-gaap-2024.xsd")?;

// Without caching (local files only)
let loader = TaxonomyLoader::new();
let taxonomy = loader.load("/local/path/to/taxonomy.xsd")?;
```
