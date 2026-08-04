# Plan: Taxonomy Dimension Loading from Actual XBRL Files

**Stream:** D: Taxonomy Core  
**Original issue:** #35
**Implementation:** PR #37 (`5a577da681b9c3207843c56d1c0cee3c22e21c2d`)
**Status:** ✅ Implemented; historical plan reconciled

---

## Reconciled outcome

The implementation planned here landed in PR #37, which closed issue #35.
The repository now contains the `xbrlkit-taxonomy-loader` crate and the CLI
`inspect-taxonomy` command. This document records the delivered contract and
keeps the remaining test gaps explicit; it is no longer a build plan.

## Delivered scope

| Planned capability | Current evidence |
| --- | --- |
| Public loader API | `crates/taxonomy-loader/src/lib.rs`: `load_taxonomy`, `TaxonomyLoader::new`, and `with_cache_dir` |
| Schema parsing | `crates/taxonomy-loader/src/schema.rs`: hypercube, dimension, domain, import, and include handling |
| Definition linkbase parsing | `crates/taxonomy-loader/src/linkbase.rs`: dimension arcs, domain-member arcs, and hypercube associations |
| Taxonomy construction | `taxonomy_dimensions::DimensionTaxonomy` is populated by the schema and linkbase parsers |
| CLI integration | `crates/xbrlkit-cli/src/main.rs`: `Command::InspectTaxonomy` |
| Focused proof | Unit tests in `taxonomy-loader/src/{lib,schema,linkbase}.rs` cover synthetic schemas, linkbases, URL validation, and cache behavior |
| Acceptance scenarios | `specs/features/taxonomy/taxonomy_loader.feature` covers loader availability, schemas, linkbases, caching, imports, and validation integration |

## Acceptance ledger

- [x] The `taxonomy-loader` crate exists with a stable internal loading seam.
- [x] Minimal synthetic XSD input is parsed for dimension taxonomy elements.
- [x] Minimal definition linkbase input is parsed for supported arcs.
- [x] Parsed inputs build a `DimensionTaxonomy`.
- [x] `xbrlkit inspect-taxonomy <entrypoint>` is wired to the loader.
- [ ] A live SEC taxonomy integration test is not part of the current local
  proof surface. External-network coverage is intentionally deferred until a
  deterministic transport seam is available; issue #211 tracks that work.
- [x] The implementation was delivered with the quality-gate evidence recorded
  by PR #37. Future implementation changes must rerun the repository's current
  gates rather than treating this historical plan as a fresh receipt.

## Current contract and boundaries

- Local paths are loaded without network access.
- HTTP and HTTPS loading is available through the blocking `reqwest` client;
  `with_cache_dir` enables the process's file cache.
- Schema imports/includes and linkbase references are followed recursively with
  visited-path protection.
- The plan does not promise coverage of every XBRL taxonomy edge case or a
  live-network test in normal BDD/focused runs.
- No public API, receipt, or schema change is required to reconcile this plan.

## Follow-ups

- Issue #211: add a deterministic HTTP transport seam and fixture-backed tests
  for remote success/failure plus cache hit/miss behavior.
- Keep later taxonomy-loader refactors and dependency changes in their own
  issue/PR slices; do not reopen this historical implementation plan for them.

## Rollback

This is source-truth documentation only. Revert the documentation commit if
the historical status needs correction; implementation rollback belongs to the
separate PRs that changed the loader.
