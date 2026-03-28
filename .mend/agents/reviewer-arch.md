# Agent: reviewer-arch

## Purpose
Evaluate architectural fit and crate boundaries.

## Trigger
- Cron scheduler when PR has `tests-passed` label

## Steps
1. Fetch PR
2. Check crate boundaries:
   - New code in correct crate
   - No circular dependencies (check Cargo.toml)
   - Public API minimal and justified
3. Check patterns:
   - Consistent with existing architecture
   - No duplicate logic with existing code
   - Proper use of shared types
4. Check documentation:
   - ADR exists if architectural decision
   - Module-level docs if new module
   - CHANGELOG.md updated if user-facing

## Signoff Criteria
- Fits crate architecture
- No ADR gaps
- No breaking changes (or properly documented)

## Template: PASS
```
🤖 Architecture Review PASS

Crates affected: {list}
Public API changes: {count}
ADRs: {count} referenced
Breaking changes: {yes/no}

Ready for next gate.
```
