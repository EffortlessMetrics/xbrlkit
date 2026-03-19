---
name: Batch PR Cleanup
about: Track cleanup of multiple related PRs
title: '[Batch] '
labels: ['batch-cleanup']
assignees: ''

---

## PR Inventory

| PR | Branch | Current State | Next Action | Handoff | Owner |
|----|--------|---------------|-------------|---------|-------|
|  |  |  |  | `.mend/handoffs/pr-<n>.md` |  |

## State Definitions

```
[needs-rebase] → [ready-for-review] → [needs-review] 
                      ↑                    ↓
               [implementing] ← [reviewed-needs-work]
                      ↑                    ↓
               [ready-for-review] → [ready-to-merge] → [MERGED]
```

- `status/needs-rebase` — Behind main or has conflicts
- `status/ready-for-review` — Ready for human review pass
- `status/needs-review` — Awaiting reviewer assignment
- `status/reviewed-needs-work` — Reviewed, changes requested
- `status/implementing` — Author addressing feedback
- `status/ready-to-merge` — Green CI, approved, ready
- `status/blocked-on-X` — Waiting for dependency
- `status/abandoned` — Superseded, close me

## Dependencies

```
PR #X → PR #Y → PR #Z
```

## Handoff Pattern

For each PR requiring work, create `.mend/handoffs/pr-<n>.md`:

```markdown
# Handoff: PR #<n>

## PR Details
- Number: #<n>
- Branch: 
- Current state: 

## What I Found
<!-- Read the diff, what changed -->

## Conflicts/Blockers
- [ ] File: `path/to/file.rs` — conflict with main
- [ ] Alpha check failing: 

## Fix Strategy
1. Rebase: `git rebase main`
2. Resolve: <specific files>
3. Verify: `cargo test && cargo xtask alpha-check`

## Minimal Spawn Prompt
```
Read .mend/handoffs/pr-<n>.md
Read .mend/friction.md for rebase patterns
Branch: <branch>
Files: <list>
Verify: cargo test && cargo xtask alpha-check
Update handoff with results.
```

## Status
- [ ] Handoff written
- [ ] Work spawned/isolated
- [ ] Conflicts resolved
- [ ] Tests pass
- [ ] Ready for review
- [ ] Merged
```

## Checklist

- [ ] Full inventory: `gh pr list --state open`
- [ ] Create handoff for each PR needing work
- [ ] Read each PR diff (not just files)
- [ ] Apply status labels based on actual state
- [ ] Identify review iteration cycles
- [ ] Plan merge order respecting dependencies
- [ ] Spawn worktrees for mechanical rebases
- [ ] Review before merge (not after)
- [ ] Update golden files if needed (deterministic = spawn)
- [ ] Verify alpha-check passes
- [ ] Merge and delete branches
- [ ] Close this issue when complete

## Progress Log

| Time | Action | PR | Notes |
|------|--------|-----|-------|
|  |  |  |  |

## Notes

<!-- Friction discovered, patterns observed -->
