# Implementation roadmap

This roadmap is the durable direction for improving xbrlkit. It is not a
replacement for the live GitHub issue/PR board: before each slice, reconcile
`origin/main`, the current issue and PR graph, active worktrees, source-truth
artifacts, and the exact candidate head.

## Long-term objective

Make xbrlkit more correct, usable, reviewable, and evidence-backed through a
sustained sequence of bounded, review-forward issue and PR slices. Each slice
should leave the repository easier to understand and the next slice easier to
build, test, review, and hand off.

The target state is not “all open work is merged.” It is a product and
maintenance system in which:

- externally meaningful behavior has a scenario or an explicit contract;
- fallible paths and useful diagnostics are preferred over hidden panics;
- schemas, DTOs, generated artifacts, and producer projections agree;
- focused proof is deterministic, receipt-backed, and honest about its limits;
- user-facing CLI, BDD, and maintainer workflows explain failure and next
  actions clearly; and
- release and CI claims are supported by current evidence rather than stale
  badges, plans, or queue snapshots.

## Work sequence

Work themes are ordered by dependency. A later theme may proceed in parallel
only when its source-truth and proof boundaries do not overlap an earlier
active lane.

### 1. Contract and source-truth integrity

Align stable DTOs, JSON schemas, producer projections, scenario metadata, and
generated goldens. Keep schema registration and validation ownership explicit.

Typical proof includes focused serialization tests, `cargo xtask schema-check`,
`cargo xtask feature-grid`, `cargo xtask impact`, and direct validation of
generated output where the registered checker does not yet cover the artifact.

Non-goals are speculative schema expansion, undocumented public fields, and
claiming compatibility from compilation alone.

### 2. Fallible and diagnostic paths

Remove or narrow panic-family behavior in production and test seams, propagate
typed/contextual errors, and make malformed-input behavior observable. Keep
each panic-audit slice local to one crate or responsibility boundary.

Typical proof includes the focused failure-path test, strict Clippy, a scoped
panic-family audit, and the repository's relevant scenario or acceptance gate.
The result is not a repository-wide panic-freedom claim unless a separate,
complete audit proves that claim.

### 3. Deterministic scenarios and test quality

Strengthen BDD parser contracts, fixture lifecycle and cache boundaries,
fallible test setup, focused regression tests, and no-network behavior in
normal acceptance runs. Prefer small tests that fail before the change and
pass after it.

Typical proof includes the narrow package test, the exact BDD selector,
`cargo xtask doctor`, `feature-grid`, `impact`, and `alpha-check` when the
changed seam affects the active scenario surface.

### 4. Usable workflows and public ergonomics

Improve CLI error messages, documented `Result` contracts, maintainer
commands, supported workflow examples, and user-facing diagnostics. Keep
domain logic separate from filesystem, process, network, environment, and
wall-clock concerns where that boundary improves reliability or testing.

Non-goals are broad API redesigns, new support promises, and behavior changes
without an issue, scenario, or durable contract.

### 5. Evidence, CI economics, and release readiness

Reduce duplicate or low-signal checks, keep dependency and policy state
reviewable, and make package/release claims traceable to current receipts.
Use the cheapest proof that is sufficient for the changed risk, and route
deeper proof to the appropriate full-suite, nightly, or release lane.

This theme is complete only when the relevant exact head, hosted checks,
review state, generated artifacts, and follow-up boundaries are reconciled;
local green tests alone are not release readiness.

## Current coordination boundaries

These dependencies are checkpoints, not completion claims and must be
re-verified before work resumes:

- The fallible workspace-root fallback in issue [#449](https://github.com/EffortlessMetrics/xbrlkit/issues/449) is a separate lane from the schema-validation test in [PR #420](https://github.com/EffortlessMetrics/xbrlkit/pull/420). Its implementation must still reconcile shared `repo_root()` callers such as `xtask/src/schema_check.rs` and current ownership before editing, but #420 is not a prerequisite.
- Fixture-cache freshness work in issue [#376](https://github.com/EffortlessMetrics/xbrlkit/issues/376) has a merged stacked child ([PR #381](https://github.com/EffortlessMetrics/xbrlkit/pull/381)) but remains dependent on the open parent [PR #371](https://github.com/EffortlessMetrics/xbrlkit/pull/371) reaching the default branch.
- Open PRs are candidate work, not landed behavior. An aligned PR should be improved in place only when its branch is not actively owned; otherwise leave a review or builder-ready handoff.

## Slice contract

For every selected issue and PR:

1. research current repository, GitHub, specs, policies, plans, and ownership;
2. record the target seam, acceptance criteria, proof commands, non-goals,
   rollback path, and claim boundary in the appropriate issue or plan;
3. implement one review-forward responsibility on an isolated branch;
4. run local proof against the exact head and reconcile hosted checks separately;
5. update source truth, receipts, issue linkage, and PR explanation; and
6. preserve unrelated dirty work and clean only lane-created artifacts after
   the PR's lifecycle permits it.

## Historical context

The original phases remain useful as product history:

### Phase 0

- control plane and receipts
- feature-grid, bundle, impact, and test-ac

### Phase 1

- filing manifest and IXDS wedges

### Phase 2

- taxonomy and SEC core profile

### Phase 3

- export, diff, and oracle lanes

Those historical labels do not override the current source truth or imply
that their old queue, metrics, or PR references are current.
