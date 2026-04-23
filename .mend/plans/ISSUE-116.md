# Plan: Implement XBRL Context Parsing in xbrl-contexts Crate

**Stream:** A: Core XBRL Parsing  
**Issue:** #116  
**Date:** 2026-04-10  
**Status:** 🔍 Research Complete → Ready for ADR & Integration

---

## Current State Analysis

### What's Already Built (✅ Surprisingly Complete)

The `xbrl-contexts` crate is **far more complete than the issue suggests**:

| Component | State | Notes |
|-----------|-------|-------|
| Core data model | ✅ Complete | `Context`, `EntityIdentifier`, `Period`, `ContextSet` |
| XML parsing | ✅ Complete | `parse_contexts()` using `roxmltree` |
| Explicit dimensions | ✅ Complete | `explicitMember` parsing in segment/scenario |
| Typed dimensions | ✅ Complete | `typedMember` with nested value extraction |
| Normalized IDs | ✅ Complete | Case-insensitive context ID lookup |
| Error handling | ✅ Complete | `ContextError` enum with `thiserror` |
| Unit tests | ✅ Comprehensive | 12+ tests covering all major scenarios |

### The Real Gap 🎯

**Integration with the rest of the xbrlkit ecosystem is missing:**

1. **No ADR document** explaining design decisions (required by deliverables)
2. **No integration with `ixhtml-scan`** for Inline XBRL context extraction
3. **No BDD scenarios** for context parsing acceptance criteria
4. **No synthetic fixtures** in the expected location
5. **Context completeness validation** exists but may not use this crate

---

## Research Findings

### XBRL 2.1 Context Specification

Contexts contain:
- **Entity**: Identifier (scheme + value) + optional segment
- **Period**: Instant, Duration (start/end), or Forever
- **Scenario**: Optional dimensional container

### Inline XBRL 1.1 Context Location

Contexts are defined in `<ix:resources>` within `<ix:header>`:
```xml
<ix:header>
  <ix:resources>
    <xbrli:context id="ctx-1">
      <xbrli:entity>...</xbrli:entity>
      <xbrli:period>...</xbrli:period>
    </xbrli:context>
  </ix:resources>
</ix:header>
```

### SEC EFM Context Validation Rules

- Context IDs must be unique (case-insensitive)
- All facts must reference valid contexts
- Entity scheme should be `http://www.sec.gov/CIK` for SEC filings
- Period dates must be valid XSD dates

---

## Implementation Plan

### Phase 1: ADR Document (1-2 hours)

Create `adr/ADR-009-context-parsing-design.md`:

```markdown
# ADR-009: XBRL Context Parsing Design

## Decision
Use `roxmltree` for zero-copy XML parsing with a typed data model.

## Rationale
- roxmltree already in dependency tree (used by other crates)
- Zero-copy parsing reduces allocations for large filings
- Serde-compatible types enable JSON serialization for receipts

## Context Data Model
(Context diagram showing relationships)

## Dimensional Handling
- Explicit members: QName pairs
- Typed members: Nested XML value extraction
- Mixed containers supported

## Integration Points
| From | To | Purpose |
|------|-----|---------|
| ixhtml-scan | xbrl-contexts | Extract contexts from Inline XBRL |
| xbrl-contexts | context-completeness | Validate fact references |
| xbrl-contexts | dimensional-rules | Validate dimension usage |
```

### Phase 2: Synthetic Fixtures (1-2 hours)

Create test fixtures in `fixtures/synthetic/contexts/`:

```
fixtures/synthetic/contexts/
├── simple-instant/
│   └── context.xml       # Basic instant context
├── simple-duration/
│   └── context.xml       # Basic duration context
├── with-segment/
│   └── context.xml       # Entity segment with dimensions
├── with-scenario/
│   └── context.xml       # Scenario with dimensions
├── explicit-dimensions/
│   └── context.xml       # Multiple explicit members
├── typed-dimensions/
│   └── context.xml       # Typed member values
└── inline-xbrl/
    └── ixbrl.html        # Full Inline XBRL with ix:resources
```

### Phase 3: BDD Scenarios (1-2 hours)

Create `specs/features/foundation/context_parsing.feature`:

```gherkin
@REQ-XK-CONTEXT-PARSE
@layer.foundation
Feature: XBRL Context Parsing

  @alpha-candidate
  @AC-XK-CONTEXT-PARSE-001
  Scenario: Parse instant period context
    Given an XBRL context with instant period "2024-12-31"
    When context parsing runs
    Then the parsed context has period type "instant"
    And the instant date is "2024-12-31"

  @alpha-candidate
  @AC-XK-CONTEXT-PARSE-002
  Scenario: Parse duration period context
    Given an XBRL context with duration "2024-01-01" to "2024-12-31"
    When context parsing runs
    Then the parsed context has period type "duration"
    And the start date is "2024-01-01"
    And the end date is "2024-12-31"

  @alpha-candidate
  @AC-XK-CONTEXT-PARSE-003
  Scenario: Parse context with explicit dimensions
    Given an XBRL context with explicit dimension "us-gaap:StatementScenarioAxis"
    And the member is "us-gaap:ScenarioActualMember"
    When context parsing runs
    Then the context has 1 dimensional member
    And the member is not typed

  @alpha-candidate
  @AC-XK-CONTEXT-PARSE-004
  Scenario: Parse context with typed dimensions
    Given an XBRL context with typed dimension "dim:CustomAxis"
    And the typed value is "CUSTOM-123"
    When context parsing runs
    Then the context has 1 dimensional member
    And the member is typed
    And the typed value is "CUSTOM-123"

  @alpha-candidate
  @AC-XK-CONTEXT-PARSE-005
  Scenario: Extract contexts from Inline XBRL
    Given an Inline XBRL document with ix:resources
    And the resources contain 2 contexts
    When inline context extraction runs
    Then 2 contexts are extracted
    And all contexts have valid IDs
```

### Phase 4: ixhtml-scan Integration (2-3 hours)

Extend `ixhtml-scan` to extract contexts from `ix:resources`:

```rust
// crates/ixhtml-scan/src/lib.rs

/// Extract contexts from Inline XBRL header resources.
pub fn extract_contexts_from_inline(html: &str) -> Result<ContextSet, ContextError> {
    let resources = extract_resources_section(html)?;
    parse_contexts(&resources)
}

/// Find and extract the ix:resources section.
fn extract_resources_section(html: &str) -> Result<String, ContextError> {
    // Find <ix:resources> ... </ix:resources>
    // Return inner XML content
}
```

Add to `Cargo.toml`:
```toml
[dependencies]
xbrl-contexts = { path = "../xbrl-contexts" }
```

### Phase 5: Validation Integration (1-2 hours)

Ensure `context-completeness` crate uses `xbrl-contexts`:

```rust
// crates/context-completeness/src/lib.rs
use xbrl_contexts::{ContextSet, parse_contexts};

pub fn validate_context_completeness(
    facts: &[Fact],
    context_set: &ContextSet,
) -> Vec<ValidationFinding> {
    // Use ContextSet for O(1) lookups
}
```

---

## Acceptance Criteria

- [ ] ADR-009 created in `adr/` explaining context parsing design
- [ ] Synthetic fixtures created in `fixtures/synthetic/contexts/`
- [ ] BDD scenarios created in `specs/features/foundation/context_parsing.feature`
- [ ] `ixhtml-scan` integration for extracting contexts from Inline XBRL
- [ ] `context-completeness` uses `xbrl-contexts` crate
- [ ] All tests pass: `cargo test -p xbrl-contexts`
- [ ] All quality gates pass: `cargo xtask alpha-check`

---

## Branch Strategy

```
mend/issue-116-context-parsing
├── Commit 1: ADR-009 document
├── Commit 2: Synthetic fixtures
├── Commit 3: BDD scenarios
├── Commit 4: ixhtml-scan integration
└── Commit 5: Validation integration + final tests
```

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Existing code has edge case bugs | Medium | Low | Add comprehensive fixtures |
| Inline XBRL namespace variations | Medium | Medium | Test with real SEC filings |
| Integration complexity | Low | Low | Small, focused commits |

---

## Notes

- The existing `xbrl-contexts` implementation is surprisingly complete
- Main work is documentation, integration, and test fixtures
- Consider this a "polish and integrate" task rather than "build from scratch"

---

## Related

- Existing context parsing: `crates/xbrl-contexts/src/lib.rs` (330 lines)
- Inline XBRL scanning: `crates/ixhtml-scan/src/lib.rs`
- Context completeness: `crates/context-completeness/src/lib.rs`
- BDD scenarios: `specs/features/foundation/context_completeness.feature`
