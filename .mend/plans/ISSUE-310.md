# Issue #310: bounded panic-audit slices

Status: active multi-PR maintenance plan.

## Selected slice

Convert the five xbrl-stream unit tests and the crate doctest from expect-based
parse handling to fallible anyhow Result paths. The parser and public API stay
unchanged; only test/example error propagation changes.

## Acceptance criteria

- The five unit tests return Result and propagate parse errors with `?`.
- The doctest demonstrates the same fallible parse path without expect or
  unwrap.
- The xbrl-stream focused test and doctest suite passes.
- No production parser behavior, public type, schema, or scenario contract
  changes.

## Proof commands

- cargo test -p xbrl-stream --locked --offline
- cargo clippy -p xbrl-stream --all-targets --locked --offline -- -D warnings
- cargo fmt --all --check
- git diff --check

## Non-goals

- Do not change XbrlStreamReader or FactHandler signatures.
- Do not modify taxonomy-loader, dimensional-rules, export-run, or CLI panic
  sites owned by separate follow-up slices.
- Do not claim that the repository-wide panic audit is complete.
