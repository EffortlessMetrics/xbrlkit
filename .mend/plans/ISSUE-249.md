# Plan: cargo-machete Dependency Audit (Issue #249)

## Overview

This plan addresses Issue #249: running a `cargo-machete` dependency audit across the xbrlkit workspace to identify and remove unused dependencies, reducing compile times and binary sizes.

The 2026-04-22 friction scan could not run `cargo machete` (not installed). This plan scopes the work to install the tool, run it, act on findings, and optionally integrate it into CI.

---

## Acceptance Criteria Breakdown

### AC-1: Install `cargo-machete`
- **Given**: The scan environment has a Rust toolchain
- **When**: `cargo install cargo-machete` is run
- **Then**: The `cargo machete` command is available

### AC-2: Run `cargo machete` across workspace
- **Given**: The tool is installed and the workspace is clean
- **When**: `cargo machete` runs from repo root
- **Then**: It scans all workspace members and produces a report of potential unused dependencies

### AC-3: Remove confirmed unused dependencies
- **Given**: The machete report identifies candidates
- **When**: Each candidate is verified (not used in code, not a dev-dependency edge case, not intentionally kept for feature flags)
- **Then**: Unused deps are removed from `Cargo.toml` files; CI passes after removal

### AC-4 (Optional): Add `cargo machete` to CI
- **Given**: The audit is clean
- **When**: A new CI job runs `cargo machete` on every PR
- **Then**: PRs introducing unused dependencies fail fast

---

## Preliminary Findings (from manual inspection)

| Crate | Status | Notes |
|-------|--------|-------|
| `archive-zip` | **Likely unused** | Only declared in workspace `Cargo.toml` and its own `Cargo.toml`. Zero Rust source references. Stub crate (`open_zip()` → `Ok(())`). |
| `cockpit-export` | **Actually used** | Referenced by `xtask` (`cockpit_pack`) and `xbrlkit-bdd-steps`. Machete may still flag it if symbols aren't imported in certain crates, but code review confirms usage. |

> **Important**: Manual inspection is a heuristic. `cargo machete` may surface additional unused dependencies not listed in the issue. All findings must be confirmed before removal.

---

## Proposed Approach

### Phase 1: Tool Installation & Scan
1. Install `cargo-machete`: `cargo install cargo-machete` (or `cargo binstall cargo-machete` if preferred).
2. Run `cargo machete` from repo root.
3. Capture full output into `.kimi/friction/machete-report.md`.

### Phase 2: Triage & Verification
For each reported unused dependency:
1. **Cross-reference**: `grep -r "crate_name" --include="*.rs"` to confirm no source usage.
2. **Check `dev-dependencies`**: Machete can have false positives with dev-dependencies (e.g., test-only crates). Verify against `Cargo.toml` sections.
3. **Check feature-gated usage**: Some deps are pulled in only under specific features. Review `[features]` tables.
4. **Check build-dependencies / proc-macros**: Machete may not account for all edge cases.

### Phase 3: Removal
1. Remove confirmed unused entries from individual crate `Cargo.toml` files.
2. If a workspace-local crate becomes unreferenced entirely (like `archive-zip`), consider removing the crate directory **or** completing its implementation (out of scope for this issue; defer to follow-up).
3. Run `cargo check --workspace` to ensure no hidden transitive breakage.
4. Run `cargo test --workspace` to confirm nothing was a dev-only dependency in disguise.

### Phase 4: CI Integration (Optional)
1. Add a `machete` job to `.github/workflows/ci.yml`:
   ```yaml
   machete:
     runs-on: ubuntu-latest
     steps:
       - uses: actions/checkout@v4
       - uses: dtolnay/rust-toolchain@1.92.0
       - uses: Swatinem/rust-cache@v2
       - run: cargo install cargo-machete
       - run: cargo machete
   ```
2. Alternatively use `cargo-binstall` in CI for faster install, or cache the binary.

---

## Files to Modify/Create

### New Files (1)
1. `.kimi/friction/machete-report.md` — Raw scan output for traceability

### Modified Files (variable count; depends on scan)
- Individual `crates/*/Cargo.toml` files — remove unused `[dependencies]` entries
- `Cargo.toml` (workspace root) — remove unused workspace dependency declarations if a dep is dropped entirely
- `.github/workflows/ci.yml` — add machete job (optional AC-4)

### Estimated File Count
- **New**: 1 (report)
- **Modified**: 2–10 `Cargo.toml` files (heuristic based on workspace size)

---

## Test Strategy

