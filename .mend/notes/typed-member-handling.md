# Typed Member Handling Research

## Issue #56: Handle typedMember for typed dimensions

## XBRL Specification

Typed dimensions use `xbrldi:typedMember` elements with nested typed values, unlike explicit members that reference domain members via QName.

### Format

```xml
<!-- Explicit member (already supported) -->
<xbrldi:explicitMember dimension="us-gaap:StatementScenarioAxis">
    us-gaap:ScenarioActualMember
</xbrldi:explicitMember>

<!-- Typed member (needs implementation) -->
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

From XBRL-CSV context:
```xml
<xbrldi:typedMember dimension="dim-int:openAxis1_Taxis">
    <dim-int:typedMember1>123</dim-int:typedMember1>
</xbrldi:typedMember>
```

## Current Implementation

Location: `crates/xbrl-contexts/src/lib.rs`

The `DimensionMember` struct already supports typed dimensions:
```rust
pub struct DimensionMember {
    pub dimension: String,
    pub member: String,
    pub is_typed: bool,
    pub typed_value: Option<String>,
}
```

But parsing only handles `explicitMember`:
```rust
fn parse_dimensional_container(node: &roxmltree::Node) -> DimensionalContainer {
    // ...
    if child.tag_name().name() == "explicitMember" {
        // Handle explicit member
    }
    // TODO: Handle typedMember for typed dimensions
}
```

## Implementation Approach

1. **Parse typedMember elements**: Detect `typedMember` tag name
2. **Extract typed value**: Get text content from nested element
3. **Store raw XML**: Preserve full typed member XML for complex cases
4. **Update DimensionMember**: Set `is_typed=true` and populate `typed_value`

### Parsing Strategy

For typed members:
- The `dimension` attribute contains the dimension QName
- The member value comes from the first child element's text content
- The `member` field should store the typed value
- The `is_typed` flag should be `true`

Example:
```xml
<xbrldi:typedMember dimension="tax:dCustomer">
    <cust>12345</cust>
</xbrldi:typedMember>
```

Should produce:
```rust
DimensionMember {
    dimension: "tax:dCustomer",
    member: "12345",
    is_typed: true,
    typed_value: Some("12345"),
}
```

### Handling Complex Typed Members

Some typed members may have complex nested structures:
```xml
<xbrldi:typedMember dimension="tax:dPhone">
    <phone>
        <country>7</country>
        <city>7</city>
        <number>5555555</number>
    </phone>
</xbrldi:typedMember>
```

For these cases:
1. Extract text from first child as the primary value
2. Optionally store full XML in `raw_xml` for later processing

## Testing Strategy

1. Add unit tests for simple typed members
2. Add unit tests for typed members with nested elements
3. Add test with mixed explicit and typed members
4. Ensure backward compatibility with explicit-only contexts

## Files to Modify

- `crates/xbrl-contexts/src/lib.rs` - Main parsing logic
- `specs/features/taxonomy/dimensions.feature` - Add BDD scenarios
- `fixtures/synthetic/dimensions/` - Add test fixtures

## Acceptance Criteria

- [x] typedMember elements are parsed from dimensional contexts
- [x] Typed values stored correctly in DimensionMember
- [x] BDD scenarios pass for typed dimension handling
- [x] Backward compatibility with explicit dimensions maintained
- [x] All quality gates pass
