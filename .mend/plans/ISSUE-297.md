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

Each construction currently defines the `us-gaap:ScenarioDomain` domain with
Actual and Forecast members, the
`us-gaap:StatementScenarioAxis` explicit dimension, and the dimension-to-domain
link. The fallback additionally defines `dim:CustomerAxis`.

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
scenario taxonomy. The helper should return the same `DimensionTaxonomy`
shape currently produced by the three BDD paths. Keep the typed
`dim:CustomerAxis` addition in the fallback path only if the current call
site requires it, or expose a narrowly named private option rather than
adding a general-purpose fixture abstraction.

The helper must preserve:

- `us-gaap:ScenarioDomain` as the domain QName;
- Actual and Forecast members with their current order, parent, and `None`
  labels;
- `us-gaap:StatementScenarioAxis` as a non-required explicit dimension;
- the existing dimension-to-domain mapping;
- the fallback's `dim:CustomerAxis` typed dimension and `xs:string` value type;
- all existing loader-context, validation-context, and execution-state
  mutations around the construction.

## Acceptance criteria

- [ ] The three equivalent BDD constructions delegate to one private helper
  or one helper plus a clearly named typed-dimension extension.
- [ ] Existing BDD scenarios observe the same taxonomy members, QNames,
  requiredness, and typed-dimension behavior before and after the refactor.
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
execute. They do not prove equivalence of unrelated dimensional-rules or
taxonomy-dimensions unit fixtures, which remain out of scope.

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
