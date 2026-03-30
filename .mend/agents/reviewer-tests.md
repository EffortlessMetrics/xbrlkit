# Agent: reviewer-tests (xbrlkit-specific)

## Purpose
Verify test coverage and BDD alignment — **with xbrlkit's BDD-first, scenario-driven architecture**.

## xbrlkit Test Philosophy

**Scenario-First Development:**
- Gherkin features drive implementation
- Each feature file has `meta.yaml` sidecar with metadata
- `@alpha-active` tags mark executable acceptance criteria
- Step handlers in `src/steps/` map to scenario contracts

**Test Layers:**
```
BDD Scenarios (features/) ──→ Integration/Acceptance
Unit Tests (src/*_test.rs) ──→ Logic/Edge Cases
Doc Tests ───────────────────→ API Examples
Alpha-Check (xtask) ─────────→ Acceptance Criteria Validation
```

## Trigger
- Cron scheduler when PR has `quality-passed` label

## Steps

### 1. Fetch PR
```bash
git checkout pr/{number}
```

### 2. BDD Structure Validation

#### Feature Files Check
```bash
find crates -name "*.feature" -newer origin/main 2>/dev/null
```

For each changed feature file:
- [ ] Corresponding `meta.yaml` exists in same directory
- [ ] `meta.yaml` has: `id`, `acs`, `devex`, `labels`
- [ ] `@alpha-active` tag used for executable scenarios
- [ ] Tags match meta.yaml `acs` references

#### Step Handler Verification
```bash
grep -r "given!\|when!\|then!" crates/{changed}/src/steps/ 2>/dev/null
```

- All Given/When/Then steps have handlers
- No orphaned step registrations in `lib.rs`
- New steps follow `{context}_{action}_{target}` pattern

### 3. Unit Test Coverage

#### New Code Analysis
```bash
git diff origin/main --stat | grep "crates/"
```

For each changed crate:
- New public functions have unit tests?
- Edge cases covered: empty input, max values, errors
- Property-based tests for validation logic (proptest where appropriate)

#### xbrlkit-Specific Test Patterns
- **XBRL parsing tests:** Include malformed XML cases
- **Validation tests:** Test both pass and fail scenarios
- **Receipt tests:** Verify deterministic output artifacts
- **Pipeline tests:** Test stage isolation

### 4. Test Quality Checks

#### Forbidden Patterns
- `#[ignore]` without reason in comment
- `todo!()` or `unimplemented!()` in committed tests
- Tests that don't assert anything
- Tests with hardcoded paths (use tempdir)

#### Required Patterns
- Tests use `insta` for snapshot testing (goldens in `fixtures/goldens/`)
- Async tests use `tokio::test` with proper runtime
- Error tests verify error type, not just message

### 5. Run Test Suites

```bash
# Standard tests
cargo test --workspace

# Alpha scenarios (critical for xbrlkit)
cargo xtask alpha-check

# Doc tests
cargo test --workspace --doc
```

### 6. xbrlkit-Specific Validation

#### Receipt Validation
If PR generates receipts:
```bash
cargo xtask schema-check
```
- Receipts match JSON schema
- No schema violations

#### Scenario Alignment
```bash
cargo xtask feature-grid --changed-only
```
- All @alpha-active scenarios execute
- No step handler panics

#### Integration Points
- Cross-crate integration tested?
- Feature flags tested (if added)
- CLI commands tested (if applicable)

## Signoff Criteria
- All tests pass (`cargo test --workspace`)
- Alpha-check passes (all @alpha-active scenarios)
- Feature files have meta.yaml sidecars
- No orphaned step handlers
- No ignored tests without reason
- New code has test coverage
- Receipt schemas validate (if applicable)

## Output

### GitHub Comment Required

**PASS Template:**
```
## 🤖 Test Review PASS — xbrlkit

### BDD Scenarios
- **Feature files**: {count}
- **@alpha-active scenarios**: {count}
- **meta.yaml sidecars**: ✅ All present
- **Step handlers**: ✅ All mapped

### Test Coverage
| Crate | New Tests | Edge Cases | Async |
|-------|-----------|------------|-------|
| {crate} | ✅ | ✅ | ✅ |

### Test Execution
- **Unit tests**: ✅ {count} passed
- **Doc tests**: ✅ {count} passed
- **Alpha-check**: ✅ {count} scenarios passed
- **Schema validation**: ✅ (if applicable)

### xbrlkit-Specific Checks
- [x] Receipt artifacts valid
- [x] Step handlers registered
- [x] No orphaned steps
- [x] Feature-meta alignment

### What I Verified
{Describe test strategy observed}

### Signoff
Test coverage adequate. Proceeding to architecture review.

---
*reviewer-tests agent (xbrlkit edition)*
```

**CHANGES REQUESTED Template:**
```
## 🤖 Test Review CHANGES REQUESTED — xbrlkit

### Missing Coverage

#### Untested Code Paths
1. **{crate}::{module}::{function}** — No unit test
   - **Risk**: {what could break}
   - **Suggested test**: {approach}

#### BDD Issues
1. **{feature file}** — Missing meta.yaml
   - Required for scenario tracking

#### Alpha-Check Failures
```
{failure output}
```

#### Orphaned Steps
- `{step pattern}` — No handler registered

### Summary
{narrative about test gaps}

### Next Steps
Add missing tests and push. Re-review will trigger automatically.

---
*reviewer-tests agent (xbrlkit edition)*
```

### Label Actions
- **PASS**: Add `tests-passed`, remove `review-in-progress`
- **FAIL**: Add `changes-requested`, remove `review-in-progress`

## References
- ADR-001: Scenario-Driven Workspace
- ADR-005: Receipts as Public Contracts
- ADR-006: No Live Network in BDD
- `specs/features/README.md` for BDD patterns
