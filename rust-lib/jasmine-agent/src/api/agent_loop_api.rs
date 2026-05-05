// Agent Loop FRB API — two-phase approach for FRB v2 compatibility.
// Phase 1: start_agent_loop → returns events + tool calls
// Phase 2: continue_agent_loop → receives tool results, continues loop

use crate::chat_protocol::agent_loop::{self, AgentEvent, ToolCallInfo};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Result of one agent loop round.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentLoopResult {
    /// Events from this round (Content, Reasoning, Usage, Done, Error).
    pub events: Vec<AgentEvent>,
    /// Tool calls that need to be executed (empty if round is done).
    pub pending_tool_calls: Vec<ToolCallInfo>,
    /// Accumulated assistant content from this round.
    pub content: String,
    /// Internal state for continuing the loop (JSON).
    pub state_json: String,
    /// Whether the loop is complete (no more tool calls).
    pub is_done: bool,
}

/// Internal state passed between rounds.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct AgentLoopState {
    base_url: String,
    api_key: String,
    model_id: String,
    messages: Vec<Value>,
    tools: Option<Vec<Value>>,
    stream: bool,
    is_reasoning: bool,
    thinking_budget: Option<i32>,
    temperature: Option<f64>,
    max_tokens: Option<i32>,
    use_response_api: bool,
    round: u32,
    system_prompt: Option<String>,
    custom_headers: Option<Vec<(String, String)>>,
    custom_body: Option<Value>,
}

/// Extract pending tool calls from collected events.
fn extract_pending_tool_calls(events: &[AgentEvent]) -> Vec<ToolCallInfo> {
    events
        .iter()
        .filter_map(|e| match e {
            AgentEvent::ToolCalls { calls } => Some(calls.clone()),
            _ => None,
        })
        .flatten()
        .collect()
}

/// Start the OpenAI-compatible agent loop. Returns first round result.
#[flutter_rust_bridge::frb]
pub fn start_openai_agent_loop(
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
    let messages: Vec<Value> =
        serde_json::from_str(&messages_json).map_err(|e| format!("Invalid messages: {}", e))?;
    let tools: Option<Vec<Value>> = match tools_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid tools: {}", e))?),
        None => None,
    };

    let state = AgentLoopState {
        base_url,
        api_key,
        model_id,
        messages: messages.clone(),
        tools: tools.clone(),
        stream,
        is_reasoning,
        thinking_budget,
        temperature,
        max_tokens,
        use_response_api,
        round: 1,
        system_prompt: None,
        custom_headers: None,
        custom_body: None,
    };

    let mut events = Vec::new();
    let result = agent_loop::openai_loop::run_openai_agent_loop(
        &state.base_url,
        &state.api_key,
        &state.model_id,
        &messages,
        tools.as_deref(),
        stream,
        is_reasoning,
        thinking_budget,
        temperature,
        max_tokens,
        use_response_api,
        None,
        None,
        |_name, _args| Ok("{}".to_string()),
        |event| events.push(event.clone()),
    );

    let content = result.unwrap_or_default();
    let pending_tool_calls = extract_pending_tool_calls(&events);
    let is_done = pending_tool_calls.is_empty();

    let loop_result = AgentLoopResult {
        events,
        pending_tool_calls,
        content,
        state_json: serde_json::to_string(&state).unwrap_or_default(),
        is_done,
    };
    serde_json::to_string(&loop_result).map_err(|e| format!("Serialize: {}", e))
}

/// Continue the agent loop after tool execution.
/// Receives tool results and runs the next round.
#[flutter_rust_bridge::frb]
pub fn continue_openai_agent_loop(
    state_json: String,
    tool_results_json: String,
) -> Result<String, String> {
    let mut state: AgentLoopState =
        serde_json::from_str(&state_json).map_err(|e| format!("Invalid state: {}", e))?;
    let tool_results: Vec<serde_json::Value> =
        serde_json::from_str(&tool_results_json).map_err(|e| format!("Invalid results: {}", e))?;

    // Append tool results to messages
    for tr in &tool_results {
        state.messages.push(tr.clone());
    }
    state.round += 1;

    let custom_headers_ref: Option<Vec<(String, String)>> = state.custom_headers.clone();
    let custom_body_ref: Option<Value> = state.custom_body.clone();

    let mut events = Vec::new();
    let result = agent_loop::openai_loop::run_openai_agent_loop(
        &state.base_url,
        &state.api_key,
        &state.model_id,
        &state.messages,
        state.tools.as_deref(),
        state.stream,
        state.is_reasoning,
        state.thinking_budget,
        state.temperature,
        state.max_tokens,
        state.use_response_api,
        custom_headers_ref.as_deref(),
        custom_body_ref.as_ref(),
        |_name, _args| Ok("{}".to_string()),
        |event| events.push(event.clone()),
    );

    let content = result.unwrap_or_default();
    let pending_tool_calls = extract_pending_tool_calls(&events);
    let is_done = pending_tool_calls.is_empty();

    let loop_result = AgentLoopResult {
        events,
        pending_tool_calls,
        content,
        state_json: serde_json::to_string(&state).unwrap_or_default(),
        is_done,
    };
    serde_json::to_string(&loop_result).map_err(|e| format!("Serialize: {}", e))
}

