use anyhow::{Context, bail};
use serde_json::Value;
use std::path::Path;

pub(super) fn run() -> anyhow::Result<()> {
    write_contract_artifacts()?;

    let repo_root = super::repo_root();
    let checks = [
        (
            "contracts/schemas/feature.grid.v1.json",
            "artifacts/feature.grid.v1.json",
        ),
        (
            "contracts/schemas/bundle.manifest.v1.json",
            "artifacts/bundles/AC-XK-SEC-INLINE-001.json",
        ),
        (
            "contracts/schemas/impact.report.v1.json",
            "artifacts/impact/impact.report.v1.json",
        ),
        (
            "contracts/schemas/scenario.run.v1.json",
            "artifacts/runs/scenario.run.v1.json",
        ),
        (
            "contracts/schemas/ixds.assembly.v1.json",
            "artifacts/ixds/ixds.assembly.v1.json",
        ),
        (
            "contracts/schemas/taxonomy.resolve.v1.json",
            "artifacts/taxonomy/taxonomy.resolve.v1.json",
        ),
        (
            "contracts/schemas/validation.report.v1.json",
            "artifacts/validation/validation.report.v1.json",
        ),
    ];

    for (schema_path, document_path) in checks {
        validate_document(&repo_root.join(schema_path), &repo_root.join(document_path))?;
    }

    println!("schema-check: validated 7 artifact(s)");
    Ok(())
}

fn write_contract_artifacts() -> anyhow::Result<()> {
    let grid = super::load_grid()?;
    super::write_json(
        &super::repo_root().join("artifacts/feature.grid.v1.json"),
        &grid,
    )?;
    super::bundle("AC-XK-SEC-INLINE-001")?;
    super::impact(&["specs/features/workflow/bundle.feature".to_string()])?;
    super::test_ac("AC-XK-TAXONOMY-001")?;
    super::test_ac("AC-XK-IXDS-002")?;
    Ok(())
}

fn validate_document(schema_path: &Path, document_path: &Path) -> anyhow::Result<()> {
    let schema = read_json(schema_path)?;
    let document = read_json(document_path)?;
    validate_value(&schema, &schema, &document, "$")
        .with_context(|| format!("validating {}", document_path.display()))
}

fn read_json(path: &Path) -> anyhow::Result<Value> {
    let bytes =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&bytes).with_context(|| format!("parsing {}", path.display()))
}

fn validate_value(
    root_schema: &Value,
    schema: &Value,
    value: &Value,
    path: &str,
) -> anyhow::Result<()> {
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        let resolved = resolve_ref(root_schema, reference)?;
        return validate_value(root_schema, resolved, value, path);
    }

    if let Some(type_decl) = schema.get("type") {
        validate_type(type_decl, value, path)?;
    }

    if let Some(const_value) = schema.get("const") {
        if value != const_value {
            bail!("{path}: expected constant value {const_value}");
        }
    }

    if let Some(enum_values) = schema.get("enum").and_then(Value::as_array) {
        if !enum_values.iter().any(|candidate| candidate == value) {
            bail!("{path}: value {value} is not in enum {enum_values:?}");
        }
    }

    if let Some(required) = schema.get("required").and_then(Value::as_array) {
        let object = value
            .as_object()
            .with_context(|| format!("{path}: required properties apply only to objects"))?;
        for property in required {
            let Some(property_name) = property.as_str() else {
                bail!("{path}: required list contains a non-string entry");
            };
            if !object.contains_key(property_name) {
                bail!("{path}: missing required property {property_name}");
            }
        }
    }

    if let Some(properties) = schema.get("properties").and_then(Value::as_object) {
        let object = value
            .as_object()
            .with_context(|| format!("{path}: properties apply only to objects"))?;
        for (property_name, property_schema) in properties {
            if let Some(property_value) = object.get(property_name) {
                validate_value(
                    root_schema,
                    property_schema,
                    property_value,
                    &format!("{path}.{property_name}"),
                )?;
            }
        }

        if schema
            .get("additionalProperties")
            .and_then(Value::as_bool)
            .is_some_and(|allowed| !allowed)
        {
            for property_name in object.keys() {
                if !properties.contains_key(property_name) {
                    bail!("{path}: unexpected property {property_name}");
                }
            }
        }
    }

    if let Some(items_schema) = schema.get("items") {
        let array = value
            .as_array()
            .with_context(|| format!("{path}: items apply only to arrays"))?;
        for (index, item) in array.iter().enumerate() {
            validate_value(root_schema, items_schema, item, &format!("{path}[{index}]"))?;
        }
    }

    Ok(())
}

fn validate_type(type_decl: &Value, value: &Value, path: &str) -> anyhow::Result<()> {
    match type_decl {
        Value::String(expected_type) => {
            if matches_type(expected_type, value) {
                Ok(())
            } else {
                bail!("{path}: expected type {expected_type}");
            }
        }
        Value::Array(expected_types) => {
            let matches_any = expected_types.iter().any(|candidate| {
                candidate
                    .as_str()
                    .is_some_and(|expected_type| matches_type(expected_type, value))
            });
            if matches_any {
                Ok(())
            } else {
                bail!("{path}: value did not match any allowed type");
            }
        }
        _ => bail!("{path}: unsupported schema type declaration"),
    }
}

fn matches_type(expected_type: &str, value: &Value) -> bool {
    match expected_type {
        "array" => value.is_array(),
        "boolean" => value.is_boolean(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "null" => value.is_null(),
        "number" => value.is_number(),
        "object" => value.is_object(),
        "string" => value.is_string(),
        _ => false,
    }
}

fn resolve_ref<'a>(root_schema: &'a Value, reference: &str) -> anyhow::Result<&'a Value> {
    let pointer = reference
        .strip_prefix('#')
        .with_context(|| format!("only local refs are supported: {reference}"))?;
    root_schema
        .pointer(pointer)
        .with_context(|| format!("unresolved schema reference {reference}"))
}

#[cfg(test)]
mod tests {
    use super::validate_value;
    use serde_json::json;

    #[test]
    fn validates_simple_schema() {
        let schema = json!({
            "type": "object",
            "additionalProperties": false,
            "properties": {
                "name": { "type": "string" },
                "tags": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            },
            "required": ["name", "tags"]
        });
        let document = json!({
            "name": "xbrlkit",
            "tags": ["alpha", "engine"]
        });

        validate_value(&schema, &schema, &document, "$").expect("schema should validate");
    }

    #[test]
    fn rejects_missing_required_property() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": { "type": "string" }
            },
            "required": ["name"]
        });
        let document = json!({});

        let error =
            validate_value(&schema, &schema, &document, "$").expect_err("validation should fail");

        assert!(error.to_string().contains("missing required property"));
    }
}
