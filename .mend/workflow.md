# xbrlkit Development Workflow

**Principle:** Every change is documented, researched, planned, reviewed, and explained.

## The Pipeline

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Issue     │ →  │  Research   │ →  │    Plan     │ →  │   Build     │ →  │   Review    │
│  (Create)   │    │  (Verify)   │    │  (Design)   │    │ (Implement) │    │ (Refine)    │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                                                                    ↓
┌─────────────┐    ┌─────────────┐
│   Merge     │ ←  │  CI Green   │
│  (Final)    │    │  (Validate) │
└─────────────┘    └─────────────┘
```

## Stage Definitions

### 1. Issue Creation
**Who:** Anyone (discovery)
**Output:** GitHub issue with:
- Problem statement (what needs to happen)
- Context (why it matters)
- Acceptance criteria (done when...)
- References (specs, docs, related issues)

### 2. Research & Verification
**Who:** Maintainer (investigation)
**Output:** Detailed comment on issue:
- Technical findings
- Spec references (XBRL, SEC, etc.)
- Prior art in codebase
- Open questions

### 3. Plan Review
**Who:** Maintainer (design)
**Output:** Detailed comment with:
- Approach decision
- Crate boundaries
- API sketch
- Testing strategy
- Risks and mitigations

### 4. Build
**Who:** Builder (implement)
**Output:** PR with:
- Implementation matching plan
- Tests (unit + integration)
- Documentation updates
- ADR if architectural

### 5. Review
**Who:** Reviewer (critique)
**Output:** PR comments with:
- Issues found
- Suggested fixes
- Explanations (why this matters)

### 6. Refine
**Who:** Builder (address)
**Output:** Commits with:
- Fixes applied
- Comments explaining what changed and why

### 7. CI Validation
**Gates:**
- `cargo fmt --all --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `cargo xtask alpha-check`

### 8. Merge
**Who:** Maintainer (with authority)
**Action:** Squash merge, delete branch, update queue

## Special Patterns

### Adding New Scenarios (Golden Files)

When adding a new BDD scenario that updates the feature grid:

```bash
# 1. Run feature-grid to generate new grid
cargo xtask feature-grid

# 2. Copy to golden file
cp artifacts/feature.grid.v1.json tests/goldens/feature.grid.v1.json

# 3. Include in PR
git add tests/goldens/feature.grid.v1.json
```

**Why:** Alpha-check compares generated grid against golden. New scenarios change the grid.

### Issue Comments with Complex Markdown

For multi-line comments with code blocks, tables, etc.:

```bash
# Write to temp file
cat > /tmp/issue-{N}-research.md << 'EOF'
## Research Findings

| Item | Status |
|------|--------|
| Foo | ✅ |

```rust
fn example() {}
```
EOF

# Use body-file, not inline --body
gh issue comment {N} --body-file /tmp/issue-{N}-research.md
```

**Why:** Avoids shell escaping nightmares with backticks and quotes.

### Queue State Management

When picking up work:
1. Move issue from "Current Queue" to "In Progress"
2. Update stage: 📋 → 🔍 (Research)
3. Create branch: `mend/issue-{N}-brief-description`

When completing:
1. Move to "Completed" section
2. Update stage: 🔄 → ✅ (Complete)
3. Update queue timestamp

## Documentation Requirements

Every PR must update:
- [ ] Code (implementation)
- [ ] Tests (coverage)
- [ ] ADR (if architectural decision)
- [ ] `.mend/notes/` (research findings)
- [ ] `.mend/friction/` (if process issues encountered)

## Queue Discipline

1. Pick next issue from `.mend/pr-queue.md`
2. Move through pipeline stages
3. Update issue with progress comments
4. Only one active build per maintainer
5. Merge → close issue → queue next

## Quality Bar

**Acceptable:**
- Solves the stated problem
- Passes all gates
- Is documented
- Has test coverage

**Not Acceptable:**
- Works but unexplained
- Missing tests
- Breaks existing scenarios
- Undocumented API changes
