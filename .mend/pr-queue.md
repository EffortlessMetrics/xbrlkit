# xbrlkit Autonomous PR Queue

**Mission:** Use this repo to refine high-quality autonomous PR workflows. Quality > quantity. Log all friction.

## Active Queue

| # | PR | Branch | Status | Blocker | Notes |
|---|----|--------|--------|---------|-------|
| 1 | #26 | `mend/merge-dimensional-validation` | 🔄 Fixing clippy | None | Lint cleanup + doc fixes |
| 2 | - | `cleanup/remove-personal-docs` | ⏳ Pending | #26 merge | Close #14, clean house |
| 3 | - | `refactor/reorganize-agent-directories` | ⏳ Pending | #26 merge | Close #15, .kimi → .mend |
| 4 | - | `feat/activate-feature-grid` | ⏳ Pending | Review | SCN-XK-WORKFLOW-001, likely stale |
| 5 | - | `feat/activate-filing-manifest` | ⏳ Pending | Review | SCN-XK-MANIFEST-001, likely stale |

## Friction Log

### 2026-03-23: PR #26 Clippy Errors
**What happened:** Formatting passed, but clippy failed on doc markdown (QName → `QName`, is_all → `is_all`)

**Friction:** 
- Local clippy didn't catch these before push? (Need: pre-push hook)
- Multiple round trips: format → push → fail → fix → push → fail → fix
- No cargo/rust in env initially - had to install rustup mid-flow

**Fix applied:** Fixed all doc markdown issues, converted `match` to `let...else`, removed manual Default impl

**Pattern to refine:** Pre-push should run: fmt → clippy → test → alpha-check

## Workflow Iterations

### v1.0 (Current)
- Spawn per PR
- Report back on completion or block
- Manual queue tracking in this file

### v1.1 (Next)
- [ ] Pre-push checklist script (`scripts/pre-push-check.sh`)
- [ ] Automated queue progression
- [ ] Friction auto-logging to `.mend/friction/`

## Quality Gates (xbrlkit-specific)

Before ANY push:
1. `cargo fmt --all --check` passes
2. `cargo clippy --workspace --all-targets -- -D warnings` passes  
3. `cargo test --workspace` passes
4. `cargo xtask alpha-check` passes

Merge authority: CI green = merge. No human gate.
