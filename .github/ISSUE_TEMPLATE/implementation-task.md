---
name: Implementation Task
about: Execute a planned chunk from a handoff
title: '[Implement] '
labels: ['implementation']
assignees: ''

---

## Context

- **Planning issue:** #
- **Handoff file:** `.mend/handoffs/<chunk>.md`
- **Original estimate:** 

## Pre-Flight Checklist

Before starting:
- [ ] Read handoff file completely
- [ ] Read `.mend/friction.md` for relevant pitfalls
- [ ] Verify no overlapping work in progress

## Execution

### 1. Setup
```bash
git checkout main
git pull
git checkout -b feat/<chunk>
```

### 2. Implement
Following the handoff's fix strategy:
- [ ] Step 1: 
- [ ] Step 2: 
- [ ] Step 3: 

### 3. Update Handoff

Append to `.mend/handoffs/<chunk>.md`:

```markdown
## Implementation Notes

### What Changed
<!-- Brief summary of actual changes -->

### Key Decisions Made
<!-- Any deviations from handoff strategy -->

### What to Watch For (Reviewer Briefing)
<!-- Specific areas of uncertainty -->
- 

### Test Results
```
<!-- paste test output -->
```

### Verification
- [ ] `cargo fmt --all` passes
- [ ] `cargo clippy -p <pkg> --tests -- -D warnings` passes
- [ ] `cargo test -p <pkg>` passes
- [ ] Handoff updated with reviewer briefing
```

### 4. Final Verification

```bash
cargo fmt --all
cargo clippy -p <pkg> --tests -- -D warnings
cargo test -p <pkg>
```

## Completion Checklist

- [ ] Implementation matches handoff strategy (or documents deviations)
- [ ] Handoff updated with reviewer briefing
- [ ] All verification commands pass
- [ ] Branch pushed
- [ ] PR created with `Closes #<planning-issue>`
- [ ] PR references this implementation issue

## PR Description Template

```markdown
## What
<!-- One line from handoff problem statement -->

## How
<!-- Summary of changes -->

## Verification
- [ ] Tests added/updated
- [ ] `cargo test` passes
- [ ] `cargo clippy` clean

## Handoff
Planning: #<number>
Implementation: #<this-issue>
```
