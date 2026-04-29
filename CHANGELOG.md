# Changelog

## Unreleased

- Added `cargo xtask package-check` and CI coverage to keep publishable crates packaging cleanly for crates.io.
- Switched internal workspace dependencies to inherited workspace entries so packaged manifests carry registry-compatible versions.
- Marked workspace-only tooling crates, scenario crates, and the workspace-bound CLI as `publish = false`.
- Removed unused `async-tokio` feature from `quick-xml` dependency in `xbrl-stream`, along with the unused `tokio` dev-dependency.

## 0.1.0-alpha.1 - 2026-03-17

Initial public alpha of `xbrlkit`.

What works in this alpha:

- Profile-pack loading from disk for `sec/efm-77/opco`
- `xbrlkit-cli describe-profile --profile ... --json`
- `xbrlkit-cli validate-fixture --profile ... [--json]`
- Active SEC validation slices for `AC-XK-SEC-INLINE-001`, `AC-XK-TAXONOMY-001`, `AC-XK-TAXONOMY-002`, `AC-XK-DUPLICATES-001`, `AC-XK-IXDS-001`, and `AC-XK-IXDS-002`
- Shared scenario execution used by `cargo xtask test-ac`, `cargo xtask bdd --tags @alpha-active`, and `cargo xtask alpha-check`
- Checked-in schemas and goldens for the current alpha outputs

What is still intentionally limited:

- Filing manifest derivation from raw submissions
- Required-facts enforcement beyond the current active slices
- Broad EDGAR ingestion
- Broad XBRL processor coverage
- Oracle and differential lanes
- Wider BDD coverage outside the active alpha slices

Contribution policy for this alpha:

- License: `AGPL-3.0-or-later`
- External contributions require signing the repository CLA
