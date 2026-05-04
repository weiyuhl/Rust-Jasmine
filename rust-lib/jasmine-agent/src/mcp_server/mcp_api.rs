use serde_json::Value;

use crate::{
    mcp_server::models::{ClientInfo, InitializeRequest, JsonRpcMessage},
    mcp_server::protocol::{
        is_version_supported, negotiate_version, McpTransportType, DEFAULT_VERSION,
        SUPPORTED_VERSIONS,
    },
    mcp_server::server::{
        export_servers_as_ui_json, normalize_args_for_tool, McpServerConfig, McpServerManager,
        McpToolConfig,
    },
};

// ── FRB exposed API ────────────────────────────────────────

/// Get supported MCP protocol versions.
pub fn supported_mcp_versions() -> Vec<String> {
    SUPPORTED_VERSIONS.iter().map(|v| v.to_string()).collect()
}

/// Get the default MCP protocol version.
pub fn default_mcp_version() -> String {
    DEFAULT_VERSION.to_string()
}

/// Negotiate the best common protocol version.
pub fn negotiate_mcp_version(
    client_versions: Vec<String>,
    server_versions: Vec<String>,
) -> Option<String> {
    negotiate_version(&client_versions, &server_versions)
}

/// Check if a protocol version is supported.
pub fn is_mcp_version_supported(version: String) -> bool {
    is_version_supported(&version)
}

/// Build a JSON-RPC request message.
pub fn build_json_rpc_request(
    id_json: String,
    method: String,
    params_json: Option<String>,
) -> Result<String, String> {
    let id: Value =
        serde_json::from_str(&id_json).map_err(|e| format!("Invalid id JSON: {}", e))?;
    let params: Option<Value> = params_json
        .map(|s| serde_json::from_str(&s))
        .transpose()
        .map_err(|e| format!("Invalid params JSON: {}", e))?;
    let msg = JsonRpcMessage::request(id, method, params);
    serde_json::to_string(&msg).map_err(|e| format!("Serialize: {}", e))
}

/// Build a JSON-RPC notification message.
pub fn build_json_rpc_notification(
    method: String,
    params_json: Option<String>,
) -> Result<String, String> {
    let params: Option<Value> = params_json
        .map(|s| serde_json::from_str(&s))
        .transpose()
        .map_err(|e| format!("Invalid params JSON: {}", e))?;
    let msg = JsonRpcMessage::notification(method, params);
    serde_json::to_string(&msg).map_err(|e| format!("Serialize: {}", e))
}

/// Build an initialize request.
pub fn build_initialize_request(
    client_name: String,
    client_version: String,
    protocol_version: Option<String>,
) -> Result<String, String> {
    let req = InitializeRequest {
        client_info: ClientInfo {
            name: client_name,
            version: client_version,
            capabilities: None,
            metadata: None,
        },
        protocol_version: protocol_version.unwrap_or_else(|| DEFAULT_VERSION.to_string()),
    };
    serde_json::to_string(&req).map_err(|e| format!("Serialize: {}", e))
}

