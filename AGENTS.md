# Agent guidance

## Working model

Treat the repo as a scenario compiler.

- Work from `specs/spec_ledger.yaml` and `specs/features/**/*.feature`.
- Use sidecar metadata in `*.meta.yaml` to bound edit scope.
- Prefer `cargo xtask bundle <AC|SCN|tag>` before editing.
- Keep changes local to the crates listed in the scenario metadata unless the scenario explicitly widens scope.

## Repo rules

- No live network in normal BDD or focused acceptance runs.
- Profile changes belong under `profiles/` as data, not hard-coded branches.
- New externally-meaningful behaviors need a scenario.
- New DTOs crossing crate boundaries need a schema update under `contracts/schemas/`.
- Output must be deterministic and receipt-backed.

## Fast loop

```bash
cargo xtask doctor
cargo xtask feature-grid
cargo xtask impact --changed <paths...>
cargo xtask test-ac <AC-ID>
```
