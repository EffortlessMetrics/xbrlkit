# Plan: Taxonomy Loader BDD Scenarios (Issue #100)

## Overview

This plan addresses Issue #100 which tracks the implementation of 8 comprehensive BDD (Behavior-Driven Development) scenarios for the Taxonomy Loader component in PR #97. The Taxonomy Loader is responsible for loading XBRL dimension taxonomies from XSD schema files and definition linkbases, enabling validation of facts against real US-GAAP and SEC taxonomies.

The scenarios cover taxonomy loading from various sources, error handling, cache management, and integration with the validation system.

## Acceptance Criteria Breakdown

### AC-XK-TAX-LOAD-001: Load Dimension Definitions from Schema
- **Given**: A taxonomy schema with dimension elements
- **When**: The taxonomy is loaded
- **Then**: The taxonomy should contain dimensions with explicit dimensions having domains

### AC-XK-TAX-LOAD-002: Load Domain Hierarchies from Definition Linkbase
- **Given**: A taxonomy definition linkbase with domain members
- **When**: The taxonomy is loaded
- **Then**: Domains should have members maintaining parent-child relationships

### AC-XK-TAX-LOAD-003: Load Typed Dimension Definitions
- **Given**: A taxonomy with typed dimensions
- **When**: The taxonomy is loaded
- **Then**: Typed dimensions should have valid XSD value types

### AC-XK-TAX-LOAD-004: Load Hypercube Definitions
- **Given**: A taxonomy with hypercube elements
- **When**: The taxonomy is loaded
- **Then**: Hypercubes should contain their dimensions referencing their domains

### AC-XK-TAX-LOAD-005: Cache Taxonomy Files Locally
- **Given**: A taxonomy URL to load and a cache directory is configured
- **When**: The taxonomy is loaded
- **Then**: The taxonomy file should be cached and subsequent loads should use the cache

### AC-XK-TAX-LOAD-006: Handle Schema Imports Recursively
- **Given**: A taxonomy schema that imports another schema
- **When**: The taxonomy is loaded
- **Then**: Imported schemas should be loaded with all dimension definitions available

### AC-XK-TAX-LOAD-007: Validate Dimension-Member Against Loaded Taxonomy
- **Given**: A loaded taxonomy with dimension definitions
- **When**: Validating a dimension-member pair against the taxonomy
- **Then**: The validation should pass for valid combinations

### AC-XK-TAX-LOAD-008: Reject Invalid Member Against Loaded Taxonomy
- **Given**: A loaded taxonomy with dimension definitions
- **When**: Validating an invalid dimension-member pair
- **Then**: The validation should fail with an appropriate error finding

## Proposed Approach

### Architecture
The implementation follows the existing BDD infrastructure pattern in xbrlkit:

1. **Feature Files**: Cucumber/Gherkin syntax defining scenarios in `specs/features/taxonomy/`
2. **Step Definitions**: Rust implementations in `crates/xbrlkit-bdd-steps/src/lib.rs`
3. **Component Library**: The actual `taxonomy-loader` crate providing the loading functionality
4. **Meta Sidecars**: YAML metadata files for scenario tracking and grid compilation

### Key Design Decisions

1. **Synchronous API**: The taxonomy loader uses a blocking reqwest client to avoid tokio runtime dependencies in the core library, making it easier to use in synchronous contexts.

2. **Caching Strategy**: Local file caching uses URL-to-filepath transformation (replacing `/`, `:`, `?`, `&`, `=` with `_`) for cache key generation.

3. **Recursive Loading**: Schema imports are handled recursively with circular import detection via a `visited` HashSet.

4. **Error Handling**: Custom `TaxonomyLoaderError` enum covers I/O, HTTP, parsing, and URL scheme errors.

## Files to Modify/Create

### New Files (2)
1. `specs/features/taxonomy/taxonomy_loader.feature` - 8 BDD scenarios in Gherkin syntax
2. `specs/features/taxonomy/taxonomy_loader.meta.yaml` - Scenario metadata sidecar

### Modified Files (5)
1. `crates/taxonomy-loader/src/lib.rs` - Convert to blocking reqwest (remove tokio runtime dependency)
2. `crates/taxonomy-loader/Cargo.toml` - Update dependencies
3. `crates/xbrlkit-bdd-steps/src/lib.rs` - Add taxonomy loader step handlers (~279 lines)
4. `crates/xbrlkit-bdd-steps/Cargo.toml` - Add taxonomy-loader dependency
5. `tests/goldens/feature.grid.v1.json` - Update golden file with new scenarios

### Dependencies
- `Cargo.lock` - Updated to reflect dependency changes

## Test Strategy

### Unit Tests (in taxonomy-loader)
- `test_loader_new()` - Basic loader construction
- `test_loader_with_cache()` - Cache directory configuration
- `test_url_to_cache_path()` - Cache path generation
- `test_fetch_url_success()` - HTTP fetching with mocking
- `test_fetch_url_uses_cache()` - Cache hit behavior
- `test_fetch_url_http_error()` - HTTP error handling
- `test_fetch_url_invalid_scheme()` - URL scheme validation

### Integration Tests (BDD Scenarios)
All 8 scenarios tagged with `@alpha-active` for inclusion in the alpha readiness gate:
- `cargo xtask alpha-check` must pass
- Total scenarios: 52 (including the 8 new ones)

### Test Data
- WireMock for HTTP mocking in unit tests
- Synthetic taxonomy fixtures for BDD scenarios
- Existing fixture infrastructure in `fixtures/` directory

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| HTTP client blocking changes affect existing async code | Medium | Medium | Unit tests verify blocking behavior; tokio::task::block_in_place used where needed |
| Cache invalidation not implemented | Low | Low | Documented limitation; cache can be cleared manually |
| Circular imports in taxonomies | Medium | High | HashSet-based visited tracking prevents infinite loops |
| Large taxonomy files cause memory issues | Low | Medium | Streaming not yet implemented; monitor with large SEC taxonomies |
| Network timeouts in CI | Medium | Low | Configurable 30s timeout; mock server for tests |

## Estimated Effort

| Task | Estimate |
|------|----------|
| Create feature file with 8 scenarios | 2h |
| Create meta.yaml sidecar | 1h |
| Implement BDD step handlers | 4h |
| Convert taxonomy-loader to blocking reqwest | 2h |
| Update dependencies and Cargo.toml files | 1h |
| Unit tests for loader | 3h |
| Update feature grid golden file | 1h |
| Integration testing and debugging | 3h |
| **Total** | **~17 hours** |

## Implementation Notes

### PR #97 Details
- Branch: `feat/taxonomy-loader-scenarios`
- 8 scenarios passing: SCN-XK-TAX-LOAD-001 through SCN-XK-TAX-LOAD-008
- All scenarios tagged with `@alpha-active` and `@speed.fast`
- Part of the autonomous workflow initiative

### Next Steps After Plan Approval
1. Review the plan with stakeholders
2. Merge PR #97 after final review
3. Run `cargo xtask alpha-check` to verify all scenarios pass
4. Update documentation if needed

---
*Plan created by planner-initial agent*