/// Validate an MCP server config.
pub fn validate_mcp_server_config(config_json: String) -> Result<(), String> {
    let config: McpServerConfig =
        serde_json::from_str(&config_json).map_err(|e| format!("Invalid config JSON: {}", e))?;
    let errors = McpServerManager::validate_config(&config);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

/// Build a default MCP server config JSON template.
pub fn create_default_mcp_config(
    name: String,
    transport: String,
    url: String,
) -> Result<String, String> {
    let transport = match transport.to_lowercase().as_str() {
        "sse" => McpTransportType::Sse,
        "http" => McpTransportType::Http,
        "stdio" => McpTransportType::Stdio,
        "inmemory" => McpTransportType::InMemory,
        _ => McpTransportType::Sse,
    };
    let config = McpServerConfig {
        id: uuid::Uuid::new_v4().to_string(),
        enabled: true,
        name,
        transport,
        url,
        tools: vec![],
        headers: std::collections::HashMap::new(),
        command: None,
        args: vec![],
        env: std::collections::HashMap::new(),
        working_directory: None,
    };
    serde_json::to_string(&config).map_err(|e| format!("Serialize: {}", e))
}

/// Parse connection metadata into a displayable string (tool names, etc.).
pub fn summarize_server_tools(tools_json: String) -> Result<String, String> {
    let tools: Vec<McpToolConfig> =
        serde_json::from_str(&tools_json).map_err(|e| format!("Invalid tools JSON: {}", e))?;
    let enabled: Vec<&str> = tools
        .iter()
        .filter(|t| t.enabled)
        .map(|t| t.name.as_str())
        .collect();
    let disabled: Vec<&str> = tools
        .iter()
        .filter(|t| !t.enabled)
        .map(|t| t.name.as_str())
        .collect();
    let mut parts = Vec::new();
    if !enabled.is_empty() {
        parts.push(format!("Enabled: {}", enabled.join(", ")));
    }
    if !disabled.is_empty() {
        parts.push(format!("Disabled: {}", disabled.join(", ")));
    }
    Ok(parts.join(" | "))
}

/// Convert McpLogLevel name to its string representation.
pub fn log_level_name(level: String) -> String {
    match level.as_str() {
        "debug" => "debug",
        "info" => "info",
        "notice" => "notice",
        "warning" => "warning",
        "error" => "error",
        "critical" => "critical",
        "alert" => "alert",
        "emergency" => "emergency",
        _ => "info",
    }
    .to_string()
}

/// Export server configs to UI-friendly JSON format.
pub fn export_mcp_servers_ui_json(
    servers_json: String,
    is_desktop: bool,
) -> Result<String, String> {
    let servers: Vec<McpServerConfig> =
        serde_json::from_str(&servers_json).map_err(|e| format!("Invalid JSON: {}", e))?;
    let output = export_servers_as_ui_json(&servers, is_desktop);
    serde_json::to_string(&output).map_err(|e| format!("Serialize: {}", e))
}

/// Parse and replace all servers from various import JSON formats.
/// Returns the parsed list of McpServerConfig as JSON.
pub fn parse_mcp_import_json(raw_json: String, is_desktop: bool) -> Result<String, String> {
    let parsed: Value =
        serde_json::from_str(&raw_json).map_err(|e| format!("Invalid JSON: {}", e))?;

    let mut next: Vec<McpServerConfig> = Vec::new();
    let mut builtin_seen = false;
    let mut builtin_enabled = true;

    // Format 1: { "mcpServers": { id: { ... } } }
    if let Some(servers_map) = parsed.get("mcpServers").and_then(|v| v.as_object()) {
        for (id, cfg) in servers_map {
            let cfg = match cfg.as_object() {
                Some(o) => o,
                None => continue,
            };
            let type_lower = cfg
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_lowercase();
            if type_lower == "inmemory" {
                builtin_seen = true;
                builtin_enabled = cfg
                    .get("isActive")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                continue;
            }
            let has_stdio_shape = cfg.contains_key("command")
                || cfg.contains_key("args")
                || cfg.contains_key("env")
                || type_lower == "stdio";
            if has_stdio_shape {
                if !is_desktop {
                    continue;
                }
                let enabled = cfg
                    .get("isActive")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true);
                let name = cfg
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or(id)
                    .to_string();
                let cmd = match cfg.get("command").and_then(|v| v.as_str()) {
                    Some(c) if !c.trim().is_empty() => c.trim().to_string(),
                    _ => continue,
                };
                let args: Vec<String> = cfg
                    .get("args")
                    .and_then(|v| v.as_array())
                    .map(|a| {
                        a.iter()
                            .filter_map(|x| x.as_str().map(String::from))
                            .collect()
                    })
                    .unwrap_or_default();
                let env: std::collections::HashMap<String, String> = cfg
                    .get("env")
                    .and_then(|v| v.as_object())
                    .map(|o| {
                        o.iter()
                            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                            .collect()
                    })
                    .unwrap_or_default();
                let wd = cfg
                    .get("workingDirectory")
                    .and_then(|v| v.as_str())
                    .map(String::from);
                let name_final = if name.trim().is_empty() {
                    id.clone()
                } else {
                    name
                };

                next.push(McpServerConfig {
                    id: id.clone(),
                    enabled,
                    name: name_final,
                    transport: McpTransportType::Stdio,
                    url: String::new(),
                    tools: vec![],
                    headers: std::collections::HashMap::new(),
                    command: Some(cmd),
                    args,
                    env,
                    working_directory: wd,
                });
                continue;
            }

            // SSE/HTTP
            let transport = if type_lower.contains("http") {
                McpTransportType::Http
            } else {
                McpTransportType::Sse
            };
            let enabled = cfg
                .get("isActive")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let name = cfg
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or(id)
                .to_string();
            let url = match cfg
                .get("baseUrl")
                .and_then(|v| v.as_str())
                .map(|s| s.trim().to_string())
            {
                Some(u) if !u.is_empty() => u,
                _ => continue,
            };
            let headers: std::collections::HashMap<String, String> = cfg
                .get("headers")
                .and_then(|v| v.as_object())
                .map(|o| {
                    o.iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect()
                })
                .unwrap_or_default();
            let name_final = if name.trim().is_empty() {
                id.clone()
            } else {
                name
            };

            next.push(McpServerConfig {
                id: id.clone(),
                enabled,
                name: name_final,
                transport,
                url,
                tools: vec![],
                headers,
                command: None,
                args: vec![],
                env: std::collections::HashMap::new(),
                working_directory: None,
            });
        }
        if builtin_seen {
            next.push(McpServerConfig {
                id: "kelivo_fetch".to_string(),
                enabled: builtin_enabled,
                name: "@kelivo/fetch".to_string(),
                transport: McpTransportType::InMemory,
                url: String::new(),
                tools: vec![],
                headers: std::collections::HashMap::new(),
                command: None,
                args: vec![],
                env: std::collections::HashMap::new(),
                working_directory: None,
            });
        }
    } else {
        return Err("Unrecognized MCP JSON format. Expected 'mcpServers' key.".to_string());
    }

    if next.is_empty() {
        return Err("No valid MCP servers found in JSON".to_string());
    }
    serde_json::to_string(&next).map_err(|e| format!("Serialize: {}", e))
}