| Gate | Command |
|------|---------|
| Workspace compiles | `cargo check --workspace` |
| Tests pass | `cargo test --workspace` |
| Alpha check | `cargo xtask alpha-check` |
| Package check | `cargo xtask package-check` |
| Format & lints | `cargo fmt --check` / `cargo clippy --workspace --all-targets -- -D warnings` |

All gates must pass after dependency removals.

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Machete false positive (dev-only or feature-gated dep) | Medium | Medium | Manual `grep` verification before any removal |
| Removing a dep that is used indirectly (e.g., via macro expansion) | Low | High | `cargo check --workspace` catches most cases |
| `cargo-machete` install failure in scan env | Low | Low | Try `cargo binstall` or pin version |
| Workspace-local crate (`archive-zip`) is a placeholder for future work | Medium | Low | Confirm with maintainer; do not delete crate directory without explicit go-ahead |
| CI job adds latency | Low | Low | Use caching; job is fast (~seconds) after initial setup |

---

## Estimated Effort

| Task | Estimate |
|------|----------|
| Install `cargo-machete` and run scan | 15 min |
| Triage: verify each reported unused dep | 1–2h |
| Remove confirmed unused deps | 30 min |
| Run full CI gate (`check`, `test`, `clippy`, `alpha-check`) | 30 min |
| Add optional CI job + test in PR | 30 min |
| **Total** | **~3–4 hours** |

---

## Implementation Notes

- **Do not delete crate directories** (e.g., `crates/archive-zip/`). Only remove dependency entries from `Cargo.toml` files. Deleting a crate is a separate architectural decision.
- If `archive-zip` is confirmed unused and removed from workspace deps, its crate directory will no longer be built. This is acceptable and reduces compile time.
- Keep the machete report in `.kimi/friction/` for audit trail.

### Next Steps After Plan Approval
1. Run `cargo machete` and capture output
2. Triage findings, verify each candidate
3. Open PR with removals
4. Optionally add CI job in same PR or follow-up

## Deep Review (reviewer-deep-plan)

### Independent Assessment: PASS

#### Edge Cases Examined
- **Workspace dependency declared at root, used in one crate**: If machete flags a workspace-level dep, verify whether it should move to the individual crate's `[dependencies]` rather than be removed entirely.
- **Renamed dependencies (`package = "..."`)**: Machete may struggle with `dep = { package = "other-name" }` patterns. Verify with `grep` against the actual import name used in source.
- **Optional dependencies (`optional = true`)**: These are often feature-gated. A dep flagged as unused may simply need its feature name verified in `[features]` tables.
- **Example/bench-only dependencies**: Deps used exclusively in `examples/` or `benches/` may be flagged. Run `cargo check --workspace --examples --benches` as a separate gate.
- **Build-dependencies / proc-macros**: Machete can flag deps needed at build time or macro-only usage. Verify `build.rs` and macro expansion before removal.

#### Risk Assessment
| Risk | Severity | Mitigation |
|------|----------|------------|
| False positive on re-exported / trait-only usage | Medium | Manual `grep` + `cargo check --workspace --all-features` |
| Removing a dep that is a placeholder for future work | Low | Plan explicitly defers crate directory deletion; Git history preserves removed entries |
| `cargo install cargo-machete` in CI adds 2–3 min per run | Medium | Prefer `taiki-e/install-action@cargo-machete` or `cargo-binstall` over bare `cargo install` |
| Unpinned machete version breaks CI on upstream changes | Medium | Pin with `--version X.Y.Z` or `--locked` flag in CI |
| `Cargo.lock` drift after dep removal | Low | Include `Cargo.lock` in PR; run `cargo update -p <removed>` or let cargo resolve naturally |

#### Alternatives Evaluated
- **`cargo-udeps`**: Requires nightly Rust toolchain. Slower to run. Rejected for this workspace.
- **`cargo-shear`**: Newer and faster than machete, but less established. Machete is the right choice for now; can migrate later if needed.
- **Pre-commit hook vs CI gate**: CI-only is correct — machete is workspace-level, not per-commit.
- **`taiki-e/install-action` vs `cargo install`**: Install-action is ~10× faster in CI. Recommended but not blocking.

#### Additional Watch Items
1. Run `cargo check --workspace --all-features` and `cargo check --workspace --examples --benches` after removals — catches feature-gated and example-only deps.
2. Pin `cargo-machete` version in CI for reproducibility.
3. If a workspace dep moves from root `Cargo.toml` to a member crate, update the member's `[dependencies]` to use `dep = "version"` or `dep = { workspace = true }` as appropriate.
4. Re-run `cargo machete` post-removal to confirm clean scan.

---
*Plan created by planner-initial agent*
