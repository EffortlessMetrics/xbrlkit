## Summary
23 crates have placeholder implementations with `src/lib.rs` < 20 lines.

## Placeholder Crates

| Crate | Lines | Implementation |
|-------|-------|----------------|
| calc11 | 6 | `calculate_ready() -> false` |
| xbrl-dimensions | 6 | `normalize_dimension() -> lowercase` |
| xbrl-linkbases | 6 | `has_linkbase_support() -> false` |
| xbrl-units | 6 | (placeholder) |
| render-json | 6 | (placeholder) |
| archive-zip | 7 | (placeholder) |
| corpus-fs | 8 | (placeholder) |
| oracle-compare | 8 | (placeholder) |
| xbrlkit-conform | 8 | (placeholder) |
| xbrlkit-interop-tests | 8 | (placeholder) |
| xbrlkit-test-grid | 8 | (placeholder) |
| sec-http | 9 | (placeholder) |
| taxonomy-cache | 9 | (placeholder) |
| xbrlkit-core | 9 | (placeholder) |
| edgar-identity | 10 | (placeholder) |
| render-md | 11 | (placeholder) |
| taxonomy-package | 11 | (placeholder) |
| export-run | 13 | (placeholder) |
| oim-normalize | 13 | (placeholder) |
| cockpit-export | 15 | (placeholder) |
| edgar-sgml | 17 | (placeholder) |
| taxonomy-types | 17 | (placeholder) |
| filing-load | 19 | (placeholder) |

## Recommendations

1. **Document intent**: Add README.md to each explaining intended purpose
2. **Prioritize**: Identify which crates are needed for alpha release
3. **Consider removal**: Remove from workspace if not needed near-term

---
*Discovered by Scout Agent on 2026-04-01*
