# Agentic SDLC Workflow — xbrlkit

## Philosophy
Every PR goes through multiple agent passes until quality gate satisfied. No merge without explicit agent signoff.

## Workflow Stages

### 1. Branch Creation (Tiny Slices)
- Max 1-2 ACs per branch
- Clear scope in branch name: `feat/SCN-XK-TAX-LOAD-001-002`
- Pre-push: alpha-check must pass locally

### 2. CI Gate (Automated)
- GitHub Actions: build, test, clippy, fmt
- Alpha-check: 33 scenarios must pass
- **Output**: ✅ Green or ❌ Fail (blocks review)

### 3. Agent Review Gate (Multi-Pass)
Spawned agents evaluate:

| Pass | Focus | Agent | Signoff Criteria |
|------|-------|-------|------------------|
| 1 | Code quality | `reviewer-quality` | No clippy warnings, idiomatic Rust, proper error handling |
| 2 | Test coverage | `reviewer-tests` | All new code tested, BDD steps match feature file |
| 3 | Architecture | `reviewer-arch` | Fits crate boundaries, no circular deps, ADR if needed |
| 4 | Integration | `reviewer-integ` | Alpha-check passes, no golden drift, fixtures valid |

**Bounce conditions**: Any pass can request changes and bounce PR back to author (me).

### 4. Human Review Gate (Your Call)
- PR labeled `in-review`
- You approve or request changes
- If changes needed → bounce to author

### 5. Merge Gate (Agent Final Signoff)
**Trigger options:**
1. **GitHub webhook** — When CI green + all review labels present
2. **Cron poll** — Regular check for ready-to-merge PRs

**Agent actions:**
- Verify all prior gates passed (labels present)
- Verify human approval
- Final verification: squash commit message, changelog entry, version bump
- **Agent comments**: "🤖 AGENTIC MERGE APPROVED"
- **Then and only then**: execute merge with squash, delete branch

**Note:** This is explicit agent-controlled merge, not auto-merge. The merge agent performs final checks before the merge operation.

## Cron Jobs

### `xbrlkit-review-scheduler` (Every 15 min)
```
For each open PR:
  If CI green AND no `review-in-progress` label:
    If missing quality review → spawn reviewer-quality
    If missing test review → spawn reviewer-tests  
    If missing arch review → spawn reviewer-arch
    If missing integ review → spawn reviewer-integ
  
  If all reviews passed AND human approved AND not yet merged:
    → spawn merger-final-signoff
```

### `xbrlkit-merge-trigger` (Alternative: GitHub webhook)
```
On CI completion (success) OR label change:
  If all review labels present AND human approved:
    → spawn merger-final-signoff immediately
```

### `xbrlkit-tree-cleanup` (Every 6 hours)
```
For each working tree:
  If branch merged to main → delete local branch
  If branch stale (>7 days) → mark for decision
  If uncommitted changes → commit with timestamp or stash
  Run `git gc` for housekeeping
```

### `xbrlkit-memory-log` (On every agent session end)
```
Append to .mend/session-log.md:
  - Timestamp
  - Session type (review/author/merge)
  - PR/issue worked on
  - Actions taken
  - Blockers encountered
  - Learning/insights
```

### `xbrlkit-retrospective` (Weekly)
```
Spawn retrospective agent:
  - Read .mend/session-log.md
  - Identify friction patterns
  - Suggest workflow improvements
  - Create ADR if architectural lesson learned
  - Update this workflow doc if process improvement found
```

## Agent Definitions
See `.mend/agents/` directory:
- `reviewer-quality.md`
- `reviewer-tests.md`
- `reviewer-arch.md`
- `reviewer-integ.md`
- `merger-final-signoff.md`

## Labels

| Label | Meaning | Who sets |
|-------|---------|----------|
| `wip` | Work in progress, not ready | Author |
| `ready-for-review` | CI green, author done | Author |
| `review-in-progress` | Agent currently reviewing | Scheduler |
| `quality-passed` | Quality review complete | reviewer-quality |
| `tests-passed` | Test review complete | reviewer-tests |
| `arch-passed` | Architecture review complete | reviewer-arch |
| `integ-passed` | Integration review complete | reviewer-integ |
| `changes-requested` | Bounced for revision | Any reviewer |
| `in-review` | Ready for human review | Agent when all passed |
| `agent-merge-approved` | Final signoff complete | merger-final-signoff |

## Safety Mechanisms

1. **No force push to main** — Already enforced by branch protection
2. **No auto-merge** — Explicit agent signoff required per your instruction
3. **Working tree isolation** — Each agent gets clean checkout
4. **Memory logging** — Every session logged for audit trail
5. **Bounce limit** — After 3 bounces, escalate to human
6. **Stale detection** — PRs >7 days without activity flagged

## Implementation Phases

### Phase 1 (Now)
- [x] Create agent definition files in `.mend/agents/`
- [ ] Set up review-scheduler cron (disabled, dry-run mode)
- [ ] Create label set via `gh label create`
- [ ] Test on PR #97

### Phase 2 (After validation)
- [ ] Enable scheduler for real
- [ ] Add tree-cleanup cron
- [ ] Add memory-log hook
- [ ] Document in ADR-008

### Phase 3 (Optimization)
- [ ] Weekly retrospective cron
- [ ] Parallel review passes (if safe)
- [ ] Review time estimation

---
*Draft workflow — pending your refinement*
