# CLAUDE.md

This repo is optimized for scenario-bounded agentic development.

## Source of truth

- `specs/spec_ledger.yaml`
- `specs/features/**/*.feature`
- `specs/features/**/*.meta.yaml`
- `profiles/sec/**`
- `contracts/schemas/**`

## Preferred sequence

1. Read the requirement and AC from `spec_ledger.yaml`.
2. Read the feature and sidecar metadata.
3. Run `cargo xtask bundle <AC|SCN|tag>`.
4. Edit only the allowed roots unless the scenario metadata is changed intentionally.
5. Run `cargo xtask test-ac <AC-ID>`.
6. Emit or inspect receipts under `artifacts/`.
