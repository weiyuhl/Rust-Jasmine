use std::sync::atomic::{AtomicI64, Ordering};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::transport::{self, McpTransportConfig};

/// MCP connection state.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum McpConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Error(String),
}

/// MCP client connection that manages a single MCP server connection.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpClientConnection {
    pub server_id: String,
    pub server_name: String,
    pub state: McpConnectionState,
    pub config: McpTransportConfig,
    pub server_info: Option<Value>,
    pub server_capabilities: Option<Value>,
}

/// Result of a tool call.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpToolCallResult {
    pub content: Vec<Value>,
    pub is_error: bool,
}

/// MCP event for streaming to Dart.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum McpEvent {
    Connected {
        server_info: Value,
    },
    ToolsListed {
        tools: Vec<Value>,
    },
    ToolCalled {
        result: McpToolCallResult,
    },
    Error {
        message: String,
        code: Option<i32>,
    },
    Disconnected,
}

static REQUEST_ID: AtomicI64 = AtomicI64::new(1);

fn next_request_id() -> i64 {
    REQUEST_ID.fetch_add(1, Ordering::Relaxed)
}

/// Build a JSON-RPC request message.
pub fn build_request(method: &str, params: Option<Value>) -> Value {
    let id = next_request_id();
    let mut msg = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
    });
    if let Some(p) = params {
        msg["params"] = p;
    }
    msg
}

/// Build a JSON-RPC notification (no id, no response expected).
pub fn build_notification(method: &str, params: Option<Value>) -> Value {
    let mut msg = serde_json::json!({
        "jsonrpc": "2.0",
        "method": method,
    });
    if let Some(p) = params {
        msg["params"] = p;
    }
    msg
}

/// Send a JSON-RPC request and get the response.
fn send_request(
    config: &McpTransportConfig,
    method: &str,
    params: Option<Value>,
) -> Result<Value, String> {
    let msg = build_request(method, params);
    let msg_str = serde_json::to_string(&msg).map_err(|e| format!("Serialize: {}", e))?;

    let result_str = transport::send_http_message(config, &msg_str)?;
    let result: Value =
        serde_json::from_str(&result_str).map_err(|e| format!("Parse response: {}", e))?;

    if let Some(error) = result.get("error") {
        let code = error.get("code").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
        let message = error
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown error");
        return Err(format!("JSON-RPC error {}: {}", code, message));
    }

    Ok(result.get("result").cloned().unwrap_or(Value::Null))
}

/// Send initialize request and parse server info.
pub fn initialize(config: &McpTransportConfig) -> Result<(Value, Value), String> {
    let params = serde_json::json!({
        "protocolVersion": "2025-03-26",
        "capabilities": {},
        "clientInfo": {
            "name": "Kelivo",
            "version": "1.0.0"
        }
    });

    let result = send_request(config, "initialize", Some(params))?;

    let server_info = result
        .get("serverInfo")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let capabilities = result
        .get("capabilities")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));

    // Send initialized notification
    let _ = send_notification(config, "notifications/initialized", None);

    Ok((server_info, capabilities))
}

/// Send a JSON-RPC notification (fire-and-forget).
fn send_notification(
    config: &McpTransportConfig,
    method: &str,
    params: Option<Value>,
) -> Result<(), String> {
    let msg = build_notification(method, params);
    let msg_str = serde_json::to_string(&msg).map_err(|e| format!("Serialize: {}", e))?;

    let _ = transport::send_http_message(config, &msg_str);
    Ok(())
}

/// List tools from the MCP server.
pub fn list_tools(config: &McpTransportConfig) -> Result<Vec<Value>, String> {
    let result = send_request(config, "tools/list", None)?;

    let tools = result
        .get("tools")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    Ok(tools)
}

/// Call a tool on the MCP server.
pub fn call_tool(
    config: &McpTransportConfig,
    tool_name: &str,
    arguments: Option<Value>,
) -> Result<McpToolCallResult, String> {
    let params = serde_json::json!({
        "name": tool_name,
        "arguments": arguments.unwrap_or(Value::Object(serde_json::Map::new()))
    });

    let result = send_request(config, "tools/call", Some(params))?;

    let content = result
        .get("content")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let is_error = result
        .get("isError")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    Ok(McpToolCallResult { content, is_error })
}

/// List resources from the MCP server.
pub fn list_resources(config: &McpTransportConfig) -> Result<Vec<Value>, String> {
    let result = send_request(config, "resources/list", None)?;

    let resources = result
        .get("resources")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    Ok(resources)
}

/// Read a resource from the MCP server.
pub fn read_resource(config: &McpTransportConfig, uri: &str) -> Result<Value, String> {
    let params = serde_json::json!({"uri": uri});
    let result = send_request(config, "resources/read", Some(params))?;
    Ok(result)
}

/// List prompts from the MCP server.
pub fn list_prompts(config: &McpTransportConfig) -> Result<Vec<Value>, String> {
    let result = send_request(config, "prompts/list", None)?;

    let prompts = result
        .get("prompts")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    Ok(prompts)
}

/// Get a prompt from the MCP server.
pub fn get_prompt(
    config: &McpTransportConfig,
    name: &str,
    arguments: Option<Value>,
) -> Result<Value, String> {
    let params = serde_json::json!({
        "name": name,
        "arguments": arguments.unwrap_or(Value::Object(serde_json::Map::new()))
    });

    let result = send_request(config, "prompts/get", Some(params))?;
    Ok(result)
}

/// Complete the MCP connection lifecycle: initialize → list tools.
pub fn connect_and_list_tools(
    config: &McpTransportConfig,
) -> Result<(Value, Vec<Value>), String> {
    let (server_info, _capabilities) = initialize(config)?;
    let tools = list_tools(config)?;
    Ok((server_info, tools))
}
