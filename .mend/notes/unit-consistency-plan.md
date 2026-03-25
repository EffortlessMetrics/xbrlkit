# Unit Consistency Validation — Implementation Plan

**Issue:** #82  
**Date:** 2026-03-25  
**Status:** 📐 Plan → Ready for Build

---

## Overview

Implement unit consistency validation to ensure XBRL facts use appropriate units for their concept types (monetary values use currency, share counts use shares, etc.).

## Architecture Decision

**Chosen:** Option B — New `unit-rules` crate

Following the established pattern from `numeric-rules`, create a dedicated crate for unit validation logic. This separates validation concerns from basic unit utilities.

---

## Implementation Tasks

### Phase 1: Crate Setup (Est. 2-4 hours)

- [ ] Create `crates/unit-rules/` directory
- [ ] Add `Cargo.toml` with dependencies:
  - `xbrl-report-types` (for Fact, Unit)
  - `sec-profile-types` (for UnitRules config)
  - `validation-report-types` (for ValidationFinding)
- [ ] Create `src/lib.rs` with module structure
- [ ] Add to workspace `Cargo.toml`

### Phase 2: Core Validation Logic (Est. 1 day)

- [ ] Create `src/validator.rs` with `UnitValidator` struct
- [ ] Implement concept → expected unit type mapping:
  ```rust
  pub enum ExpectedUnitType {
      Monetary,      // iso4217:XXX
      Shares,        // xbrli:shares
      Pure,          // xbrli:pure
      PerShare,      // Derived units (e.g., USDPerShare)
  }
  ```
- [ ] Implement pattern-based concept matching:
  - `.*Shares.*` → Shares
  - `.*Employees.*` → Pure
  - `.*PerShare.*` → PerShare
  - Monetary detection via taxonomy type or explicit list
- [ ] Generate `ValidationFinding` for mismatches

### Phase 3: Configuration (Est. 4-6 hours)

- [ ] Add `UnitRules` struct to `sec-profile-types/src/lib.rs`:
  ```rust
  #[derive(Debug, Clone, Default, Serialize, Deserialize)]
  pub struct UnitRules {
      pub monetary_concepts: Vec<String>,
      pub share_concepts: Vec<String>,
      pub pure_concepts: Vec<String>,
      pub per_share_concepts: Vec<String>,
  }
  ```
- [ ] Add `unit_rules: UnitRules` field to `NumericRules`
- [ ] Create `profiles/efm-77/unit_rules.yaml` with EFM 77 rules

### Phase 4: Integration (Est. 4-6 hours)

- [ ] Add `validate_unit_consistency()` call in `validation-run/src/lib.rs`
- [ ] Wire into `ValidationPipeline`
- [ ] Ensure findings flow through to report output

### Phase 5: BDD Scenarios (Est. 1 day)

- [ ] Create `bdd/features/unit_consistency.feature`:
  ```gherkin
  Feature: Unit Consistency Validation
    As a compliance officer
    I want facts to have appropriate units for their concepts
    So that financial data is correctly interpreted

    @alpha-candidate
    Scenario: Monetary fact with currency unit passes
      Given a fact with concept "us-gaap:Revenue" and unit "iso4217:USD"
      When unit consistency validation runs
      Then no findings are reported

    @alpha-candidate
    Scenario: Monetary fact with non-currency unit fails
      Given a fact with concept "us-gaap:Revenue" and unit "xbrli:shares"
      When unit consistency validation runs
      Then a unit-inconsistency error is reported

    @alpha-candidate
    Scenario: Share fact with shares unit passes
      Given a fact with concept "us-gaap:CommonStockSharesOutstanding" and unit "xbrli:shares"
      When unit consistency validation runs
      Then no findings are reported

    @alpha-candidate
    Scenario: Share fact with wrong unit fails
      Given a fact with concept "us-gaap:CommonStockSharesOutstanding" and unit "iso4217:USD"
      When unit consistency validation runs
      Then a unit-inconsistency error is reported
  ```

### Phase 6: Alpha Check (Est. 2-4 hours)

- [ ] Create `scenarios/SCN-XK-SEC-UNIT-001.feature`
- [ ] Add AC-XK-SEC-UNIT-001 to `xtask/src/alpha_check.rs`
- [ ] Run `cargo xtask alpha-check` and ensure passing

---

## Files to Create/Modify

### New Files
| Path | Purpose |
|------|---------|
| `crates/unit-rules/Cargo.toml` | Crate manifest |
| `crates/unit-rules/src/lib.rs` | Module exports |
| `crates/unit-rules/src/validator.rs` | Core validation logic |
| `crates/unit-rules/src/patterns.rs` | Concept pattern matching |
| `profiles/efm-77/unit_rules.yaml` | EFM 77 unit rules config |
| `bdd/features/unit_consistency.feature` | BDD scenarios |
| `scenarios/SCN-XK-SEC-UNIT-001.feature` | Alpha-check scenario |

### Modified Files
| Path | Change |
|------|--------|
| `Cargo.toml` (workspace) | Add `unit-rules` to members |
| `sec-profile-types/src/lib.rs` | Add `UnitRules` struct |
| `validation-run/Cargo.toml` | Add `unit-rules` dependency |
| `validation-run/src/lib.rs` | Wire validation into pipeline |
| `xtask/src/alpha_check.rs` | Add AC-XK-SEC-UNIT-001 |

---

## Dependencies

**Blocked by:** None — can proceed immediately  
**Blocks:** None

---

## Risks & Mitigations

| Risk | Mitigation |
|------|------------|
| Pattern matching too broad | Start with explicit concept lists, add patterns incrementally |
| Custom unit definitions | Support user-configured derived units in profile |
| Performance on large filings | Use parallel iteration for fact validation |

---

## Acceptance Criteria (from Research)

- **AC-XK-SEC-UNIT-001:** Unit inconsistency detected
  - Monetary fact with non-currency unit → error
  - Share fact with non-shares unit → error

- **AC-XK-SEC-UNIT-002:** Valid unit references pass
  - All facts have appropriate units for their concept type

---

## Open Questions (Resolve During Build)

1. **Monetary detection:** Start with explicit configuration or attempt taxonomy type detection?
   - *Decision:* Start explicit, expand later

2. **Severity:** Warning or error by default?
   - *Decision:* Error (unit inconsistency is a data quality issue)

3. **Custom units:** How to handle company-specific per-share definitions?
   - *Decision:* Allow regex patterns in configuration

---

## Estimates

| Phase | Est. Time |
|-------|-----------|
| Crate Setup | 2-4 hours |
| Core Logic | 1 day |
| Configuration | 4-6 hours |
| Integration | 4-6 hours |
| BDD Scenarios | 1 day |
| Alpha Check | 2-4 hours |
| **Total** | **2-3 days** |

---

## Next Action

Move to 🔨 Build stage and begin Phase 1 (crate setup).

---

**Related:** `.mend/notes/unit-consistency-research.md`
