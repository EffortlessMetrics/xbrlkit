# Taxonomy HTTP Fetching - Research Document

## Current State Analysis

### taxonomy-loader Structure
- **Location:** `crates/taxonomy-loader/`
- **Current capability:** Only local file loading via `fetch_file()`
- **TODO:** `fetch_url()` method exists but returns `UnsupportedUrl` error
- **Cache support:** Partial - `cache_dir` exists but HTTP fetching not implemented

### Key Code Locations
1. **Main loader:** `src/lib.rs` - `TaxonomyLoader` struct with `fetch_url()` TODO
2. **Error types:** `src/error.rs` - Already has `UnsupportedUrl` variant
3. **Cache path:** `url_to_cache_path()` method exists for URL-to-file mapping

## HTTP Client Decision: reqwest

### Why reqwest?
| Feature | reqwest | ureq | hyper |
|---------|---------|------|-------|
| Async support | ✅ Native | ❌ Blocking only | ✅ Low-level |
| Redirect following | ✅ Automatic | ⚠️ Manual | ⚠️ Manual |
| Connection pooling | ✅ Built-in | ✅ Yes | ⚠️ Manual |
| Timeout handling | ✅ Built-in | ✅ Yes | ⚠️ Manual |
| Error ergonomics | ✅ Excellent | ✅ Good | ⚠️ Verbose |
| XBRL use case fit | ✅ Best | ⚠️ Okay | ❌ Too low-level |

### Additional benefits:
- Already supports `rustls-tls` (pure Rust, no OpenSSL dependency issues)
- Well-maintained (1M+ downloads/month)
- Used by major projects (AWS SDK, etc.)

## Caching Strategy

### Cache Location
```
~/.cache/xbrlkit/taxonomy/
├── xbrl_fasb_org_us-gaap_2024_entire_us-gaap-2024.xsd
├── xbrl_sec_gov...
└── ...
```

### Cache Logic
1. Check cache first (if `cache_dir` configured)
2. If cache hit → return cached content
3. If cache miss → fetch via HTTP
4. On successful fetch → write to cache, then return content

### URL-to-Path Mapping
- Use existing `url_to_cache_path()` which replaces `/ : ? & =` with `_`
- Use `std::fs::create_dir_all` for directory creation

## Error Handling

### New Error Variants Added (in `TaxonomyLoaderError`):
```rust
#[error("HTTP request failed for {0}: {1}")]
HttpError(String, String),

#[error("URL parsing error: {0}")]
UrlParse(#[from] url::ParseError),
```

### HTTP Error Cases Handled:
- Network failures (timeout, DNS, connection refused)
- HTTP error status codes (4xx, 5xx)
- Redirect loops (handled by reqwest)
- Invalid content/encoding
- Cache write failures (non-fatal, log warning)

## Implementation Summary

### Dependencies Added
```toml
[dependencies]
reqwest = { version = "0.12", features = ["rustls-tls"], default-features = false }
url = "2.5"
tokio = { workspace = true }

[dev-dependencies]
wiremock = "0.6"
tempfile = "3"
```

### Key Implementation Details

1. **HTTP Client**: Built with 30-second timeout and custom User-Agent header
2. **Sync API Compatibility**: Used `tokio::task::block_in_place` with `block_on` to bridge async HTTP in sync API
3. **Caching**: Cache directory auto-created on write; cache hits bypass HTTP entirely
4. **URL Validation**: Only http:// and https:// schemes are allowed

### Test Coverage
- ✅ HTTP fetch success
- ✅ Cache hit (bypasses HTTP)
- ✅ HTTP error handling (404)
- ✅ Invalid URL scheme rejection
- ✅ URL-to-cache-path conversion

### Quality Gates Passed
- ✅ `cargo check` - workspace compiles
- ✅ `cargo test` - all 12 tests pass
- ✅ `cargo clippy` - clean (only MSRV warning unrelated to changes)
- ✅ `cargo fmt` - formatted

## Files Modified
1. `crates/taxonomy-loader/Cargo.toml` - Added dependencies
2. `crates/taxonomy-loader/src/error.rs` - Added `HttpError` and `UrlParse` variants
3. `crates/taxonomy-loader/src/lib.rs` - Implemented `fetch_url()` with caching

## Usage Example

```rust
use taxonomy_loader::TaxonomyLoader;

// With caching
let loader = TaxonomyLoader::with_cache_dir("~/.cache/xbrlkit/taxonomy");
let taxonomy = loader.load("https://xbrl.fasb.org/us-gaap/2024/entire/us-gaap-2024.xsd")?;

// Without caching (no HTTP support without cache_dir)
let loader = TaxonomyLoader::new();
let taxonomy = loader.load("/local/path/to/taxonomy.xsd")?;
```

## Future Improvements
- Add cache TTL (time-based expiration)
- Add cache size limits
- Support conditional GET with ETag/If-Modified-Since
- Add progress callbacks for large taxonomy downloads
