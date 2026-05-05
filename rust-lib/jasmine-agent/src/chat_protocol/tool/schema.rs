use serde_json::{json, Value};

/// Clean a tool definition for cross-provider compatibility.
/// - Removes `$schema` key
/// - Converts `const` to `enum` for providers that don't support const
/// - Recursively processes nested schemas
pub fn clean_tool_for_compatibility(tool: &Value) -> Value {
    let mut out = tool.clone();
    if let Some(obj) = out.as_object_mut() {
        obj.remove("$schema");
        // Process function.parameters
        if let Some(function) = obj.get_mut("function") {
            if let Some(fn_obj) = function.as_object_mut() {
                if let Some(params) = fn_obj.get_mut("parameters") {
                    let cleaned = clean_schema_node(params);
                    *params = cleaned;
                }
            }
        }
        // Also handle top-level parameters (Responses API format)
        if let Some(params) = obj.get_mut("parameters") {
            let cleaned = clean_schema_node(params);
            *params = cleaned;
        }
    }
    out
}

/// Recursively clean a JSON schema node.
fn clean_schema_node(node: &Value) -> Value {
    match node {
        Value::Object(obj) => {
            let mut out = obj.clone();
            // Convert const → enum
            if let Some(const_val) = out.remove("const") {
                if const_val.is_string() || const_val.is_number() || const_val.is_boolean() {
                    out.insert("enum".to_string(), json!([const_val]));
                }
            }
            // Remove $schema recursively
            out.remove("$schema");
            // Process properties
            if let Some(props) = out.get_mut("properties") {
                if let Some(props_obj) = props.as_object_mut() {
                    let cleaned: serde_json::Map<String, Value> = props_obj
                        .iter()
                        .map(|(k, v)| (k.clone(), clean_schema_node(v)))
                        .collect();
                    *props = Value::Object(cleaned);
                }
            }
            // Process items
            if let Some(items) = out.get_mut("items") {
                let cleaned = clean_schema_node(items);
                *items = cleaned;
            }
            Value::Object(out)
        }
        _ => node.clone(),
    }
}

/// Convert tools from function-call format to Responses API flat format.
pub fn to_responses_tools_format(tools: &[Value]) -> Vec<Value> {
    tools
        .iter()
        .map(|tool| {
            if tool.get("type").and_then(|v| v.as_str()) != Some("function") {
                return tool.clone();
            }
            let fn_obj = match tool.get("function").and_then(|v| v.as_object()) {
                Some(obj) => obj,
                None => return tool.clone(),
            };
            let mut out = serde_json::Map::new();
            out.insert("type".into(), json!("function"));
            if let Some(name) = fn_obj.get("name") {
                out.insert("name".into(), name.clone());
            }
            if let Some(desc) = fn_obj.get("description") {
                out.insert("description".into(), desc.clone());
            }
            if let Some(params) = fn_obj.get("parameters") {
                out.insert("parameters".into(), params.clone());
            }
            if let Some(strict) = tool.get("strict").or(fn_obj.get("strict")) {
                if let Some(b) = strict.as_bool() {
                    out.insert("strict".into(), json!(b));
                }
            }
            Value::Object(out)
        })
        .collect()
}

/// Convert OpenAI-style tools to Claude's input_schema format.
pub fn to_claude_tools_format(tools: &[Value]) -> Vec<Value> {
    tools
        .iter()
        .filter_map(|tool| {
            let fn_val = tool.get("function")?;
            let name = fn_val.get("name").and_then(|v| v.as_str())?;
            let desc = fn_val
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let params = fn_val
                .get("parameters")
                .cloned()
                .unwrap_or_else(|| json!({"type": "object"}));

            let mut entry = json!({
                "name": name,
                "input_schema": params,
            });
            if !desc.is_empty() {
                entry["description"] = json!(desc);
            }
            Some(entry)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_const_to_enum() {
        let schema = json!({"type": "string", "const": "hello"});
        let cleaned = clean_schema_node(&schema);
        assert!(cleaned.get("const").is_none());
        assert!(cleaned.get("enum").is_some());
    }

    #[test]
    fn test_to_claude_tools() {
        let tools = vec![json!({
            "type": "function",
            "function": {"name": "search", "description": "search the web", "parameters": {"type": "object"}}
        })];
        let result = to_claude_tools_format(&tools);
        assert_eq!(result[0]["name"], "search");
        assert!(result[0].get("input_schema").is_some());
    }
}
