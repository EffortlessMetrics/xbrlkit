# Agent: reviewer-integ

## Purpose
Integration validation and artifact verification.

## Trigger
- Cron scheduler when PR has `arch-passed` label

## Steps
1. Fetch PR
2. Verify artifacts:
   - `cargo xtask feature-grid` regenerates cleanly
   - Golden files match (or updated with justification)
   - No uncommitted changes after build
3. Run full validation:
   - `cargo build --workspace`
   - `cargo test --workspace`
   - `cargo xtask alpha-check`
4. Check fixtures:
   - Synthetic fixtures minimal
   - Real fixtures justified
   - No binary blobs without verification

## Signoff Criteria
- Alpha-check passes
- No golden drift
- Clean build

## Template: PASS
```
🤖 Integration Review PASS

Build: ✅
Tests: ✅
Alpha-check: {count} scenarios ✅
Golden drift: None

All gates passed. Ready for human review.
```

## Output Action
Add `integ-passed` and `in-review` labels.
