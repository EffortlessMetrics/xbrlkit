# PR Review: mend/dimension-scenarios

**Reviewer:** Kimi Mend  
**Date:** 2026-03-20  
**Branch:** mend/dimension-scenarios  
**Scope:** BDD scenarios for dimensional validation with step handlers

---

## Overall Verdict: **COMMENT**

The PR provides a solid foundation for dimensional validation scenarios with good coverage of core use cases. The scenarios follow Gherkin best practices and the step handlers are implemented. However, there are several items that should be addressed before full activation.

---

## 1. Scenario Coverage Assessment ✅

| Scenario ID | Description | AC Mapping | Status |
|-------------|-------------|------------|--------|
| SCN-XK-DIM-001 | Valid dimension-member pair | AC-XK-DIM-001 | ✅ Defined |
| SCN-XK-DIM-002 | Invalid dimension-member pair | AC-XK-DIM-002 | ✅ Defined |
| SCN-XK-DIM-003 | Missing required dimension | AC-XK-DIM-003 | ✅ Defined |
| SCN-XK-DIM-004 | Unknown dimension rejected | AC-XK-DIM-004 | ✅ Defined |

**Coverage Analysis:**
- ✅ **Good coverage of core dimension validation cases:**
  - Valid pair (positive test)
  - Invalid member (negative test)
  - Missing required dimension (boundary case)
  - Unknown dimension (error handling)
- ⚠️ **Missing scenarios that could be considered:**
  - Domain hierarchy validation
  - Hypercube applicability checks
  - Typed dimension validation
  - Default member handling

**Recommendation:** The 4 scenarios provide a solid MVP for dimension validation. Additional edge cases can be added in future iterations.

---

## 2. Step Handler Implementation Quality

### New Types Added (lib.rs)
```rust
#[derive(Debug, Clone, Default)]
pub struct DimensionContext {
    pub dimension: Option<String>,
    pub member: Option<String>,
    pub concept: Option<String>,
}
```

**Assessment:**
- ✅ Clean separation of dimension state from World
- ✅ Follows existing pattern in codebase
- ⚠️ **Issue:** Simulation logic in When steps is hardcoded:
  ```rust
  if dimension == "us-gaap:StatementScenarioAxis" {
      if member == "us-gaap:ScenarioActualMember" {
          // Valid - no findings
      }
  }
  ```
  This will need to be replaced with actual taxonomy validation once the underlying crates are ready.

### Given Step Handlers
| Step Pattern | Implementation | Status |
|--------------|----------------|--------|
| `the taxonomy has dimension definitions` | Stub - returns true | ⚠️ Placeholder |
| `the taxonomy has domain hierarchies` | Stub - returns true | ⚠️ Placeholder |
| `the taxonomy has hypercube definitions` | Stub - returns true | ⚠️ Placeholder |
| `a context with dimension "..."` | Sets dimension_context.dimension | ✅ |
| `the member "..."` / `an invalid member "..."` | Sets dimension_context.member | ✅ |
| `a fact for concept "..."` | Sets dimension_context.concept | ⚠️ Incomplete parsing |
| `a context without that dimension` | No-op (relies on default) | ⚠️ Implicit behavior |
| `a context with unknown dimension "..."` | Sets dimension | ✅ |

### When Step Handlers
| Step Pattern | Implementation | Status |
|--------------|----------------|--------|
| `I validate the dimension-member pair` | Simulation logic | ⚠️ Needs real implementation |
| `I validate the fact dimensions` | Simulation logic | ⚠️ Needs real implementation |

### Then Step Handlers
| Step Pattern | Implementation | Status |
|--------------|----------------|--------|
| `the validation should pass` | Checks validation_findings empty | ✅ |
| `the validation should fail` | Checks validation_findings not empty | ✅ |
| `no findings should be reported` | Checks validation_findings empty | ✅ (duplicate) |
| `an "..." finding should be reported` | Checks for specific finding | ✅ |

**Recommendations:**
1. Add TODO comments for simulation code that needs replacement
2. Consider consolidating duplicate "no findings" assertions
3. Implement proper parsing for "requiring dimension" in Given steps

---

## 3. Meta.yaml Correctness

**File:** `specs/features/taxonomy/dimensions.meta.yaml`