/// Start the Claude agent loop. Returns first round result.
#[flutter_rust_bridge::frb]
pub fn start_claude_agent_loop(
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
    let messages: Vec<Value> =
        serde_json::from_str(&messages_json).map_err(|e| format!("Invalid messages: {}", e))?;
    let tools: Option<Vec<Value>> = match tools_json {
        Some(s) => Some(serde_json::from_str(&s).map_err(|e| format!("Invalid tools: {}", e))?),
        None => None,
    };

    let state = AgentLoopState {
        base_url,
        api_key,
        model_id,
        messages: messages.clone(),
        tools: tools.clone(),
        stream,
        is_reasoning,
        thinking_budget,
        temperature,
        max_tokens,
        use_response_api: false,
        round: 1,
        system_prompt: system_prompt.clone(),
        custom_headers: None,
        custom_body: None,
    };

    let mut events = Vec::new();
    let result = agent_loop::claude_loop::run_claude_agent_loop(
        &state.base_url,
        &state.api_key,
        &state.model_id,
        &messages,
        system_prompt.as_deref(),
        tools.as_deref(),
        stream,
        is_reasoning,
        thinking_budget,
        temperature,
        max_tokens,
        None,
        None,
        |_name, _args| Ok("{}".to_string()),
        |event| events.push(event.clone()),
    );

    let content = result.unwrap_or_default();
    let pending_tool_calls = extract_pending_tool_calls(&events);
    let is_done = pending_tool_calls.is_empty();

    let loop_result = AgentLoopResult {
        events,
        pending_tool_calls,
        content,
        state_json: serde_json::to_string(&state).unwrap_or_default(),
        is_done,
    };
    serde_json::to_string(&loop_result).map_err(|e| format!("Serialize: {}", e))
}

/// Continue the Claude agent loop after tool execution.
#[flutter_rust_bridge::frb]
pub fn continue_claude_agent_loop(
    state_json: String,
    tool_results_json: String,
) -> Result<String, String> {
    let mut state: AgentLoopState =
        serde_json::from_str(&state_json).map_err(|e| format!("Invalid state: {}", e))?;
    let tool_results: Vec<serde_json::Value> =
        serde_json::from_str(&tool_results_json).map_err(|e| format!("Invalid results: {}", e))?;

    // Append tool results to messages (Claude uses user role with tool_result blocks)
    for tr in &tool_results {
        state.messages.push(tr.clone());
    }
    state.round += 1;

    let custom_headers_ref: Option<Vec<(String, String)>> = state.custom_headers.clone();
    let custom_body_ref: Option<Value> = state.custom_body.clone();

    let mut events = Vec::new();
    let result = agent_loop::claude_loop::run_claude_agent_loop(
        &state.base_url,
        &state.api_key,
        &state.model_id,
        &state.messages,
        state.system_prompt.as_deref(),
        state.tools.as_deref(),
        state.stream,
        state.is_reasoning,
        state.thinking_budget,
        state.temperature,
        state.max_tokens,
        custom_headers_ref.as_deref(),
        custom_body_ref.as_ref(),
        |_name, _args| Ok("{}".to_string()),
        |event| events.push(event.clone()),
    );

    let content = result.unwrap_or_default();
    let pending_tool_calls = extract_pending_tool_calls(&events);
    let is_done = pending_tool_calls.is_empty();

    let loop_result = AgentLoopResult {
        events,
        pending_tool_calls,
        content,
        state_json: serde_json::to_string(&state).unwrap_or_default(),
        is_done,
    };
    serde_json::to_string(&loop_result).map_err(|e| format!("Serialize: {}", e))
}

