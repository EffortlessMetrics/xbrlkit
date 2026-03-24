# Alpha Gate Scenario Implementation Notes

## Research Findings

### 1. Feature File Analysis
**File:** `specs/features/workflow/alpha_check.feature`

Current scenario:
```gherkin
@REQ-XK-WORKFLOW
@layer.workflow
@suite.synthetic
Feature: Alpha check

  @AC-XK-WORKFLOW-003
  @SCN-XK-WORKFLOW-005
  @speed.fast
  Scenario: Run the alpha readiness gate
    Given the active alpha scenarios are implemented
    When I run the alpha readiness gate
    Then the alpha readiness checks pass
```

**Missing:** `@alpha-active` tag (needs to be added)

### 2. Alpha Readiness Gate Understanding
**Source:** `xtask/src/alpha_check.rs`

The `cargo xtask alpha-check` command performs:
1. Runs `doctor()` - sanity checks required repo directories
2. Compiles feature grid to JSON
3. Runs `schema_check::run()`
4. Runs `test_ac()` for all `ACTIVE_ALPHA_ACS` acceptance criteria
5. Runs `bdd("@alpha-active")` - executes scenarios tagged with `@alpha-active`
6. Compares output files against golden files
7. Runs CLI commands and validates outputs

The alpha gate ensures the codebase is ready for alpha release by testing:
- All active ACs (13 acceptance criteria)
- All scenarios tagged with `@alpha-active`
- Golden file comparisons
- CLI integration tests

### 3. Step Handler Patterns
**Source:** `crates/xbrlkit-bdd-steps/src/lib.rs`

Three main handler functions:
- `handle_given(world, scenario, step)` - returns `Ok(true)` if handled
- `handle_when(world, scenario, step)` - returns `Ok(true)` if handled  
- `handle_then(world, step)` - returns `Ok(())` on success

The pattern for adding new steps:
1. Check step text with `if step.text == "..."` or `strip_prefix`
2. Perform the action or assertion
3. Return `Ok(true)` for Given/When handlers to indicate handled
4. Return `Ok(())` for Then handlers

### 4. Required Step Implementations

#### `Given the active alpha scenarios are implemented`
- **Action:** Verify scenarios with `@alpha-active` tag exist in the feature grid
- **Implementation:** Check that grid has scenarios with the tag
- **Error:** Fail if no alpha-active scenarios found

#### `When I run the alpha readiness gate`
- **Action:** Execute `cargo xtask alpha-check` as subprocess
- **Implementation:** Use `std::process::Command` to run the command
- **Store:** Capture output and exit code in `world.cli_output`

#### `Then the alpha readiness checks pass`
- **Action:** Verify alpha-check exited with code 0
- **Implementation:** Check stored exit code
- **Error:** Fail if non-zero exit code

### 5. World State Extensions Needed
The `World` struct already has `cli_output: Option<String>` field which can be used to store the alpha-check output.

## Implementation Plan

1. Add `@alpha-active` tag to the feature file scenario
2. Add `Given` handler in `handle_given()` function
3. Add `When` handler in `handle_when()` function
4. Add `Then` handler in `handle_then()` function
5. Run `cargo xtask alpha-check` to verify
6. Run quality gates
7. Create PR and merge

## Dependencies
No new dependencies needed. Using existing:
- `std::process::Command` for subprocess execution
- Existing `World` fields for state management

## Implementation Complete

### Changes Made

#### 1. Feature File (`specs/features/workflow/alpha_check.feature`)
- Added `@alpha-active` tag to SCN-XK-WORKFLOW-005 scenario

#### 2. Step Handlers (`crates/xbrlkit-bdd-steps/src/lib.rs`)

**World struct extension:**
- Added `cli_exit_code: Option<i32>` field to store command exit code
- Updated `World::new()` to initialize the new field

**Given handler:**
```rust
if step.text == "the active alpha scenarios are implemented" {
    // Parse feature files to verify @alpha-active tag exists
    // Iterates through specs/features/*.feature files
    // Checks for @alpha-active string in content
}
```

**When handler:**
```rust
if step.text == "I run the alpha readiness gate" {
    // Execute: cargo xtask alpha-check
    // Captures stdout, stderr, and exit code
    // Stores in world.cli_output and world.cli_exit_code
}
```

**Then handler:**
```rust
"the alpha readiness checks pass" => {
    // Verify exit code is 0
    // Include output in error message on failure
}
```

#### 3. Sidecar File (`specs/features/workflow/alpha_check.meta.yaml`)
- Already configured for SCN-XK-WORKFLOW-005 with correct crates and dependencies

### Acceptance Criteria Status
- [x] `@alpha-active` tag added to SCN-XK-WORKFLOW-005
- [x] `Given the active alpha scenarios are implemented` step handler implemented
- [x] `When I run the alpha readiness gate` step handler implemented  
- [x] `Then the alpha readiness checks pass` step handler implemented
- [ ] Quality gates pass (requires cargo)
- [ ] PR merged (requires git operations)

### Next Steps
1. Run `cargo check -p xbrlkit-bdd-steps` to verify compilation
2. Run `cargo xtask alpha-check` to verify scenario passes
3. Run `make quick` for quality gates
4. Create PR and merge
