# tests-unit Micro-Agent

## Purpose
Run unit tests across modified crates and verify all pass.

## Trigger Label
- **Input:** `quality-final-passed`
- **Output:** `tests-unit-passed` OR `tests-unit-failed`

## Scope
Execute unit tests for crates affected by the PR.

## Commands

```bash
# Navigate to repo
cd /root/.openclaw/xbrlkit

# Run cargo xtask doctor first
cargo xtask doctor

# Get list of changed files from PR
CHANGED_FILES=$(gh pr view <PR_NUMBER> --json files --jq '.files[].path')

# Identify affected crates
AFFECTED_CRATES=$(echo "$CHANGED_FILES" | grep -E '^crates/' | cut -d'/' -f2 | sort -u)

# Run unit tests for affected crates
for crate in $AFFECTED_CRATES; do
    echo "Testing crate: $crate"
    cargo test -p "$crate" --lib 2>&1 | tee "/tmp/test-${crate}.log"
done

# If no specific crates affected, run all unit tests
cargo test --lib 2>&1 | tee /tmp/test-all.log
```

## Review Criteria

| Check | Pass Criteria |
|-------|---------------|
| All tests pass | 0 test failures |
| No compilation errors | Clean build |
| Test coverage | Tests exist for new code |

## Label Transition

On success:
```bash
gh pr edit <PR_NUMBER> --repo EffortlessMetrics/xbrlkit --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --repo EffortlessMetrics/xbrlkit --add-label "tests-unit-passed"
```

On failure:
```bash
gh pr edit <PR_NUMBER> --repo EffortlessMetrics/xbrlkit --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --repo EffortlessMetrics/xbrlkit --add-label "tests-unit-passed"
gh pr comment <PR_NUMBER> --repo EffortlessMetrics/xbrlkit --body "## Unit Test Review Failed

Test failures detected. Please fix and re-trigger."
```