### Positive Findings
- ✅ All 4 scenarios have unique IDs (SCN-XK-DIM-001 through 004)
- ✅ AC IDs map correctly (AC-XK-DIM-001 through 004)
- ✅ Consistent req_id: REQ-XK-DIMENSIONS across all scenarios
- ✅ Appropriate crate dependencies listed
- ✅ Profile pack set to `sec/efm-77/opco`
- ✅ Suite: synthetic, Speed: fast

### Issues Identified

| Issue | Severity | Description |
|-------|----------|-------------|
| Empty fixtures | ⚠️ Medium | All scenarios have `fixtures: []` - this is noted as expected but limits test realism |
| REQ-XK-DIMENSIONS undefined | ⚠️ Medium | The requirement ID is referenced but not defined in any visible requirements file |
| Missing receipts definition | 📝 Low | No explicit definition of what receipts should contain |

### Structure Validation
```yaml
feature_id: FEAT-XK-DIMENSIONS
layer: taxonomy
module: dimensions
scenarios:
  SCN-XK-DIM-001:
    ac_id: AC-XK-DIM-001
    req_id: REQ-XK-DIMENSIONS
    crates: [taxonomy-dimensions, dimensional-rules, xbrl-contexts, xbrlkit-bdd, xbrlkit-bdd-steps, xtask]
    fixtures: []
    profile_pack: sec/efm-77/opco
    receipts: [scenario.run.v1]
    suite: synthetic
    speed: fast
```

**Assessment:** Well-structured, follows established patterns from other meta.yaml files.

---

## 4. Alpha Check Integration

**Current State:**
- ✅ All scenarios tagged with `@alpha-active`
- ✅ AC IDs (AC-XK-DIM-001 through 004) defined in meta.yaml
- ⚠️ **Missing:** No evidence of alpha check feature referencing these ACs

**Integration Check:**
```bash
# Looking for alpha check references to dimension ACs
grep -r "AC-XK-DIM" specs/features/workflow/alpha_check.feature
# Result: No matches found
```

**Recommendation:** 
- Consider adding these ACs to the alpha check feature once step handlers are fully implemented
- Or create a separate alpha gate for taxonomy features

---

## 5. Scenario Clarity and Gherkin Best Practices

### Feature Structure
```gherkin
Feature: XBRL Dimensional Validation
  As an XBRL processor
  I want to validate dimensional aspects of facts
  So that I can ensure dimension-member pairs are valid according to taxonomy
```

**Assessment:**
- ✅ Clear user story format
- ✅ Business value is explicit
- ✅ Appropriate scope for the feature

### Background Section
```gherkin
Background:
  Given the taxonomy has dimension definitions
  And the taxonomy has domain hierarchies
  And the taxonomy has hypercube definitions
```

**Assessment:**
- ✅ Good use of Background for shared preconditions
- ⚠️ These steps are currently stubs - will need real implementation

### Individual Scenarios

#### SCN-XK-DIM-001: Valid dimension-member pair
```gherkin
@alpha-active
Scenario: Valid dimension-member pair passes validation
  Given a context with dimension "us-gaap:StatementScenarioAxis"
  And the member "us-gaap:ScenarioActualMember"
  When I validate the dimension-member pair
  Then the validation should pass
  And no findings should be reported
```
- ✅ Clear, declarative language
- ✅ Specific test data (us-gaap concepts)
- ✅ Single responsibility (one validation check)
- ⚠️ "Then the validation should pass" and "And no findings should be reported" are slightly redundant

#### SCN-XK-DIM-002: Invalid dimension-member pair
```gherkin
@alpha-active
Scenario: Invalid dimension-member pair fails validation
  Given a context with dimension "us-gaap:StatementScenarioAxis"
  And an invalid member "us-gaap:NonExistentMember"
  When I validate the dimension-member pair
  Then the validation should fail
  And an "XBRL.DIMENSION.INVALID_MEMBER" finding should be reported
```
- ✅ Good negative test
- ✅ Specific error code assertion
- ✅ "invalid member" phrasing is clear

