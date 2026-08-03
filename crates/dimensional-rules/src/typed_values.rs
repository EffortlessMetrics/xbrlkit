use xbrl_contexts::DimensionMember;
use xbrl_report_types::ValidationFinding;

/// Validate a typed dimension value against its declared `value_type`.
pub(super) fn validate_typed_dimension_value(
    dim_member: &DimensionMember,
    dimension: &taxonomy_dimensions::Dimension,
) -> Result<(), ValidationFinding> {
    let value = dim_member
        .typed_value
        .as_deref()
        .unwrap_or(&dim_member.member);

    // Get the value_type from the typed dimension
    let value_type = match dimension {
        taxonomy_dimensions::Dimension::Typed { value_type, .. } => value_type.as_str(),
        taxonomy_dimensions::Dimension::Explicit { .. } => return Ok(()), // Should not happen since we checked is_typed()
    };

    // Check for empty value
    if value.trim().is_empty() {
        return Err(ValidationFinding {
            rule_id: "XBRL.DIMENSION.EMPTY_TYPED_VALUE".to_string(),
            severity: "error".to_string(),
            message: format!("Typed dimension {} has empty value", dim_member.dimension),
            member: Some(dim_member.dimension.clone()),
            subject: Some(value.to_string()),
        });
    }

    // Validate based on value_type
    match value_type {
        "xs:string" | "string" => Ok(()), // Any non-empty string is valid
        "xs:decimal" | "decimal" => validate_decimal(value, dim_member),
        "xs:integer" | "integer" => validate_integer(value, dim_member),
        "xs:date" | "date" => validate_date(value, dim_member),
        "xs:dateTime" | "dateTime" => validate_datetime(value, dim_member),
        "xs:boolean" | "boolean" => validate_boolean(value, dim_member),
        "xs:anyURI" | "anyURI" => validate_uri(value, dim_member),
        _ => {
            // Unknown types pass validation (extensibility)
            Ok(())
        }
    }
}

/// Validate decimal format.
fn validate_decimal(value: &str, dim_member: &DimensionMember) -> Result<(), ValidationFinding> {
    // Check for valid decimal pattern: optional sign, digits, optional decimal point and digits
    let trimmed = value.trim();
    if trimmed
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c == '+')
        && trimmed.chars().filter(|&c| c == '.').count() <= 1
        && !trimmed.starts_with("..")
        && !trimmed.ends_with('.')
        && trimmed != "-"
        && trimmed != "+"
        && trimmed != "."
        && trimmed != "-."
        && trimmed != "+."
    {
        // Try to parse to ensure it's a valid number
        if trimmed.parse::<f64>().is_ok() {
            return Ok(());
        }
    }

    Err(ValidationFinding {
        rule_id: "XBRL.DIMENSION.INVALID_TYPED_VALUE".to_string(),
        severity: "error".to_string(),
        message: format!(
            "Value '{}' is not a valid decimal for dimension {}",
            value, dim_member.dimension
        ),
        member: Some(dim_member.dimension.clone()),
        subject: Some(value.to_string()),
    })
}

/// Validate integer format.
fn validate_integer(value: &str, dim_member: &DimensionMember) -> Result<(), ValidationFinding> {
    let trimmed = value.trim();
    if !trimmed.is_empty()
        && trimmed
            .chars()
            .skip_while(|&c| c == '-' || c == '+')
            .all(|c| c.is_ascii_digit())
        && trimmed != "-"
        && trimmed != "+"
        && trimmed.parse::<i64>().is_ok()
    {
        return Ok(());
    }

    Err(ValidationFinding {
        rule_id: "XBRL.DIMENSION.INVALID_TYPED_VALUE".to_string(),
        severity: "error".to_string(),
        message: format!(
            "Value '{}' is not a valid integer for dimension {}",
            value, dim_member.dimension
        ),
        member: Some(dim_member.dimension.clone()),
        subject: Some(value.to_string()),
    })
}

