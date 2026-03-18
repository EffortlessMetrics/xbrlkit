# XBRLKit Maintainer Vision v2

## Core Insight

I'm a single agent playing multiple roles. Rather than spawning specialists, I **operate in modes** with clear entry/exit criteria and handoffs to "future me."

## Operating Modes

| Mode | Purpose | Entry Commands | Output | Handoff To |
|------|---------|----------------|--------|------------|
| **Scout** | Find work worth doing | `/swarm-protocol`, `/coding-standards`, `/swarm-priorities` | GitHub Issues with SLICE definitions | Builder mode |
| **Builder** | Implement in worktrees | `/swarm-protocol`, `/coding-standards`, `/swarm-priorities` | Draft PR | Reviewer mode |
| **Reviewer** | Judge alignment | `/swarm-protocol`, `/coding-standards`, `/pr-ready` | Review judgment | Ops mode (approve) or Builder mode (changes) |
| **Ops** | Merge and cleanup | `/swarm-protocol`, `/green-merge` | Merged PR, clean main | Scout mode |
| **Improver** | 20% improvements | `/swarm-protocol`, `/coding-standards`, `/swarm-priorities` | Docs, tests, devex | Reviewer mode |

## Mode Discipline

**Rules:**
1. Declare mode at session start
2. Keep 3-5 live todo items, named with actual commands
3. Complete mode checklist before switching
4. Document handoffs in GitHub (issues, PRs, comments)
5. Never merge without explicit `/green-merge`

## Entry Commands

### `/swarm-protocol`
Load context from:
- `MEMORY.md` - long-term memory
- `memory/YYYY-MM-DD.md` - recent work
- `repos/xbrlkit/` - codebase state
- Current GitHub issues/PRs

### `/coding-standards`
Load from:
- Existing codebase patterns
- `docs/` conventions
- Established crate boundaries

### `/swarm-priorities`
Current priorities (ordered):
1. All `@alpha-active` scenarios passing
2. Context parsing implementation
3. CLI usability improvements
4. Documentation completeness

### `/pr-ready`
PR is ready for review when:
- [ ] Compiles: `cargo build --workspace`
- [ ] Tests pass: `cargo xtask test-ac <AC-ID>`
- [ ] Alpha clean: `cargo xtask alpha-check`
- [ ] Clippy clean: `cargo clippy --workspace`
- [ ] <300 lines (or justified)
- [ ] One concern only
- [ ] Clear PR description

### `/green-merge`
Merge criteria:
- [ ] Reviewer mode approved
- [ ] CI passing (if enabled)
- [ ] Steven approved OR I'm confident
- [ ] No open questions

## Checklists by Mode

### Scout Mode Checklist
- [ ] Code archaeology: read relevant crates
- [ ] Identify gaps vs `@alpha-active`
- [ ] Estimate size and complexity
- [ ] Write GitHub Issue with SLICE definition
- [ ] File any discovered follow-up issues

### Builder Mode Checklist
- [ ] Create worktree: `git worktree add ../xbrlkit-<slice> -b slice/<name>`
- [ ] Write/modify code following patterns
- [ ] Run local verification
- [ ] Push draft PR with clear description
- [ ] Handoff to reviewer mode

### Reviewer Mode Checklist
- [ ] Review for ONE thing (not three bundled)
- [ ] Check <300 lines
- [ ] Verify follows patterns
- [ ] Check hexagonal boundaries
- [ ] Verify BDD coverage
- [ ] Judge: approve or request changes

### Ops Mode Checklist
- [ ] Verify `/green-merge` criteria
- [ ] Merge PR
- [ ] Delete branch
- [ ] Close Issue if complete
- [ ] Update MEMORY.md with lessons

### Improver Mode Checklist
- [ ] Identify friction from recent work
- [ ] Pick 20% improvement (docs, tests, devex)
- [ ] Keep slice small (<200 lines ideal)
- [ ] Draft PR
- [ ] Handoff to reviewer mode

## SLICE Definition Template

```markdown
## SLICE: <short-name>

**Problem:** <one sentence>
**AC-ID:** AC-XK-<AREA>-<NNN>
**Scope:** <files/crates affected>
**Size:** <estimated hours>

### Files
- `crates/<crate>/src/lib.rs` - <change>
- `specs/features/<area>/<feature>.feature` - <scenarios>
- `fixtures/...` - <test data>

### Verification
```bash
cargo xtask test-ac AC-XK-<AREA>-<NNN>
```

### Risks
- <what could go wrong>
```

## Mode Switching Rules

**When to switch:**
- Scout → Builder: Issue filed, ready to implement
- Builder → Reviewer: Draft PR pushed
- Reviewer → Ops: Approved, `/green-merge` called
- Reviewer → Builder: Changes requested
- Ops → Scout: Merge complete, looking for next work
- Any → Improver: 20% time, low-priority gap found
- Improver → Reviewer: Improvement PR ready

**Never:**
- Merge from Builder mode (must review first)
- Start building without a SLICE definition
- Mix modes in one session (finish the checklist)

## Current State

**Mode:** Reviewer (reviewing PR #10)
**Todo:**
- [ ] `cargo xtask alpha-check` on PR #10 branch
- [ ] Verify changes align with hexagonal architecture
- [ ] Check BDD coverage is complete
- [ ] Judgment: approve or request changes

## Success Metrics

**Weekly:**
- 2-4 slices merged
- Alpha-check always green on main
- One process improvement documented

**Monthly:**
- All `@alpha-active` scenarios passing
- Context parsing MVP working
- CLI usable for basic validation

**Quarterly:**
- Industry reference for XBRL/iXBRL tooling
- Clear path for EFM 78+ support

---

*This document lives in `docs/MAINTAINER_VISION.md` and evolves with each PR.*
