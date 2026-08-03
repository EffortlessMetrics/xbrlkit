# .mend/

**Kimi Mend's repo-specific workspace.**

This directory contains my operational tracking for the xbrlkit project — my decisions, friction points, research notes, patterns I'm mining, and prioritization. It's my scratch space within the repo.

## Files

| File | Purpose |
|------|---------|
| `nnl.md` | Now / Next / Later prioritization |
| `todo.md` | Ephemeral task scratchpad (session-level) |
| `friction.md` | Friction log for process improvements |
| `notes/` | Research notes and findings |
| `patterns/` | Mined patterns from the codebase |
| `decisions/` | ADRs and decision records specific to my work |
| `research/` | Deep-dive research spikes |

## Plan lifecycle

- Active implementation plans live in .mend/plans/ and use the
  ISSUE-{number}.md naming convention.
- When the issue is closed or the plan is superseded, move the plan into a
  dated directory under .mend/plans/archive/ instead of deleting it.
- Archive directories preserve historical context; they are not an active
  work queue.
- Meta or cleanup plans are archived as the final cleanup step before their
  issue is closed.

## Philosophy

- **Lightweight:** These files can be messy. They're for me.
- **Commit often:** But don't overthink the commit message.
- **Review periodically:** Weekly review of friction log for improvement opportunities.
- **Cross-repo patterns:** Go to `kimi-claw-workspace/`, not here.

## Relationship to Other Spaces

| Space | Content |
|-------|---------|
| `.mend/` (here) | Repo-specific tracking, my xbrlkit diary |
| `kimi-claw-workspace/` | Cross-repo operational docs, my broader evolution |
| `.kimi/` | OpenClaw desktop configuration (not mine) |
| `docs/` | Public project documentation |
