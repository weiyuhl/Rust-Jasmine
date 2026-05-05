use serde_json::Value;

use super::tool_calls::ToolCallDelta;

/// A parsed delta from an OpenAI-compatible SSE chunk.
#[derive(Clone, Debug, serde::Serialize)]
pub struct OpenAIDelta {
    pub content: String,
    pub reasoning_content: Option<String>,
    pub reasoning_details: Option<Value>,
    pub role: Option<String>,
    pub finish_reason: Option<String>,
    pub tool_call_deltas: Vec<ToolCallDelta>,
    pub usage: Option<OpenAIUsage>,
    pub images: Option<Vec<Value>>,
}

/// Token usage extracted from an OpenAI chunk.
#[derive(Clone, Debug, serde::Serialize)]
pub struct OpenAIUsage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

/// Parse an OpenAI-compatible JSON chunk from SSE.
///
/// Handles:
/// - `choices[0].delta.content` → content delta
/// - `choices[0].delta.reasoning_content` → reasoning text
/// - `choices[0].delta.reasoning_details` → reasoning metadata
/// - `choices[0].delta.tool_calls[]` → tool call delta
/// - `choices[0].finish_reason` → stream completion signal
/// - `usage` → token usage
/// - `images` / `choices[0].delta.content_parts[].image_url` → image output
pub fn parse_openai_chunk(json: &Value) -> OpenAIDelta {
    let choices = json.get("choices").and_then(|c| c.as_array());
    if choices.is_none() {
        return OpenAIDelta {
            content: String::new(),
            reasoning_content: None,
            reasoning_details: None,
            role: None,
            finish_reason: None,
            tool_call_deltas: Vec::new(),
            usage: extract_usage(json),
            images: None,
        };
    }

    let choices = choices.unwrap();
    if choices.is_empty() {
        return OpenAIDelta {
            content: String::new(),
            reasoning_content: None,
            reasoning_details: None,
            role: None,
            finish_reason: None,
            tool_call_deltas: Vec::new(),
            usage: extract_usage(json),
            images: None,
        };
    }

    let c0 = &choices[0];
    let delta = c0.get("delta");

    let content = delta
        .and_then(|d| d.get("content"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let reasoning_content = delta
        .and_then(|d| d.get("reasoning_content"))
        .and_then(|v| v.as_str())
        .map(String::from);

    let reasoning_details = delta.and_then(|d| d.get("reasoning_details")).cloned();

    let role = delta.and_then(|d| d.get("role")).and_then(|v| v.as_str()).map(String::from);

    let finish_reason = c0
        .get("finish_reason")
        .and_then(|v| v.as_str())
        .map(String::from);

    // Tool call deltas
    let tool_call_deltas = extract_tool_call_deltas(delta);

    // Image output
    let images = extract_images(delta);

    OpenAIDelta {
        content,
        reasoning_content,
        reasoning_details,
        role,
        finish_reason,
        tool_call_deltas,
        usage: extract_usage(json),
        images,
    }
}

/// Extract token usage from the top-level `usage` field.
pub fn extract_usage(json: &Value) -> Option<OpenAIUsage> {
    let usage = json.get("usage")?;
    Some(OpenAIUsage {
        prompt_tokens: usage.get("prompt_tokens").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
        completion_tokens: usage
            .get("completion_tokens")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32,
        total_tokens: usage.get("total_tokens").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
    })
}

fn extract_tool_call_deltas(delta: Option<&Value>) -> Vec<ToolCallDelta> {
    let tool_calls = match delta.and_then(|d| d.get("tool_calls")).and_then(|v| v.as_array()) {
        Some(tc) => tc,
        None => return Vec::new(),
    };

    tool_calls
        .iter()
        .filter_map(|tc| {
            let index = tc.get("index").and_then(|v| v.as_i64()).unwrap_or(0);
            let id = tc
                .get("id")
                .and_then(|v| v.as_str())
                .map(String::from);
            let function = tc.get("function")?;
            let name = function
                .get("name")
                .and_then(|v| v.as_str())
                .map(String::from);
            let args_fragment = function
                .get("arguments")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            Some(ToolCallDelta {
                index: index as i32,
                id,
                name,
                args_fragment: if args_fragment.is_empty() { None } else { Some(args_fragment) },
            })
        })
        .collect()
}

fn extract_images(delta: Option<&Value>) -> Option<Vec<Value>> {
    let delta = delta?;
    // Direct images array on delta
    if let Some(imgs) = delta.get("images").and_then(|v| v.as_array()) {
        return Some(imgs.clone());
    }
    // content_parts with image_url type
    if let Some(parts) = delta.get("content_parts").and_then(|v| v.as_array()) {
        let images: Vec<Value> = parts
            .iter()
            .filter(|p| p.get("type").and_then(|t| t.as_str()) == Some("image_url"))
            .cloned()
            .collect();
        if !images.is_empty() {
            return Some(images);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_content_delta() {
        let json = json!({
            "choices": [{"delta": {"content": "Hello"}, "index": 0}]
        });
        let delta = parse_openai_chunk(&json);
        assert_eq!(delta.content, "Hello");
        assert!(delta.finish_reason.is_none());
    }

    #[test]
    fn test_parse_tool_call_delta() {
        let json = json!({
            "choices": [{"delta": {
                "tool_calls": [{"index": 0, "id": "call_1", "function": {"name": "search", "arguments": r#"{"q":"hello"}"#}}]
            }, "index": 0}]
        });
        let delta = parse_openai_chunk(&json);
        assert_eq!(delta.tool_call_deltas.len(), 1);
    }

    #[test]
    fn test_parse_finish_reason() {
        let json = json!({
            "choices": [{"delta": {}, "finish_reason": "stop", "index": 0}]
        });
        let delta = parse_openai_chunk(&json);
        assert_eq!(delta.finish_reason, Some("stop".to_string()));
    }
}
