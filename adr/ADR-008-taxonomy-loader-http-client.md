# ADR-008: Taxonomy-Loader HTTP Client Architecture

**Status:** Accepted  
**Date:** 2026-03-28  
**Deciders:** xbrlkit Architecture Team

---

## Decision

Use **reqwest** with the **blocking** client API for HTTP fetching in the taxonomy-loader crate. Explicitly avoid async/tokio for this component.

---

## Context

The taxonomy-loader crate needs to fetch XBRL taxonomy files from remote sources (SEC, XBRL International, FASB). These taxonomies can be large (multi-MB XSD and linkbase files) and are frequently referenced by URL in XBRL instance documents.

Key requirements:
- Fetch taxonomy files via HTTP/HTTPS
- Cache downloaded files locally to avoid repeated downloads
- Handle network errors gracefully
- Maintain synchronous API compatibility (existing consumers expect sync)
- Avoid OpenSSL dependency issues (prefer pure Rust TLS)

The decision needed to be made on sync vs async HTTP client architecture, balancing simplicity against potential concurrency performance benefits.

---

## Decision Details

### Selected Approach

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

---

## Rationale

### Why Blocking reqwest?

1. **Taxonomy loading is typically sequential** - Schemas depend on each other; parallel fetching provides limited benefit
2. **Async overhead not justified** - For the expected use case (loading at startup), async runtime adds complexity without proportional benefit
3. **Blocking code is easier to reason about** - Simpler control flow, easier debugging
4. **Easier unit testing** - No async runtime needed in tests
5. **Reduced dependency tree** - No tokio runtime required
6. **Existing API compatibility** - Consumers expect sync APIs; async would require breaking changes

### Why Not Alternatives?

| Library | Verdict | Rationale |
|---------|---------|-----------|
| **reqwest (blocking)** | ✅ Selected | Native blocking API, excellent ergonomics, automatic redirect handling, connection pooling |
| ureq | ❌ Rejected | Blocking only, less ergonomic, manual redirect handling |
| hyper | ❌ Rejected | Too low-level, requires significant boilerplate for simple use case |
| async reqwest | ❌ Rejected | Would require async runtime changes throughout the codebase |

---

## Consequences

### Positive

| Aspect | Benefit |
|--------|---------|
| Simplicity | Simpler implementation and control flow |
| Testing | Easier unit testing without async runtime |
| Cognitive Overhead | Reduced mental model complexity |
| Dependencies | Smaller dependency footprint (no tokio) |
| Compatibility | Maintains sync API for existing consumers |
| TLS | Pure Rust TLS via rustls (no OpenSSL) |
| Features | Automatic redirect following, connection pooling, professional User-Agent |

### Negative / Trade-offs

| Aspect | Impact | Mitigation |
|--------|--------|------------|
| High-concurrency scenarios | Less efficient for parallel fetching | Sequential loading is acceptable for taxonomy use case |
| Async ecosystem | Cannot leverage async ecosystem | Not needed for this component |
| Blocking I/O in async contexts | Could cause issues if called from async code | Document clearly; taxonomy loading is startup-phase |
| 30-second timeout | May be insufficient for very slow connections | Can be made configurable in future |
| Cache TTL | No expiration (files never expire) | Acceptable for stable taxonomy releases |

---

## Related

- **Issue #102**: ADR: Taxonomy-loader HTTP client architecture (this ADR)
- **PR #97**: Taxonomy loader implementation with HTTP fetching
- **ADR-007**: Validation pattern for SEC rules
- **taxonomy-loader crate**: `crates/taxonomy-loader/`

---

## Usage Example

```rust
use taxonomy_loader::TaxonomyLoader;
use std::path::PathBuf;

// With caching (enables HTTP support)
let cache_dir = dirs::cache_dir().unwrap_or_else(|| PathBuf::from(".cache")).join("xbrlkit/taxonomy");
let loader = TaxonomyLoader::with_cache_dir(&cache_dir);
let taxonomy = loader.load("https://xbrl.fasb.org/us-gaap/2024/entire/us-gaap-2024.xsd")?;

// Without caching (local files only)
let loader = TaxonomyLoader::new();
let taxonomy = loader.load("/local/path/to/taxonomy.xsd")?;
```

---

## Notes

- The decision is already implemented; this ADR captures the rationale for future reference
- Cache write failures are non-fatal: network fetch succeeds even if disk write fails
- HTTP fetching is opt-in via `with_cache_dir()` to maintain predictable default behavior
