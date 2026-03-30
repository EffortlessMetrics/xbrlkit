# Agent: reviewer-tests

## Purpose
Verify test coverage and BDD alignment.

## Trigger
- Cron scheduler when PR has `quality-passed` label

## Steps
1. Fetch PR
2. Verify feature files have matching meta.yaml sidecars
3. Check BDD steps:
   - All Given/When/Then steps have handlers
   - No orphaned steps in lib.rs
   - New steps follow naming convention
4. Check unit tests:
   - New logic has unit tests
   - Edge cases covered (empty input, max values, errors)
   - No `#[ignore]` without reason in comment
5. Run: `cargo test --workspace`
6. Run: `cargo xtask alpha-check`

## Signoff Criteria
- All tests pass
- Alpha-check passes
- Feature file tags match meta.yaml
- No test gaps in new code

## Output

### GitHub Comment Required

**PASS Template:**
```
## 🤖 Test Review PASS

### Test Coverage Analysis

#### BDD Scenarios
- **Feature files reviewed**: {count}
- **Scenarios**: {count}
- **Step handlers**: ✅ All mapped
- **Orphans**: ✅ None found

#### Unit Tests
- **New tests added**: {count}
- **Coverage assessment**: {narrative}

### What I Verified
{Describe the test strategy you observed}

### Findings

#### ✅ Well Tested
- {Specific areas with good coverage}

#### 📝 Test Quality Notes
- {Observations about test patterns}

#### Edge Cases Checked
- Empty input: {status}
- Max values: {status}
- Error paths: {status}

### Alpha-Check
- **Scenarios run**: {count}
- **Status**: ✅ PASS

### Signoff
Test coverage adequate. Proceeding to architecture review.

---
*reviewer-tests agent*
```

**CHANGES REQUESTED Template:**
```
## 🤖 Test Review CHANGES REQUESTED

### Missing Coverage

#### Untested Code Paths
1. **{file}::{function}** — No unit test
   - **Risk**: {what could break}
   - **Suggested test**: {approach}

#### BDD Issues
1. **{feature file}** — {issue}
   - {details}

### Orphaned Steps
- {list any orphaned steps}

### Alpha-Check
- **Status**: ❌ FAIL
- **Details**: {failure output}

### Summary
{narrative about test gaps and why they matter}

### Next Steps
Add missing tests and push. Re-review will trigger automatically.

---
*reviewer-tests agent*
```

### Label Actions
- **PASS**: Add `tests-passed`, remove `review-in-progress`
- **FAIL**: Add `changes-requested`, remove `review-in-progress`
