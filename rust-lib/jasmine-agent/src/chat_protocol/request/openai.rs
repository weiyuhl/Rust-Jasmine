use serde_json::{json, Value};
use std::collections::HashMap;

/// Build the API endpoint URL for an OpenAI-compatible request.
pub fn build_openai_url(base_url: &str, chat_path: Option<&str>, use_response_api: bool) -> String {
    let base = base_url.trim_end_matches('/');
    if use_response_api {
        format!("{}/responses", base)
    } else {
        let path = chat_path.unwrap_or("/chat/completions");
        let path = if path.starts_with('/') {
            path.to_string()
        } else {
            format!("/{}", path)
        };
        format!("{}{}", base, path)
    }
}

/// Build default headers for an OpenAI-compatible API request.
pub fn build_openai_headers(
    api_key: &str,
    is_openrouter: bool,
) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    headers.insert("Authorization".to_string(), format!("Bearer {}", api_key));
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    if is_openrouter {
        headers.insert("HTTP-Referer".to_string(), "https://github.com/weiyuhl/kelivo".to_string());
        headers.insert("X-OpenRouter-Title".to_string(), "Kelivo".to_string());
    }

    headers
}

/// Build a Chat Completions request body.
pub fn build_chat_body(
    model_id: &str,
    messages: &[Value],
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
        "messages": messages,
        "stream": stream,
    });

    if let Some(t) = temperature {
        if !should_omit_temperature_for_reasoning(model_id, thinking_budget, is_reasoning) {
            body["temperature"] = json!(t);
        }
    }
    if let Some(p) = top_p {
        body["top_p"] = json!(p);
    }
    if let Some(mt) = max_tokens {
        body["max_tokens"] = json!(mt);
    }

    if let Some(tools) = tools {
        if !tools.is_empty() {
            body["tools"] = json!(tools);
            body["tool_choice"] = json!("auto");
        }
    }

    if is_reasoning {
        let effort = effort_for_budget(thinking_budget);
        if effort != "off" && effort != "auto" {
            body["reasoning_effort"] = json!(effort);
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

/// Build a Responses API request body.
pub fn build_responses_body(
    model_id: &str,
    input: &[Value],
    instructions: Option<&str>,
    tools: Option<&[Value]>,
    stream: bool,
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i32>,
    thinking_budget: Option<i32>,
    is_reasoning: bool,
) -> Value {
    let mut body = json!({
        "model": model_id,
        "input": input,
        "stream": stream,
    });

    if let Some(inst) = instructions {
        if !inst.is_empty() {
            body["instructions"] = json!(inst);
        }
    }
    if let Some(t) = temperature {
        body["temperature"] = json!(t);
    }
    if let Some(p) = top_p {
        body["top_p"] = json!(p);
    }
    if let Some(mt) = max_tokens {
        body["max_output_tokens"] = json!(mt);
    }

    if let Some(tools) = tools {
        if !tools.is_empty() {
            body["tools"] = json!(tools);
            body["tool_choice"] = json!("auto");
        }
    }

    if is_reasoning {
        let effort = effort_for_budget(thinking_budget);
        if effort != "off" {
            body["reasoning"] = json!({"summary": "auto"});
            if effort != "auto" {
                body["reasoning"]["effort"] = json!(effort);
            }
        }
    }

    body
}

/// Build a DALL-E images generation request body.
pub fn build_images_body(
    model_id: &str,
    prompt: &str,
    n: Option<i32>,
    size: Option<&str>,
    quality: Option<&str>,
) -> Value {
    let mut body = json!({
        "model": model_id,
        "prompt": prompt,
    });
    if let Some(n) = n {
        body["n"] = json!(n);
    }
    if let Some(s) = size {
        body["size"] = json!(s);
    }
    if let Some(q) = quality {
        body["quality"] = json!(q);
    }
    body
}

/// Build the images generation API URL.
pub fn build_images_url(base_url: &str) -> String {
    format!("{}/images/generations", base_url.trim_end_matches('/'))
}

// ── internal helpers ──

fn should_omit_temperature_for_reasoning(
    model_id: &str,
    thinking_budget: Option<i32>,
    is_reasoning: bool,
) -> bool {
    if !is_reasoning {
        return false;
    }
    let reasoning_on = thinking_budget.map(|b| b != 0).unwrap_or(false);
    if !reasoning_on {
        return false;
    }
    // GPT-5.2/5.4: temperature only allowed when effort is auto
    let lower = model_id.to_lowercase();
    if lower.starts_with("gpt-5.2") || lower.starts_with("gpt-5.4") {
        let effort = effort_for_budget(thinking_budget);
        return effort != "auto";
    }
    // Claude adaptive-only models (Mythos, 4.7): always omit when reasoning
    if lower.contains("mythos") || lower.contains("claude-4-7") {
        return true;
    }
    // GPT-5: omit for non-auto efforts
    if lower.starts_with("gpt-5") {
        let effort = effort_for_budget(thinking_budget);
        return effort != "auto";
    }
    false
}

/// Map thinking budget (tokens) to effort level.
fn effort_for_budget(budget: Option<i32>) -> &'static str {
    match budget {
        None | Some(-1) => "auto",
        Some(b) if b < 1024 => "off",
        Some(b) if b <= 2000 => "low",
        Some(b) if b <= 20000 => "medium",
        Some(b) if b <= 64000 => "high",
        _ => "xhigh",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_chat_url() {
        let url = build_openai_url("https://api.openai.com/v1", None, false);
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_build_chat_url_normalizes_custom_path() {
        let url = build_openai_url(
            "https://api.openai.com/v1",
            Some("chat/completions"),
            false,
        );
        assert_eq!(url, "https://api.openai.com/v1/chat/completions");
    }

    #[test]
    fn test_build_responses_url() {
        let url = build_openai_url("https://api.openai.com/v1", None, true);
        assert_eq!(url, "https://api.openai.com/v1/responses");
    }

    #[test]
    fn test_build_chat_body_minimal() {
        let body = build_chat_body(
            "gpt-4o",
            &[json!({"role": "user", "content": "hi"})],
            None, true, None, None, None, None, false, None,
        );
        assert_eq!(body["model"], "gpt-4o");
        assert_eq!(body["stream"], true);
    }
}
