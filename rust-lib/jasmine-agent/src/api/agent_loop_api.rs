// Agent Loop FRB API — provides streaming agent loop for OpenAI and Claude providers.

use crate::chat_protocol::agent_loop::{self};

/// Run the OpenAI-compatible agent loop.
///
/// Executes the full agent cycle: send request → parse SSE → yield events →
/// execute tool calls → loop until done.
///
/// Returns a JSON string of the final accumulated content.
///
/// # Arguments
/// * `config_json` - Provider config JSON
/// * `messages_json` - Messages array JSON
/// * `tools_json` - Optional tool definitions JSON
/// * `stream` - Whether to use SSE streaming
/// * `is_reasoning` - Whether model supports reasoning
/// * `thinking_budget` - Reasoning token budget
/// * `temperature` - Sampling temperature
/// * `max_tokens` - Max completion tokens
/// * `use_response_api` - Use OpenAI Responses API
/// * `on_event_json` - Callback that receives AgentEvent JSON
#[flutter_rust_bridge::frb]
pub fn run_openai_agent_loop(
    base_url: String,
    api_key: String,
    model_id: String,
    messages_json: String,
    tools_json: Option<String>,
    stream: bool,
    is_reasoning: bool,
    thinking_budget: Option<i32>,
    temperature: Option<f64>,
    max_tokens: Option<i32>,
    use_response_api: bool,
) -> Result<String, String> {
    let messages: Vec<serde_json::Value> =
        serde_json::from_str(&messages_json).map_err(|e| format!("Invalid messages: {}", e))?;
    let tools: Option<Vec<serde_json::Value>> = match tools_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid tools: {}", e))?),
        None => None,
    };

    let mut events = Vec::new();
    let result = agent_loop::openai_loop::run_openai_agent_loop(
        &base_url,
        &api_key,
        &model_id,
        &messages,
        tools.as_deref(),
        stream,
        is_reasoning,
        thinking_budget,
        temperature,
        max_tokens,
        use_response_api,
        None,  // custom_headers
        None,  // custom_body
        |_name, _args| Ok("{}".to_string()), // placeholder tool callback
        |event| events.push(event.clone()),
    );

    let response = serde_json::json!({
        "content": result.unwrap_or_default(),
        "events": events,
    });
    serde_json::to_string(&response).map_err(|e| format!("Serialize: {}", e))
}

/// Run the Claude agent loop.
///
/// Executes the full agent cycle for Claude API: send request → parse SSE → yield events →
/// execute tool calls → loop until done.
///
/// Returns a JSON string of the final accumulated content.
#[flutter_rust_bridge::frb(sync)]
pub fn run_claude_agent_loop(
    base_url: String,
    api_key: String,
    model_id: String,
    messages_json: String,
    system_prompt: Option<String>,
    tools_json: Option<String>,
    stream: bool,
    is_reasoning: bool,
    thinking_budget: Option<i32>,
    temperature: Option<f64>,
    max_tokens: Option<i32>,
) -> Result<String, String> {
    let messages: Vec<serde_json::Value> =
        serde_json::from_str(&messages_json).map_err(|e| format!("Invalid messages: {}", e))?;
    let tools: Option<Vec<serde_json::Value>> = match tools_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid tools: {}", e))?),
        None => None,
    };

    let mut events = Vec::new();
    let result = agent_loop::claude_loop::run_claude_agent_loop(
        &base_url,
        &api_key,
        &model_id,
        &messages,
        system_prompt.as_deref(),
        tools.as_deref(),
        stream,
        is_reasoning,
        thinking_budget,
        temperature,
        max_tokens,
        None,  // custom_headers
        None,  // custom_body
        |_name, _args| Ok("{}".to_string()), // placeholder tool callback
        |event| events.push(event.clone()),
    );

    let response = serde_json::json!({
        "content": result.unwrap_or_default(),
        "events": events,
    });
    serde_json::to_string(&response).map_err(|e| format!("Serialize: {}", e))
}
