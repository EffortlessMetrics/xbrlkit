---
name: Research and Planning
about: Break down work into manageable chunks before implementation
title: '[Plan] '
labels: ['planning', 'research']
assignees: ''

---

## Problem/Goal

What are we trying to accomplish?

## Research Questions

- [ ] What do we need to learn?
- [ ] What are the unknowns?
- [ ] What could go wrong?

## Decomposition

Break into independent chunks. Each chunk produces a handoff.

| Chunk | Estimate | Dependencies | Can Parallel? | Handoff File |
|-------|----------|--------------|---------------|--------------|
|  |  |  |  | `.mend/handoffs/<chunk>.md` |

## Spike Tasks

- [ ] Research spike issue: #
- [ ] Proof of concept:
- [ ] ADR drafted:

## Implementation Plan

Order of operations:

1. 
2. 
3. 

## Handoff Template

For each chunk, create `.mend/handoffs/<chunk>.md`:

```markdown
# Handoff: <Chunk Name>

## Context
- Planning issue: #<number>
- Chunk: <name>
- Estimated: <time>

## Problem Statement
<!-- One sentence -->

## Investigation Summary
<!-- What was learned -->

## Code Excerpts
<!-- 10-30 lines of relevant code -->
```rust
// Key code here
```

## Fix Strategy
<!-- Specific steps to implement -->
1. 
2. 
3. 

## Test Template
<!-- Pre-filled test skeleton -->
```rust
#[test]
fn test_<name>() {
    // TODO: implement
}
```

## Known Pitfalls
<!-- Check .mend/friction.md for relevant patterns -->
- 

## Decisions Made
<!-- Non-obvious choices, rejected alternatives -->
- Decision: <what>
- Why: <reason>
- Rejected: <alternative>

## Files to Touch
<!-- Exact file list -->
- `path/to/file.rs`

## Verification
<!-- Commands to run -->
```bash
cargo fmt --all
cargo clippy -p <pkg> --tests -- -D warnings
cargo test -p <pkg>
```

## Status
- [ ] Investigation complete
- [ ] Handoff written
- [ ] Implementation started
- [ ] Implementation complete
- [ ] Tests pass
- [ ] Ready for review

## Notes for Reviewer
<!-- What to watch for -->
- 
```

## Spawn Strategy

Which chunks spawn isolated sessions?

| Chunk | Spawn? | Reason | Minimal Prompt |
|-------|--------|--------|----------------|
|  | Yes/No |  | 7 lines, references handoff |

### Minimal Spawn Prompt Template

```
Read .mend/handoffs/<chunk>.md for context.
Read .mend/friction.md for known pitfalls.
Branch: feat/<chunk>
Files: <list from handoff>
Verify: cargo fmt && cargo clippy && cargo test
Append reviewer briefing to handoff when done.
```

## Definition of Done

- [ ] Research complete
- [ ] Work decomposed into chunks < 2h each
- [ ] Handoff files created for each chunk
- [ ] Spawn strategy defined with minimal prompts
- [ ] Issues created for each chunk
- [ ] PRs reference planning issue

## Notes
