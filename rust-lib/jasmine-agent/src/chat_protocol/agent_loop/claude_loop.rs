use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use super::{AgentEvent, ToolCallInfo, ToolResultInfo};
use crate::chat_protocol::request::claude as claude_req;

/// Execute the Claude agent loop.
///
/// Sends the initial request, parses SSE chunks, yields events via callback,
/// executes tool calls, and loops until no more tool calls are returned.
pub fn run_claude_agent_loop<F>(
    base_url: &str,
    api_key: &str,
    model_id: &str,
    messages: &[Value],
    system_prompt: Option<&str>,
    tools: Option<&[Value]>,
    stream: bool,
    is_reasoning: bool,
    thinking_budget: Option<i32>,
    temperature: Option<f64>,
    max_tokens: Option<i32>,
    custom_headers: Option<&[(String, String)]>,
    custom_body: Option<&Value>,
    mut on_tool_call: F,
    mut on_event: impl FnMut(&AgentEvent),
) -> Result<String, String>
where
    F: FnMut(&str, &str) -> Result<String, String>,
{
    let mut current_messages: Vec<Value> = messages.to_vec();
    let current_system = system_prompt.map(String::from);
    let mut content_buf = String::new();
    let mut round = 0u32;
    let max_rounds = 50;

    loop {
        round += 1;
        if round > max_rounds {
            on_event(&AgentEvent::Error {
                message: "Max agent loop rounds exceeded".to_string(),
            });
            break;
        }

        // Build request body
        let body = claude_req::build_claude_body(
            model_id,
            &current_messages,
            current_system.as_deref(),
            tools,
            stream,
            temperature,
            None,
            max_tokens,
            thinking_budget,
            is_reasoning,
            custom_body,
        );

        // Build URL and headers
        let url = claude_req::build_claude_url(base_url);
        let mut headers = claude_req::build_claude_headers(api_key);
        if let Some(extra) = custom_headers {
            for (k, v) in extra {
                headers.insert(k.clone(), v.clone());
            }
        }

        let body_str =
            serde_json::to_string(&body).map_err(|e| format!("Serialize body: {}", e))?;

        // Send request
        let mut req = ureq::post(&url).timeout(Duration::from_secs(120));
        for (k, v) in &headers {
            req = req.set(k, v);
        }

        let resp = req
            .send_string(&body_str)
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        let status = resp.status();
        if status < 200 || status >= 300 {
            let err_body = resp.into_string().unwrap_or_default();
            return Err(format!("HTTP {}: {}", status, err_body));
        }

        // Parse response
        content_buf.clear();
        let mut thinking_buf = String::new();
        let mut tool_calls: Vec<ToolCallInfo> = Vec::new();
        let mut current_tool_use_id = String::new();
        let mut current_tool_name = String::new();
        let mut current_tool_args_buf = String::new();
        let mut stop_reason = String::new();

        if stream {
            // SSE streaming
            let body = resp
                .into_string()
                .map_err(|e| format!("Read body: {}", e))?;
            let reader = BufReader::new(body.as_bytes());

            for line in reader.lines() {
                let line = line.unwrap_or_default();
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                if !line.starts_with("data:") {
                    continue;
                }

                let data = line[5..].trim();
                if data.is_empty() {
                    continue;
                }

                let event: Value =
                    serde_json::from_str(data).map_err(|e| format!("Parse event: {}", e))?;

                let event_type = event.get("type").and_then(|v| v.as_str()).unwrap_or("");

                match event_type {
                    "content_block_start" => {
                        let block = event.get("content_block");
                        if let Some(block) = block {
                            let block_type =
                                block.get("type").and_then(|v| v.as_str()).unwrap_or("");
                            if block_type == "tool_use" {
                                current_tool_use_id =
                                    block.get("id").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                current_tool_name =
                                    block.get("name").and_then(|v| v.as_str()).unwrap_or("").to_string();
                                current_tool_args_buf.clear();
                            }
                        }
                    }
                    "content_block_delta" => {
                        let delta = event.get("delta");
                        if let Some(delta) = delta {
                            let delta_type =
                                delta.get("type").and_then(|v| v.as_str()).unwrap_or("");
                            match delta_type {
                                "text_delta" => {
                                    let text =
                                        delta.get("text").and_then(|v| v.as_str()).unwrap_or("");
                                    content_buf.push_str(text);
                                    on_event(&AgentEvent::Content {
                                        text: text.to_string(),
                                    });
                                }
                                "thinking_delta" => {
                                    let text =
                                        delta.get("thinking").and_then(|v| v.as_str()).unwrap_or("");
                                    thinking_buf.push_str(text);
                                    on_event(&AgentEvent::Reasoning {
                                        text: text.to_string(),
                                    });
                                }
                                "input_json_delta" => {
                                    let json_str = delta
                                        .get("input_json")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    current_tool_args_buf.push_str(json_str);
                                }
                                _ => {}
                            }
                        }
                    }
                    "content_block_stop" => {
                        if !current_tool_name.is_empty() {
                            let args: Value =
                                serde_json::from_str(&current_tool_args_buf).unwrap_or(Value::Null);
                            tool_calls.push(ToolCallInfo {
                                id: current_tool_use_id.clone(),
                                name: current_tool_name.clone(),
                                arguments: args,
                            });
                            current_tool_use_id.clear();
                            current_tool_name.clear();
                            current_tool_args_buf.clear();
                        }
                    }
                    "message_delta" => {
                        if let Some(delta) = event.get("delta") {
                            stop_reason = delta
                                .get("stop_reason")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                        }
                        // Usage
                        if let Some(usage) = event.get("usage") {
                            let pt = usage
                                .get("input_tokens")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0) as i32;
                            let ct = usage
                                .get("output_tokens")
                                .and_then(|v| v.as_i64())
                                .unwrap_or(0) as i32;
                            on_event(&AgentEvent::Usage {
                                prompt_tokens: pt,
                                completion_tokens: ct,
                                total_tokens: pt + ct,
                            });
                        }
                    }
                    "message_start" => {
                        if let Some(message) = event.get("message") {
                            if let Some(usage) = message.get("usage") {
                                let pt = usage
                                    .get("input_tokens")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0)
                                    as i32;
                                on_event(&AgentEvent::Usage {
                                    prompt_tokens: pt,
                                    completion_tokens: 0,
                                    total_tokens: pt,
                                });
                            }
                        }
                    }
                    "error" => {
                        let msg = event
                            .get("error")
                            .and_then(|e| e.get("message"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown error");
                        return Err(format!("Claude error: {}", msg));
                    }
                    _ => {}
                }
            }
        } else {
            // Non-streaming
            let body = resp
                .into_string()
                .map_err(|e| format!("Read body: {}", e))?;
            let json: Value =
                serde_json::from_str(&body).map_err(|e| format!("Parse response: {}", e))?;

            // Extract content blocks
            if let Some(content) = json.get("content").and_then(|v| v.as_array()) {
                for block in content {
                    let block_type = block.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    match block_type {
                        "text" => {
                            let text =
                                block.get("text").and_then(|v| v.as_str()).unwrap_or("");
                            content_buf.push_str(text);
                            on_event(&AgentEvent::Content {
                                text: text.to_string(),
                            });
                        }
                        "thinking" => {
                            let text = block
                                .get("thinking")
                                .and_then(|v| v.as_str())
                                .unwrap_or("");
                            thinking_buf.push_str(text);
                            on_event(&AgentEvent::Reasoning {
                                text: text.to_string(),
                            });
                        }
                        "tool_use" => {
                            let id = block
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let name = block
                                .get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            let args = block
                                .get("input")
                                .cloned()
                                .unwrap_or(Value::Null);
                            tool_calls.push(ToolCallInfo { id, name, arguments: args });
                        }
                        _ => {}
                    }
                }
            }

            stop_reason = json
                .get("stop_reason")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();

            // Usage
            if let Some(usage) = json.get("usage") {
                let pt = usage
                    .get("input_tokens")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                let ct = usage
                    .get("output_tokens")
                    .and_then(|v| v.as_i64())
                    .unwrap_or(0) as i32;
                on_event(&AgentEvent::Usage {
                    prompt_tokens: pt,
                    completion_tokens: ct,
                    total_tokens: pt + ct,
                });
            }
        }

        // Check if we have tool calls
        if tool_calls.is_empty() || stop_reason == "end_turn" {
            on_event(&AgentEvent::Done);
            break;
        }

        // Emit tool calls event
        on_event(&AgentEvent::ToolCalls {
            calls: tool_calls.clone(),
        });

        // Execute tool calls
        let mut tool_results = Vec::new();
        for tc in &tool_calls {
            let args_str = serde_json::to_string(&tc.arguments).unwrap_or_default();
            match on_tool_call(&tc.name, &args_str) {
                Ok(result) => {
                    tool_results.push(ToolResultInfo {
                        id: tc.id.clone(),
                        name: tc.name.clone(),
                        arguments: tc.arguments.clone(),
                        content: result,
                    });
                }
                Err(e) => {
                    tool_results.push(ToolResultInfo {
                        id: tc.id.clone(),
                        name: tc.name.clone(),
                        arguments: tc.arguments.clone(),
                        content: format!("Error: {}", e),
                    });
                }
            }
        }

        // Emit tool results event
        on_event(&AgentEvent::ToolResults {
            results: tool_results.clone(),
        });

        // Build Claude tool_use blocks for assistant message
        let mut blocks: Vec<Value> = Vec::new();
        if !content_buf.is_empty() {
            blocks.push(serde_json::json!({
                "type": "text",
                "text": content_buf,
            }));
        }
        for tc in &tool_calls {
            blocks.push(serde_json::json!({
                "type": "tool_use",
                "id": tc.id,
                "name": tc.name,
                "input": tc.arguments,
            }));
        }

        if !blocks.is_empty() {
            current_messages.push(serde_json::json!({
                "role": "assistant",
                "content": blocks,
            }));
        }

        // Add tool result message
        let result_blocks: Vec<Value> = tool_results
            .iter()
            .map(|tr| {
                serde_json::json!({
                    "type": "tool_result",
                    "tool_use_id": tr.id,
                    "content": tr.content,
                })
            })
            .collect();

        current_messages.push(serde_json::json!({
            "role": "user",
            "content": result_blocks,
        }));

        // Reset for next round
        content_buf.clear();
        thinking_buf.clear();
        stop_reason.clear();
    }

    Ok(content_buf)
}
