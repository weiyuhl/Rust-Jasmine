use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::content::Content;

/// Tool definition (2025-03-26 compliant).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "inputSchema")]
    pub input_schema: Value,
    #[serde(rename = "supportsProgress", skip_serializing_if = "Option::is_none")]
    pub supports_progress: Option<bool>,
    #[serde(rename = "supportsCancellation", skip_serializing_if = "Option::is_none")]
    pub supports_cancellation: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Tool call request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallToolRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Value>,
}

/// Tool call result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CallToolResult {
    pub content: Vec<Content>,
    #[serde(rename = "isStreaming", default)]
    pub is_streaming: bool,
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
}

/// List tools result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListToolsResult {
    pub tools: Vec<Tool>,
}

/// Tools list changed notification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolsListChangedNotification {}
