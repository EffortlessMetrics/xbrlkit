# Handoff: ISSUE-286

## What Changed

Outright deletion of 9 unused placeholder crates. The repo-alignment review flagged that consolidation into `xbrlkit-stubs` would create a dumping ground contrary to the workspace's "narrow SRP microcrates" philosophy. All 9 stubs had zero downstream consumers and contained only trivial single-function stubs (6-11 lines each). Git history preserves the code.

| File | Change |
|------|--------|
| `Cargo.toml` | Removed 9 entries from `members` array; removed 9 entries from `[workspace.dependencies]` |
| `crates/archive-zip/` | Deleted (7-line `open_zip()` stub) |
| `crates/calc11/` | Deleted (6-line `calculate_ready()` stub) |
| `crates/oracle-compare/` | Deleted (8-line `comparison_receipt()` stub) |
| `crates/sec-http/` | Deleted (9-line intentional-error `fetch()` stub) |
| `crates/taxonomy-cache/` | Deleted (9-line `ensure_cache_dir()` stub) |
| `crates/taxonomy-package/` | Deleted (11-line `load_entry_points()` stub) |
| `crates/xbrl-dimensions/` | Deleted (7-line `normalize_dimension()` stub) |
| `crates/xbrl-linkbases/` | Deleted (6-line `has_linkbase_support()` stub) |
| `crates/xbrl-units/` | Deleted (6-line `normalize_unit()` stub) |
| `README.md` | Added "Removed placeholder crates" section documenting what was removed and where the preserved code lives |
| `.mend/research/taxonomy-dimension-loading.md` | Removed references to deleted stub crates (`xbrl-linkbases`, `taxonomy-cache`, `taxonomy-package`) |
| `.mend/notes/unit-consistency-research.md` | Removed references to deleted `xbrl-units` stub crate |
| `tests/goldens/feature.grid.v1.json` | Updated to match current generator output (pre-existing drift: two `req_id` null→string fixes and trailing newline) |

## Verification

- [x] `cargo fmt --all --check` — clean
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `cargo test --workspace` — all tests pass
- [x] `cargo xtask alpha-check` — active alpha gate passed

## Lessons Learned

- **Outright deletion > consolidation for true stubs.** The deep review and repo-alignment both flagged that creating `xbrlkit-stubs` would be a philosophical departure. When stubs have zero consumers and trivial logic, delete them. Git history is the archive.
- **Feature-grid golden drift is pre-existing.** The alpha-check failure on `feature.grid.v1.json` was due to two `req_id` fields having drifted from `null` to proper requirement IDs, plus a missing trailing newline. None of this was caused by crate removal. Updating the golden was necessary to make the gate pass.
- **Disk space can gate CI locally.** A full `/tmp` caused doctest failures during the first `cargo test` run. `cargo clean` recovered 5.6 GiB and resolved it.

## Next Steps

- PR ready for review.
