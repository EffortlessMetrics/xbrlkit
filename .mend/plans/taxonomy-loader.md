# Plan: Taxonomy Dimension Loading from Actual XBRL Files

**Stream:** D: Taxonomy Core  
**Issue:** #35  
**Date:** 2026-03-23  
**Status:** 🔨 Ready for Build

---

## Decision: Create New `taxonomy-loader` Crate

After reviewing the codebase structure, I recommend creating a new `taxonomy-loader` crate rather than extending existing stubs. This provides clear separation of concerns:

| Crate | Responsibility |
|-------|---------------|
| `taxonomy-dimensions` | Type definitions (already exists ✅) |
| `taxonomy-loader` | **Orchestrate loading from files** (NEW) |
| `taxonomy-cache` | ~~Local taxonomy package storage (extend later)~~ — **removed in #286** |
| `xbrl-linkbases` | ~~Linkbase type definitions (extend later)~~ — **removed in #286** |

---

## Implementation Plan

### Phase 1: Crate Skeleton
```
crates/taxonomy-loader/
├── Cargo.toml
├── src/
│   ├── lib.rs          # Public API: load_taxonomy(entrypoint)
│   ├── schema.rs       # XSD parsing module
│   ├── linkbase.rs     # Definition linkbase parsing
│   └── error.rs        # TaxonomyLoaderError enum
```

**Dependencies:**
- `roxmltree` for XML parsing (check if already in tree)
- `taxonomy-dimensions` for types
- `thiserror` for error handling

### Phase 2: Schema Parsing
Parse `.xsd` files to identify:
- Elements with `xbrldt:hypercubeItem` → Hypercube
- Elements with `xbrldt:dimensionItem` → Dimension
- Elements with `xbrli:domainItemType` → Domain member

**Key XML patterns to handle:**
```xml
<xsd:element
    id="us-gaap_StatementTable"
    name="StatementTable"
    substitutionGroup="xbrldt:hypercubeItem"
    type="xbrli:stringItemType"
    xbrli:periodType="duration"/>
```

### Phase 3: Linkbase Parsing
Parse `_def.xml` definition linkbases for arc relationships:
- `hypercube-dimension` → Hypercube → Dimension
- `dimension-domain` → Dimension → Domain
- `domain-member` → Parent → Child member
- `all`/`notAll` → Closed vs open hypercubes

**Arc role constants:**
- `http://xbrl.org/int/dim/arcrole/hypercube-dimension`
- `http://xbrl.org/int/dim/arcrole/dimension-domain`
- `http://xbrl.org/int/dim/arcrole/domain-member`

### Phase 4: Integration
- Add to `validation-run` crate
- Wire into CLI as `xbrlkit inspect-taxonomy <entrypoint>`
- Integration test with real SEC taxonomy entry point

---

## API Sketch

```rust
// lib.rs
pub struct TaxonomyLoader {
    cache_dir: Option<PathBuf>,
}

impl TaxonomyLoader {
    pub fn new() -> Self;
    pub fn with_cache_dir(path: impl Into<PathBuf>) -> Self;
    
    pub fn load(&self, entrypoint: &str) -> Result<DimensionTaxonomy, TaxonomyLoaderError>;
}

// Convenience function
pub fn load_taxonomy(entrypoint: &str) -> Result<DimensionTaxonomy, TaxonomyLoaderError>;
```

---

## Testing Strategy

| Test Type | Coverage |
|-----------|----------|
| Unit | Schema/linkbase parsers in isolation |
| Integration | Real SEC taxonomy entry point (DEI or US-GAAP) |
| Golden | Serialized `DimensionTaxonomy` snapshot comparison |

**Test fixtures:**
- Minimal synthetic XSD with dimension elements
- Minimal synthetic definition linkbase
- Real SEC entry point (cached)

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| XBRL spec edge cases | Start with common patterns, iterate based on real taxonomies |
| Circular schema imports | Track visited URLs with HashSet |
| XML namespace complexity | Use roxmltree's namespace resolution |
| Performance with large taxonomies | Profile before optimizing; lazy loading as v2 |

---

## Acceptance Criteria

- [ ] `taxonomy-loader` crate created with clean API
- [ ] Can parse minimal synthetic XSD for dimension elements
- [ ] Can parse minimal synthetic definition linkbase for arcs
- [ ] Can build `DimensionTaxonomy` from parsed files
- [ ] CLI command `xbrlkit inspect-taxonomy <entrypoint>` outputs taxonomy structure
- [ ] Integration test with real SEC taxonomy passes
- [ ] All quality gates pass (fmt, clippy, test, alpha-check)

---

## Branch Strategy

```
mend/issue-35-taxonomy-loader
├── Commit 1: Crate skeleton + dependencies
├── Commit 2: Schema parsing implementation
├── Commit 3: Linkbase parsing implementation
├── Commit 4: Integration + CLI command
└── Commit 5: Tests + documentation
```

---

## Notes

- Research document: `.mend/research/taxonomy-dimension-loading.md`
- This plan is ready for build phase — will create PR once implemented