/// Normalize tool call arguments based on the tool's JSON Schema.
pub fn normalize_tool_arguments(
    schema_json: Option<String>,
    args_json: String,
) -> Result<String, String> {
    let schema: Option<Value> = schema_json
        .map(|s| serde_json::from_str(&s))
        .transpose()
        .map_err(|e| format!("Invalid schema JSON: {}", e))?;
    let args: Value =
        serde_json::from_str(&args_json).map_err(|e| format!("Invalid args JSON: {}", e))?;
    let normalized = normalize_args_for_tool(schema.as_ref(), &args);
    serde_json::to_string(&normalized).map_err(|e| format!("Serialize: {}", e))
}

/// Build a tools/list JSON-RPC request.
pub fn build_list_tools_request(id_json: String) -> Result<String, String> {
    build_json_rpc_request(
        id_json,
        crate::mcp_server::protocol::methods::LIST_TOOLS.to_string(),
        None,
    )
}

/// Build a tools/call JSON-RPC request.
pub fn build_call_tool_request(
    id_json: String,
    tool_name: String,
    args_json: Option<String>,
) -> Result<String, String> {
    let args: Option<Value> = args_json
        .map(|s| serde_json::from_str(&s))
        .transpose()
        .map_err(|e| format!("Invalid args JSON: {}", e))?;
    let params = serde_json::json!({
        "name": tool_name,
        "arguments": args.unwrap_or(serde_json::json!({}))
    });
    let msg = JsonRpcMessage::request(
        serde_json::from_str(&id_json).map_err(|e| format!("Invalid id: {}", e))?,
        crate::mcp_server::protocol::methods::CALL_TOOL.to_string(),
        Some(params),
    );
    serde_json::to_string(&msg).map_err(|e| format!("Serialize: {}", e))
}

/// Build a resources/list JSON-RPC request.
pub fn build_list_resources_request(id_json: String) -> Result<String, String> {
    build_json_rpc_request(
        id_json,
        crate::mcp_server::protocol::methods::LIST_RESOURCES.to_string(),
        None,
    )
}

/// Build a resources/read JSON-RPC request.
pub fn build_read_resource_request(id_json: String, uri: String) -> Result<String, String> {
    let params = serde_json::json!({"uri": uri});
    let msg = JsonRpcMessage::request(
        serde_json::from_str(&id_json).map_err(|e| format!("Invalid id: {}", e))?,
        crate::mcp_server::protocol::methods::READ_RESOURCE.to_string(),
        Some(params),
    );
    serde_json::to_string(&msg).map_err(|e| format!("Serialize: {}", e))
}

