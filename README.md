# xbrlkit

[![CI](https://github.com/EffortlessMetrics/xbrlkit/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/EffortlessMetrics/xbrlkit/actions/workflows/ci.yml)
[![Coverage](https://github.com/EffortlessMetrics/xbrlkit/actions/workflows/coverage.yml/badge.svg?branch=main)](https://github.com/EffortlessMetrics/xbrlkit/actions/workflows/coverage.yml)
[![Codecov](https://codecov.io/gh/EffortlessMetrics/xbrlkit/branch/main/graph/badge.svg)](https://codecov.io/gh/EffortlessMetrics/xbrlkit)
[![MSRV](https://img.shields.io/badge/MSRV-1.92-blue.svg)](Cargo.toml)
[![License](https://img.shields.io/badge/license-AGPL--3.0--or--later-blue.svg)](LICENSE)

Alpha Rust workspace for XBRL / iXBRL + EDGAR processing, aimed first at SEC operating-company filings.

Codecov is execution-surface telemetry only; see [Coverage](docs/ci/coverage.md) for what the badge does and does not claim.

## What this workspace is for

This workspace is designed as a governed foundation for a validating processor aimed first at SEC operating-company filings. The current design is built around:

- scenario-first development with Gherkin + sidecar metadata
- narrow SRP microcrates
- one canonical internal report model
- versioned SEC profile packs as data
- deterministic receipts at every stage
- boring local commands and JSON outputs that outer automation can consume

## Workspace shape

- `contracts/` – JSON schemas and contract docs
- `specs/` – spec ledger, Gherkin features, rule registry, devex flows
- `profiles/` – versioned SEC profile packs
- `fixtures/` – tiny synthetic inputs and goldens
- `corpus/` – imported suites and pinned real filings
- `crates/` – engine crates, semantic leaves, use-case crates, adapters, facade, CLI
- `xtask/` – repo-local developer automation for feature-grid, bundle, impact, schema-check, and focused AC runs

## Supported surface

Working today:

- profile-pack loading from disk for `profiles/sec/efm-77/opco`
- `cargo run -p xbrlkit-cli -- describe-profile --profile sec/efm-77/opco --json`
- `cargo run -p xbrlkit-cli -- validate-fixture --profile sec/efm-77/opco [--json] <files...>`
- `cargo xtask doctor`
- `cargo xtask feature-grid`
- `cargo xtask impact --changed <paths...>`
- `cargo xtask schema-check`
- `cargo xtask bdd --tags @alpha-active`
- `cargo xtask alpha-check`
- `cargo xtask test-ac` for:
  - `AC-XK-SEC-INLINE-001`
  - `AC-XK-TAXONOMY-001`
  - `AC-XK-TAXONOMY-002`
  - `AC-XK-DUPLICATES-001`
  - `AC-XK-IXDS-001`
  - `AC-XK-IXDS-002`

Still minimal or not implemented yet:

- filing manifest derivation from raw submissions
- required-facts enforcement from the SEC profile pack
- mature BDD execution; `cargo xtask test-ac` is the authoritative scenario runner for active slices
- oracle comparison and broad differential lanes
- broad EDGAR / XBRL coverage beyond the active synthetic SEC slices

## Quick start

```bash
cargo xtask doctor
cargo xtask feature-grid
cargo xtask schema-check
cargo xtask bdd --tags @alpha-active
cargo xtask impact --changed crates/ixds-assemble/src/lib.rs
cargo xtask test-ac AC-XK-IXDS-002
cargo xtask alpha-check
cargo run -p xbrlkit-cli -- describe-profile --profile sec/efm-77/opco --json
cargo run -p xbrlkit-cli -- validate-fixture --profile sec/efm-77/opco --json fixtures/synthetic/inline/ixds-two-file-01/member-a.html fixtures/synthetic/inline/ixds-two-file-01/member-b.html
```

## License and contributions

`xbrlkit` is licensed under `AGPL-3.0-or-later`.

External contributions require signing the repository [CLA](./CLA.md). `xbrlkit`'s public alpha contribution policy is `AGPL-3.0-or-later` plus CLA, and accepted contributions remain under the same license.

Please do not use public issues for security-sensitive reports. See [SECURITY.md](./SECURITY.md).

## Next build milestones

1. Keep the active alpha gate boring and stable as more slices land.
2. Build the filing-manifest path from real fixture inputs instead of synthesized placeholders.
3. Add the next real SEC slice on top of the same profile-pack and fixture-first loop.
4. Broaden BDD coverage beyond the active alpha scenarios only after the next real slice exists.
