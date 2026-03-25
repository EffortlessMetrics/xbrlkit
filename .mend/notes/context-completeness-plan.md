# Context Completeness Validation — Implementation Plan

**Issue:** #83  
**Date:** 2026-03-25  
**Status:** 📐 Plan → 🔨 Build

---

## Overview

Implement context completeness validation to ensure all XBRL facts reference valid, defined contexts.

## Architecture

**Crate:** `context-completeness` (follows numeric-rules/unit-rules pattern)

## Implementation Tasks

### Phase 1: Crate Setup (Est. 1 hour)

- [ ] Create `crates/context-completeness/` directory
- [ ] Add `Cargo.toml` with dependencies:
  - `xbrl-report-types` (Fact, ValidationFinding)
  - `xbrl-contexts` (ContextSet)
- [ ] Create `src/lib.rs`
- [ ] Add to workspace `Cargo.toml`

### Phase 2: Core Validation Logic (Est. 2-3 hours)

- [ ] Implement `validate_context_completeness(facts, contexts)` function
- [ ] Handle case-insensitive context ID matching
- [ ] Handle facts without context_ref (skip or error?)
- [ ] Generate ValidationFinding for missing contexts

### Phase 3: Unit Tests (Est. 1 hour)

- [ ] Test valid context references
- [ ] Test missing context references
- [ ] Test case-insensitive matching
- [ ] Test empty context set

### Phase 4: Integration (Est. 1 hour)

- [ ] Add `context-completeness` dependency to `validation-run/Cargo.toml`
- [ ] Add validation call in `validation-run/src/lib.rs`
- [ ] After IXDS assembly, before other validations

### Phase 5: BDD Scenarios (Est. 2 hours)

- [ ] Create `specs/features/foundation/context_completeness.feature`:
  ```gherkin
  Feature: Context Completeness Validation
    As a compliance officer
    I want all facts to reference valid contexts
    So that financial data is properly attributed

    @alpha-candidate
    Scenario: All facts reference valid contexts
      Given an XBRL report with contexts "ctx-1" and "ctx-2"
      And facts referencing contexts "ctx-1" and "ctx-2"
      When context completeness validation runs
      Then no findings are reported

    @alpha-candidate
    Scenario: Fact references missing context
      Given an XBRL report with context "ctx-1"
      And a fact referencing context "ctx-missing"
      When context completeness validation runs
      Then a context-missing error is reported

    @alpha-candidate
    Scenario: Context ID matching is case-insensitive
      Given an XBRL report with context "CTX-1"
      And a fact referencing context "ctx-1"
      When context completeness validation runs
      Then no findings are reported
  ```

### Phase 6: Alpha Check (Est. 1 hour)

- [ ] Create `scenarios/SCN-XK-CONTEXT-001.feature`
- [ ] Add AC-XK-CONTEXT-001 to `xtask/src/alpha_check.rs`
- [ ] Run `cargo xtask alpha-check`

## Files to Create/Modify

### New Files
| Path | Purpose |
|------|---------|
| `crates/context-completeness/Cargo.toml` | Crate manifest |
| `crates/context-completeness/src/lib.rs` | Validation logic |
| `specs/features/foundation/context_completeness.feature` | BDD scenarios |
| `scenarios/SCN-XK-CONTEXT-001.feature` | Alpha-check scenario |

### Modified Files
| Path | Change |
|------|--------|
| `Cargo.toml` (workspace) | Add `context-completeness` to members |
| `validation-run/Cargo.toml` | Add `context-completeness` dependency |
| `validation-run/src/lib.rs` | Wire validation into pipeline |
| `xtask/src/alpha_check.rs` | Add AC-XK-CONTEXT-001 |

## Acceptance Criteria

- **AC-XK-CONTEXT-001:** Missing context detected
  - Fact with non-existent context_ref → error finding
  
- **AC-XK-CONTEXT-002:** Valid context references pass
  - All facts reference existing contexts → no findings

- **AC-XK-CONTEXT-003:** Case-insensitive matching
  - Context IDs matched regardless of case

## Estimates

| Phase | Est. Time |
|-------|-----------|
| Crate Setup | 1 hour |
| Core Logic | 2-3 hours |
| Unit Tests | 1 hour |
| Integration | 1 hour |
| BDD Scenarios | 2 hours |
| Alpha Check | 1 hour |
| **Total** | **1.5-2 days** |

## Next Action

Begin Phase 1: Create context-completeness crate.

---

**Related:** `.mend/notes/context-completeness-research.md`
