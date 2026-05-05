// Chat protocol FRB API — delegates to chat_protocol business modules.
// All functions use JSON strings for complex types; FRB auto-generates Dart bindings.

use crate::chat_protocol;

// ── Request: OpenAI ─────────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_build_openai_url(
    base_url: String,
    chat_path: Option<String>,
    use_response_api: bool,
) -> String {
    chat_protocol::request::openai::build_openai_url(
        &base_url,
        chat_path.as_deref(),
        use_response_api,
    )
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_build_openai_chat_body(
    model_id: String,
    messages_json: String,
    tools_json: Option<String>,
    stream: bool,
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i32>,
    thinking_budget: Option<i32>,
    is_reasoning: bool,
    extra_body_json: Option<String>,
) -> Result<String, String> {
    let messages: serde_json::Value =
        serde_json::from_str(&messages_json).map_err(|e| format!("Invalid messages: {}", e))?;
    let tools: Option<Vec<serde_json::Value>> = match tools_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid tools: {}", e))?),
        None => None,
    };
    let extra_body: Option<serde_json::Value> = match extra_body_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid extra_body: {}", e))?),
        None => None,
    };

    let body = chat_protocol::request::openai::build_chat_body(
        &model_id,
        messages.as_array().map(|a| a.as_slice()).unwrap_or(&[]),
        tools.as_deref(),
        stream,
        temperature,
        top_p,
        max_tokens,
        thinking_budget,
        is_reasoning,
        extra_body.as_ref(),
    );
    serde_json::to_string(&body).map_err(|e| format!("Serialize: {}", e))
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_build_openai_responses_body(
    model_id: String,
    input_json: String,
    instructions: Option<String>,
    tools_json: Option<String>,
    stream: bool,
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i32>,
    thinking_budget: Option<i32>,
    is_reasoning: bool,
) -> Result<String, String> {
    let input: serde_json::Value =
        serde_json::from_str(&input_json).map_err(|e| format!("Invalid input: {}", e))?;
    let tools: Option<Vec<serde_json::Value>> = match tools_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid tools: {}", e))?),
        None => None,
    };

    let body = chat_protocol::request::openai::build_responses_body(
        &model_id,
        input.as_array().map(|a| a.as_slice()).unwrap_or(&[]),
        instructions.as_deref(),
        tools.as_deref(),
        stream,
        temperature,
        top_p,
        max_tokens,
        thinking_budget,
        is_reasoning,
    );
    serde_json::to_string(&body).map_err(|e| format!("Serialize: {}", e))
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_build_images_body(
    model_id: String,
    prompt: String,
    n: Option<i32>,
    size: Option<String>,
    quality: Option<String>,
) -> String {
    serde_json::to_string(&chat_protocol::request::openai::build_images_body(
        &model_id, &prompt, n, size.as_deref(), quality.as_deref(),
    ))
    .unwrap_or_default()
}

// ── Request: Claude ─────────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_build_claude_url(base_url: String) -> String {
    chat_protocol::request::claude::build_claude_url(&base_url)
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_build_claude_body(
    model_id: String,
    messages_json: String,
    system_prompt: Option<String>,
    tools_json: Option<String>,
    stream: bool,
    temperature: Option<f64>,
    top_p: Option<f64>,
    max_tokens: Option<i32>,
    thinking_budget: Option<i32>,
    is_reasoning: bool,
    extra_body_json: Option<String>,
) -> Result<String, String> {
    let messages: serde_json::Value =
        serde_json::from_str(&messages_json).map_err(|e| format!("Invalid messages: {}", e))?;
    let tools: Option<Vec<serde_json::Value>> = match tools_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid tools: {}", e))?),
        None => None,
    };
    let extra_body: Option<serde_json::Value> = match extra_body_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid extra_body: {}", e))?),
        None => None,
    };

    let body = chat_protocol::request::claude::build_claude_body(
        &model_id,
        messages.as_array().map(|a| a.as_slice()).unwrap_or(&[]),
        system_prompt.as_deref(),
        tools.as_deref(),
        stream,
        temperature,
        top_p,
        max_tokens,
        thinking_budget,
        is_reasoning,
        extra_body.as_ref(),
    );
    serde_json::to_string(&body).map_err(|e| format!("Serialize: {}", e))
}

// ── Parse: SSE ──────────────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_parse_sse_line(line: String) -> Option<String> {
    match chat_protocol::parse::sse::parse_sse_line(&line) {
        Some(event) => {
            if event.is_done {
                Some("[DONE]".to_string())
            } else {
                Some(event.data)
            }
        }
        None => None,
    }
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_is_sse_done(line: String) -> bool {
    chat_protocol::parse::sse::is_sse_done(&line)
}

// ── Parse: OpenAI Stream ────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_parse_openai_chunk(json_str: String) -> Result<String, String> {
    let json: serde_json::Value =
        serde_json::from_str(&json_str).map_err(|e| format!("Invalid JSON: {}", e))?;
    let delta = chat_protocol::parse::openai_stream::parse_openai_chunk(&json);
    serde_json::to_string(&delta).map_err(|e| format!("Serialize: {}", e))
}

