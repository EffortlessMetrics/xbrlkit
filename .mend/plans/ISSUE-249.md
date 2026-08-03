# Issue #249: bounded dependency-audit slices

Status: active multi-PR maintenance plan.

## Current state

The original issue asks for a workspace dependency audit. A current cargo-machete 0.9.2 run against origin/main reports confirmed candidates in several crates. Existing PRs already own some of those findings, including the serde_json edges, placeholder crates, render-md, and xbrl-stream dependency cleanup.

The audit must therefore land as independent slices. No slice should claim that the entire workspace is clean until every remaining finding has been reconciled.

## Selected slice

This slice removes five manifest-only dependency edges from xtask:

- serde_yaml
- walkdir
- sec-profile-types
- validation-run
- xbrl-report-types

Source search confirms none of these names are referenced by xtask source files. The dependencies remain available to the crates that actually use them.

## Acceptance criteria

- xtask builds and tests without the five direct dependencies.
- Cargo.lock records the updated xtask dependency list.
- cargo-machete no longer reports these five xtask entries.
- No runtime, public API, schema, profile, or scenario behavior changes occur.
- The CI audit guard remains a follow-up until all current workspace findings are resolved; adding a hard-failing guard before then would make main fail immediately.

## Proof commands

- cargo check -p xtask --locked --offline
- cargo test -p xtask --locked --offline
- cargo clippy -p xtask --all-targets --locked --offline -- -D warnings
- cargo machete
- cargo fmt --all --check
- git diff --check

The cargo-machete command is expected to remain non-zero until the other
workspace findings are handled. For this slice, proof is the absence of an
xtask finding and the explicit reporting of the remaining unrelated findings.

## Non-goals

- Do not remove workspace dependencies used by other crates.
- Do not delete placeholder crates; PR #363 owns that separate cleanup.
- Do not add the CI audit command in this slice; track it after the remaining findings land.
