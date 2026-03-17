# Add a scenario

1. Add or update an AC in `specs/spec_ledger.yaml`.
2. Add a `.feature` file or extend an existing one under `specs/features/`.
3. Add the `*.meta.yaml` sidecar with owning crates, fixtures, receipts, and allowed edit roots.
4. Run `cargo xtask feature-grid`.
