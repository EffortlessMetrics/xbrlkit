# Issue #203: Dependency audit ledger and sequencing plan

## Status

Builder-ready audit ledger, reconciled against `origin/main` at
`e4d941c11ceb753866548a6a6b920917959dc456` on 2026-08-05.

The original issue reported eight crates and a single broad cleanup. The
current `cargo machete --with-metadata` run reports seventeen dependency edges
across nine packages. Several edges are already owned by active PRs, so this
issue remains an index and sequencing contract rather than a license to make
overlapping manifest edits.

A fresh audit from a disposable worktree at the same exact `origin/main` head
on 2026-08-05 returned exit code `1` with seventeen edges across nine
packages and no `Cargo.lock` change. The prior plan head `545f370` is not
`origin/main` and is not used as the baseline.

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

### Live ownership evidence

The following owner, branch, and head records were refreshed against GitHub on
2026-08-05. They identify ownership only; they do not claim that any PR has
merged or that its dependency-removal proof is complete.

| Lane | Owner | Branch | Current head | Bounded claim |
| --- | --- | --- | --- | --- |
| PR #391 (`receipt-types`, `scenario-contract`) | EffortlessSteven | `codex/issue308-remove-unused-deps` | `d582d85fbd98247a3cb2a1c1a2b834bf2ad2a37e` | remove the two `serde_json` edges |
| PR #355 (`xbrl-stream`) | EffortlessSteven | `codex/pr318-clean` | `0244e412905d5eeb26afe2a0fa65094800b807d6` | remove the async dependency edges |
| PR #354 (`xbrlkit-cli`) | EffortlessSteven | `codex/pr341-clean` | `2d0215fdeea5880f9ea6cafa41e8b1ca7d77078d` | remove the `render-md` edge |
| PR #405 (`xtask`) | EffortlessSteven | `codex/issue249-machete-ci` | `dbec1fa0a1a9fc4d1e1bc8284a4e33921f3f2320` | remove the five listed `xtask` edges |
| PR #492 (`dimensional-rules`) | EffortlessSteven | `codex/issue308-dimensional-rules` | `e71606ed86e40e6e07fc02cfb8141b19a9884833` | remove `serde` and `thiserror` |
| PR #475 (`unit-rules`) | EffortlessSteven | `codex/issue330-unit-rules-serde` | `54e7ca615e7ef3644bcf38b948a7d7d9ea403313` | remove `serde` |
| PR #491 (`validation-run`) | EffortlessSteven | `codex/issue308-validation-run` | `fcb39514751b0412e978566e95ef97522cdbbc1f` | remove `context-completeness` |
| PR #386 (`taxonomy-loader` cache keys) | EffortlessSteven | `codex/issue233-cache-path` | `90ab770fc11dc1fbe0d32b6c482d506789c6b561` | isolate URL-cache keys |
| PR #400 (`taxonomy-loader` warning) | EffortlessSteven | `codex/issue216-tracing-warning` | `7f2c4a0443b1026fc532d942354e837b9b919410` | preserve content and redact cache-write warnings |
| PR #401 (`taxonomy-loader` cache/import tracking) | EffortlessSteven | `codex/issue248-cache-tracking-repair` | `8ee14719e2b9247fe8779fcc06adc55902eac372` | track cache reuse and imported schemas |
| PR #429/#436 (taxonomy-loader transport plan/seam) | EffortlessSteven | `codex/issue211-plan` / `codex/issue211-transport` | `cce9647ae39da6bf24e6c92039274133c7ab8429` / `d114fc15f13e1c28b328ad7fd3934ebdf19da07e` | bound deterministic transport coverage |

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
- [ ] The taxonomy-loader re-audit runs the executable feature runner for the
  four governed scenarios and maps each result explicitly:
  - `cargo xtask bdd --tags @SCN-XK-TAX-LOAD-005` — cache reuse;
  - `cargo xtask bdd --tags @SCN-XK-TAX-LOAD-006` — recursive schema imports;
  - `cargo xtask bdd --tags @SCN-XK-TAX-LOAD-007` — valid dimension-member
    validation; and
  - `cargo xtask bdd --tags @SCN-XK-TAX-LOAD-008` — invalid-member findings.
  Remaining transport-specific proof is recorded separately from
  dependency-removal proof.
- [ ] No public API, runtime behavior, schema, profile, or scenario contract
  changes as a consequence of manifest-only cleanup.

## Proof template for each implementation slice

```text
cargo metadata --locked --no-deps --format-version 1
cargo tree -p <package> --edges all --locked --offline
cargo check -p <package> --all-targets --all-features --locked --offline
cargo test -p <package> --all-targets --all-features --locked --offline
cargo clippy -p <package> --all-targets --locked --offline -- -D warnings
cargo fmt --all -- --check
cargo machete --with-metadata
git diff --check
```

The exact package, supported feature set, build scripts, examples, generated
inputs, and whether workspace checks are warranted belong in the selected PR;
the PR must document any unsupported `--all-features` or all-targets case. A
passing package proof establishes that slice, not a clean workspace audit or
release readiness. Run `cargo machete` from a disposable clean worktree when
possible; otherwise restore only incidental audit-generated `Cargo.lock`
changes before the final `git diff --check`. The receipt must describe the
post-restoration diff, while preserving any intentional lockfile delta from
the selected dependency-removal slice.

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
