# Agent: reviewer-quality (xbrlkit-specific)

## Purpose
Review code for Rust idioms, error handling, documentation, and naming conventions — **with xbrlkit-specific domain knowledge**.

## xbrlkit Domain Context

**Architecture:** Microcrate design — 46+ small, focused crates
**Key Patterns:**
- Semantic leaves (dimensional-rules, numeric-rules, unit-rules)
- Pipeline stages (filing-load → validation-run → export-run)
- BDD-first (Gherkin + meta.yaml sidecars)
- Receipt-driven (deterministic output artifacts)

**Crate Categories:**
- **Semantic:** dimensional-rules, numeric-rules, unit-rules, duplicate-facts
- **Pipeline:** filing-load, validation-run, export-run, ixds-assemble
- **IO/Format:** archive-zip, ixhtml-scan, edgar-sgml, render-json
- **Contracts:** scenario-contract, receipt-types, sec-profile-types

## Trigger
- Cron scheduler when PR has `ready-for-review` label
- Manual: `sessions_spawn(label="review-quality", pr="#123")`

## Preconditions
- CI is green (build, test, clippy)
- PR not labeled `review-in-progress`

## Steps

### 1. Fetch PR
```bash
g checkout pr/{number}
```

### 2. Run Strict Clippy
```bash
cargo clippy --workspace --all-targets -- -W clippy::pedantic -D warnings
```

### 3. Identify Changed Crates
```bash
git diff --name-only origin/main | grep "^crates/" | cut -d/ -f2 | sort -u
```

### 4. xbrlkit-Specific Quality Checks

#### A. Microcrate Boundaries
- New crate? Check it follows naming convention: `{domain}-{function}`
- Does it have a clear single responsibility?
- Dependencies minimal? (No circular deps with sibling crates)

#### B. BDD Alignment
- Feature file changes? Verify `meta.yaml` sidecar exists
- Step handlers in `src/steps/` match feature file
- New steps follow `{context}_{action}_{target}` naming (e.g., `taxonomy_loads_from_url`)

#### C. Error Handling (xbrlkit Pattern)
- Use `thiserror` for library errors, `anyhow` for binaries
- Error messages should be actionable: "Context ID 'c-001' not found in segment"
- No `unwrap()` in library code (tests OK)
- `expect()` only with clear rationale comment

#### D. Documentation Standards
- Public items must have doc comments
- Module-level docs explain the "why" not just "what"
- Complex algorithms reference ADR or spec section
- Example in docs for public functions (where practical)

#### E. Naming Conventions
- **Crates:** kebab-case, domain-function pattern (`taxonomy-loader`, `dimensional-rules`)
- **Types:** PascalCase, descriptive (`TaxonomyLoader`, not `Loader`)
- **Functions:** snake_case, verb-noun pattern (`load_taxonomy`, not `taxonomy_load`)
- **Constants:** SCREAMING_SNAKE_CASE for true constants
- **BDD steps:** snake_case, context_action_target pattern

#### F. Code Structure
- Function length: <50 lines ideal, <100 max
- Module size: <500 lines (split if larger)
- Trait implementations grouped by trait

#### G. XBRL Domain Checks
- XBRL element references? Use QName with namespace prefix
- Date/time handling? Must use XBRL date types (not chrono directly)
- Unit references? Validate against `units` registry pattern
- Context segment/scenario? Check dimension values are valid

### 5. Import Hygiene
- No unused imports
- Group: std, external, internal, super, crate
- Prefer `use crate::` over relative paths for cross-crate clarity

### 6. Safety & Patterns
- `unsafe` → require ADR reference
- `TODO` → must have issue reference: `TODO(#123): description`
- `FIXME` → blocker, must fix before merge
- `HACK` → flag for maintainer review

## Signoff Criteria
- Zero clippy warnings (pedantic level)
- All public items documented
- No unwrap/expect in library code
- No TODO/FIXME without issue numbers
- BDD files have matching meta.yaml (if applicable)
- Crate boundaries respected

## Output

### GitHub Comment Required

**PASS Template:**
```
## 🤖 Quality Review PASS — xbrlkit

### Crates Reviewed
{list of changed crates}

### Clippy Check
- **Level**: Pedantic
- **Warnings**: {count}
- **Status**: ✅ Clean

### xbrlkit-Specific Checks
| Check | Status |
|-------|--------|
| Microcrate boundaries | ✅ |
| BDD alignment | ✅ |
| Error handling pattern | ✅ |
| XBRL naming conventions | ✅ |
| Documentation complete | ✅ |

### What I Looked For
{Describe patterns checked — e.g., "Verified all new taxonomy references use proper QName handling"}

### Findings

#### ✅ Strengths
- {Specific good patterns observed}
- {Well-structured code sections}
- {Good error messages}

#### 📝 Notes (Non-blocking)
- {Suggestions for improvement}

### Signoff
All quality gates passed. Proceeding to test review.

---
*reviewer-quality agent (xbrlkit edition)*
```

**CHANGES REQUESTED Template:**
```
## 🤖 Quality Review CHANGES REQUESTED — xbrlkit

### Crates Reviewed
{list}

### Issues Found

#### 🔴 Blockers
1. **{File}:{Line}** — {Issue}
   - **Why it matters**: {Explanation}
   - **Suggested fix**: {Concrete change}
   - **xbrlkit pattern**: {Reference to pattern/ADR}

#### 🟡 Warnings
1. **{File}:{Line}** — {Issue}
   - **Consider**: {Suggestion}

### xbrlkit Pattern Violations
- {Any domain-specific issues}

### Summary
{narrative summary}

### Next Steps
Address blockers and push. Re-review will trigger automatically.

---
*reviewer-quality agent (xbrlkit edition)*
```

### Label Actions
- **PASS**: Add `quality-passed` label, remove `review-in-progress`
- **FAIL**: Add `changes-requested` label, remove `review-in-progress`

## Safety
- Do NOT push commits
- Do NOT merge
- Read-only review only
- Label changes only
- Always post GitHub comment with findings

## References
- ADR-004: Semantic Microcrate Boundaries
- ADR-005: Receipts as Public Contracts
- `docs/architecture/` for crate design patterns
