# Crate status map

This workspace contains small crates that are at different points on the path
from shared type or adapter boundary to an integrated user-facing flow. This
map records the current reverse-dependency evidence for the crates that are
not yet consumed by another workspace package.

## Current unreferenced surfaces

The following sixteen packages have zero reverse dependencies in the current
workspace graph. Their source is intentionally retained, but each needs an
explicit integration owner and issue before it should be promoted or removed.

| Package | Current source surface | Status and next decision |
| --- | --- | --- |
| `xbrl-units` | `normalize_unit` | Unreferenced normalization helper. Assign a unit-validation integration issue or retire after an explicit decision. |
| `xbrl-dimensions` | `normalize_dimension` | Unreferenced normalization helper. Assign a dimensional integration issue or retire after an explicit decision. |
| `xbrl-linkbases` | `has_linkbase_support` returns `false` | Unreferenced WIP boundary. Define the linkbase integration target before promotion. |
| `calc11` | `calculate_ready` returns `false` | Unreferenced calculation WIP boundary. Define the calculation-validation target before promotion. |
| `archive-zip` | `open_zip` returns an empty success | Unreferenced archive WIP boundary. Define the filing-ingestion target before promotion. |
| `oracle-compare` | `comparison_receipt` emits a warning receipt | Unreferenced oracle-comparison boundary. Define the differential-validation consumer before promotion. |
| `sec-http` | `fetch` rejects live HTTP in the default flow | Unreferenced offline boundary. Define the opt-in SEC transport owner and integration issue before promotion. |
| `taxonomy-cache` | `ensure_cache_dir` | Unreferenced cache helper. Define the taxonomy-loader integration target before promotion. |
| `taxonomy-package` | `load_entry_points` | Unreferenced taxonomy-package helper. Define the package-loading consumer before promotion. |
| `xbrlkit-conform` | `schema_exists` | Unreferenced conformance helper. Define the conformance-runner consumer before promotion. |
| `unit-rules` | `UnitValidator` | Unreferenced unit-validation crate. Confirm its integration target or retire after an explicit decision. |
| `xbrlkit-cli` | `xbrlkit-cli` binary | Unreferenced executable root. It is an entry point, not a dead library; track its downstream launch surface separately. |
| `xbrlkit-core` | facade re-exports | Unreferenced facade root. Confirm the intended application consumer before promotion. |
| `xbrlkit-interop-tests` | receipt-oriented interop test surface | Unreferenced test/support root. Track its harness entry point separately from library consumers. |
| `xbrlkit-test-grid` | scenario-contract test-grid surface | Unreferenced test/support root. Track its harness entry point separately from library consumers. |
| `xtask` | repository control-plane binary | Unreferenced executable root. It is invoked directly by repository commands, not through a workspace library dependency. |

“Unreferenced” describes the current workspace dependency graph. It is not an
approval to delete a package, and it does not claim that the package has no
external consumers.

## Reconciled graph fact

`diff-run` appeared in the original audit list for issue #339, but it is now
consumed by `xbrlkit-core` and is therefore excluded from the
unreferenced list. Re-run the graph audit when package wiring changes.

## Audit basis

The map was reconciled from the current `Cargo.toml` manifests and the
workspace package graph produced by:

```text
cargo metadata --format-version 1 --locked --offline
```

The graph result is a point-in-time maintenance signal. Future integration or
retirement work should update this map and link the governing issue or plan.
