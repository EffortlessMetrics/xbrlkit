# Issue #203: Dependency audit ledger and sequencing plan

## Status

Builder-ready audit ledger, reconciled against `origin/main` at
`e4d941c11ceb753866548a6a6b920917959dc456` on 2026-08-04.

The original issue reported eight crates and a single broad cleanup. The
current `cargo machete --with-metadata` run reports seventeen dependency edges
across ten packages. Several edges are already owned by active PRs, so this
issue remains an index and sequencing contract rather than a license to make
overlapping manifest edits.

## Current audit ledger

| Finding | Current lane | Disposition |
| --- | --- | --- |
| `receipt-types -> serde_json` | PR #391 | Remove in the owned slice; recheck after merge |
| `scenario-contract -> serde_json` | PR #391 | Remove in the owned slice; recheck after merge |
| `xbrl-stream -> tokio` | PR #355 | Remove with the async feature cleanup |
| `xbrl-stream -> xbrl-report-types` | PR #355 | Remove in the owned slice |
| `xbrlkit-cli -> render-md` | PR #354 | Remove in the owned slice |
| five `xtask` edges: `sec-profile-types`, `serde_yaml`, `validation-run`, `walkdir`, `xbrl-report-types` | PR #405 | Remove in the owned slice |
| `dimensional-rules -> serde`, `thiserror` | Active dimensional-rules refactor lanes | Re-audit after those lanes; do not race their manifests or source |
| `unit-rules -> serde` | Active unit-rules refactor lanes | Re-audit after the owner-controlled lanes settle |
| `validation-run -> context-completeness` | Active validation-run lanes | Re-audit after the owner-controlled lanes settle |
| `taxonomy-loader -> serde_json`, `tempfile`, `tokio` | Active taxonomy-loader lanes and #211 | Reconcile with the HTTP/cache work before removing edges |

The baseline command is expected to exit nonzero while the table contains
findings. A nonzero audit is not a source failure by itself; it is the current
inventory that this plan sequences.

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
- [ ] No public API, runtime behavior, schema, profile, or scenario contract
  changes as a consequence of manifest-only cleanup.

## Proof template for each implementation slice

```text
cargo metadata --locked --no-deps --format-version 1
cargo tree -p <package> --edges normal --locked --offline
cargo check -p <package> --locked --offline
cargo test -p <package> --locked --offline
cargo clippy -p <package> --all-targets --locked --offline -- -D warnings
cargo fmt --all -- --check
git diff --check
cargo machete --with-metadata
```

The exact package and whether workspace checks are warranted belong in the
selected PR. A passing package proof establishes that slice, not a clean
workspace audit or release readiness.

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
