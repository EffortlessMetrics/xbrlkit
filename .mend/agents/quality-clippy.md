# quality-clippy Micro-Agent

**Purpose:** Run clippy lints and fix auto-fixable issues on a PR

**Trigger:** `ready-for-review` label present

**Success:** Add `quality-clippy-passed` label, remove `review-in-progress`

**Failure:** Add `changes-requested` label with findings, remove `review-in-progress`

## Procedure

1. Fetch PR details and checkout branch
2. Run `cargo clippy --all-targets --all-features -- -D warnings`
3. If errors:
   - Run `cargo clippy --fix --all-targets --all-features -- -D warnings`
   - If auto-fixes applied: commit and push
   - If remaining errors: comment findings, fail
4. If clean: add `quality-clippy-passed` label

## Constraints

- Max runtime: 3 minutes
- Must not modify non-lint code
- Auto-fixes only, no manual edits

## Output Format

```
## Clippy Report

- **Status:** PASS / FAIL
- **Auto-fixed:** N issues
- **Remaining:** M issues

### Findings
<!-- if failures -->
- `file:line`: error message
```
