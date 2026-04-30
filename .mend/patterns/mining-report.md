# XBRLKit Pattern Mining Report

Generated: 2025-03-18
Agent: kimi-claw

## Current State

### Crate Structure (38 crates)
- **Core:** xbrl-core, xbrl-types, xbrl-facts, xbrl-contexts, ~~xbrl-dimensions~~ (removed #286)
- **SEC/EFM:** sec-profile-types, efm-rules
- **Validation:** validation-run, validation-types
- **Parsing:** ixbrl-parse, ixbrl-dom
- **CLI:** xbrlkit-cli, xtask
- **BDD:** scenario-runner, xbrlkit-bdd-steps
- **Infrastructure:** receipt-types, receipt-store, render-md

### Validation Patterns Identified

#### Pattern 1: EFM Rules (Established)
```rust
// crates/efm-rules/src/lib.rs
pub fn validate_inline_restrictions(...) -> ValidationResult
pub fn validate_required_facts(...) -> ValidationResult  // NEW
```
- Validates SEC-specific rules
- Returns structured findings with rule IDs
- Used by scenario-runner for BDD

#### Pattern 2: Profile-Based Validation
- Profile JSON defines capabilities
- Validation checks profile.supported_rules
- Allows gradual rule rollout

### BDD Coverage Analysis

| Feature | Scenarios | Alpha-Active |
|---------|-----------|--------------|
| IXDS Assembly | 2 | 2 |
| Taxonomy | 2 | 2 |
| Duplicate Facts | 1 | 1 |
| SEC Inline | 1 | 1 |
| SEC Required Facts | 2 | 2 |

**Total: 8 alpha-active scenarios**

### Quick Wins Identified

1. **Alpha-Check Bug** (blocks CI)
   - BDD runner has cross-scenario contamination
   - Location: scenario-runner/src/lib.rs
   - Fix: Reset validation report between scenarios

2. **Documentation Gap**
   - Missing ADR for validation architecture
   - No documented pattern for adding new SEC rules

3. **Test Coverage**
   - Only 8 scenarios active
   - Many more scenarios in specs/features/ not @alpha-active

### Medium-Effort Opportunities

1. **Additional SEC Required Facts**
   - DEI has 20+ required facts for 10-K
   - Current: 3 facts implemented
   - Opportunity: Batch implement remaining

2. **Context Parsing**
   - Research indicates this is complex (~62h estimated)
   - Needed for full XBRL validation

3. **IXBRL Parse Improvements**
   - Error handling could be more granular
   - Performance optimization for large filings

### Architectural Questions

1. **Rule Organization**
   - Current: All SEC rules in efm-rules
   - Alternative: Separate crates per rule category?

2. **Profile Evolution**
   - How to version profiles as rules expand?
   - Migration strategy for existing validations?

## Top 5 Recommendations

1. **Fix alpha-check bug** - Unblocks CI, enables reliable merges
2. **Document validation pattern** - ADR for adding new rules
3. **Implement remaining DEI facts** - Complete required facts validation
4. **Activate more BDD scenarios** - Increase coverage
5. **Research context parsing** - Major capability unlock

## Research Queue

- [ ] SEC EDGAR Filer Manual for required facts list
- [ ] XBRL Dimensions specification for context parsing
- [ ] Performance benchmarks for large filings
