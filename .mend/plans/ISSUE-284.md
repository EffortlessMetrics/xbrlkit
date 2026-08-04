# Plan: Reconcile the Broken-Reference Scout Inventory (Issue #284)

**Stream:** Plan quality and source truth  
**Issue:** #284  
**Status:** Reconciled — no production repair identified  
**Verified:** 2026-08-04

## Objective

Reconcile the scout report for broken references in plan documents against the
current repository before asking a builder to create files or repair paths.
This plan records the verified disposition of the inventory and prevents
delegated or historical work from being duplicated.

## Current repository evidence

The original issue table is not an authoritative inventory. The current
repository contains these plan documents under `.mend/plans/`:

- `ISSUE-98.md`
- `ISSUE-100.md`
- `ISSUE-101.md`
- `ISSUE-102-review.md`
- `ISSUE-102.md`
- `ISSUE-284.md`
- `scheduler-log.md`
- `taxonomy-loader.md`

The issue's earlier reference to `.mend/plans/ISSUE-284.md` was not itself
committed on the base branch; this document supplies the missing source-truth
artifact on this branch. `scheduler-log.md` is retained as historical planning
evidence, not treated as a builder plan or a missing implementation target.

The cited paths were checked in their surrounding plan context:

- `ISSUE-100.md` points to repository paths that exist, including the
  taxonomy feature, metadata sidecar, loader source, manifests, and lockfile.
- `ISSUE-101.md` records `docs/MAINTAINER_VISION.md` as intentionally deleted
  by historical PR #14. Its absence is an expected historical state, not a
  current missing implementation target.
- `ISSUE-102.md` points to `adr/ADR-008-taxonomy-loader-http-client.md`, which
  exists. Its references to a possible ADR index and future updates are plan
  notes, not broken repository links.
- `taxonomy-loader.md` points to
  `.mend/research/taxonomy-dimension-loading.md`, which exists.

The issue's entries for untracked plans and scout artifacts belong to the
separate plan-tracking scope of issue #225. The benchmark entries belong to
the benchmark lane tracked by issue #250 and PR #408. Those work items are
not duplicated here.

## Disposition

| Reported category | Disposition |
| --- | --- |
| Existing plan paths | Verified against current files; no repair required |
| Historical `MAINTAINER_VISION.md` path | Retain as an explicit deletion record in ISSUE-101 |
| Aspirational files in plan tables | Keep marked as planned; do not create them as a reference repair |
| Scheduler log | Retain as historical planning evidence; no path repair required |
| Untracked/orphaned plans and scout artifacts | Defer to issue #225 and related plan-maintenance work |
| Benchmark target references | Verified in PR #408; do not duplicate |

No current production, schema, or documentation-path change is required by
this inventory after reconciliation.

## Acceptance criteria

- [x] The scout inventory is verified against the current `.mend/plans/`
      contents and surrounding plan text.
- [x] Historical deletion is distinguished from a broken current reference.
- [x] Aspirational and delegated work is not converted into fabricated files.
- [x] The issue's source-truth plan is committed in `.mend/plans/`.
- [ ] Issue #225 independently resolves untracked/orphaned plan disposition.

## Proof commands

Run from the repository root:

```text
Get-ChildItem .mend/plans -Filter *.md
Get-ChildItem .mend/plans -Filter *.md | Measure-Object
rg -n -g '*.md' '`[^`]+`' .mend/plans
Test-Path docs/MAINTAINER_VISION.md
Test-Path adr/ADR-008-taxonomy-loader-http-client.md
Test-Path .mend/research/taxonomy-dimension-loading.md
git diff --check
```

These checks establish the current file inventory, including the scheduler log
and this source-truth plan, plus the specific historical or existing paths
discussed above. They do not prove that every prose reference in every future
plan is semantically correct; future plan changes must repeat the contextual
audit.

## Non-goals

- Do not create historical, aspirational, or delegated files solely to satisfy
  the old scout table.
- Do not duplicate issue #225, #242, or #250 implementation work.
- Do not add a heuristic CI checker without a separately verified design for
  resolving prose references and avoiding the scout's false positives.
- Do not change production code or public contracts.

## Rollback

Revert this plan-only PR. No runtime, schema, generated artifact, or user data
changes are involved.
