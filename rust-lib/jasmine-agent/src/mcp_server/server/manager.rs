use std::collections::HashMap;

use crate::mcp_server::protocol::McpStatus;
use crate::mcp_server::server::config::{McpServerConfig, McpToolConfig};

/// Managed MCP server with runtime state.
#[derive(Clone, Debug)]
pub struct ManagedMcpServer {
    pub config: McpServerConfig,
    pub status: McpStatus,
    pub error_message: Option<String>,
}

impl ManagedMcpServer {
    pub fn new(config: McpServerConfig) -> Self {
        ManagedMcpServer {
            config,
            status: McpStatus::Idle,
            error_message: None,
        }
    }

    pub fn is_connected(&self) -> bool {
        self.status == McpStatus::Connected
    }
}

/// MCP server registry and lifecycle manager.
pub struct McpServerManager {
    servers: HashMap<String, ManagedMcpServer>,
    request_timeout_ms: u64,
}

impl McpServerManager {
    pub fn new() -> Self {
        let mut mgr = McpServerManager {
            servers: HashMap::new(),
            request_timeout_ms: 30_000, // default 30s
        };
        mgr.ensure_builtin_fetch_server();
        mgr
    }

    pub fn request_timeout_ms(&self) -> u64 {
        self.request_timeout_ms
    }

    pub fn request_timeout_seconds(&self) -> u64 {
        self.request_timeout_ms / 1000
    }

    pub fn update_request_timeout(&mut self, timeout_ms: u64) {
        if timeout_ms > 0 {
            self.request_timeout_ms = timeout_ms;
        }
    }

    // ── CRUD ──

    pub fn add_server(&mut self, config: McpServerConfig) {
        self.servers
            .insert(config.id.clone(), ManagedMcpServer::new(config));
    }

    pub fn remove_server(&mut self, id: &str) -> Option<ManagedMcpServer> {
        self.servers.remove(id)
    }

    pub fn update_server(&mut self, config: McpServerConfig) {
        if let Some(existing) = self.servers.get_mut(&config.id) {
            existing.config = config;
        } else {
            self.servers
                .insert(config.id.clone(), ManagedMcpServer::new(config));
        }
    }

    pub fn get_server(&self, id: &str) -> Option<&ManagedMcpServer> {
        self.servers.get(id)
    }

    pub fn get_server_mut(&mut self, id: &str) -> Option<&mut ManagedMcpServer> {
        self.servers.get_mut(id)
    }

    pub fn list_servers(&self) -> Vec<&ManagedMcpServer> {
        self.servers.values().collect()
    }

    pub fn server_count(&self) -> usize {
        self.servers.len()
    }

    // ── Connection state ──

    pub fn set_status(&mut self, id: &str, status: McpStatus) {
        if let Some(server) = self.servers.get_mut(id) {
            server.status = status;
        }
    }

    pub fn set_error(&mut self, id: &str, error: &str) {
        if let Some(server) = self.servers.get_mut(id) {
            server.status = McpStatus::Error;
            server.error_message = Some(error.to_string());
        }
    }

    pub fn error_for(&self, id: &str) -> Option<String> {
        self.servers.get(id)?.error_message.clone()
    }

    pub fn connected_servers(&self) -> Vec<&ManagedMcpServer> {
        self.servers.values().filter(|s| s.is_connected()).collect()
    }

    // ── Tools ──

    /// Get enabled tools from the selected server set.
    pub fn get_enabled_tools_for_servers(
        &self,
        server_ids: &std::collections::HashSet<String>,
    ) -> Vec<McpToolConfig> {
        self.servers
            .iter()
            .filter(|(id, _)| server_ids.contains(*id))
            .flat_map(|(_, s)| s.config.tools.iter())
            .filter(|t| t.enabled)
            .cloned()
            .collect()
    }

