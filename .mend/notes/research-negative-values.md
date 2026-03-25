# Research: Negative Value Validation

**Issue:** #80  
**Started:** 2026-03-25  
**Status:** 🔍 Research Phase

---

## SEC EFM Rules for Negative Values

### EFM 6.6.24 - Signs of Numbers

The SEC specifies that certain facts must not have negative values. This is determined by:
1. **Concept definition** in the taxonomy
2. **Balance type** attribute (debit/credit)
3. **Specific SEC rules** for DEI and financial statement concepts

### Taxonomy Indicators for Non-Negative Values

#### 1. Balance Type (us-gaap)
- **Debit balance types**: Assets, Expenses
  - These can typically be positive (normal balance)
  - Negative values may indicate contra-accounts or errors
- **Credit balance types**: Liabilities, Equity, Revenue
  - These can typically be positive (normal balance)
  - Negative values may indicate errors

#### 2. DEI Concepts (Never Negative)
The following DEI concepts should never have negative values:
- `dei:EntityCommonStockSharesOutstanding` - Shares can't be negative
- `dei:EntityPublicFloat` - Market cap can't be negative
- `dei:CommonStockSharesAuthorized` - Authorized shares
- `dei:CommonStockSharesIssued` - Issued shares
- `dei:CommonStockSharesOutstanding` - Outstanding shares
- `dei:EntityNumberOfEmployees` - Employee count

#### 3. Financial Statement Concepts
Common non-negative concepts:
- Share counts (all types)
- Entity counts
- Number of properties/facilities
- Asset quantities (when absolute)

## Implementation Approach

### Option 1: Taxonomy-Aware (Recommended)
Use taxonomy metadata to determine if a concept allows negative values:
1. Load taxonomy definition for each fact
2. Check for `xbrli:nonNegativeInteger` type
3. Check for balance type indicators
4. Check concept name patterns (Shares, Count, Number, etc.)

### Option 2: Profile-Based Rule
Define prohibited-negative concepts in profile pack:
```yaml
negative_value_rules:
  prohibited_concepts:
    - dei:EntityCommonStockSharesOutstanding
    - dei:EntityNumberOfEmployees
    # ... etc
```

### Option 3: Hybrid
Combine taxonomy awareness with profile overrides for edge cases.

## Recommended Implementation

**Approach:** Taxonomy-aware with profile overrides

**Crate:** `crates/numeric-rules/` (new crate for numeric validations)

**Key Functions:**
```rust
pub fn validate_negative_values(
    facts: &[Fact],
    taxonomy: &TaxonomyLoader,
    profile: &ProfilePack,
) -> Vec<ValidationFinding>
```

**Logic:**
1. For each numeric fact with negative value
2. Look up concept in taxonomy
3. Check if concept type is non-negative
4. Check profile for explicit prohibitions
5. Generate finding if negative value is prohibited

**Rule ID Format:**
- `SEC.NEGATIVE_VALUE.<CONCEPT_NAME>`

## Next Steps

1. Create `numeric-rules` crate structure
2. Implement taxonomy concept lookup for type checking
3. Implement negative value detection
4. Create BDD scenarios
5. Create test fixtures

## References

- EFM 6.6.24: Signs of Numbers
- DEI Taxonomy 2024
- US-GAAP Taxonomy 2024