// ── Parse: Claude Stream ────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_parse_claude_event(json_str: String) -> Result<String, String> {
    let json: serde_json::Value =
        serde_json::from_str(&json_str).map_err(|e| format!("Invalid JSON: {}", e))?;
    let event = chat_protocol::parse::claude_stream::parse_claude_event(&json);
    serde_json::to_string(&event).map_err(|e| format!("Serialize: {}", e))
}

// ── Parse: Tool Calls ───────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_aggregate_tool_call(
    index: i32,
    id: Option<String>,
    name: Option<String>,
    args_fragment: Option<String>,
    aggregator_json: Option<String>,
) -> Result<String, String> {
    let mut agg: chat_protocol::parse::tool_calls::ToolCallAggregator = match aggregator_json {
        Some(s) => serde_json::from_str(&s).unwrap_or_default(),
        None => Default::default(),
    };
    agg.push(&chat_protocol::parse::tool_calls::ToolCallDelta {
        index,
        id,
        name,
        args_fragment,
    });
    serde_json::to_string(&agg).map_err(|e| format!("Serialize: {}", e))
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_finalize_tool_calls(aggregator_json: String) -> Result<String, String> {
    let agg: chat_protocol::parse::tool_calls::ToolCallAggregator =
        serde_json::from_str(&aggregator_json).map_err(|e| format!("Invalid aggregator: {}", e))?;
    let calls = agg.current_calls();
    serde_json::to_string(&calls).map_err(|e| format!("Serialize: {}", e))
}

// ── Message ─────────────────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_copy_message(msg_json: String) -> Result<String, String> {
    let msg: serde_json::Value =
        serde_json::from_str(&msg_json).map_err(|e| format!("Invalid message: {}", e))?;
    let result = chat_protocol::message::builder::copy_chat_message(&msg);
    serde_json::to_string(&result).map_err(|e| format!("Serialize: {}", e))
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_parse_text_and_images(raw: String) -> String {
    let parsed = chat_protocol::message::image::parse_text_and_images(&raw);
    serde_json::to_string(&parsed).unwrap_or_default()
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_sanitize_unicode(text: String) -> String {
    chat_protocol::message::sanitizer::sanitize_unicode(&text)
}

// ── Vendor ──────────────────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_classify_vendor(
    provider_id: String,
    base_url: String,
    model_id: String,
) -> String {
    let vendor = chat_protocol::request::openai_vendor::classify_vendor(
        &provider_id, &base_url, &model_id,
    );
    format!("{:?}", vendor)
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_completion_tokens_key(vendor_str: String) -> String {
    match vendor_str.as_str() {
        "Azure" | "Mimo" => "max_completion_tokens",
        _ => "max_tokens",
    }
    .to_string()
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_needs_reasoning_echo(vendor_str: String) -> bool {
    matches!(vendor_str.as_str(), "DeepSeek" | "Mimo" | "Kimi")
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_normalize_reasoning_effort(effort: String, model_id: String) -> String {
    chat_protocol::request::openai_vendor::normalize_reasoning_effort(&effort, &model_id)
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_sanitize_gpt5_body(body_json: String, model_id: String, effort: String) -> Result<String, String> {
    let mut body: serde_json::Value =
        serde_json::from_str(&body_json).map_err(|e| format!("Invalid body: {}", e))?;
    chat_protocol::request::openai_vendor::sanitize_gpt5_sampling(&mut body, &model_id, &effort);
    serde_json::to_string(&body).map_err(|e| format!("Serialize: {}", e))
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_is_kimi_thinking_model(model_id: String) -> bool {
    chat_protocol::request::openai_vendor::is_kimi_thinking_model(&model_id)
}

// ── Tool ────────────────────────────────────────────────────
#[flutter_rust_bridge::frb(sync)]
pub fn chat_clean_tool_schema(tool_json: String) -> Result<String, String> {
    let parsed: serde_json::Value =
        serde_json::from_str(&tool_json).map_err(|e| format!("Invalid JSON: {}", e))?;
    // Handle both single tool object and array of tools
    let cleaned = match parsed {
        serde_json::Value::Array(ref arr) => {
            let result: Vec<serde_json::Value> = arr
                .iter()
                .map(|t| chat_protocol::tool::schema::clean_tool_for_compatibility(t))
                .collect();
            serde_json::Value::Array(result)
        }
        _ => chat_protocol::tool::schema::clean_tool_for_compatibility(&parsed),
    };
    serde_json::to_string(&cleaned).map_err(|e| format!("Serialize: {}", e))
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_to_claude_tools_format(tools_json: String) -> Result<String, String> {
    let tools: Vec<serde_json::Value> =
        serde_json::from_str(&tools_json).map_err(|e| format!("Invalid tools: {}", e))?;
    let result = chat_protocol::tool::schema::to_claude_tools_format(&tools);
    serde_json::to_string(&result).map_err(|e| format!("Serialize: {}", e))
}

#[flutter_rust_bridge::frb(sync)]
pub fn chat_is_builtin_search_supported(model_id: String, provider_type: String) -> bool {
    match provider_type.as_str() {
        "openai" => chat_protocol::tool::builtin::is_openai_builtin_search_supported(&model_id),
        "claude" => chat_protocol::tool::builtin::is_claude_builtin_search_supported(&model_id),
        _ => false,
    }
}
