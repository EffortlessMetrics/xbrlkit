# Issue #297: Narrow the repeated synthetic taxonomy setup

## Status

Builder-ready plan draft, reconciled against `origin/main` at
`e4d941c11ceb753866548a6a6b920917959dc456`.

The original issue describes a shared taxonomy fixture crate. Current source
evidence does not support treating every related taxonomy construction as the
same fixture: domains, member labels, requiredness, hypercubes, and typed
dimensions differ. The first implementation slice is therefore intentionally
local to the equivalent BDD constructions.

## Problem

`crates/xbrlkit-bdd-steps/src/lib.rs` constructs the same minimal explicit
scenario taxonomy in three paths:

1. the `a loaded taxonomy with dimension definitions` Given step;
2. the `I validate the dimension-member pair` When step; and
3. `create_synthetic_taxonomy`, the fallback used when a fixture-backed load
   is unavailable.

The Given and When constructions currently define the
`us-gaap:ScenarioDomain` domain with Actual and Forecast members, the
`us-gaap:StatementScenarioAxis` explicit dimension, and the dimension-to-domain
link. `create_synthetic_taxonomy` is called only by `I load the taxonomy` when
the selected path contains `fixtures/` but the schema file is missing. That
fallback adds `dim:CustomerAxis` with value type `xs:string`; the current Given
steps do not require that typed dimension.

This duplication can drift and makes BDD behavior harder to review. The
existing taxonomy-loader setup refactor in PR #383 is a separate seam and must
land or otherwise be reconciled before editing the same `lib.rs` file.

## Reconciled duplication matrix

| Surface | Same scenario fixture? | Disposition |
| --- | --- | --- |
| BDD Given, BDD When, `create_synthetic_taxonomy` | Yes, with the fallback's typed dimension addition | Extract one private BDD helper and preserve the typed-dimension extension |
| `dimensional-rules` test taxonomy | No: uses `StatementScenarioDomain`, Actual/Budget labels, required dimension, hypercube, and Revenue association | Keep local; do not hide these semantics behind the BDD helper |
| `taxonomy-dimensions` unit test taxonomy | No: it is a focused unit fixture with a different domain shape and one member | Keep local to the crate |

## Selected design

Add a private helper in `xbrlkit-bdd-steps/src/lib.rs` for the shared BDD
scenario taxonomy used by the Given and When paths. The missing-schema
fallback should call that helper and then add its narrowly required
`dim:CustomerAxis` typed dimension. Do not add the typed dimension to the
shared Given/When shape or expose a general-purpose fixture option.

The helper must preserve:

- `us-gaap:ScenarioDomain` as the domain QName;
- Actual and Forecast members with their current order, parent, and `None`
  labels;
- `us-gaap:StatementScenarioAxis` as a non-required explicit dimension;
- the existing dimension-to-domain mapping;
- the missing-schema fallback's `dim:CustomerAxis` typed dimension and
  `xs:string` value type;
- all existing loader-context, validation-context, and execution-state
  mutations around the construction.

### Missing-schema fallback proof contract

The implementation PR must add a focused `run_scenario` test (or an equally
direct BDD-runner test) for the existing missing-schema branch. The test must
select a scenario whose `fixtures/synthetic/taxonomy/standard-location-01/`
directory is present while `schema.xsd` is absent, then verify the resulting
`World` state rather than only asserting that the step returns `Ok(())`:

- `TaxonomyLoaderContext.loaded` is `true` and a taxonomy is present;
- `us-gaap:ScenarioDomain` contains the Actual and Forecast members with their
  existing order and parent values;
- `us-gaap:StatementScenarioAxis` remains a non-required explicit dimension
  and maps to `us-gaap:ScenarioDomain`;
- the fallback-only `dim:CustomerAxis` typed dimension has value type
  `xs:string`;
- unrelated validation context and `World.execution` state are unchanged by
  the taxonomy-loading step.

The test must use the repository's existing synthetic path and remain
fixture-free at runtime; it must not introduce a network dependency or turn
the fallback into a general fixture abstraction.

## Acceptance criteria

- [ ] The three equivalent BDD constructions delegate to one private helper
  or the missing-schema fallback delegates to that helper plus a clearly
  named typed-dimension extension.
- [ ] Existing BDD scenarios observe the same taxonomy members, QNames,
  requiredness, and typed-dimension behavior before and after the refactor.
- [ ] The missing-schema fallback remains covered by
  `@AC-XK-TAX-LOAD-003`: with the current fixture directory present but
  `schema.xsd` absent, the focused runner proof verifies the complete fallback
  contract above, including domain members, dimension mapping, non-required
  explicit-dimension behavior, and the fallback-only typed dimension. The
  implementation PR must strengthen the existing BDD oracle if it currently
  accepts the path without distinguishing those values.
- [ ] `dimensional-rules` and `taxonomy-dimensions` keep their distinct local
  fixtures; no misleading cross-crate abstraction is introduced.
- [ ] No public API, schema, profile, receipt, or production validation
  behavior changes.
- [ ] The implementation remains within `crates/xbrlkit-bdd-steps/src/lib.rs`
  unless a concrete dependency or test seam requires a separately planned
  file.

## Dependencies and sequencing

- Reconcile PR #383 (`c817e6f6467a6d82d5136e0fe61223937dce8f29`) first because
  it edits the same BDD file. Do not race its owner; restack or re-audit the
  helper after that PR lands.
- Issue #306 and issue #307 overlap this BDD-local seam. Treat this plan as the
  narrowed implementation lane and supersede or update those issue plans
  before opening a duplicate implementation PR.
- A new `taxonomy-test-fixtures` crate is not a prerequisite for this slice.
  Reconsider that broader boundary only after the local helper is proven and
  genuinely identical cross-crate consumers remain.

## Proof commands

```text
cargo fmt --all -- --check
cargo test -p xbrlkit-bdd-steps --locked --offline
cargo clippy -p xbrlkit-bdd-steps --all-targets --locked --offline -- -D warnings
cargo xtask bdd --tags @AC-XK-TAX-LOAD-001
cargo xtask bdd --tags @AC-XK-TAX-LOAD-002
cargo xtask bdd --tags @AC-XK-TAX-LOAD-003
cargo xtask bdd --tags @AC-XK-TAX-LOAD-004
cargo xtask bdd --tags @AC-XK-TAX-LOAD-006
git diff --check
```

The focused BDD selectors establish that the affected Given/When paths still
execute. `@AC-XK-TAX-LOAD-003` additionally establishes the deterministic
missing-schema fallback and typed-dimension extension. They do not prove
equivalence of unrelated dimensional-rules or taxonomy-dimensions unit
fixtures, which remain out of scope.

## Non-goals

- No new workspace crate or dependency.
- No migration of the distinct `dimensional-rules` or
  `taxonomy-dimensions` fixtures.
- No production taxonomy model change.
- No broad BDD module split or loader setup refactor.
- No benchmark or CI policy change.

## Rollback and follow-up

Revert the private helper extraction; the change has no persisted state or
public API migration. If a later inventory proves a truly shared cross-crate
fixture contract, open a separate plan with an explicit dependency graph and
label-preservation contract rather than widening this PR.
