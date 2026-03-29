# Plan: ADR-008 Taxonomy-Loader HTTP Client Architecture

## Issue
#102 - ADR: Taxonomy-loader HTTP client architecture

## Status
✅ COMPLETED

## Summary
Create Architecture Decision Record (ADR) documenting the HTTP client architecture
selection for the taxonomy-loader component.

## Decision
Use **reqwest** with the **blocking** client API for HTTP fetching in the taxonomy-loader
crate. Explicitly avoid async/tokio for this component.

## Rationale
- Taxonomy loading is typically sequential (schemas depend on each other)
- Async overhead not justified for startup-phase loading
- Blocking code is easier to reason about and test
- Reduced dependency tree (no tokio runtime required)
- Existing API compatibility (consumers expect sync APIs)

## Implementation

### Selected Dependencies
```toml
[dependencies]
reqwest = { version = "0.12", features = ["rustls-tls", "blocking"], default-features = false }
url = "2.5"
```

### Key Features
- HTTP/HTTPS fetching with timeout (30s default)
- Disk caching for downloaded taxonomy files
- Pure Rust TLS via rustls (no OpenSSL)
- Automatic redirect handling and connection pooling
- Professional User-Agent header

### Pattern
- `TaxonomyLoader::with_cache_dir()` - enables HTTP fetching + disk caching
- `TaxonomyLoader::new()` - local files only (no HTTP)

### Cache Strategy
1. Check cache first (if configured)
2. Cache hit → return cached content
3. Cache miss → fetch via HTTP
4. On successful fetch → write to cache (non-fatal on failure)

## Error Handling
- Network timeouts (configurable)
- HTTP error status codes
- Cache write failures (non-fatal)
- Invalid URLs

## Deliverables
- [x] ADR-008 document at `adr/ADR-008-taxonomy-loader-http-client.md`
- [x] PR #107 opened with `ready-for-review` label
- [x] All agent reviews passed (quality, tests, arch, integ)

## Related
- PR #97: Taxonomy loader BDD scenarios
- taxonomy-loader crate: `crates/taxonomy-loader/`
