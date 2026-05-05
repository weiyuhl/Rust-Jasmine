use serde_json::{json, Value};
use std::collections::HashMap;

/// Copy and sanitize a single chat message, preserving key fields.
/// Mirrors Kelivo's `_copyChatCompletionMessage`.
pub fn copy_chat_message(msg: &Value) -> Value {
    let role = msg
        .get("role")
        .and_then(|v| v.as_str())
        .unwrap_or("user")
        .to_string();

    let content = msg.get("content").cloned().unwrap_or_else(|| json!(""));

    let mut out = json!({
        "role": role,
        "content": content,
    });

    // Preserve optional name on non-tool roles
    if role != "tool" {
        if let Some(name) = msg.get("name").and_then(|v| v.as_str()) {
            if !name.is_empty() {
                out["name"] = json!(name);
            }
        }
    }

    // Preserve assistant tool_calls + reasoning
    if role == "assistant" {
        if let Some(tool_calls) = msg.get("tool_calls").and_then(|v| v.as_array()) {
            if !tool_calls.is_empty() {
                let cleaned: Vec<Value> = tool_calls
                    .iter()
                    .filter_map(|tc| tc.as_object().map(|obj| {
                        let mut copy = obj.clone();
                        copy.remove("metadata");
                        Value::Object(copy)
                    }))
                    .collect();
                if !cleaned.is_empty() {
                    out["tool_calls"] = json!(cleaned);
                }
            }
        }
        if let Some(fc) = msg.get("function_call") {
            out["function_call"] = fc.clone();
        }
        if let Some(rc) = msg.get("reasoning_content") {
            out["reasoning_content"] = rc.clone();
        }
    }

    // Preserve tool role linkage
    if role == "tool" {
        if let Some(tci) = msg.get("tool_call_id").and_then(|v| v.as_str()) {
            if !tci.is_empty() {
                out["tool_call_id"] = json!(tci);
            }
        }
    }

    out
}

/// Extract system prompts from a message list.
/// Returns (concatenated system prompts, non-system messages).
pub fn extract_system_prompts(messages: &[Value]) -> (Option<String>, Vec<Value>) {
    let mut system = String::new();
    let mut rest = Vec::new();

    for msg in messages {
        let role = msg.get("role").and_then(|v| v.as_str()).unwrap_or("");
        if role == "system" {
            let content = msg.get("content").and_then(|v| v.as_str()).unwrap_or("");
            if !content.is_empty() {
                if system.is_empty() {
                    system = content.to_string();
                } else {
                    system = format!("{}\n\n{}", system, content);
                }
            }
        } else {
            rest.push(msg.clone());
        }
    }

    let system_opt = if system.is_empty() { None } else { Some(system) };
    (system_opt, rest)
}

/// Merge custom headers from extraHeaders and model overrides.
pub fn merge_custom_headers(
    base: &mut HashMap<String, String>,
    custom_headers: Option<&HashMap<String, String>>,
    extra_headers: Option<&HashMap<String, String>>,
) {
    if let Some(ch) = custom_headers {
        for (k, v) in ch {
            base.insert(k.clone(), v.clone());
        }
    }
    if let Some(eh) = extra_headers {
        for (k, v) in eh {
            base.insert(k.clone(), v.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_assistant_with_tool_calls() {
        let msg = json!({
            "role": "assistant",
            "content": "",
            "tool_calls": [{"id": "call_1", "function": {"name": "search", "arguments": "{}"}, "metadata": {}}]
        });
        let result = copy_chat_message(&msg);
        let tcs = result["tool_calls"].as_array().unwrap();
        assert_eq!(tcs.len(), 1);
        assert!(tcs[0].get("metadata").is_none());
    }

    #[test]
    fn test_extract_system_prompts() {
        let msgs = vec![
            json!({"role": "system", "content": "You are helpful."}),
            json!({"role": "user", "content": "hi"}),
        ];
        let (sys, rest) = extract_system_prompts(&msgs);
        assert_eq!(sys, Some("You are helpful.".to_string()));
        assert_eq!(rest.len(), 1);
    }
}
