use serde_json::Value;

use crate::chat_protocol::ToolCallInfo;


/// Parsed Claude SSE event types.
#[derive(Clone, Debug, serde::Serialize)]
pub enum ClaudeEvent {
    MessageStart,
    ContentBlockStart {
        index: i32,
        content_type: String,
        id: Option<String>,
        name: Option<String>,
    },
    ContentBlockDelta {
        index: i32,
        text_delta: Option<String>,
        thinking_delta: Option<String>,
        signature_delta: Option<String>,
        input_json_delta: Option<String>,
    },
    ContentBlockStop {
        index: i32,
    },
    MessageDelta {
        stop_reason: Option<String>,
        stop_sequence: Option<String>,
    },
    MessageStop,
    Ping,
    Usage(Option<ClaudeUsage>),
    Error(String),
    Unknown,
}

/// Token usage from a Claude event.
#[derive(Clone, Debug, serde::Serialize)]
pub struct ClaudeUsage {
    pub input_tokens: i32,
    pub output_tokens: i32,
    pub cache_read_input_tokens: Option<i32>,
    pub cache_creation_input_tokens: Option<i32>,
}

/// Parse a JSON value from Claude's SSE into a typed event.
pub fn parse_claude_event(json: &Value) -> ClaudeEvent {
    let event_type = json.get("type").and_then(|v| v.as_str()).unwrap_or("");

    match event_type {
        "message_start" => {
            let _usage = extract_claude_usage(json.get("message"));
            ClaudeEvent::MessageStart
        }

        "content_block_start" => {
            let block = match json.get("content_block").and_then(|b| b.as_object()) {
                Some(b) => b,
                None => return ClaudeEvent::Unknown,
            };
            let index = json.get("index").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let content_type = block
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let id = block.get("id").and_then(|v| v.as_str()).map(String::from);
            let name = block.get("name").and_then(|v| v.as_str()).map(String::from);

            ClaudeEvent::ContentBlockStart {
                index,
                content_type,
                id,
                name,
            }
        }

        "content_block_delta" => {
            let index = json.get("index").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            let delta = json.get("delta");
            let text_delta = delta
                .and_then(|d| d.get("text"))
                .and_then(|v| v.as_str())
                .map(String::from);
            let thinking_delta = delta
                .and_then(|d| d.get("thinking"))
                .and_then(|v| v.as_str())
                .map(String::from);
            let signature_delta = delta
                .and_then(|d| d.get("signature"))
                .and_then(|v| v.as_str())
                .map(String::from);
            let input_json_delta = delta
                .and_then(|d| d.get("input_json"))
                .and_then(|v| v.as_str())
                .or_else(|| {
                    delta
                        .and_then(|d| d.get("partial_json"))
                        .and_then(|v| v.as_str())
                })
                .map(String::from);

            ClaudeEvent::ContentBlockDelta {
                index,
                text_delta,
                thinking_delta,
                signature_delta,
                input_json_delta,
            }
        }

        "content_block_stop" => {
            let index = json.get("index").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
            ClaudeEvent::ContentBlockStop { index }
        }

        "message_delta" => {
            let delta = json.get("delta");
            let stop_reason = delta
                .and_then(|d| d.get("stop_reason"))
                .and_then(|v| v.as_str())
                .map(String::from);
            let stop_sequence = delta
                .and_then(|d| d.get("stop_sequence"))
                .and_then(|v| v.as_str())
                .map(String::from);
            let _usage = extract_claude_usage(Some(json));
            ClaudeEvent::MessageDelta {
                stop_reason,
                stop_sequence,
            }
        }

        "message_stop" => {
            let _usage = extract_claude_usage(Some(json));
            ClaudeEvent::MessageStop
        }

        "ping" => ClaudeEvent::Ping,

        "error" => {
            let msg = json
                .get("error")
                .and_then(|e| e.get("message"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown error")
                .to_string();
            ClaudeEvent::Error(msg)
        }

        _ => ClaudeEvent::Unknown,
    }
}

/// Extract token usage from a Claude message_start or message_delta event.
pub fn extract_claude_usage(message: Option<&Value>) -> Option<ClaudeUsage> {
    let usage = message?.get("usage")?;
    Some(ClaudeUsage {
        input_tokens: usage.get("input_tokens").and_then(|v| v.as_i64()).unwrap_or(0) as i32,
        output_tokens: usage
            .get("output_tokens")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32,
        cache_read_input_tokens: usage
            .get("cache_read_input_tokens")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
        cache_creation_input_tokens: usage
            .get("cache_creation_input_tokens")
            .and_then(|v| v.as_i64())
            .map(|v| v as i32),
    })
}

/// Convert Claude tool_use blocks back to OpenAI-compatible ToolCallInfo format.
///
/// This is called after the stream completes when we have full tool_use blocks,
/// converting the Claude-native format to the shared ToolCallInfo used by the app.
pub fn claude_blocks_to_tool_calls(blocks: &[Value]) -> Vec<ToolCallInfo> {
    blocks
        .iter()
        .filter_map(|block| {
            let type_str = block.get("type").and_then(|v| v.as_str())?;
            if type_str != "tool_use" {
                return None;
            }
            let id = block.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let name = block.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
            let input = block.get("input").cloned().unwrap_or(Value::Null);
            if id.is_empty() || name.is_empty() {
                return None;
            }
            Some(ToolCallInfo {
                id,
                name,
                arguments: input,
                metadata: None,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_parse_content_block_start() {
        let json = json!({
            "type": "content_block_start",
            "index": 0,
            "content_block": {"type": "tool_use", "id": "tool_1", "name": "search"}
        });
        match parse_claude_event(&json) {
            ClaudeEvent::ContentBlockStart { index, content_type, id, name } => {
                assert_eq!(index, 0);
                assert_eq!(content_type, "tool_use");
                assert_eq!(id, Some("tool_1".into()));
                assert_eq!(name, Some("search".into()));
            }
            _ => panic!("Expected ContentBlockStart"),
        }
    }

    #[test]
    fn test_parse_text_delta() {
        let json = json!({
            "type": "content_block_delta",
            "index": 0,
            "delta": {"type": "text_delta", "text": "Hello"}
        });
        match parse_claude_event(&json) {
            ClaudeEvent::ContentBlockDelta { text_delta, .. } => {
                assert_eq!(text_delta, Some("Hello".into()));
            }
            _ => panic!("Expected ContentBlockDelta"),
        }
    }

    #[test]
    fn test_parse_message_stop() {
        let json = json!({
            "type": "message_delta",
            "delta": {"stop_reason": "end_turn"}
        });
        match parse_claude_event(&json) {
            ClaudeEvent::MessageDelta { stop_reason, .. } => {
                assert_eq!(stop_reason, Some("end_turn".into()));
            }
            _ => panic!("Expected MessageDelta"),
        }
    }
}
