# Plan: Public `Result` Error Documentation (Issue #329)

## Objective

Document the failure contracts of public functions that return `Result`, so
callers and generated API documentation can see when an operation may fail.
This is a documentation-only campaign: no runtime behavior or error types
change.

## Slices

### Slice 1: independent library APIs

This slice covers the four public functions that do not overlap the active
scenario-runner or xbrl-stream PRs:

- `sec-profile-types::load_profile_from_workspace`
- `xbrl-contexts::parse_contexts`
- `taxonomy-dimensions::DimensionTaxonomy::validate_member`
- `xbrlkit-feature-grid::compile`

Acceptance criteria:

- Each function has a concise `# Errors` section describing its actual error
  conditions.
- No executable code or public error type changes.
- Rustdoc compiles and the affected workspace crates pass their focused checks.

Proof:

- `cargo fmt --all --check`
- focused crate tests and Clippy with `-D warnings`
- workspace tests, doctests, Clippy, and repository fast-loop checks

### Deferred slices

The remaining issue scope is intentionally deferred to avoid overlapping active
work and to keep each PR review-forward:

- `scenario-runner`: 13 public functions, including `execute_scenario` and
  assertion helpers; coordinate with PR #358 before editing this crate.
- `xbrl-stream`: `on_fact`, `on_context`, and `on_unit`; coordinate with PR #355
  before editing this crate.
- `xbrlkit-bdd-steps::run_scenario`; keep with the scenario-runner/BDD step
  documentation slice.

Each deferred slice should preserve the same documentation-only claim and add
its own focused proof before being marked complete.

## Non-goals

- changing function signatures, error enums, or runtime behavior;
- changing the Clippy policy or suppressing `missing_errors_doc` globally;
- editing files owned by active PRs;
- claiming that this plan completes all of issue #329 until the deferred slices
  land.
