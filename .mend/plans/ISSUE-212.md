# Issue #212: derive alpha-check selection from the feature grid

## Status

Plan prepared against `origin/main` at `e4d941c11ceb753866548a6a6b920917959dc456`.
Implementation is intentionally queued behind the contract work in PR #380 and the declared execution-mode work in PR #389.

## Problem and current evidence

`xtask/src/alpha_check.rs` owns a hardcoded `ACTIVE_ALPHA_ACS` list. That list is a second source of truth for scenario readiness and can drift from feature and sidecar metadata.

The original proposal cannot be implemented safely by filtering only on `@alpha-active` yet:

- `origin/main` `ScenarioRecord` has no scenario-tag field.
- PR #380 (`bda46dab5a3a83472fac7e185b5e3378323e97d0`) adds tags to the feature-grid contract and compiler, but explicitly leaves alpha-check selection unchanged. Its current head also includes the repaired malformed mixed-tag rejection; the PR remains open and must land before this plan can consume the contract as a base.
- PR #389 (`260531329da100d9cfa72d6a5b0523e74491f412`) adds declared `test_type` and `test_tag` data used to distinguish BDD and scenario-runner dispatch, but it is not merged into `main`.
- The existing list includes ACs whose execution is not equivalent to the global `@alpha-active` BDD run. Deriving every tagged AC and invoking `test-ac` would either duplicate BDD execution or drop an existing direct check.

## Target seam

After the prerequisite contracts land, replace the constant and its loop in `xtask/src/alpha_check.rs` with a private, deterministic selection function over the loaded `FeatureGrid`.

The selection result should make the two alpha-check tracks explicit:

1. `direct_ac_ids`: sorted, deduplicated AC IDs that are declared for direct/scenario-runner execution.
2. `bdd_tag`: the single `@alpha-active` BDD gate, which executes tagged BDD scenarios once.
3. `excluded_or_ambiguous`: records that cannot be safely classified, including missing AC IDs, missing declared mode/tag, unsupported modes, or legacy IDs not represented by the new metadata.

The implementation must fail closed on an ambiguous record. It must not silently infer direct execution from a lifecycle tag alone.

### Required merged metadata contract

The implementation must consume the merged contracts from #380 and #389 as two
separate inputs:

- #380 supplies normalized scenario lifecycle tags, including the exact
  `@alpha-active` membership used by the single BDD gate.
- #389 supplies the declared execution `test_type` and exact `test_tag` used
  to identify the test declaration for a scenario.

`test_tag` is not a substitute for a lifecycle tag, and lifecycle tags are not
an execution-mode declaration. The implementation must use an explicit,
documented matrix of supported `test_type`/`test_tag`/`ac_id` combinations from
the merged ledger contract. It must reject missing, unsupported, or
contradictory combinations with the scenario ID, field, and value in the
error. In particular, it must not infer direct execution from crate membership,
feature text, scenario IDs, or a BDD tag; nor may it invent a
`scenario-runner` value if the reconciled ledger uses a different canonical
value.

The prerequisite sequence is therefore: repair and prove #380's malformed
mixed-tag handling; land and prove #389's declaration contract; record the
canonical dispatch matrix and reconcile the current direct-ID inventory; then
implement #212. A plan or implementation that leaves that matrix implicit is
not ready to replace `ACTIVE_ALPHA_ACS`.

## Migration and compatibility decision

Before replacing the constant, compute a compatibility report against the current `ACTIVE_ALPHA_ACS` behavior using the post-#380/#389 grid:

- every current direct AC must have an equivalent metadata classification;
- every derived direct AC must have a current proof path;
- BDD-only records must not be invoked a second time through the direct loop;
- any legacy-only AC, including streaming entries if they are not represented by the merged metadata, must be classified in the ledger/sidecar contract or retained in a narrowly named compatibility exception with an owner and removal condition.

If this comparison is not empty, stop the implementation slice and update the governing scenario metadata first. Do not merge a selector that merely makes the count look current.

## Acceptance criteria

- [ ] `ACTIVE_ALPHA_ACS` is removed from `xtask/src/alpha_check.rs`.
- [ ] Alpha selection is derived from the loaded feature grid and declared execution metadata.
- [ ] Direct AC IDs are sorted and deduplicated deterministically.
- [ ] BDD-only scenarios are covered by the existing `@alpha-active` run exactly once.
- [ ] Missing or unsupported classification data produces a contextual error naming the scenario and field.
- [ ] The implementation proves the supported dispatch matrix and rejects contradictory lifecycle/tag/mode combinations; it does not infer mode from unrelated metadata.
- [ ] A compatibility test proves the derived plan preserves the intended current alpha-check coverage, including streaming and BDD lanes.
- [ ] The machine-readable alpha summary records the discovered direct count and BDD selection count, or the output contract is updated with a separate receipt-backed selection section.
- [ ] No production behavior, feature tags, or ledger semantics are changed silently by the implementation PR.

## Focused proof

Run after #380 and #389 are reconciled on the implementation branch:

```text
cargo test -p scenario-contract -p xbrlkit-feature-grid -p xtask --locked --offline
cargo clippy -p scenario-contract -p xbrlkit-feature-grid -p xtask --all-targets --locked --offline -- -D warnings
cargo xtask feature-grid
cargo xtask schema-check
cargo xtask alpha-check
cargo fmt --all -- --check
git diff --check
```

Add fixture-style unit coverage for:

- duplicate scenarios sharing one AC;
- one direct/scenario-runner record;
- one BDD-only record with an exact test tag;
- missing AC, missing mode/tag, and unsupported mode;
- a legacy streaming/direct record;
- contradictory mode/tag/AC combinations, including a lifecycle `@alpha-active` tag paired with an unsupported or missing declared execution mode;
- deterministic ordering independent of input order.

The acceptance receipt must show that the selected direct IDs and BDD count are derived from the exact grid used by the run. A green alpha check proves this selection and execution path only; it does not prove release readiness.

## Likely files

- `xtask/src/alpha_check.rs`
- `xtask/src/main.rs` or a focused `xtask` helper module for selection tests
- `crates/scenario-contract/src/lib.rs` only if the merged contract needs a query helper
- `contracts/schemas/feature.grid.v1.json` and `tests/goldens/feature.grid.v1.json` only if the prerequisite contract requires regeneration
- the authoritative feature sidecars/ledger entries only for explicit classification repairs discovered by the compatibility test

## Non-goals

- Do not repair PR #380's malformed tag handling in this lane.
- Do not duplicate PR #389's declared-mode or receipt work.
- Do not redesign BDD execution, scenario-runner receipts, or the feature-grid schema beyond the minimum prerequisite contract.
- Do not remove the global `@alpha-active` BDD gate.
- Do not close #212 based on a plan or on a feature-grid count alone.

## Rollback and follow-ups

The implementation can be rolled back by restoring the constant and its existing loop; no persisted data migration is required. Any compatibility exception must become a builder-ready follow-up issue with the affected scenario IDs, source-truth file, proof command, and removal condition.
