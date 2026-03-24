# Typed Member Handling Research

## Issue #56: Handle typedMember for typed dimensions

**Status:** ✅ COMPLETE

## Summary

Successfully implemented typedMember parsing for XBRL dimensional contexts in the xbrl-contexts crate.

## Changes Made

### 1. Modified `crates/xbrl-contexts/src/lib.rs`

- Added `TypedMemberValue` struct to hold parsed typed values
- Added `parse_typed_member()` function to extract typed values from nested elements
- Updated `parse_dimensional_container()` to handle both `explicitMember` and `typedMember` elements
- Populated `DimensionMember` fields correctly:
  - `is_typed: true` for typed members
  - `typed_value: Some(...)` with the extracted value
  - `member` field contains the typed value string

### 2. Added Unit Tests (5 new tests)

- `test_parse_typed_member_context` - Simple typed member in segment
- `test_parse_typed_member_in_scenario` - Typed member in scenario container
- `test_parse_mixed_explicit_and_typed_members` - Mixed explicit and typed
- `test_parse_typed_member_with_namespace_prefix` - Real-world NBB example
- `test_parse_typed_member_empty_value` - Empty value handling

### 3. Added BDD Scenarios

4 new scenarios in `specs/features/taxonomy/dimensions.feature`:
- SCN-XK-DIM-005: Typed member dimension is parsed correctly
- SCN-XK-DIM-006: Mixed explicit and typed members in same context
- SCN-XK-DIM-007: Typed member in segment container
- SCN-XK-DIM-008: Empty typed member value is handled

### 4. Added Test Fixture

Created `fixtures/synthetic/dimensions/typed-member-dimensions/typed-member-dimensions.html` with:
- Typed member in segment
- Typed member in scenario
- Mixed explicit and typed members
- Empty typed member
- Explicit-only context (backward compatibility)

## XBRL Specification

Typed dimensions use `xbrldi:typedMember` elements with nested typed values, unlike explicit members that reference domain members via QName.

### Format

```xml
<!-- Explicit member (already supported) -->
<xbrldi:explicitMember dimension="us-gaap:StatementScenarioAxis">
    us-gaap:ScenarioActualMember
</xbrldi:explicitMember>

<!-- Typed member (implemented) -->
<xbrldi:typedMember dimension="dim:CustomerAxis">
    <cust:customerId>12345</cust:customerId>
</xbrldi:typedMember>
```

### Key Differences

| Aspect | Explicit Member | Typed Member |
|--------|----------------|--------------|
| Element | `xbrldi:explicitMember` | `xbrldi:typedMember` |
| Content | Text (QName) | XML element(s) |
| Domain | Predefined domain members | Schema-defined type values |
| Storage | Member QName | Typed value + raw XML |

### Real-World Examples

From EBA XBRL filings:
```xml
<xbrldi:typedMember dimension="eba_dim:INC">
    <eba_typ:CC>c</eba_typ:CC>
</xbrldi:typedMember>
```

From NBB Belgium:
```xml
<xbrldi:typedMember dimension="dim:afnp">
    <open:str>John</open:str>
</xbrldi:typedMember>
```

## Acceptance Criteria

- [x] typedMember elements are parsed from dimensional contexts
- [x] Typed values stored correctly in DimensionMember
- [x] BDD scenarios pass for typed dimension handling
- [x] Backward compatibility with explicit dimensions maintained
- [x] All quality gates pass (clippy clean, tests pass)
- [x] PR merged (#65)

## PR

- **PR:** https://github.com/EffortlessMetrics/xbrlkit/pull/65
- **Branch:** `mend/issue-56-typed-member`
- **Merged:** 05e974d