#### SCN-XK-DIM-003: Missing required dimension
```gherkin
@alpha-active
Scenario: Missing required dimension is detected
  Given a fact for concept "us-gaap:Revenue" requiring dimension "us-gaap:StatementScenarioAxis"
  And a context without that dimension
  When I validate the fact dimensions
  Then the validation should fail
  And an "XBRL.DIMENSION.MISSING_REQUIRED" finding should be reported
```
- ⚠️ **Grammar issue:** "a fact for concept "us-gaap:Revenue" requiring dimension..." doesn't parse correctly in step handler
- The step handler only captures the concept, not the "requiring dimension" part
- Current implementation assumes Revenue requires the dimension based on hardcoded logic

**Suggested fix:**
```gherkin
Given a fact for concept "us-gaap:Revenue"
And the concept requires dimension "us-gaap:StatementScenarioAxis"
And a context without that dimension
```

#### SCN-XK-DIM-004: Unknown dimension
```gherkin
@alpha-active
Scenario: Unknown dimension is rejected
  Given a context with unknown dimension "custom:UnknownAxis"
  When I validate the dimension-member pair
  Then the validation should fail
  And an "XBRL.DIMENSION.UNKNOWN" finding should be reported
```
- ✅ Clear scenario
- ✅ Tests custom namespace handling
- ✅ Appropriate error code

---

## Activation Blockers

### Must Fix Before Activation

| # | Blocker | Severity | Location |
|---|---------|----------|----------|
| 1 | **Step handler for "requiring dimension" incomplete** | 🔴 High | lib.rs line ~145 |
| 2 | **Simulation code needs TODO markers** | 🟡 Medium | lib.rs When handlers |
| 3 | **Background steps are stubs** | 🟡 Medium | dimensions.feature |

### Recommended Before Merge

| # | Item | Rationale |
|---|------|-----------|
| 1 | Add fixture definitions | Currently `fixtures: []` limits test value |
| 2 | Define REQ-XK-DIMENSIONS | Requirement traceability |
| 3 | Consolidate duplicate assertions | "no findings" vs "validation should pass" |

### Can Be Deferred

| # | Item | Rationale |
|---|------|-----------|
| 1 | Additional edge case scenarios | Can be added iteratively |
| 2 | Full alpha check integration | Depends on overall taxonomy feature readiness |
| 3 | Real taxonomy validation | Depends on underlying crate implementation |

---

## Summary

### Strengths
1. **Good scenario coverage** for MVP dimension validation
2. **Clean Gherkin** following best practices
3. **Proper meta.yaml structure** with correct AC mappings
4. **Consistent tagging** with @alpha-active

### Weaknesses
1. **Incomplete step handler** for SCN-XK-DIM-003 (requiring dimension parsing)
2. **Placeholder Background steps** that need real implementation
3. **Simulation-based When steps** that will need replacement

### Recommended Action
**COMMENT** - The PR provides good value but needs minor fixes before activation:
1. Fix the step handler for "requiring dimension" parsing
2. Add TODO markers to simulation code
3. Consider adding fixture requirements

After these fixes, the scenarios should be **APPROVE** for merge with follow-up tickets for:
- Real taxonomy validation implementation
- Additional edge case scenarios
- Alpha check integration

---

## Appendix: File Locations

```
/root/.openclaw/xbrlkit/specs/features/taxonomy/dimensions.feature      (1629 bytes)
/root/.openclaw/xbrlkit/specs/features/taxonomy/dimensions.meta.yaml    (1077 bytes)
/root/.openclaw/xbrlkit/crates/xbrlkit-bdd-steps/src/lib.rs            (12680 bytes - modified)
```

## Appendix: Scenario Traceability Matrix

| Scenario | AC ID | Req ID | Status |
|----------|-------|--------|--------|
| SCN-XK-DIM-001 | AC-XK-DIM-001 | REQ-XK-DIMENSIONS | 🟡 Pending fixtures |
| SCN-XK-DIM-002 | AC-XK-DIM-002 | REQ-XK-DIMENSIONS | 🟡 Pending fixtures |
| SCN-XK-DIM-003 | AC-XK-DIM-003 | REQ-XK-DIMENSIONS | 🔴 Step handler issue |
| SCN-XK-DIM-004 | AC-XK-DIM-004 | REQ-XK-DIMENSIONS | 🟡 Pending fixtures |
