# Agentic SDLC Workflow — xbrlkit

## Philosophy
Every PR goes through multiple agent passes until quality gate satisfied. No merge without explicit agent signoff. Fully agentic — no human review gates.

## Workflow Stages

### 1. Branch Creation (Tiny Slices)
- Max 1-2 ACs per branch
- Clear scope in branch name: `feat/SCN-XK-TAX-LOAD-001-002`
- Pre-push: alpha-check must pass locally

### 2. CI Gate (Automated)
- GitHub Actions: build, test, clippy, fmt
- Alpha-check: 33 scenarios must pass
- **Output**: ✅ Green or ❌ Fail (blocks review)

### 3. Agent Review Gate (Multi-Pass)\n| Pass | Agent | Focus | Signoff Criteria |
|------|-------|-------|------------------|
| 1 | `reviewer-quality` | Code quality | No clippy warnings, idiomatic Rust, proper error handling |
| 2 | `reviewer-tests` | Test coverage | All new code tested, BDD steps match feature file |
| 3 | `reviewer-arch` | Architecture | Fits crate boundaries, no circular deps, ADR if needed |
| 4 | `reviewer-integ` | Integration | Alpha-check passes, no golden drift, fixtures valid |
| 5 | `reviewer-agentic` | Agentic review + CI | Comprehensive review, all prior gates verified |
| 6 | `reviewer-deep` | Deep improvements | Final cleanup, optimizations, edge cases |
| 7 | `maintainer-alignment` | Maintainer review | Code direction, strategic alignment, final checks |
| 8 | `merger-final` | Merge approval | Verify all gates, execute merge |

**Bounce conditions**: Any pass can request changes and bounce PR back to author.

## Agent Details

### reviewer-quality
- Strict clippy (pedantic level)
- Idiomatic Rust patterns
- Error handling (no unwrap in lib code)
- Documentation coverage

### reviewer-tests  
- BDD step alignment
- Unit test coverage
- Edge case handling
- Fixture validity

### reviewer-arch
- Crate boundaries
- No circular deps
- ADR requirements
- Breaking change assessment

### reviewer-integ
- Alpha-check passes
- No golden drift
- Clean build
- Artifact verification

### reviewer-agentic (NEW)
**Trigger**: After integ-passed + CI green

**Scope**:
- Comprehensive cross-cutting review
- Verify all prior gates actually passed
- Check for gaps between review passes
- Ensure CI is still green (re-run if needed)
- Validate PR description matches changes
- Check for security concerns

**Signoff**: `agentic-passed` label

### reviewer-deep (NEW)
**Trigger**: After agentic-passed

**Scope**:
- Final improvements and cleanup
- Performance optimizations
- Edge case hardening
- Documentation polish
- Comment quality
- Naming refinements
- Remove any remaining TODOs/FIXMEs

**Signoff**: `deep-passed` label

### maintainer-alignment (NEW)
**Trigger**: After deep-passed

**Scope**:
- Code direction alignment
- Strategic fit assessment
- Architecture consistency with roadmap
- API surface review
- Breaking change approval
- Deprecation handling

**Signoff**: `maintainer-approved` label

### merger-final
**Trigger**: After maintainer-approved

**Scope**:
- Verify all gate labels present
- Final CI check
- Squash commit message format
- CHANGELOG.md updated
- Execute merge with squash
- Delete branch

**Signoff**: `agent-merge-approved` label + merge commit

## Cron Jobs

### `xbrlkit-review-scheduler` (Every 15 min)
```
For each open PR:
  If CI green AND no `review-in-progress`:
    Spawn next required agent based on missing labels:
      - no quality-passed → reviewer-quality
      - quality but no tests → reviewer-tests
      - tests but no arch → reviewer-arch
      - arch but no integ → reviewer-integ
      - integ but no agentic → reviewer-agentic
      - agentic but no deep → reviewer-deep
      - deep but no maintainer → maintainer-alignment
      - maintainer-approved → merger-final
```

### `xbrlkit-tree-cleanup` (Every 6 hours)
```
Clean merged branches, stash uncommitted, git gc
```

## Labels

| Label | Meaning | Who sets |
|-------|---------|----------|
| `wip` | Work in progress | Author |
| `ready-for-review` | CI green, ready for agents | Author |
| `review-in-progress` | Agent currently reviewing | Scheduler |
| `quality-passed` | Quality review complete | reviewer-quality |
| `tests-passed` | Test review complete | reviewer-tests |
| `arch-passed` | Architecture review complete | reviewer-arch |
| `integ-passed` | Integration review complete | reviewer-integ |
| `agentic-passed` | Agentic review complete | reviewer-agentic |
| `deep-passed` | Deep improvements complete | reviewer-deep |
| `maintainer-approved` | Maintainer alignment complete | maintainer-alignment |
| `changes-requested` | Bounced for revision | Any reviewer |
| `agent-merge-approved` | Merge complete | merger-final |

## Safety

1. **No force push to main**
2. **No human merge** — Only merger-final agent executes merge
3. **8 agent gates** — Quality → Tests → Arch → Integ → Agentic → Deep → Maintainer → Merge
4. **Bounce limit** — After 3 bounces, escalate to human
5. **Audit trail** — Every agent action logged

## Implementation

### Phase 1 (Done)
- [x] Base reviewer agents (quality, tests, arch, integ)
- [x] Labels created
- [x] Cron jobs defined

### Phase 2 (Now)
- [ ] Create `reviewer-agentic.md` agent definition
- [ ] Create `reviewer-deep.md` agent definition
- [ ] Create `maintainer-alignment.md` agent definition
- [ ] Update `merger-final-signoff.md` with new trigger logic
- [ ] Enable scheduler

### Phase 3 (Test)
- [ ] Run full pipeline on PR #103
- [ ] Validate bounce behavior
- [ ] Measure review latency

---
*Fully agentic workflow — no human review gates*
