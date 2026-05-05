use serde_json::Value;
use std::io::{BufRead, BufReader};
use std::time::Duration;

use super::{AgentEvent, ToolAccumulator, ToolResultInfo};
use crate::chat_protocol::request::openai as openai_req;

/// Execute the OpenAI-compatible agent loop.
///
/// Sends the initial request, parses SSE chunks, yields events via callback,
/// executes tool calls, and loops until no more tool calls are returned.
///
/// # Arguments
/// * `base_url` - Provider base URL
/// * `api_key` - API key for authentication
/// * `model_id` - Model ID to use
/// * `messages` - Initial messages array (JSON)
/// * `tools` - Optional tool definitions (JSON)
/// * `stream` - Whether to use SSE streaming
/// * `is_reasoning` - Whether model supports reasoning
/// * `thinking_budget` - Reasoning token budget
/// * `temperature` - Sampling temperature
/// * `max_tokens` - Max completion tokens
/// * `use_response_api` - Use OpenAI Responses API instead of Chat Completions
/// * `custom_headers` - Extra headers to include
/// * `custom_body` - Extra body fields to merge
/// * `on_tool_call` - Callback for executing tools; receives (name, args_json) → result_json
///
/// Returns the final accumulated content text.
pub fn run_openai_agent_loop<F>(
    base_url: &str,
    api_key: &str,
    model_id: &str,
    messages: &[Value],
    tools: Option<&[Value]>,
    stream: bool,
    is_reasoning: bool,
    thinking_budget: Option<i32>,
    temperature: Option<f64>,
    max_tokens: Option<i32>,
    use_response_api: bool,
    custom_headers: Option<&[(String, String)]>,
    custom_body: Option<&Value>,
    mut on_tool_call: F,
    mut on_event: impl FnMut(&AgentEvent),
) -> Result<String, String>
where
    F: FnMut(&str, &str) -> Result<String, String>,
{
    let mut current_messages: Vec<Value> = messages.to_vec();
    let mut round = 0u32;
    let max_rounds = 50; // Safety limit

    let content_buf = String::new();

    loop {
        round += 1;
        if round > max_rounds {
            on_event(&AgentEvent::Error {
                message: "Max agent loop rounds exceeded".to_string(),
            });
            break;
        }

        // Build request body
        let body = if use_response_api {
            openai_req::build_responses_body(
                model_id,
                &current_messages,
                None,
                tools,
                stream,
                temperature,
                None,
                max_tokens,
                thinking_budget,
                is_reasoning,
            )
        } else {
            openai_req::build_chat_body(
                model_id,
                &current_messages,
                tools,
                stream,
                temperature,
                None,
                max_tokens,
                thinking_budget,
                is_reasoning,
                custom_body,
            )
        };

        // Build URL
        let path = if use_response_api {
            "/responses"
        } else {
            "/chat/completions"
        };
        let url = format!("{}{}", base_url.trim_end_matches('/'), path);

        // Build headers
        let mut headers = std::collections::HashMap::new();
        headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        if stream {
            headers.insert("Accept".to_string(), "text/event-stream".to_string());
        } else {
            headers.insert("Accept".to_string(), "application/json".to_string());
        }
        if let Some(extra) = custom_headers {
            for (k, v) in extra {
                headers.insert(k.clone(), v.clone());
            }
        }

        let body_str =
            serde_json::to_string(&body).map_err(|e| format!("Serialize body: {}", e))?;

        // Send request
        let mut req = ureq::post(&url)
            .timeout(Duration::from_secs(120));
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
        let mut content_buf = String::new();
        let mut tool_acc = ToolAccumulator::new();
        let mut has_tool_calls = false;

        if stream {
            // SSE streaming
            let body = resp
                .into_string()
                .map_err(|e| format!("Read body: {}", e))?;
            let reader = BufReader::new(body.as_bytes());
            let mut sse_buf = String::new();

            for line in reader.lines() {
                let line = line.unwrap_or_default();
                let line = line.trim();
                if line.is_empty() && !sse_buf.is_empty() {
                    sse_buf.clear();
                    continue;
                }

                if !line.starts_with("data:") {
                    continue;
                }

                let data = line[5..].trim();
                if data == "[DONE]" {
                    break;
                }

                let chunk: Value =
                    serde_json::from_str(data).map_err(|e| format!("Parse chunk: {}", e))?;

                if use_response_api {
                    let event_type = chunk.get("type").and_then(|v| v.as_str()).unwrap_or("");
                    match event_type {
                        "response.output_text.delta" => {
                            let delta = chunk.get("delta").and_then(|v| v.as_str()).unwrap_or("");
                            content_buf.push_str(delta);
                            on_event(&AgentEvent::Content {
                                text: delta.to_string(),
                            });
                        }
                        "response.output_item.added" => {
                            // Tool call added
                        }
                        _ => {}
                    }
                } else {
                    // Chat Completions format
                    if let Some(choices) = chunk.get("choices").and_then(|v| v.as_array()) {
                        if let Some(c0) = choices.first() {
                            if let Some(delta) = c0.get("delta") {
                                // Content
                                if let Some(text) = delta.get("content").and_then(|v| v.as_str()) {
                                    if !text.is_empty() {
                                        content_buf.push_str(text);
                                        on_event(&AgentEvent::Content {
                                            text: text.to_string(),
                                        });
                                    }
                                }
                                // Reasoning
                                if let Some(rc) = delta
                                    .get("reasoning_content")
                                    .and_then(|v| v.as_str())
                                {
                                    if !rc.is_empty() {
                                        on_event(&AgentEvent::Reasoning {
                                            text: rc.to_string(),
                                        });
                                    }
                                }
                                // Tool call deltas
                                if let Some(tcs) = delta
                                    .get("tool_calls")
                                    .and_then(|v| v.as_array())
                                {
                                    for tc in tcs {
                                        let idx = tc
                                            .get("index")
                                            .and_then(|v| v.as_i64())
                                            .unwrap_or(0) as i32;
                                        let id = tc
                                            .get("id")
                                            .and_then(|v| v.as_str());
                                        let name = tc
                                            .get("function")
                                            .and_then(|f| f.get("name"))
                                            .and_then(|v| v.as_str());
                                        let args = tc
                                            .get("function")
                                            .and_then(|f| f.get("arguments"))
                                            .and_then(|v| v.as_str());
                                        tool_acc.push(idx, id, name, args);
                                        has_tool_calls = true;
                                    }
                                }
                            }
                            // Usage
                            if let Some(usage) = chunk.get("usage") {
                                let pt = usage
                                    .get("prompt_tokens")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0) as i32;
                                let ct = usage
                                    .get("completion_tokens")
                                    .and_then(|v| v.as_i64())
                                    .unwrap_or(0) as i32;
                                on_event(&AgentEvent::Usage {
                                    prompt_tokens: pt,
                                    completion_tokens: ct,
                                    total_tokens: pt + ct,
                                });
                            }
                        }
                    }
                }
            }
        } else {
            // Non-streaming
            let body = resp
                .into_string()
                .map_err(|e| format!("Read body: {}", e))?;
            let json: Value =
                serde_json::from_str(&body).map_err(|e| format!("Parse response: {}", e))?;

            if use_response_api {
                if let Some(text) = json.get("output_text").and_then(|v| v.as_str()) {
                    content_buf.push_str(text);
                    on_event(&AgentEvent::Content {
                        text: text.to_string(),
                    });
                }
            } else {
                if let Some(choices) = json.get("choices").and_then(|v| v.as_array()) {
                    if let Some(c0) = choices.first() {
                        if let Some(message) = c0.get("message") {
                            if let Some(text) = message.get("content").and_then(|v| v.as_str()) {
                                content_buf.push_str(text);
                                on_event(&AgentEvent::Content {
                                    text: text.to_string(),
                                });
                            }
                            if let Some(rc) =
                                message.get("reasoning_content").and_then(|v| v.as_str())
                            {
                                if !rc.is_empty() {
                                    on_event(&AgentEvent::Reasoning {
                                        text: rc.to_string(),
                                    });
                                }
                            }
                            if let Some(tcs) =
                                message.get("tool_calls").and_then(|v| v.as_array())
                            {
                                for tc in tcs {
                                    let id = tc.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                    let name = tc
                                        .get("function")
                                        .and_then(|f| f.get("name"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    let args_str = tc
                                        .get("function")
                                        .and_then(|f| f.get("arguments"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("{}");
                                    tool_acc.push(
                                        0,
                                        Some(id),
                                        Some(name),
                                        Some(args_str),
                                    );
                                    has_tool_calls = true;
                                }
                            }
                        }
                    }
                }
                // Usage
                if let Some(usage) = json.get("usage") {
                    let pt = usage.get("prompt_tokens").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                    let ct = usage
                        .get("completion_tokens")
                        .and_then(|v| v.as_i64())
                        .unwrap_or(0) as i32;
                    on_event(&AgentEvent::Usage {
                        prompt_tokens: pt,
                        completion_tokens: ct,
                        total_tokens: pt + ct,
                    });
                }
            }
        }

        // Check if we have tool calls
        if !has_tool_calls {
            on_event(&AgentEvent::Done);
            break;
        }

        let tool_calls = tool_acc.finalize();
        tool_acc.clear();

        if tool_calls.is_empty() {
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

        // Build assistant tool call message
        let assistant_calls: Vec<Value> = tool_calls
            .iter()
            .map(|tc| {
                serde_json::json!({
                    "id": tc.id,
                    "type": "function",
                    "function": {
                        "name": tc.name,
                        "arguments": serde_json::to_string(&tc.arguments).unwrap_or_default()
                    }
                })
            })
            .collect();

        let mut assistant_msg = serde_json::json!({
            "role": "assistant",
            "tool_calls": assistant_calls
        });
        if !content_buf.is_empty() {
            assistant_msg["content"] = Value::String(content_buf.clone());
        }
        current_messages.push(assistant_msg);

        // Add tool result messages
        for tr in &tool_results {
            current_messages.push(serde_json::json!({
                "role": "tool",
                "tool_call_id": tr.id,
                "name": tr.name,
                "content": tr.content,
            }));
        }

        // Reset for next round
        content_buf.clear();
        has_tool_calls = false;
    }

    Ok(content_buf)
}
