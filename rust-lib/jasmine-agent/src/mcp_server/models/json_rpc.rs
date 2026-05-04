use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC message (2025-03-26 compliant).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonRpcMessage {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Value>,
}

impl JsonRpcMessage {
    pub fn is_notification(&self) -> bool {
        self.id.is_none() && self.method.is_some()
    }

    pub fn is_request(&self) -> bool {
        self.id.is_some() && self.method.is_some()
    }

    pub fn is_response(&self) -> bool {
        self.id.is_some() && (self.result.is_some() || self.error.is_some())
    }

    /// Create a new request.
    pub fn request(id: Value, method: String, params: Option<Value>) -> Self {
        JsonRpcMessage {
            jsonrpc: super::super::protocol::JSON_RPC_VERSION.to_string(),
            id: Some(id),
            method: Some(method),
            params,
            result: None,
            error: None,
        }
    }

    /// Create a success response.
    pub fn success(id: Value, result: Value) -> Self {
        JsonRpcMessage {
            jsonrpc: super::super::protocol::JSON_RPC_VERSION.to_string(),
            id: Some(id),
            method: None,
            params: None,
            result: Some(result),
            error: None,
        }
    }

    /// Create an error response.
    pub fn error_response(id: Value, code: i32, message: &str) -> Self {
        let err = serde_json::json!({"code": code, "message": message});
        JsonRpcMessage {
            jsonrpc: super::super::protocol::JSON_RPC_VERSION.to_string(),
            id: Some(id),
            method: None,
            params: None,
            result: None,
            error: Some(err),
        }
    }

    /// Create a notification (no id).
    pub fn notification(method: String, params: Option<Value>) -> Self {
        JsonRpcMessage {
            jsonrpc: super::super::protocol::JSON_RPC_VERSION.to_string(),
            id: None,
            method: Some(method),
            params,
            result: None,
            error: None,
        }
    }
}
