# Issue #203: Dependency audit ledger and sequencing plan

## Status

Builder-ready audit ledger, reconciled against `origin/main` at
`e4d941c11ceb753866548a6a6b920917959dc456` on 2026-08-04.

The original issue reported eight crates and a single broad cleanup. The
current `cargo machete --with-metadata` run reports seventeen dependency edges
across nine packages. Several edges are already owned by active PRs, so this
issue remains an index and sequencing contract rather than a license to make
overlapping manifest edits.

The recorded baseline toolchain is `cargo-machete 0.9.1`, Cargo `1.92.0
(344c4567c 2025-10-21)`, and rustc `1.92.0 (ded5c06cf 2025-12-08)`. These
versions are part of the audit receipt because dependency-audit output can
vary by tool version.

## Current audit ledger

| Finding | Current lane | Disposition |
| --- | --- | --- |
| `receipt-types -> serde_json` | PR #391 | Remove in the owned slice; recheck after merge |
| `scenario-contract -> serde_json` | PR #391 | Remove in the owned slice; recheck after merge |
| `xbrl-stream -> tokio` | PR #355 | Remove with the async feature cleanup |
| `xbrl-stream -> xbrl-report-types` | PR #355 | Remove in the owned slice |
| `xbrlkit-cli -> render-md` | PR #354 | Remove in the owned slice |
| five `xtask` edges: `sec-profile-types`, `serde_yaml`, `validation-run`, `walkdir`, `xbrl-report-types` | PR #405 | Remove in the owned slice |
| `dimensional-rules -> serde`, `thiserror` | PR #492 | Re-audit after the owner-controlled removal slice |
| `unit-rules -> serde` | PR #475 | Re-audit after the owner-controlled removal slice |
| `validation-run -> context-completeness` | PR #491 | Re-audit after the owner-controlled removal slice |
| `taxonomy-loader -> serde_json`, `tempfile`, `tokio` | PRs #386, #400, #401, and the transport sequence in #429/#436 | Reconcile cache, warning, import, and transport ownership before removing edges |

The baseline command is expected to exit nonzero while the table contains
findings. A nonzero audit is not a source failure by itself; it is the current
inventory that this plan sequences.

For a reproducible baseline, run the audit from a disposable clean worktree
with the recorded toolchain. Treat exit code `1` as the findings-bearing
inventory state; treat exit code `0` as a clean inventory; and stop on exit
code `2` or any environment/processing error. If the audit or its metadata
probe changes `Cargo.lock`, restore that generated file before recording the
baseline so the audit receipt contains findings rather than incidental
workspace churn.

## Selected operating model

Treat each manifest/lockfile change as a separate review-forward PR unless
two edges share the same owner-controlled seam and can be proven together.
Each slice must:

1. verify the dependency is source-unused, including build scripts, examples,
   tests, feature-gated code, and generated inputs where applicable;
2. remove only the selected declaration and the Cargo-generated lockfile edge;
3. run the package's locked check, test, Clippy, metadata, and tree proof;
4. rerun `cargo machete --with-metadata` and record remaining findings without
   claiming workspace completion; and
5. update this ledger or a narrower follow-up issue when ownership changes.

The ownership row must name the concrete PR or issue, its current head when a
receipt is recorded, and the bounded claim that remains with that lane. An
unowned or stale row is a re-audit candidate, not permission to edit another
lane's manifest.

Do not add a hard-failing cargo-machete CI gate until the remaining findings
are either removed or explicitly recorded as reviewed, owned exceptions.

## Acceptance criteria

- [ ] Every original #203 finding is removed, verified as used, or linked to a
  current owner-controlled PR with a bounded claim.
- [ ] The newer taxonomy-loader findings are reconciled after the active
  taxonomy-loader/cache lanes settle; they are not silently omitted because
  they were absent from the original issue table.
- [ ] Each dependency-removal PR updates `Cargo.lock` through Cargo and proves
  that the affected package still builds, tests, and passes Clippy.
- [ ] The ledger records the exact audit command and distinguishes remaining
  findings from command/environment failures.
- [ ] The ledger records the audit toolchain and each active row has a concrete
  owner-controlled PR or issue with a bounded claim.
- [ ] The taxonomy-loader re-audit exercises the governed cache and import
  scenarios (`SCN/AC-XK-TAX-LOAD-005` through `008`) and records any remaining
  transport-specific proof separately from dependency-removal proof.
- [ ] No public API, runtime behavior, schema, profile, or scenario contract
  changes as a consequence of manifest-only cleanup.

## Proof template for each implementation slice

```text
cargo metadata --locked --no-deps --format-version 1
cargo tree -p <package> --edges normal --locked --offline
cargo check -p <package> --all-targets --all-features --locked --offline
cargo test -p <package> --all-targets --all-features --locked --offline
cargo clippy -p <package> --all-targets --locked --offline -- -D warnings
cargo fmt --all -- --check
git diff --check
cargo machete --with-metadata
```

The exact package, supported feature set, build scripts, examples, generated
inputs, and whether workspace checks are warranted belong in the selected PR;
the PR must document any unsupported `--all-features` or all-targets case. A
passing package proof establishes that slice, not a clean workspace audit or
release readiness.

## Non-goals

- No dependency removals are included in this plan-only slice.
- No broad Cargo workspace redesign or dependency-policy CI gate.
- No replacement dependency additions without a separately justified need.
- No edits to active owner-controlled PR branches.

## Rollback and follow-up

Each implementation PR can be reverted as a manifest/lockfile unit with no
persisted-data migration. After the active PRs land, open the next smallest
unowned edge slice from this ledger and refresh the exact `cargo machete`
output before editing.
