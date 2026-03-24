# Research: Taxonomy Dimension Loading from Actual XBRL Files

**Stream:** D: Taxonomy Core  
**Research Date:** 2026-03-23  
**Status:** 🔍 Research Complete → Ready for Plan

---

## Current State

### What's Built (✅)
| Component | State | Notes |
|-----------|-------|-------|
| `taxonomy-dimensions` crate | Types defined | `Dimension`, `Domain`, `DomainMember`, `Hypercube`, `DimensionTaxonomy` |
| `dimensional-rules` crate | Validation logic | Validates dimension-member pairs |
| BDD scenarios SCN-XK-DIM-001 to 004 | Passing | Uses synthetic/test data, not real taxonomies |
| `xbrl-linkbases` crate | Stub | `has_linkbase_support()` returns `false` |
| `taxonomy-cache` crate | Stub | Directory creation only |
| `taxonomy-package` crate | Stub | Returns empty `DtsDescriptor` |

### The Gap 🎯
**No actual XBRL taxonomy file parsing exists.**

Dimensions are currently:
- Hardcoded in BDD step handlers (`xbrlkit-bdd-steps/src/lib.rs`)
- Created manually in unit tests
- NOT loaded from:
  - `.xsd` schema files
  - `_def.xml` definition linkbases
  - Taxonomy packages (ZIP)

---

## What Needs to Be Built

### 1. XBRL Schema Parser (`xbrl-schema` crate?)
Parse `.xsd` files to extract:
- `<xsd:element>` with `xbrli:itemType="xbrli:domainItemType"` → Domain members
- `<xsd:element>` with `substitutionGroup="xbrldt:hypercubeItem"` → Hypercubes
- `<xsd:element>` with `substitutionGroup="xbrldt:dimensionItem"` → Dimensions

### 2. Definition Linkbase Parser (extend `xbrl-linkbases`)
Parse `_def.xml` files to extract arc relationships:
- `hypercube-dimension` → Which dimensions belong to which hypercube
- `dimension-domain` → Which domain provides members for a dimension
- `domain-member` → Hierarchical member relationships
- `all` / `notAll` → Closed vs open hypercubes

### 3. Taxonomy Loader (`taxonomy-loader` crate?)
Orchestrate loading:
- Download/resolve taxonomy packages
- Parse entry point → discover all linked schemas
- Parse definition linkbases
- Build `DimensionTaxonomy` from parsed data

### 4. Integration Points
| From | To | Purpose |
|------|-----|---------|
| `taxonomy-loader` | `taxonomy-dimensions` | Populate `DimensionTaxonomy` struct |
| `validation-run` | `taxonomy-loader` | Load taxonomy before validation |
| CLI | `taxonomy-loader` | `xbrlkit inspect-taxonomy <entrypoint>` |

---

## XBRL Dimensional Architecture (Reference)

```
Entry Point (xsd)
    │
    ├── Concept Schema (elements, types)
    │
    └── Definition Linkbase (_def.xml)
            │
            ├── Hypercube (us-gaap:StatementTable)
            │       │
            │       ├── hypercube-dimension arc → StatementScenarioAxis
            │       └── hypercube-dimension arc → StatementPeriodAxis
            │
            ├── Dimension (us-gaap:StatementScenarioAxis)
            │       │
            │       └── dimension-domain arc → StatementScenarioDomain
            │
            └── Domain (us-gaap:StatementScenarioDomain)
                    │
                    ├── domain-member arc → ScenarioActualMember (root)
                    ├── domain-member arc → ScenarioBudgetMember (root)
                    └── domain-member arc → RestatedMember (child of Actual)
```

---

## Technical Decisions Needed

### Decision 1: XML Parser Choice
| Option | Pros | Cons |
|--------|------|------|
| `roxmltree` | Fast, zero-copy, already used? | Need to verify |
| `quick-xml` | Fast, streaming, mature | More verbose API |
| `xml-rs` | Standard | Slower |

**Recommendation:** Check what's already in the dependency tree.

### Decision 2: Linkbase Arc Role Model
Arc roles are URLs. Options:
1. String matching (simple, fragile)
2. Enum with `FromStr` (type-safe, needs maintenance)
3. URI normalization + registry lookup (correct, complex)

**Recommendation:** Start with enum, add registry later.

### Decision 3: Loading Strategy
| Option | Latency | Complexity | Use Case |
|--------|---------|------------|----------|
| Lazy (on-demand) | Lower | Higher | Interactive CLI |
| Eager (all upfront) | Higher startup | Lower | Batch validation |

**Recommendation:** Eager for v1, add lazy as optimization.

---

## Proposed Crate Structure

```
crates/
├── taxonomy-dimensions       # ✅ Types (Dimension, Domain, Hypercube)
├── taxonomy-loader           # 🆕 NEW: Orchestrate loading
│   ├── schema/               # XSD parsing
│   └── linkbase/             # Definition linkbase parsing
├── xbrl-linkbases            # 📝 Extend: Currently stub
└── taxonomy-cache            # 📝 Extend: Currently stub
```

**Alternative:** Extend existing crates instead of new ones.
- Extend `xbrl-linkbases` → add definition linkbase parsing
- Extend `taxonomy-package` → add schema resolution
- New `taxonomy-loader` → orchestration layer

---

## Acceptance Criteria (for Plan phase)

1. Can parse a minimal XSD with dimension elements
2. Can parse a minimal definition linkbase with arcs
3. Can build a `DimensionTaxonomy` from parsed files
4. CLI command `xbrlkit inspect-taxonomy <entrypoint>` works
5. Integration test with real SEC taxonomy entry point passes

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| XBRL spec edge cases | High | Medium | Start with common cases, iterate |
| SEC taxonomy complexity | High | Medium | Test against real filings |
| XML parsing performance | Medium | Low | Profile before optimizing |
| Circular imports in schemas | Medium | Medium | Track visited URLs |

---

## Next Steps

1. **Create plan issue** with crate structure decision
2. **Spike:** Parse minimal SEC taxonomy entry point manually
3. **Build:** `taxonomy-loader` crate skeleton
4. **Integrate:** Wire into `validation-run`

---

## References

- XBRL Dimensions 1.0 Specification: https://specifications.xbrl.org/work-product-index-dimensions-dimensions-1.0.html
- SEC EFM Taxonomy structure: https://www.sec.gov/info/edgar/edgartaxonomies
- Current types: `crates/taxonomy-dimensions/src/lib.rs`
