// MCP Client FRB API — replaces mcp_client Dart library.
// Provides initialize, list tools, call tool, list resources/prompts via Rust HTTP.

use std::collections::HashMap;

use crate::mcp_client::connection::{self};
use crate::mcp_client::transport::{McpTransportConfig, McpTransportType};

/// Initialize MCP connection and return server info + tools list in one call.
#[flutter_rust_bridge::frb(sync)]
pub fn mcp_connect(
    url: String,
    headers_json: Option<String>,
    transport: String,
    timeout_ms: u64,
) -> Result<String, String> {
    let transport_type = match transport.as_str() {
        "sse" => McpTransportType::Sse,
        "http" => McpTransportType::StreamableHttp,
        _ => return Err(format!("Unsupported transport: {}", transport)),
    };
    let headers = parse_headers(headers_json)?;
    let config = McpTransportConfig {
        transport_type,
        url,
        headers,
        timeout_ms,
    };

    let (server_info, tools) = connection::connect_and_list_tools(&config)?;

    let result = serde_json::json!({
        "serverInfo": server_info,
        "tools": tools,
    });
    serde_json::to_string(&result).map_err(|e| format!("Serialize: {}", e))
}

/// List tools from an MCP server.
#[flutter_rust_bridge::frb(sync)]
pub fn mcp_list_tools(
    url: String,
    headers_json: Option<String>,
    transport: String,
    timeout_ms: u64,
) -> Result<String, String> {
    let config = build_config(url, headers_json, transport, timeout_ms)?;
    let tools = connection::list_tools(&config)?;
    serde_json::to_string(&tools).map_err(|e| format!("Serialize: {}", e))
}

/// Call a tool on an MCP server.
#[flutter_rust_bridge::frb(sync)]
pub fn mcp_call_tool(
    url: String,
    headers_json: Option<String>,
    transport: String,
    tool_name: String,
    args_json: Option<String>,
    timeout_ms: u64,
) -> Result<String, String> {
    let config = build_config(url, headers_json, transport, timeout_ms)?;
    let args = args_json
        .map(|s| serde_json::from_str(&s))
        .transpose()
        .map_err(|e| format!("Invalid args: {}", e))?;

    let result = connection::call_tool(&config, &tool_name, args)?;
    serde_json::to_string(&result).map_err(|e| format!("Serialize: {}", e))
}

/// List resources from an MCP server.
#[flutter_rust_bridge::frb(sync)]
pub fn mcp_list_resources(
    url: String,
    headers_json: Option<String>,
    transport: String,
    timeout_ms: u64,
) -> Result<String, String> {
    let config = build_config(url, headers_json, transport, timeout_ms)?;
    let resources = connection::list_resources(&config)?;
    serde_json::to_string(&resources).map_err(|e| format!("Serialize: {}", e))
}

/// List prompts from an MCP server.
#[flutter_rust_bridge::frb(sync)]
pub fn mcp_list_prompts(
    url: String,
    headers_json: Option<String>,
    transport: String,
    timeout_ms: u64,
) -> Result<String, String> {
    let config = build_config(url, headers_json, transport, timeout_ms)?;
    let prompts = connection::list_prompts(&config)?;
    serde_json::to_string(&prompts).map_err(|e| format!("Serialize: {}", e))
}

/// Build MCP transport config JSON (for Dart to use when calling other MCP functions).
#[flutter_rust_bridge::frb(sync)]
pub fn mcp_build_transport_config(
    url: String,
    headers_json: Option<String>,
    transport: String,
    timeout_ms: u64,
) -> Result<String, String> {
    let transport_type = match transport.as_str() {
        "sse" => McpTransportType::Sse,
        "http" => McpTransportType::StreamableHttp,
        _ => return Err(format!("Unsupported transport: {}", transport)),
    };
    let headers = parse_headers(headers_json)?;
    let config = McpTransportConfig {
        transport_type,
        url,
        headers,
        timeout_ms,
    };
    serde_json::to_string(&config).map_err(|e| format!("Serialize: {}", e))
}

// ── helpers ──

fn build_config(
    url: String,
    headers_json: Option<String>,
    transport: String,
    timeout_ms: u64,
) -> Result<McpTransportConfig, String> {
    let transport_type = match transport.as_str() {
        "sse" => McpTransportType::Sse,
        "http" => McpTransportType::StreamableHttp,
        _ => return Err(format!("Unsupported transport: {}", transport)),
    };
    let headers = parse_headers(headers_json)?;
    Ok(McpTransportConfig {
        transport_type,
        url,
        headers,
        timeout_ms,
    })
}

fn parse_headers(headers_json: Option<String>) -> Result<HashMap<String, String>, String> {
    match headers_json {
        Some(s) => serde_json::from_str(&s).map_err(|e| format!("Invalid headers: {}", e)),
        None => Ok(HashMap::new()),
    }
}