/// Validate date format (ISO 8601: YYYY-MM-DD).
fn validate_date(value: &str, dim_member: &DimensionMember) -> Result<(), ValidationFinding> {
    let trimmed = value.trim();

    // Basic pattern check for YYYY-MM-DD
    if trimmed.len() == 10 {
        let parts: Vec<&str> = trimmed.split('-').collect();
        if parts.len() == 3 {
            // Validate year, month, day are numeric
            if parts[0].len() == 4
                && parts[0].chars().all(|c| c.is_ascii_digit())
                && parts[1].len() == 2
                && parts[1].chars().all(|c| c.is_ascii_digit())
                && parts[2].len() == 2
                && parts[2].chars().all(|c| c.is_ascii_digit())
            {
                // Additional validation for valid date ranges
                if let (Ok(_year), Ok(month), Ok(day)) = (
                    parts[0].parse::<u32>(),
                    parts[1].parse::<u32>(),
                    parts[2].parse::<u32>(),
                ) && (1..=12).contains(&month)
                    && (1..=31).contains(&day)
                {
                    // Basic check passed (full calendar validation optional)
                    return Ok(());
                }
            }
        }
    }

    Err(ValidationFinding {
        rule_id: "XBRL.DIMENSION.INVALID_TYPED_VALUE".to_string(),
        severity: "error".to_string(),
        message: format!(
            "Value '{}' is not a valid date (expected YYYY-MM-DD) for dimension {}",
            value, dim_member.dimension
        ),
        member: Some(dim_member.dimension.clone()),
        subject: Some(value.to_string()),
    })
}

/// Validate datetime format (ISO 8601).
fn validate_datetime(value: &str, dim_member: &DimensionMember) -> Result<(), ValidationFinding> {
    let trimmed = value.trim();

    // Check for 'T' separator
    if trimmed.contains('T') {
        let parts: Vec<&str> = trimmed.split('T').collect();
        if parts.len() == 2 {
            // Validate date portion
            if validate_date(parts[0], dim_member).is_ok() {
                // Time portion: HH:MM:SS or HH:MM:SS.sss with optional timezone
                let time_part = parts[1];
                if !time_part.is_empty() {
                    // Basic time validation - could be extended for full RFC 3339
                    return Ok(());
                }
            }
        }
    }

    Err(ValidationFinding {
        rule_id: "XBRL.DIMENSION.INVALID_TYPED_VALUE".to_string(),
        severity: "error".to_string(),
        message: format!(
            "Value '{}' is not a valid dateTime for dimension {}",
            value, dim_member.dimension
        ),
        member: Some(dim_member.dimension.clone()),
        subject: Some(value.to_string()),
    })
}

/// Validate boolean format.
fn validate_boolean(value: &str, dim_member: &DimensionMember) -> Result<(), ValidationFinding> {
    let trimmed = value.trim().to_lowercase();

    if matches!(trimmed.as_str(), "true" | "false" | "1" | "0") {
        return Ok(());
    }

    Err(ValidationFinding {
        rule_id: "XBRL.DIMENSION.INVALID_TYPED_VALUE".to_string(),
        severity: "error".to_string(),
        message: format!(
            "Value '{}' is not a valid boolean (expected true/false/1/0) for dimension {}",
            value, dim_member.dimension
        ),
        member: Some(dim_member.dimension.clone()),
        subject: Some(value.to_string()),
    })
}

/// Validate URI format.
fn validate_uri(value: &str, dim_member: &DimensionMember) -> Result<(), ValidationFinding> {
    // Basic URI validation: must have scheme:// or be an absolute/relative path
    let trimmed = value.trim();

    // Check for common URI patterns
    if trimmed.contains("://") || trimmed.starts_with('/') || trimmed.starts_with("./") {
        return Ok(());
    }

    Err(ValidationFinding {
        rule_id: "XBRL.DIMENSION.INVALID_TYPED_VALUE".to_string(),
        severity: "error".to_string(),
        message: format!(
            "Value '{}' is not a valid URI for dimension {}",
            value, dim_member.dimension
        ),
        member: Some(dim_member.dimension.clone()),
        subject: Some(value.to_string()),
    })
}
