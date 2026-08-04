# Plan: Missing BDD Step Handlers (#219)

**Issue:** #219
**Status:** Slice 1 ready for review

## Selected slice

Make the five `negative_values.feature` scenarios executable through the
fixture-free BDD runner. The numeric rule already exists in
`crates/numeric-rules`; this slice adds the BDD Given/When/Then seam and focused
regression coverage.

## Acceptance criteria

- `@SCN-XK-SEC-NEGATIVE-001` reports an `SEC.NEGATIVE_VALUE` finding with
  `error` severity and the requested concept subject.
- `@SCN-XK-SEC-NEGATIVE-002` and `@SCN-XK-SEC-NEGATIVE-005` report no error
  findings.
- `@SCN-XK-SEC-NEGATIVE-003` reports the employee-count finding.
- `@SCN-XK-SEC-NEGATIVE-004` recognizes accounting-parentheses notation.
- The canonical fixture-free BDD command passes each selector.
- Existing fixture-backed scenario execution remains unchanged.

## Target seam and proof

- `crates/xbrlkit-bdd-steps/src/lib.rs`: synthetic fact state and handlers.
- `specs/features/sec/negative_values.feature`: existing executable contract.
- Focused unit tests for the new Given/When/Then behavior.
- `cargo xtask bdd --tags @SCN-XK-SEC-NEGATIVE-00N` for all five scenarios.
- Package tests, strict Clippy, formatting, feature-grid, doctor, and diff
  checks.

## Non-goals and follow-ups

- No changes to the already-tested numeric rule semantics.
- No implementation of the remaining #219 handler groups: SEC validation
  background, context-completeness count, taxonomy dimension assertions, or
  package-readiness handlers.
- Those groups require separate review-forward slices with their own proof.