    /// List all unique tool names across all servers.
    pub fn all_tool_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self
            .servers
            .values()
            .flat_map(|s| s.config.tools.iter().map(|t| t.name.clone()))
            .collect();
        names.sort();
        names.dedup();
        names
    }

    // ── Validation ──

    // ── Tool management ──

    /// Set whether a tool is enabled for a server.
    pub fn set_tool_enabled(&mut self, server_id: &str, tool_name: &str, enabled: bool) -> bool {
        let server = match self.servers.get_mut(server_id) {
            Some(s) => s,
            None => return false,
        };
        for t in &mut server.config.tools {
            if t.name == tool_name {
                t.enabled = enabled;
                return true;
            }
        }
        false
    }

    /// Set whether a tool requires user approval.
    pub fn set_tool_needs_approval(
        &mut self,
        server_id: &str,
        tool_name: &str,
        needs_approval: bool,
    ) -> bool {
        let server = match self.servers.get_mut(server_id) {
            Some(s) => s,
            None => return false,
        };
        for t in &mut server.config.tools {
            if t.name == tool_name {
                t.needs_approval = needs_approval;
                return true;
            }
        }
        false
    }

    /// Check if a tool requires approval across all connected servers.
    pub fn tool_needs_approval(&self, tool_name: &str) -> bool {
        self.servers
            .values()
            .filter(|s| s.is_connected() && s.config.enabled)
            .flat_map(|s| &s.config.tools)
            .any(|t| t.name == tool_name && t.enabled && t.needs_approval)
    }

    /// Find the tool config for a given server and tool name.
    pub fn tool_config(&self, server_id: &str, tool_name: &str) -> Option<&McpToolConfig> {
        self.servers
            .get(server_id)?
            .config
            .tools
            .iter()
            .find(|t| t.name == tool_name)
    }

    // ── Built-in @kelivo/fetch ──

    /// Ensure the built-in @kelivo/fetch in-memory server is present.
    pub fn ensure_builtin_fetch_server(&mut self) {
        let exists = self.servers.values().any(|s| {
            s.config.transport == crate::mcp_server::protocol::McpTransportType::InMemory
                || s.config.name == "@kelivo/fetch"
                || s.config.id == "kelivo_fetch"
        });
        if exists {
            return;
        }
        let cfg = McpServerConfig {
            id: "kelivo_fetch".to_string(),
            enabled: true,
            name: "@kelivo/fetch".to_string(),
            transport: crate::mcp_server::protocol::McpTransportType::InMemory,
            url: String::new(),
            tools: vec![],
            headers: std::collections::HashMap::new(),
            command: None,
            args: vec![],
            env: std::collections::HashMap::new(),
            working_directory: None,
        };
        self.servers
            .insert("kelivo_fetch".to_string(), ManagedMcpServer::new(cfg));
    }

    // ── Reorder ──

    pub fn reorder_servers(&mut self, old_index: usize, new_index: usize) {
        if old_index == new_index {
            return;
        }
        if old_index >= self.servers.len() || new_index >= self.servers.len() {
            return;
        }
        let ids: Vec<String> = self.servers.keys().cloned().collect();
        let moved_id = ids[old_index].clone();
        // Remove and re-insert at new position
        let server = match self.servers.remove(&moved_id) {
            Some(s) => s,
            None => return,
        };
        let mut new_servers: Vec<(String, ManagedMcpServer)> = self.servers.drain().collect();
        new_servers.insert(new_index, (moved_id, server));
        self.servers = new_servers.into_iter().collect();
    }

    // ── Validate config ──

    pub fn validate_config(config: &McpServerConfig) -> Vec<String> {
        let mut errors = Vec::new();
        if config.name.trim().is_empty() {
            errors.push("name is required".to_string());
        }
        match config.transport {
            crate::mcp_server::protocol::McpTransportType::Sse
            | crate::mcp_server::protocol::McpTransportType::Http => {
                if config.url.trim().is_empty() {
                    errors.push("url is required for SSE/HTTP transport".to_string());
                }
                if !config.url.starts_with("http://") && !config.url.starts_with("https://") {
                    errors.push("url must start with http:// or https://".to_string());
                }
            }
            crate::mcp_server::protocol::McpTransportType::Stdio => {
                if config
                    .command
                    .as_ref()
                    .map_or(true, |c| c.trim().is_empty())
                {
                    errors.push("command is required for STDIO transport".to_string());
                }
            }
            crate::mcp_server::protocol::McpTransportType::InMemory => {}
        }
        errors
    }
}

impl Default for McpServerManager {
    fn default() -> Self {
        Self::new()
    }
}
