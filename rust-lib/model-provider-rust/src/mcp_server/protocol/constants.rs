use serde::{Deserialize, Serialize};

/// MCP protocol version strings.
pub const V2025_03_26: &str = "2025-03-26";
pub const V2024_11_05: &str = "2024-11-05";

/// Supported protocol versions in order of preference.
pub const SUPPORTED_VERSIONS: &[&str] = &[V2025_03_26, V2024_11_05];
pub const DEFAULT_VERSION: &str = V2025_03_26;
pub const JSON_RPC_VERSION: &str = "2.0";

/// Standard JSON-RPC error codes.
pub const ERROR_PARSE: i32 = -32700;
pub const ERROR_INVALID_REQUEST: i32 = -32600;
pub const ERROR_METHOD_NOT_FOUND: i32 = -32601;
pub const ERROR_INVALID_PARAMS: i32 = -32602;
pub const ERROR_INTERNAL: i32 = -32603;

/// MCP-specific error codes.
pub const ERROR_RESOURCE_NOT_FOUND: i32 = -32001;
pub const ERROR_RESOURCE_ACCESS_DENIED: i32 = -32002;
pub const ERROR_TOOL_NOT_FOUND: i32 = -32003;
pub const ERROR_TOOL_EXECUTION_FAILED: i32 = -32004;
pub const ERROR_PROMPT_NOT_FOUND: i32 = -32005;
pub const ERROR_PROTOCOL_ERROR: i32 = -32006;

/// Standard MCP method names.
pub mod methods {
    pub const INITIALIZE: &str = "initialize";
    pub const INITIALIZED: &str = "notifications/initialized";
    pub const SHUTDOWN: &str = "shutdown";
    pub const LIST_TOOLS: &str = "tools/list";
    pub const CALL_TOOL: &str = "tools/call";
    pub const CANCEL_TOOL: &str = "tools/cancel";
    pub const LIST_RESOURCES: &str = "resources/list";
    pub const READ_RESOURCE: &str = "resources/read";
    pub const SUBSCRIBE_RESOURCE: &str = "resources/subscribe";
    pub const UNSUBSCRIBE_RESOURCE: &str = "resources/unsubscribe";
    pub const LIST_RESOURCE_TEMPLATES: &str = "resources/templates/list";
    pub const LIST_PROMPTS: &str = "prompts/list";
    pub const GET_PROMPT: &str = "prompts/get";
    pub const COMPLETE: &str = "completion/complete";
    pub const LIST_ROOTS: &str = "roots/list";
    pub const ADD_ROOT: &str = "roots/add";
    pub const REMOVE_ROOT: &str = "roots/remove";
    // 2025-03-26 new
    pub const BATCH: &str = "batch";
    pub const HEALTH_CHECK: &str = "health/check";
    pub const CAPABILITIES_UPDATE: &str = "capabilities/update";
    // Notifications
    pub const PROGRESS: &str = "notifications/progress";
    pub const CANCELLED: &str = "notifications/cancelled";
    pub const RESOURCE_UPDATED: &str = "notifications/resources/updated";
    pub const RESOURCE_LIST_CHANGED: &str = "notifications/resources/list_changed";
    pub const TOOL_LIST_CHANGED: &str = "notifications/tools/list_changed";
    pub const PROMPT_LIST_CHANGED: &str = "notifications/prompts/list_changed";
    pub const ROOT_LIST_CHANGED: &str = "notifications/roots/list_changed";
    pub const LOG_MESSAGE: &str = "notifications/message";
    pub const SET_LOG_LEVEL: &str = "logging/setLevel";
    // Auth (2025-03-26)
    pub const AUTHORIZE: &str = "auth/authorize";
    pub const TOKEN: &str = "auth/token";
    pub const REVOKE: &str = "auth/revoke";
    pub const REFRESH: &str = "auth/refresh";
}

/// Check if a protocol version is supported.
pub fn is_version_supported(version: &str) -> bool {
    SUPPORTED_VERSIONS.contains(&version)
}

/// Negotiate the best common version between client and server.
pub fn negotiate_version(client_versions: &[String], server_versions: &[String]) -> Option<String> {
    for v in client_versions {
        if server_versions.contains(v) && is_version_supported(v) {
            return Some(v.clone());
        }
    }
    None
}

/// MCP transport type.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum McpTransportType {
    #[serde(rename = "sse")]
    Sse,
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "stdio")]
    Stdio,
    #[serde(rename = "inmemory")]
    InMemory,
}

/// MCP connection status.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum McpStatus {
    #[serde(rename = "idle")]
    Idle,
    #[serde(rename = "connecting")]
    Connecting,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "error")]
    Error,
}

/// MCP log levels.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum McpLogLevel {
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "notice")]
    Notice,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "critical")]
    Critical,
    #[serde(rename = "alert")]
    Alert,
    #[serde(rename = "emergency")]
    Emergency,
}

impl McpLogLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            McpLogLevel::Debug => "debug",
            McpLogLevel::Info => "info",
            McpLogLevel::Notice => "notice",
            McpLogLevel::Warning => "warning",
            McpLogLevel::Error => "error",
            McpLogLevel::Critical => "critical",
            McpLogLevel::Alert => "alert",
            McpLogLevel::Emergency => "emergency",
        }
    }
}

/// MCP error type.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpError {
    pub message: String,
    pub code: Option<i32>,
}

impl McpError {
    pub fn new(message: String) -> Self {
        McpError { message, code: None }
    }

    pub fn with_code(message: String, code: i32) -> Self {
        McpError {
            message,
            code: Some(code),
        }
    }
}

impl std::fmt::Display for McpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.code {
            Some(c) => write!(f, "McpError ({}): {}", c, self.message),
            None => write!(f, "McpError: {}", self.message),
        }
    }
}
