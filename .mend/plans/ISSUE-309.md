# Issue 309: Reconcile duplicated utility work

## Current state

Issue #309 identifies three duplicated helpers:

- `extract_namespaces` in `taxonomy-loader/src/schema.rs` and `linkbase.rs`;
- `resolve_path` in the same two files;
- `sanitize_for_rule_id` in `numeric-rules` and `efm-rules`.

The taxonomy-loader pair is owned by the canonical replacement [PR #435](https://github.com/EffortlessMetrics/xbrlkit/pull/435),
whose current head (`93e4c8b18a63353a690ce7a916aca0824a476cd1`) extracts the
helpers into `xml_util.rs` and carries the broader URL and cross-platform path
coverage. PR #319 is closed and unmerged; its head
(`855253fdc04666cf05ee736293123de730963720`) is historical source material,
not the live owner. This document records that reconciliation so issue #309
does not spawn a duplicate implementation lane.

## Selected PR slice

This PR is docs-only. It records that the taxonomy-loader portion belongs to
PR #435 and narrows the remaining implementation candidate to the
cross-crate `sanitize_for_rule_id` utility.

## Acceptance criteria

- AC-309-001: The issue plan links the taxonomy-loader duplication to PR #435
  and does not propose a competing implementation.
- AC-309-002: The plan records that #435 owns namespace extraction and path
  resolution, including its cross-platform follow-up coverage.
- AC-309-003: The remaining sanitizer duplication is explicitly separate work
  with its own dependency and API decision.
- AC-309-004: This docs-only lane changes no production code or public API.

## Proof

```text
git diff --check
git status --short
```

## Follow-up

The `sanitize_for_rule_id` duplication remains a separate candidate slice.
Before implementing it, recheck current dependency topology and decide whether
an existing lightweight crate is an appropriate host. Do not modify the
taxonomy-loader seam unless #435 is superseded with explicit evidence.

## Non-goals and rollback

This lane does not change parsing behavior, path normalization, dependency
topology, public exports, or the cross-crate rule-ID utility. Rollback is the
single docs-only commit that adds this reconciliation plan.
