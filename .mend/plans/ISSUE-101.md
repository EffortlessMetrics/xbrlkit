# Implementation Plan: Legacy PR Cleanup — #11-15

**Issue:** #101  
**Created:** 2026-03-28  
**Planner:** planner-initial agent

---

## Overview

This issue tracks the systematic cleanup of 5 legacy pull requests (#11-15) that have been open for an extended period in the xbrlkit repository. The goal is to review each PR, determine if the changes are still relevant, and take appropriate action (close or rebase) to maintain a clean and manageable codebase.

This is a maintenance task that requires human decision-making for each PR based on:
- Whether the changes are obsolete or already implemented elsewhere
- Whether the changes are still relevant but need conflict resolution

---

## Acceptance Criteria Breakdown

| # | Criterion | Status |
|---|-----------|--------|
| 1 | Review PR #11 and document decision (close/rebase) | ⬜ Pending |
| 2 | Review PR #12 and document decision (close/rebase) | ⬜ Pending |
| 3 | Review PR #13 and document decision (close/rebase) | ⬜ Pending |
| 4 | Review PR #14 and document decision (close/rebase) | ⬜ Pending |
| 5 | Review PR #15 and document decision (close/rebase) | ⬜ Pending |
| 6 | Execute close action for PRs marked "close" | ⬜ Pending |
| 7 | Execute rebase action for PRs marked "rebase" | ⬜ Pending |
| 8 | Update issue #101 with final summary | ⬜ Pending |

---

## Proposed Approach

### Phase 1: Discovery & Analysis
1. Fetch details for each PR (#11-15) using GitHub CLI
2. Analyze each PR's:
   - Title and description
   - Last update date
   - Branch status (conflicts with main?)
   - Files changed
   - Commit history
   - Any existing comments or reviews

### Phase 2: Decision Documentation
1. Create a decision matrix documenting findings for each PR
2. Provide recommendation (close or rebase) with justification
3. Request human review for final decisions

### Phase 3: Execution
1. For PRs marked "close":
   - Add comment explaining reason for closure
   - Close the PR
2. For PRs marked "rebase":
   - Add comment with rebase instructions
   - Tag relevant contributors

### Phase 4: Summary
1. Update issue #101 with final summary of actions taken
2. Close issue #101 upon completion

---

## Files to Modify/Create

### New Files
- `.mend/plans/ISSUE-101.md` (this document) ✓ Created
- `.mend/work/ISSUE-101/decision-matrix.md` (decision documentation)
- `.mend/work/ISSUE-101/execution-log.md` (execution tracking)

### Modified Files
- None (this is a repository maintenance task, not code changes)

---

## Test Strategy

This is a repository maintenance task without code changes. Validation will be:

1. **Pre-execution verification:**
   - Confirm all 5 PRs are accessible
   - Verify permissions to comment/close PRs

2. **Post-execution verification:**
   - Confirm closed PRs are properly closed with explanatory comments
   - Confirm rebase PRs have clear instructions for contributors
   - Verify issue #101 is updated with final summary

3. **Human review gates:**
   - Decision matrix requires human approval before execution
   - Final summary reviewed before closing issue #101

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| PR contains valuable work accidentally closed | Low | High | Document decisions in decision matrix; require human review |
| PR close/rebase conflicts with active work | Low | Medium | Check recent activity on PRs before action |
| Permission issues preventing PR closure | Low | Medium | Verify GitHub token permissions early |
| Contributor disagreement with close decision | Medium | Low | Clear communication in closure comments |

**Overall Risk Level:** Low-Medium

---

## Estimated Effort

| Phase | Estimated Time |
|-------|----------------|
| Discovery & Analysis | 30 minutes |
| Decision Documentation | 20 minutes |
| Human Review Wait | Variable |
| Execution | 15 minutes |
| Summary & Close | 10 minutes |
| **Total (active work)** | **~75 minutes** |

---

## Notes

- This issue requires human decision-making and cannot be fully automated
- The autonomous workflow can assist with analysis and documentation, but final decisions should be human-reviewed
- Consider establishing a PR stale-bot policy to prevent future accumulation of legacy PRs
