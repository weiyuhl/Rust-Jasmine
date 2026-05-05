use serde_json::{json, Value};
use std::collections::HashMap;

/// Build the Messages API URL for Claude.
pub fn build_claude_url(base_url: &str) -> String {
    format!("{}/messages", base_url.trim_end_matches('/'))
}

/// Build default headers for a Claude API request.
pub fn build_claude_headers(api_key: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("x-api-key".to_string(), api_key.to_string());
    headers.insert("anthropic-version".to_string(), "2023-06-01".to_string());
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Accept".to_string(), "text/event-stream".to_string());
    headers
}

/// Build a Claude Messages request body.
pub fn build_claude_body(
    model_id: &str,
    messages: &[Value],
    system_prompt: Option<&str>,
    tools: Option<&[Value]>,
    stream: bool,
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i32>,
    thinking_budget: Option<i32>,
    is_reasoning: bool,
    extra_body: Option<&Value>,
) -> Value {
    let mut body = json!({
        "model": model_id,
        "max_tokens": max_tokens.unwrap_or(64000),
        "messages": messages,
        "stream": stream,
    });

    if let Some(sp) = system_prompt {
        if !sp.is_empty() {
            body["system"] = json!(sp);
        }
    }

    let omit_sampling = should_omit_sampling_params(model_id, thinking_budget);
    if !omit_sampling && !is_reasoning_enabled(thinking_budget) {
        if let Some(t) = temperature {
            body["temperature"] = json!(t);
        }
    }
    if let Some(p) = top_p {
        if top_p_compatible(model_id, thinking_budget, p) {
            body["top_p"] = json!(p);
        }
    }

    if let Some(tools) = tools {
        if !tools.is_empty() {
            body["tools"] = json!(tools);
            body["tool_choice"] = json!({"type": "auto"});
        }
    }

    // Thinking config
    if is_reasoning {
        if !is_reasoning_enabled(thinking_budget) {
            body["thinking"] = json!({"type": "disabled"});
        } else if supports_adaptive_thinking(model_id) {
            body["thinking"] = json!({"type": "adaptive", "display": "summarized"});
            let effort = effort_for_budget(thinking_budget);
            if effort != "auto" && effort != "off" {
                body["output_config"] = json!({"effort": effort});
            }
        } else if let Some(budget) = thinking_budget {
            if budget > 0 {
                body["thinking"] = json!({"type": "enabled", "budget_tokens": budget});
            } else {
                body["thinking"] = json!({"type": "disabled"});
            }
        }
    }

    if let Some(extra) = extra_body {
        if let Some(obj) = extra.as_object() {
            for (k, v) in obj {
                body[k] = v.clone();
            }
        }
    }

    body
}

// ── Claude reasoning helpers ──

/// Check whether reasoning is enabled (non-zero budget).
pub fn is_reasoning_enabled(budget: Option<i32>) -> bool {
    budget.map_or(false, |b| b != 0)
}

/// Check whether a model supports adaptive thinking (Claude 4.6+).
fn supports_adaptive_thinking(model_id: &str) -> bool {
    let lower = model_id.to_lowercase();
    if !lower.contains("claude-") {
        return false;
    }
    lower.contains("4-6") || lower.contains("4-7") || lower.contains("mythos")
}

/// Check whether the model should omit sampling params when reasoning.
fn should_omit_sampling_params(model_id: &str, budget: Option<i32>) -> bool {
    let lower = model_id.to_lowercase();
    let is_adaptive_only = lower.contains("mythos") || lower.contains("4-7");
    is_adaptive_only && is_reasoning_enabled(budget)
}

/// Check whether top_p is compatible with the current model/reasoning state.
fn top_p_compatible(_model_id: &str, budget: Option<i32>, top_p: f64) -> bool {
    if !is_reasoning_enabled(budget) {
        return true;
    }
    // Claude requires 0.95 <= top_p <= 1.0 when reasoning
    top_p >= 0.95 && top_p <= 1.0
}

/// Map thinking budget to effort level (Claude scale).
fn effort_for_budget(budget: Option<i32>) -> &'static str {
    match budget {
        None | Some(-1) => "auto",
        Some(b) if b < 1024 => "off",
        Some(b) if b <= 2000 => "low",
        Some(b) if b <= 20000 => "medium",
        Some(b) if b <= 32000 => "high",
        Some(b) if b <= 64000 => "xhigh",
        _ => "max",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_claude_url() {
        let url = build_claude_url("https://api.anthropic.com/v1");
        assert_eq!(url, "https://api.anthropic.com/v1/messages");
    }

    #[test]
    fn test_build_claude_headers() {
        let headers = build_claude_headers("sk-test");
        assert_eq!(headers.get("x-api-key").unwrap(), "sk-test");
        assert_eq!(headers.get("anthropic-version").unwrap(), "2023-06-01");
    }

    #[test]
    fn test_claude_adaptive_thinking() {
        assert!(supports_adaptive_thinking("claude-sonnet-4-6"));
        assert!(!supports_adaptive_thinking("claude-3-5-sonnet"));
    }
}
