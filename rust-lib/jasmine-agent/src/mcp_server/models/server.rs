use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Server information (2025-03-26 compliant).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    #[serde(rename = "protocolVersion", skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Client information (2025-03-26 compliant).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Server capabilities.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ServerCapabilities {
    #[serde(default)]
    pub tools: bool,
    #[serde(rename = "toolsListChanged", default)]
    pub tools_list_changed: bool,
    #[serde(default)]
    pub resources: bool,
    #[serde(rename = "resourcesListChanged", default)]
    pub resources_list_changed: bool,
    #[serde(default)]
    pub prompts: bool,
    #[serde(rename = "promptsListChanged", default)]
    pub prompts_list_changed: bool,
    #[serde(default)]
    pub logging: bool,
    #[serde(default)]
    pub sampling: bool,
}

/// Client capabilities configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClientCapabilities {
    #[serde(default)]
    pub roots: bool,
    #[serde(rename = "rootsListChanged", default)]
    pub roots_list_changed: bool,
    #[serde(default)]
    pub sampling: bool,
}

/// Initialize request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeRequest {
    #[serde(rename = "clientInfo")]
    pub client_info: ClientInfo,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}

/// Initialize result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeResult {
    #[serde(rename = "serverInfo")]
    pub server_info: ServerInfo,
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Value>,
}

/// Server health information.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerHealth {
    #[serde(default = "default_healthy")]
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(default)]
    pub connections: i32,
    #[serde(rename = "isRunning")]
    pub is_running: bool,
    #[serde(rename = "connectedSessions")]
    pub connected_sessions: i32,
    #[serde(rename = "registeredTools")]
    pub registered_tools: i32,
    #[serde(rename = "registeredResources")]
    pub registered_resources: i32,
    #[serde(rename = "registeredPrompts")]
    pub registered_prompts: i32,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "uptimeSeconds")]
    pub uptime_seconds: u64,
    #[serde(default)]
    pub metrics: Value,
}

fn default_healthy() -> String {
    "healthy".to_string()
}
