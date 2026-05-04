use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::mcp_server::protocol::McpTransportType;

/// MCP tool parameter specification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpParamSpec {
    pub name: String,
    #[serde(default)]
    pub required: bool,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub param_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
}

/// MCP tool configuration (per-server).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpToolConfig {
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub params: Vec<McpParamSpec>,
    /// Raw JSON schema for parameters, if provided by the server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// Whether this tool requires user approval before execution.
    #[serde(rename = "needsApproval", default)]
    pub needs_approval: bool,
}

fn default_enabled() -> bool {
    true
}

/// MCP server configuration.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpServerConfig {
    /// Stable identifier.
    pub id: String,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    pub name: String,
    pub transport: McpTransportType,
    /// SSE endpoint or HTTP base URL.
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub tools: Vec<McpToolConfig>,
    /// Custom HTTP headers.
    #[serde(default)]
    pub headers: std::collections::HashMap<String, String>,
    /// For STDIO (desktop-only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
    #[serde(rename = "workingDirectory", skip_serializing_if = "Option::is_none")]
    pub working_directory: Option<String>,
}

impl McpServerConfig {
    /// Get enabled tools for this server.
    pub fn enabled_tools(&self) -> Vec<&McpToolConfig> {
        self.tools.iter().filter(|t| t.enabled).collect()
    }

    /// Check if this server has a specific tool enabled.
    pub fn has_tool(&self, tool_name: &str) -> bool {
        self.tools.iter().any(|t| t.enabled && t.name == tool_name)
    }
}

/// Export servers as UI-friendly JSON (matching Kelivo format).
///
/// Shape: { "mcpServers": { "serverId": { name, type, description, isActive, baseUrl, headers, ... } } }
pub fn export_servers_as_ui_json(
    servers: &[McpServerConfig],
    is_desktop: bool,
) -> serde_json::Value {
    let mut map = serde_json::Map::new();
    for s in servers {
        if s.transport == McpTransportType::Stdio && !is_desktop {
            continue;
        }
        let mut entry = serde_json::Map::new();
        entry.insert(
            "name".to_string(),
            serde_json::Value::String(s.name.clone()),
        );
        entry.insert(
            "type".to_string(),
            serde_json::Value::String(
                match s.transport {
                    McpTransportType::Http => "streamableHttp",
                    McpTransportType::Sse => "sse",
                    McpTransportType::InMemory => "inmemory",
                    McpTransportType::Stdio => "stdio",
                }
                .to_string(),
            ),
        );
        entry.insert(
            "description".to_string(),
            serde_json::Value::String(String::new()),
        );
        entry.insert("isActive".to_string(), serde_json::Value::Bool(s.enabled));

        if s.transport != McpTransportType::Stdio && s.transport != McpTransportType::InMemory {
            entry.insert(
                "baseUrl".to_string(),
                serde_json::Value::String(s.url.clone()),
            );
            if !s.headers.is_empty() {
                let h: serde_json::Map<String, serde_json::Value> = s
                    .headers
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect();
                entry.insert("headers".to_string(), serde_json::Value::Object(h));
            }
        }

        if s.transport == McpTransportType::Stdio {
            if let Some(ref cmd) = s.command {
                if !cmd.is_empty() {
                    entry.insert(
                        "command".to_string(),
                        serde_json::Value::String(cmd.clone()),
                    );
                }
            }
            if !s.args.is_empty() {
                let a: Vec<serde_json::Value> = s
                    .args
                    .iter()
                    .map(|x| serde_json::Value::String(x.clone()))
                    .collect();
                entry.insert("args".to_string(), serde_json::Value::Array(a));
            }
            if !s.env.is_empty() {
                let e: serde_json::Map<String, serde_json::Value> = s
                    .env
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect();
                entry.insert("env".to_string(), serde_json::Value::Object(e));
            }
            if let Some(ref wd) = s.working_directory {
                if !wd.is_empty() {
                    entry.insert(
                        "workingDirectory".to_string(),
                        serde_json::Value::String(wd.clone()),
                    );
                }
            }
        }

        map.insert(s.id.clone(), serde_json::Value::Object(entry));
    }

    let mut root = serde_json::Map::new();
    root.insert("mcpServers".to_string(), serde_json::Value::Object(map));
    serde_json::Value::Object(root)
}
