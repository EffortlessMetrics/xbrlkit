# Issue 350: Register dimensional scenarios after handler prerequisite

## Current state

`specs/features/taxonomy/dimensions.feature` declares 17 scenarios,
`SCN-XK-DIM-001` through `SCN-XK-DIM-017`, all tagged `@alpha-active`.
`specs/features/taxonomy/dimensions.meta.yaml` currently registers only the
first four scenarios. The missing thirteen records are source-truth debt, but
registering them immediately would make the feature grid advertise scenarios
that the current base cannot execute.

## Verified prerequisite

On base `e4d941c11ceb753866548a6a6b920917959dc456`, an isolated candidate
sidecar containing DIM-005 through DIM-017 compiled successfully to 66 total
scenarios and 17 dimension records. The representative execution checks did
not pass:

- `cargo xtask test-ac AC-XK-DIM-005` returned `no scenario assertions implemented`.
- `cargo xtask bdd --tags @SCN-XK-DIM-005` returned an unsupported-step error
  for `I parse the context dimensions`.

The missing dimension parsing and assertion handlers are present in the open
PR #247 at its current head `bdd3c1e1ab1f2eec797cc8dc829cc4a8daeec81e`.
Therefore the sidecar/golden update must be stacked after #247 lands or after
an explicitly documented replacement provides the same handlers.

## Selected follow-up slice

After the prerequisite is available, add DIM-005 through DIM-017 to
`dimensions.meta.yaml` using the existing typed-member fixture, regenerate the
feature-grid golden, and verify the scenarios through the BDD and `test-ac`
routes. Keep the sidecar and golden change separate from handler implementation
work.

## Acceptance criteria

- AC-350-001: all 17 dimension feature scenarios have sidecar records with the
  correct AC, requirement, fixture, crate, and edit-root metadata.
- AC-350-002: `cargo xtask feature-grid` reports 17 dimension records and the
  checked-in golden matches the generated grid.
- AC-350-003: representative typed-member and typed-value scenarios execute
  through the declared BDD path, including DIM-005 and at least one valid and
  invalid typed-value case.
- AC-350-004: `cargo xtask test-ac` succeeds for the representative ACs after
  the handler prerequisite is present.
- AC-350-005: the sidecar-only PR changes no production Rust behavior.

## Proof commands

```text
cargo xtask feature-grid
cargo xtask test-ac AC-XK-DIM-005
cargo xtask test-ac AC-XK-DIM-009
cargo xtask test-ac AC-XK-DIM-010
cargo xtask bdd --tags @SCN-XK-DIM-005
cargo xtask schema-check
cargo xtask alpha-check
git diff --check
```

Also compare the generated `artifacts/feature.grid.v1.json` with
`tests/goldens/feature.grid.v1.json` and record the exact base and prerequisite
heads used for the proof.

## Non-goals and rollback

- Do not add placeholder metadata that registers an unexecutable scenario.
- Do not duplicate or modify PR #247's handler implementation.
- Do not change the feature steps, ledger declarations, or production parser
  behavior in the sidecar slice.
- Rollback is the sidecar and generated-golden commit only; no user data or
  persisted runtime state is affected.
