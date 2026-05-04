use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Connection state enum.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ConnectionState {
    #[serde(rename = "disconnected")]
    Disconnected,
    #[serde(rename = "connecting")]
    Connecting,
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "reconnecting")]
    Reconnecting,
    #[serde(rename = "error")]
    Error,
}

/// Connection result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConnectionResult {
    pub success: bool,
    pub server_id: String,
    #[serde(rename = "serverName")]
    pub server_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_info: Option<Value>,
}
