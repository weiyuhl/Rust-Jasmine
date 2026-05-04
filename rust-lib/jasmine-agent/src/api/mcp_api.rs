// MCP API bridge — FRB-annotated functions that delegate to mcp_server modules.

use crate::mcp_server::mcp_api as mcp;

// ── Protocol & versioning ───────────────────────────────────

#[flutter_rust_bridge::frb(sync)]
pub fn supported_mcp_versions() -> Vec<String> {
    mcp::supported_mcp_versions()
}

#[flutter_rust_bridge::frb(sync)]
pub fn default_mcp_version() -> String {
    mcp::default_mcp_version()
}

#[flutter_rust_bridge::frb(sync)]
pub fn negotiate_mcp_version(
    client_versions: Vec<String>,
    server_versions: Vec<String>,
) -> Option<String> {
    mcp::negotiate_mcp_version(client_versions, server_versions)
}

#[flutter_rust_bridge::frb(sync)]
pub fn is_mcp_version_supported(version: String) -> bool {
    mcp::is_mcp_version_supported(version)
}

// ── JSON-RPC building ───────────────────────────────────────

#[flutter_rust_bridge::frb]
pub fn build_json_rpc_request(
    id_json: String,
    method: String,
    params_json: Option<String>,
) -> Result<String, String> {
    mcp::build_json_rpc_request(id_json, method, params_json)
}

#[flutter_rust_bridge::frb]
pub fn build_json_rpc_notification(
    method: String,
    params_json: Option<String>,
) -> Result<String, String> {
    mcp::build_json_rpc_notification(method, params_json)
}

#[flutter_rust_bridge::frb]
pub fn build_initialize_request(
    client_name: String,
    client_version: String,
    protocol_version: Option<String>,
) -> Result<String, String> {
    mcp::build_initialize_request(client_name, client_version, protocol_version)
}

#[flutter_rust_bridge::frb]
pub fn build_list_tools_request(id_json: String) -> Result<String, String> {
    mcp::build_list_tools_request(id_json)
}

#[flutter_rust_bridge::frb]
pub fn build_call_tool_request(
    id_json: String,
    tool_name: String,
    args_json: Option<String>,
) -> Result<String, String> {
    mcp::build_call_tool_request(id_json, tool_name, args_json)
}

#[flutter_rust_bridge::frb]
pub fn build_list_resources_request(id_json: String) -> Result<String, String> {
    mcp::build_list_resources_request(id_json)
}

#[flutter_rust_bridge::frb]
pub fn build_read_resource_request(id_json: String, uri: String) -> Result<String, String> {
    mcp::build_read_resource_request(id_json, uri)
}

#[flutter_rust_bridge::frb]
pub fn build_list_prompts_request(id_json: String) -> Result<String, String> {
    mcp::build_list_prompts_request(id_json)
}

#[flutter_rust_bridge::frb]
pub fn build_get_prompt_request(
    id_json: String,
    prompt_name: String,
    args_json: Option<String>,
) -> Result<String, String> {
    mcp::build_get_prompt_request(id_json, prompt_name, args_json)
}

#[flutter_rust_bridge::frb]
pub fn build_health_check_request(id_json: String) -> Result<String, String> {
    mcp::build_health_check_request(id_json)
}

// ── Server config ───────────────────────────────────────────

#[flutter_rust_bridge::frb]
pub fn validate_mcp_server_config(config_json: String) -> Result<(), String> {
    mcp::validate_mcp_server_config(config_json)
}

#[flutter_rust_bridge::frb]
pub fn create_default_mcp_config(
    name: String,
    transport: String,
    url: String,
) -> Result<String, String> {
    mcp::create_default_mcp_config(name, transport, url)
}

#[flutter_rust_bridge::frb]
pub fn summarize_server_tools(tools_json: String) -> Result<String, String> {
    mcp::summarize_server_tools(tools_json)
}

#[flutter_rust_bridge::frb(sync)]
pub fn export_mcp_servers_ui_json(
    servers_json: String,
    is_desktop: bool,
) -> Result<String, String> {
    mcp::export_mcp_servers_ui_json(servers_json, is_desktop)
}

#[flutter_rust_bridge::frb(sync)]
pub fn parse_mcp_import_json(raw_json: String, is_desktop: bool) -> Result<String, String> {
    mcp::parse_mcp_import_json(raw_json, is_desktop)
}

// ── Tool arguments ──────────────────────────────────────────

#[flutter_rust_bridge::frb(sync)]
pub fn normalize_tool_arguments(
    schema_json: Option<String>,
    args_json: String,
) -> Result<String, String> {
    mcp::normalize_tool_arguments(schema_json, args_json)
}

// ── Constants ───────────────────────────────────────────────

#[flutter_rust_bridge::frb(sync)]
pub fn get_mcp_method_names() -> String {
    mcp::get_mcp_method_names()
}

#[flutter_rust_bridge::frb(sync)]
pub fn json_rpc_error_details(code: i32) -> String {
    mcp::json_rpc_error_details(code)
}

#[flutter_rust_bridge::frb(sync)]
pub fn log_level_name(level: String) -> String {
    mcp::log_level_name(level)
}
