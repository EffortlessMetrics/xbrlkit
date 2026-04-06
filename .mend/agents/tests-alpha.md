# Micro-Agent: tests-alpha

**Purpose:** Run alpha-check validation on PR — lightweight, focused, 2-5 min max.

**Trigger:** PR has `tests-bdd-passed` label, no `changes-requested`.

**Entry:** `review-in-progress` label present (set by scheduler).

---

## Quick Validation (in order)

### 1. Checkout PR
```bash
cd /root/.openclaw/xbrlkit
git fetch origin pull/{pr_number}/head:pr-{pr_number}
git checkout pr-{pr_number}
```

### 2. Alpha-Check (The Gate)
```bash
cargo xtask alpha-check
```
- Must pass: All @alpha-active scenarios execute
- Must pass: No step handler panics
- Must pass: Feature grid validates

### 3. Quick Test Sanity
```bash
cargo test --workspace --quiet 2>&1 | tail -20
```
- Verify no test failures

---

## Decision

### PASS (alpha-check succeeds)
```bash
gh pr edit {pr_number} --remove-label "review-in-progress" --add-label "tests-alpha-passed"
```

### FAIL (alpha-check fails)
```bash
gh pr edit {pr_number} --remove-label "review-in-progress" --add-label "tests-alpha-failed"
gh pr comment {pr_number} --body "## 🤖 tests-alpha: FAILED

Alpha-check failed. Run \`cargo xtask alpha-check\` locally to reproduce.

$(cargo xtask alpha-check 2>&1 | tail -50)
"
```

---

## Signoff
- Alpha-check passes → `tests-alpha-passed` label added
- Alpha-check fails → `tests-alpha-failed` label added, comment posted
- Always removes `review-in-progress`

---
*Micro-agent for xbrlkit review pipeline*
