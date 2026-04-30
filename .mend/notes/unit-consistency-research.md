# Unit Consistency Validation Research

**Issue:** #82
**Research Date:** 2026-03-25
**Researcher:** Kimi Mend

## Problem Statement

XBRL facts reference units that define the measurement. Inconsistent units can indicate data quality issues:
- Monetary facts should reference monetary units (USD, EUR)
- Share counts should reference shares unit
- Per-share values should reference per-share units

## Background Research

### XBRL Unit Types

XBRL defines several standard unit types in the `xbrli` (XBRL International) namespace:

1. **xbrli:pure** - For dimensionless numbers, ratios, counts
2. **xbrli:shares** - For share counts
3. **iso4217:XXX** - ISO currency codes for monetary values
4. **Custom derived units** - Like `us-gaap:USDPerShare` for per-share metrics

### SEC/EFM Requirements

While the SEC EFM doesn't explicitly mandate specific unit consistency checks, data quality best practices require:
- Monetary facts (Revenue, Assets, Liabilities) must use currency units
- Share counts must use `xbrli:shares`
- Per-share metrics should use derived units
- Pure ratios (percentages, ratios) should use `xbrli:pure`

## Implementation Analysis

### Existing Infrastructure

1. **xbrl-units crate** - ~~Currently only normalizes unit strings~~ — **removed in #286**. Validation logic to be added to `unit-rules` crate.
2. **xbrl-report-types::Fact** - Has `unit_ref: Option<String>` field
3. **validation-run** - Shows pattern for wiring new validation rules
4. **sec-profile-types** - Shows how to add rule configuration (see `NumericRules`)

### Proposed Architecture

**Option A: ~~Extend xbrl-units crate~~**
- ~~Add validation functions to existing crate~~
- xbrl-units was **removed in #286**. Use `unit-rules` crate instead.
- Simple, centralized
- May bloat unit utility crate

**Option B: Create unit-rules crate (Recommended)**
- Follows pattern of `numeric-rules`
- Separates validation logic from basic unit utilities
- Allows profile-based configuration

### Validation Logic Design

```rust
// Pattern matching for concept → expected unit type
pub fn validate_unit_consistency(
    facts: &[Fact],
    unit_rules: &UnitRules,
) -> Vec<ValidationFinding> {
    // 1. Match concept name patterns to expected unit types
    // 2. Check actual unit_ref against expected
    // 3. Return findings for mismatches
}
```

**Concept Patterns:**

| Pattern | Examples | Expected Unit |
|---------|----------|---------------|
| `.*Shares.*` | SharesOutstanding, TreasuryShares | xbrli:shares |
| `.*Employees.*` | NumberOfEmployees | xbrli:pure |
| `.*PerShare.*` | EarningsPerShare | us-gaap:USDPerShare |
| Monetary concepts* | Revenue, Assets, Cash | iso4217:XXX |

*Monetary detection: Check taxonomy type (monetaryItemType)

### Configuration Design

Add to `sec-profile-types/src/lib.rs`:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct NumericRules {
    #[serde(default)]
    pub negative_value_rules: NegativeValueRules,
    #[serde(default)]
    pub unit_rules: UnitRules,  // NEW
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct UnitRules {
    #[serde(default)]
    pub monetary_concepts: Vec<String>,  // Explicit list
    #[serde(default)]
    pub share_concepts: Vec<String>,     // Explicit list
    #[serde(default)]
    pub pure_concepts: Vec<String>,      // Explicit list
}
```

### Integration Points

1. **validation-run/src/lib.rs** - Add `validate_unit_consistency` call
2. **sec-profile-types** - Add `unit_rules` configuration
3. **Profile YAML** - Add `unit_rules.yaml` configuration file
4. **BDD scenarios** - Create `specs/features/validation/unit_consistency.feature`

## Acceptance Criteria

- **AC-XK-SEC-UNIT-001:** Unit inconsistency detected
  - Monetary fact with non-currency unit → error
  - Share fact with non-shares unit → error

- **AC-XK-SEC-UNIT-002:** Valid unit references pass
  - All facts have appropriate units for their concept type

## Implementation Tasks

1. Create `unit-rules` crate with validation logic
2. Add `UnitRules` to `sec-profile-types`
3. Wire into `validation-run`
4. Create BDD scenarios
5. Add profile configuration for EFM 77
6. Run `cargo xtask alpha-check`

## Open Questions

1. Should we detect monetary concepts by name pattern or require explicit configuration?
2. How to handle custom units (company-specific per-share definitions)?
3. Should this be a warning or error by default?

## Recommendation

Proceed with **Option B** (new `unit-rules` crate) following the established pattern from `numeric-rules`. Start with explicit configuration for known concepts, expand to pattern matching as needed.

---
**Next Step:** Move to 📐 Plan stage and create implementation plan.
