# Issue 350: Register dimensional scenarios after handler prerequisite

## Current state

`specs/features/taxonomy/dimensions.feature` declares 17 scenarios,
`SCN-XK-DIM-001` through `SCN-XK-DIM-017`. The active gate must contain only
scenarios whose handlers and assertions are runnable; DIM-005 through DIM-017
remain non-active until the verified handler prerequisite is available.
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
`dimensions.meta.yaml` with fixture metadata that is specific enough to
exercise each scenario. A shared fixture is acceptable for scenarios with the
same input contract only when it contains the required data for each selected
scenario; valid and invalid typed-value scenarios must not resolve to the same
fixture input. In particular, DIM-009 and DIM-010 need distinguishable
fixture-backed inputs, and no registered scenario may leave `fixtures` empty
when it is verified through `cargo xtask test-ac`. Regenerate the feature-grid
golden and verify the scenarios through both the BDD and `test-ac` routes. Keep
the sidecar and golden change separate from handler implementation work.

## Acceptance criteria

- AC-350-001: all 17 dimension feature scenarios have sidecar records with the
  correct AC, requirement, crate, and edit-root metadata; every scenario used
  by `cargo xtask test-ac` has non-empty fixture metadata, and typed-value
  scenarios point to inputs that distinguish their valid, invalid, empty, and
  type-specific cases.
- AC-350-002: `cargo xtask feature-grid` produces exactly the unique scenario
  IDs `SCN-XK-DIM-001` through `SCN-XK-DIM-017` in the generated grid, and the
  checked-in golden is JSON-equivalent to that generated grid after the
  authoritative generator runs.
- AC-350-003: representative typed-member and typed-value scenarios execute
  through the declared BDD path, including DIM-005, DIM-009, and DIM-010; the
  valid and invalid typed-value cases use distinguishable inputs and produce
  their respective outcomes.
- AC-350-004: `cargo xtask test-ac` succeeds for the representative ACs after
  the handler prerequisite is present, using the declared fixture metadata;
  this route proves AC dispatch and fixture-backed execution, while the BDD
  route proves the Gherkin value-specific behavior.
- AC-350-005: relative to the recorded base head, the sidecar-only PR changes
  only `specs/features/taxonomy/dimensions.meta.yaml` and
  `tests/goldens/feature.grid.v1.json`; it changes no production Rust,
  feature-step, ledger, or parser path.

## Proof commands

```text
cargo xtask feature-grid
cargo xtask test-ac AC-XK-DIM-005
cargo xtask test-ac AC-XK-DIM-009
cargo xtask test-ac AC-XK-DIM-010
cargo xtask bdd --tags @SCN-XK-DIM-005
cargo xtask bdd --tags @SCN-XK-DIM-009
cargo xtask bdd --tags @SCN-XK-DIM-010
cargo xtask schema-check
cargo xtask alpha-check
git diff --check
```

After `cargo xtask feature-grid`, run the following PowerShell assertions from
the repository root. They make the scenario-ID and golden comparison
machine-checkable rather than relying on a record count or visual diff:

```powershell
$generated = Get-Content artifacts/feature.grid.v1.json -Raw | ConvertFrom-Json
$golden = Get-Content tests/goldens/feature.grid.v1.json -Raw | ConvertFrom-Json
$expected = @(1..17 | ForEach-Object { 'SCN-XK-DIM-{0:D3}' -f $_ })
$actual = @($generated.scenarios | Where-Object { $_.scenario_id -clike 'SCN-XK-DIM-*' } | Select-Object -ExpandProperty scenario_id)
if ($actual.Count -ne 17 -or (@($actual | Sort-Object -CaseSensitive -Unique).Count -ne 17) -or (Compare-Object -CaseSensitive ($expected | Sort-Object -CaseSensitive) ($actual | Sort-Object -CaseSensitive))) { throw 'dimension grid must contain exactly DIM-001 through DIM-017 once each' }
if ((ConvertTo-Json $generated -Depth 100 -Compress) -cne (ConvertTo-Json $golden -Depth 100 -Compress)) { throw 'generated feature grid differs from the checked-in golden' }

$allowed = @('specs/features/taxonomy/dimensions.meta.yaml', 'tests/goldens/feature.grid.v1.json')
$changed = @(git diff --name-only <recorded-base-head>...HEAD)
if (@($changed | Where-Object { $_ -cnotin $allowed }).Count -ne 0 -or @($changed | Sort-Object -CaseSensitive -Unique).Count -ne $allowed.Count) { throw 'sidecar-only PR changed a path outside the two-file allowlist' }
```

Replace `<recorded-base-head>` with the exact base head recorded for the
follow-up PR. Record that base head and the prerequisite handler PR head in
the PR receipt so the allowlist and execution proof are reproducible.

## Non-goals and rollback

- Do not add placeholder metadata that registers an unexecutable scenario.
- Do not duplicate or modify PR #247's handler implementation.
- Do not change the feature steps, ledger declarations, or production parser
  behavior in the sidecar slice.
- Rollback is the sidecar and generated-golden commit only; no user data or
  persisted runtime state is affected.
