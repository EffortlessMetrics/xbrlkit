# Typed Value Validation Research

## Overview
Add validation for typed member values based on dimension's `value_type` from taxonomy.

## Current State

### taxonomy-dimensions Crate
- `Dimension::Typed` variant contains `value_type: String` field
- Example value types: `"xs:string"`, `"xs:date"`, `"xs:decimal"`, `"xs:boolean"`
- `validate_member()` returns `Ok(())` for typed dimensions without value validation

### dimensional-rules Crate  
- `validate_dimension_member()` has TODO: "Add typed value validation based on dimension's value_type"
- Currently returns `Ok(())` for typed dimensions without checking value format

### xbrl-contexts Crate
- `DimensionMember` stores:
  - `is_typed: bool` - flag indicating typed dimension
  - `typed_value: Option<String>` - the actual value
  - `member: String` - duplicate of value for typed dimensions

## XBRL Type System

Common XBRL/XSD types to support:

| Type | Pattern/Format | Example |
|------|----------------|---------|
| `xs:string` | Any string | "Hello" |
| `xs:decimal` | Decimal number | "123.45", "-67" |
| `xs:integer` | Whole number | "42", "-7" |
| `xs:date` | ISO 8601 date | "2024-03-15" |
| `xs:dateTime` | ISO 8601 datetime | "2024-03-15T10:30:00" |
| `xs:boolean` | true/false/0/1 | "true", "1" |
| `xs:anyURI` | Valid URI | "http://example.com" |

## Validation Design

### Approach
1. Extract `value_type` from `Dimension::Typed`
2. Parse and validate the typed value against the expected type
3. Return specific validation error on type mismatch

### Error Types
- `XBRL.DIMENSION.INVALID_TYPED_VALUE` - value doesn't match expected type
- `XBRL.DIMENSION.EMPTY_TYPED_VALUE` - typed value is empty/whitespace only

### Implementation Location
- Add `validate_typed_value()` function in `dimensional-rules`
- Modify `validate_dimension_member()` to call it for typed dimensions

### Type Validation Logic

```rust
fn validate_typed_value(value: &str, value_type: &str) -> Result<(), ValidationFinding> {
    match value_type {
        "xs:string" | "string" => Ok(()), // Any string is valid
        "xs:decimal" | "decimal" => validate_decimal(value),
        "xs:integer" | "integer" => validate_integer(value),
        "xs:date" | "date" => validate_date(value),
        "xs:dateTime" | "dateTime" => validate_datetime(value),
        "xs:boolean" | "boolean" => validate_boolean(value),
        "xs:anyURI" | "anyURI" => validate_uri(value),
        _ => Ok(()), // Unknown types pass validation (extensibility)
    }
}
```

### Validation Patterns

- **decimal**: Regex `^-?\d+(\.\d+)?$` or parse with `BigDecimal`
- **integer**: Regex `^-?\d+$` or parse with `i64`
- **date**: Try parse with `chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")`
- **dateTime**: Try parse with `chrono::DateTime::parse_from_rfc3339`
- **boolean**: Match against "true", "false", "1", "0" (case-insensitive)
- **anyURI**: Use `url::Url::parse()` or basic URL validation

## BDD Scenarios Needed

New scenarios for `specs/features/taxonomy/dimensions.feature`:

1. **Valid typed string value** - Any string passes
2. **Valid typed decimal value** - "123.45" passes
3. **Invalid typed decimal value** - "abc" fails
4. **Valid typed date value** - "2024-03-15" passes  
5. **Invalid typed date value** - "15-03-2024" fails
6. **Valid typed boolean value** - "true" passes
7. **Invalid typed boolean value** - "yes" fails
8. **Empty typed value** - "" fails
9. **Unknown type** - Custom type passes (extensibility)

## Dependencies

May need to add:
- `chrono` for date/datetime parsing (likely already in workspace)
- `regex` for pattern matching (optional, can use parse methods)

## Acceptance Criteria

- [x] Typed member values validated against dimension's `value_type`
- [x] Support common XBRL types: string, decimal, date, boolean
- [x] Validation errors for type mismatches
- [x] BDD scenarios pass
- [x] All quality gates pass
