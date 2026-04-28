# quality-docs Micro-Agent

**Purpose:** Verify documentation completeness for public items in changed crates

**Trigger:** `quality-clippy-passed` label present

**Success:** Add `quality-docs-passed` label, remove `review-in-progress`

**Failure:** Add `quality-docs-failed` label with findings, remove `review-in-progress`

## Procedure

1. Fetch PR details and checkout branch
2. Identify changed crates: `git diff --name-only origin/main | grep "^crates/" | cut -d/ -f2 | sort -u`
3. For each changed crate:
   - Run `cargo doc --no-deps 2>&1 | grep -E "^warning:.*missing"`
   - Check for `#![warn(missing_docs)]` in `lib.rs`
4. Check for undocumented public items:
   - `cargo rustdoc -- --warn-missing-docs 2>&1 | grep -E "missing_docs"`
5. Verify module-level docs exist for changed modules

## Constraints

- Max runtime: 2 minutes
- Read-only checks, do not modify code
- Focus on changed files only

## Output Format

```markdown
## Docs Review Report

- **Status:** PASS / FAIL
- **Crates Checked:** {list}

### Findings
<!-- if failures -->
- `crate::module::Item`: Missing doc comment
- `crate`: Missing `#![warn(missing_docs)]` attribute

### Stats
- Public items: N
- Documented: M
- Coverage: X%
```

## Signoff

- **PASS**: All public items documented, module docs present
- **FAIL**: Missing docs detected, report to PR
