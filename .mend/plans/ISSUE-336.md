# Plan: Centralize fact-based validation findings (Issue #336)

## Objective

Reduce repeated `ValidationFinding` construction while preserving the current
report shape and leaving allocation strategy changes behind an evidence gate.

## Selected slice: Phase 1

Add `ValidationFinding::for_fact` in `xbrl-report-types`. The helper preserves
the existing convention of copying `Fact.member` into `member` and
`Fact.concept` into `subject`. Migrate the fact-identity finding builders in:

- `context-completeness`;
- `numeric-rules` and decimal-precision validation; and
- `unit-rules`.

`dimensional-rules` remains unchanged in this slice because its fact-related
missing-context finding uses `Fact.concept` and `Fact.context_ref` for
domain-specific `member` and `subject` values rather than the shared
fact-identity convention.

The helper is additive and keeps `ValidationFinding` field types and serialized
shape unchanged.

## Acceptance criteria

- All selected fact-identity builders use the shared helper.
- The helper produces the same rule ID, severity, message, member, and subject
  values as the previous struct literals.
- Non-fact findings and context/stream-specific subject semantics remain
  unchanged.
- No `Arc<str>`, string interner, dependency, schema, or public field change is
  introduced in this phase.

## Proof

- focused tests for `xbrl-report-types`, `context-completeness`,
  `dimensional-rules`, `numeric-rules`, and `unit-rules`;
- strict Clippy for the affected crates;
- workspace formatting, diff, feature-grid, doctor, and changed-path impact
  checks.

## Deferred roadmap

- Phase 2: profile the resulting finding-copy paths before considering shared
  ownership types such as `Arc<str>`.
- Phase 3: consider interning only if profiling demonstrates a material
  allocation hotspot and the dependency/license boundary is accepted.

## Non-goals and rollback

This slice does not change runtime validation decisions, finding serialization,
or unrelated constructors. Reverting the helper, call-site migrations, and
this plan fully rolls back the change without data migration.
