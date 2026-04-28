# Quality Final Review Micro-Agent

## Purpose

Final quality gate before tests phase. Ensures all quality checks are coherent, documented, and the PR is ready for test execution.

## Entry Criteria (MUST have)
- `quality-clippy-passed` label
- `quality-docs-passed` label  
- `quality-patterns-passed` label

## Exit Criteria
- Either `quality-final-passed` (success) or `quality-final-failed` (blocking issues found)

## Review Checklist

### 1. Quality Coherence Check
- [ ] All clippy warnings resolved (or explicitly suppressed with justification)
- [ ] Documentation is complete for public APIs
- [ ] Code patterns follow project conventions
- [ ] No conflicting quality labels present

### 2. Pre-Test Readiness
- [ ] `cargo check --workspace` passes
- [ ] `cargo clippy --workspace --all-targets` passes
- [ ] No broken intra-doc links (`cargo doc --no-deps`)
- [ ] Formatter clean (`cargo fmt --check`)

### 3. PR State Validation
- [ ] PR has clear description
- [ ] Linked to relevant issue(s) if applicable
- [ ] No `changes-requested` label blocking (or has been re-reviewed)

## Procedure

```bash
# 1. Fetch PR
cd /root/.openclaw/xbrlkit
gh pr checkout 138

# 2. Run final quality checks
cargo check --workspace --all-targets 2>&1 | tee /tmp/quality-final-check.log
cargo clippy --workspace --all-targets -- -D warnings 2>&1 | tee /tmp/quality-final-clippy.log
cargo doc --workspace --no-deps 2>&1 | tee /tmp/quality-final-doc.log
cargo fmt --check 2>&1 | tee /tmp/quality-final-fmt.log

# 3. Analyze results
# If all pass → PASS
# If any fail → FAIL with details
```

## Decision Matrix

| Check | Pass | Fail |
|-------|------|------|
| cargo check | ✓ | Document errors, request fix |
| cargo clippy | ✓ | Document warnings, request fix |
| cargo doc | ✓ | Document link errors |
| cargo fmt | ✓ | Auto-fix or request fix |

## On Success

1. Add label: `quality-final-passed`
2. Add label: `quality-passed` (aggregate)
3. Add label: `tests-unit` (next phase)
4. Remove label: `review-in-progress`
5. Comment summary:

```markdown
## ✅ Quality Final Review Passed

All quality gates cleared:
- [x] Clippy clean
- [x] Documentation complete
- [x] Patterns consistent
- [x] Formatting clean

**Next:** Ready for `tests-unit` phase.
```

## On Failure

1. Add label: `quality-final-failed`
2. Keep blocking labels in place
3. Remove label: `review-in-progress`
4. Comment with specific issues:

```markdown
## ❌ Quality Final Review Failed

Issues found:
- [ ] `cargo check`: [errors]
- [ ] `cargo clippy`: [warnings]
- [ ] `cargo doc`: [broken links]
- [ ] `cargo fmt`: [formatting issues]

**Action Required:** Please fix the above issues before proceeding.
```

## Time Budget

**MAX 5 minutes** - This is a final gate, not a deep review. If checks take longer, something is wrong.

## Reminder

This is PR #138. Be specific in comments with file paths and line numbers where applicable.
