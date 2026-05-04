use serde_json::{json, Value};

/// Normalize tool arguments based on the tool's JSON Schema.
pub fn normalize_args_for_tool(schema: Option<&Value>, args: &Value) -> Value {
    let schema = match schema {
        Some(s) if !s.is_null() => s,
        _ => return args.clone(),
    };
    // Deep-clone via JSON round-trip to avoid mutating the original
    let cloned: Value = serde_json::from_str(&serde_json::to_string(args).unwrap())
        .unwrap_or_else(|_| args.clone());
    if let Some(normalized) = normalize_by_schema(&cloned, schema, None) {
        normalize_special_cases("", &normalized)
    } else {
        normalize_special_cases("", args)
    }
}

/// Recursively normalise a value to match its JSON Schema definition.
fn normalize_by_schema(
    value: &Value,
    schema: &Value,
    property_name: Option<&str>,
) -> Option<Value> {
    // Handle anyOf / oneOf unions
    let unions = schema_unions(schema);
    if !unions.is_empty() {
        // Source heuristic: if value is a string and property is "sources", try object branch
        if let (Value::String(_), Some("sources")) = (value, property_name) {
            let obj_branch = unions.iter().find(|m| {
                let types = schema_types(m);
                types.contains(&"object".to_string())
                    && m.get("properties").and_then(|p| p.get("type")).is_some()
            });
            if let Some(branch) = obj_branch {
                return normalize_by_schema(
                    &json!({"type": value.as_str().unwrap_or("")}),
                    branch,
                    property_name,
                );
            }
        }
        // Try each branch
        for branch in &unions {
            if let Some(result) = normalize_by_schema(value, branch, property_name) {
                return Some(result);
            }
        }
        // Fall through to first branch
        return normalize_by_schema(value, &unions[0], property_name);
    }

    let declared_types = schema_types(schema);

    if declared_types.contains(&"object".to_string()) {
        let props = schema
            .get("properties")
            .and_then(|p| p.as_object())
            .cloned()
            .unwrap_or_default();
        let required: std::collections::HashSet<String> = schema
            .get("required")
            .and_then(|r| r.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|v| v.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();
        let mut out = serde_json::Map::new();

        if let Value::Object(input) = value {
            // Copy passthrough unknowns
            for (k, v) in input {
                if !props.contains_key(k) {
                    out.insert(k.clone(), v.clone());
                }
            }
            for (key, prop_schema) in &props {
                let v = input.get(key).cloned();
                let v = match v {
                    None => {
                        if let Some(d) = prop_schema.get("default") {
                            Some(d.clone())
                        } else if required.contains(key) {
                            let enum_vals = schema_enum(prop_schema);
                            if !enum_vals.is_empty() {
                                Some(enum_vals[0].clone())
                            } else if key == "waitFor" {
                                let types = schema_types(prop_schema);
                                if types.iter().any(|t| t == "number" || t == "integer") {
                                    Some(json!(0))
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    }
                    other => other,
                };
                if let Some(val) = v {
                    out.insert(
                        key.clone(),
                        normalize_by_schema(&val, prop_schema, Some(key)).unwrap_or(val),
                    );
                }
            }
        }
        return Some(Value::Object(out));
    }

    if declared_types.contains(&"array".to_string()) {
        let items = schema.get("items").cloned().unwrap_or(json!({}));
        let list = match value {
            Value::Array(a) => a.clone(),
            _ => vec![value.clone()],
        };
        let mut out: Vec<Value> = Vec::new();
        for item in &list {
            let mut iv = item.clone();
            // Heuristic: sources array of strings → convert to {type: string}
            let item_types = schema_types(&items);
            if property_name == Some("sources")
                && item.is_string()
                && item_types.contains(&"object".to_string())
            {
                if items
                    .get("properties")
                    .and_then(|p| p.get("type"))
                    .is_some()
                {
                    iv = json!({"type": item.as_str().unwrap_or("")});
                }
            }
            out.push(normalize_by_schema(&iv, &items, property_name).unwrap_or(iv));
        }
        return Some(Value::Array(out));
    }

    if declared_types.contains(&"boolean".to_string()) {
        match value {
            Value::Bool(_) => return Some(value.clone()),
            Value::String(s) => {
                let lower = s.to_lowercase();
                if lower == "true" || lower == "1" || lower == "yes" {
                    return Some(json!(true));
                }
                if lower == "false" || lower == "0" || lower == "no" {
                    return Some(json!(false));
                }
            }
            _ => {}
        }
        return Some(value.clone());
    }

    if declared_types.contains(&"integer".to_string()) {
        match value {
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    return Some(json!(i));
                }
                if let Some(f) = n.as_f64() {
                    return Some(json!(f as i64));
                }
            }
            Value::String(s) => {
                if let Ok(i) = s.parse::<i64>() {
                    return Some(json!(i));
                }
            }
            _ => {}
        }
        return Some(value.clone());
    }

    if declared_types.contains(&"number".to_string()) {
        match value {
            Value::Number(_) => return Some(value.clone()),
            Value::String(s) => {
                if let Ok(f) = s.parse::<f64>() {
                    return Some(json!(f));
                }
            }
            _ => {}
        }
        return Some(value.clone());
    }

    if declared_types.contains(&"string".to_string()) {
        if let Value::String(_) = value {
            return Some(value.clone());
        }
        return Some(Value::String(format!("{}", value)));
    }

    // No declared type: return as-is
    Some(value.clone())
}

/// Normalize special cases for certain tool names (e.g., firecrawl_search).
fn normalize_special_cases(_tool_name: &str, args: &Value) -> Value {
    args.clone()
}

fn schema_unions(schema: &Value) -> Vec<Value> {
    let mut out = Vec::new();
    if let Some(any_of) = schema.get("anyOf").and_then(|v| v.as_array()) {
        out.extend(any_of.iter().cloned());
    }
    if let Some(one_of) = schema.get("oneOf").and_then(|v| v.as_array()) {
        out.extend(one_of.iter().cloned());
    }
    out
}

fn schema_types(schema: &Value) -> Vec<String> {
    match schema.get("type") {
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(a)) => a
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        _ => vec![],
    }
}

fn schema_enum(schema: &Value) -> Vec<Value> {
    schema
        .get("enum")
        .and_then(|e| e.as_array())
        .cloned()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_integer_conversion() {
        let schema = json!({"type": "integer"});
        assert_eq!(
            normalize_by_schema(&json!("42"), &schema, None),
            Some(json!(42))
        );
    }

    #[test]
    fn test_normalize_boolean_conversion() {
        let schema = json!({"type": "boolean"});
        assert_eq!(
            normalize_by_schema(&json!("true"), &schema, None),
            Some(json!(true))
        );
        assert_eq!(
            normalize_by_schema(&json!("false"), &schema, None),
            Some(json!(false))
        );
    }

    #[test]
    fn test_normalize_object_with_defaults() {
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "count": {"type": "integer", "default": 0}
            },
            "required": ["name"]
        });
        let result = normalize_by_schema(&json!({"name": "test"}), &schema, None);
        assert_eq!(result, Some(json!({"name": "test", "count": 0})));
    }
}
