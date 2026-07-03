use serde_json::{json, Value};

/// Generates all meaningful JSON value variations from an OpenAPI schema node.
///
/// For enums, one item per variant. For nullable fields, null plus the real
/// value. For booleans, both false and true. For objects, one base object
/// with all properties set, plus one extra object per field that has more
/// than one variant (with that field swapped to its alternate value).
/// Arrays are always empty. Scalars produce a single item.
pub fn generate_json_variants_from_schema(schema: &Value, schemas: &Value) -> Vec<Value> {
    if let Some(ref_path) = schema.get("$ref").and_then(|v| v.as_str()) {
        let name = ref_path.trim_start_matches("#/components/schemas/");
        return generate_json_variants_from_schema(&schemas[name].clone(), schemas);
    }

    if let Some(c) = schema.get("const") {
        return vec![c.clone()];
    }

    if let Some(variants) = schema.get("enum").and_then(|v| v.as_array()) {
        return variants.clone();
    }

    // oneOf: generate variants for each sub-schema and concatenate them all.
    if let Some(one_of) = schema.get("oneOf").and_then(|v| v.as_array()) {
        return one_of
            .iter()
            .flat_map(|s| generate_json_variants_from_schema(s, schemas))
            .collect();
    }

    let type_val = schema.get("type");

    if let Some(arr) = type_val.and_then(|t| t.as_array()) {
        let mut results = vec![Value::Null];
        if let Some(t) = arr.iter().find(|t| *t != "null").and_then(|t| t.as_str()) {
            results.extend(generate_json_variants_from_type(t, schema, schemas));
        }
        return results;
    }

    match type_val.and_then(|t| t.as_str()) {
        Some(t) => generate_json_variants_from_type(t, schema, schemas),
        None => vec![Value::Null],
    }
}

fn generate_json_variants_from_type(type_str: &str, schema: &Value, schemas: &Value) -> Vec<Value> {
    match type_str {
        "object" => {
            let props = match schema.get("properties").and_then(|p| p.as_object()) {
                Some(p) => p,
                None => return vec![Value::Object(serde_json::Map::new())],
            };

            // Build the cartesian product of all field variants. Start with
            // one empty combination and expand it field by field.
            let mut combinations: Vec<serde_json::Map<String, Value>> =
                vec![serde_json::Map::new()];

            for (field, field_schema) in props {
                let field_variants =
                    generate_json_variants_from_schema(field_schema, schemas);
                let mut next = Vec::new();
                for existing in &combinations {
                    for variant in &field_variants {
                        let mut combo = existing.clone();
                        combo.insert(field.clone(), variant.clone());
                        next.push(combo);
                    }
                }
                combinations = next;
            }

            combinations.into_iter().map(Value::Object).collect()
        }
        "boolean" => vec![json!(false), json!(true)],
        "array" => vec![Value::Array(vec![])],
        "string" => vec![match schema.get("format").and_then(|f| f.as_str()) {
            Some("date-time") => json!("2024-01-01T00:00:00Z"),
            Some("email") => json!("test@example.com"),
            _ => json!(""),
        }],
        "integer" => vec![json!(0)],
        "number" => vec![json!(0.0)],
        _ => vec![Value::Null],
    }
}
