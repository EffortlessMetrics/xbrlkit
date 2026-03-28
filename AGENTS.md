# Agent guidance

## Autonomous Workflow Labels

The following labels support autonomous workflow management of issues and PRs:

| Label | Color | Purpose | When to Apply |
|-------|-------|---------|---------------|
| `agent/autonomous` | 🔵 Blue | Marks issues/PRs created or managed by autonomous workflow | Auto-applied when an agent creates an issue or PR |
| `agent/in-review` | 🟡 Yellow | Indicates items currently under active review | When an agent is actively reviewing an issue/PR |
| `agent/wip` | 🟠 Orange | Work in progress - not ready for review | When an agent is working on an issue but it's not yet ready |
| `agent/needs-human` | 🔴 Red | Requires human decision before proceeding | When autonomous workflow encounters a blocker requiring human judgment |
| `agent/tech-debt` | 🟣 Purple | Technical debt, legacy code, or maintenance tasks | When technical debt is identified during autonomous workflow |

### Label Usage Guidelines

- **Agent labels** (`agent/*`) are primarily applied by autonomous agents, not humans
- Labels follow a progression: `agent/wip` → `agent/in-review` → `status/ready-to-merge`
- `agent/needs-human` triggers human notification and should be resolved quickly
- `agent/tech-debt` can be applied alongside other workflow labels

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