/// Build a prompts/list JSON-RPC request.
pub fn build_list_prompts_request(id_json: String) -> Result<String, String> {
    build_json_rpc_request(
        id_json,
        crate::mcp_server::protocol::methods::LIST_PROMPTS.to_string(),
        None,
    )
}

/// Build a prompts/get JSON-RPC request.
pub fn build_get_prompt_request(
    id_json: String,
    prompt_name: String,
    args_json: Option<String>,
) -> Result<String, String> {
    let args: Option<Value> = args_json
        .map(|s| serde_json::from_str(&s))
        .transpose()
        .map_err(|e| format!("Invalid args JSON: {}", e))?;
    let params = serde_json::json!({
        "name": prompt_name,
        "arguments": args.unwrap_or(serde_json::json!({}))
    });
    let msg = JsonRpcMessage::request(
        serde_json::from_str(&id_json).map_err(|e| format!("Invalid id: {}", e))?,
        crate::mcp_server::protocol::methods::GET_PROMPT.to_string(),
        Some(params),
    );
    serde_json::to_string(&msg).map_err(|e| format!("Serialize: {}", e))
}

/// Build the standard list of MCP method name constants (as JSON object).
pub fn get_mcp_method_names() -> String {
    serde_json::json!({
        "initialize": crate::mcp_server::protocol::methods::INITIALIZE,
        "initialized": crate::mcp_server::protocol::methods::INITIALIZED,
        "shutdown": crate::mcp_server::protocol::methods::SHUTDOWN,
        "listTools": crate::mcp_server::protocol::methods::LIST_TOOLS,
        "callTool": crate::mcp_server::protocol::methods::CALL_TOOL,
        "listResources": crate::mcp_server::protocol::methods::LIST_RESOURCES,
        "readResource": crate::mcp_server::protocol::methods::READ_RESOURCE,
        "listPrompts": crate::mcp_server::protocol::methods::LIST_PROMPTS,
        "getPrompt": crate::mcp_server::protocol::methods::GET_PROMPT,
        "complete": crate::mcp_server::protocol::methods::COMPLETE,
        "setLevel": crate::mcp_server::protocol::methods::SET_LOG_LEVEL,
        "logMessage": crate::mcp_server::protocol::methods::LOG_MESSAGE,
        "progress": crate::mcp_server::protocol::methods::PROGRESS,
        "healthCheck": crate::mcp_server::protocol::methods::HEALTH_CHECK,
    })
    .to_string()
}

/// Build a health/check JSON-RPC request (2025-03-26).
pub fn build_health_check_request(id_json: String) -> Result<String, String> {
    build_json_rpc_request(
        id_json,
        crate::mcp_server::protocol::methods::HEALTH_CHECK.to_string(),
        None,
    )
}

/// Get standard JSON-RPC error details by code.
pub fn json_rpc_error_details(code: i32) -> String {
    let (name, default_msg) = match code {
        crate::mcp_server::protocol::ERROR_PARSE => ("ParseError", "Invalid JSON was received"),
        crate::mcp_server::protocol::ERROR_INVALID_REQUEST => (
            "InvalidRequest",
            "The JSON sent is not a valid Request object",
        ),
        crate::mcp_server::protocol::ERROR_METHOD_NOT_FOUND => (
            "MethodNotFound",
            "The method does not exist / is not available",
        ),
        crate::mcp_server::protocol::ERROR_INVALID_PARAMS => {
            ("InvalidParams", "Invalid method parameter(s)")
        }
        crate::mcp_server::protocol::ERROR_INTERNAL => ("InternalError", "Internal JSON-RPC error"),
        crate::mcp_server::protocol::ERROR_RESOURCE_NOT_FOUND => {
            ("ResourceNotFound", "Resource not found")
        }
        crate::mcp_server::protocol::ERROR_TOOL_NOT_FOUND => ("ToolNotFound", "Tool not found"),
        crate::mcp_server::protocol::ERROR_TOOL_EXECUTION_FAILED => {
            ("ToolExecutionFailed", "Tool execution failed")
        }
        crate::mcp_server::protocol::ERROR_PROMPT_NOT_FOUND => {
            ("PromptNotFound", "Prompt not found")
        }
        _ => ("Unknown", ""),
    };
    serde_json::json!({"name": name, "message": default_msg}).to_string()
}
