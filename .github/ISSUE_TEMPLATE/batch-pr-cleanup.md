---
name: Batch PR Cleanup
about: Track cleanup of multiple related PRs
title: '[Batch] '
labels: ['batch-cleanup']
assignees: ''

---

## PR Inventory

| PR | Branch | Current State | Next Action |
|----|--------|---------------|-------------|
|  |  |  |  |

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

## Checklist

- [ ] Full inventory: `gh pr list --state open`
- [ ] Read each PR diff (not just files)
- [ ] Apply status labels based on actual state
- [ ] Identify review iteration cycles
- [ ] Plan merge order
- [ ] Spawn worktrees for mechanical rebases
- [ ] Review before merge
- [ ] Update golden files if needed
- [ ] Verify alpha-check passes
- [ ] Merge and delete branches
- [ ] Update this issue as state changes

## Notes

