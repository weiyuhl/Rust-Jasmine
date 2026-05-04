use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Root definition for filesystem access.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Root {
    pub uri: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Completion request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionRequest {
    #[serde(rename = "ref")]
    pub ref_field: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub argument: Option<Value>,
}

/// Completion result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompletionResult {
    pub completion: Value,
}

/// Log message notification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LogMessageNotification {
    pub level: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// Cancel request notification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CancelRequestNotification {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Progress notification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressNotification {
    #[serde(rename = "requestId")]
    pub request_id: Value,
    pub progress: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f64>,
}

/// Pending operation for cancellation support.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PendingOperation {
    pub id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "type")]
    pub op_type: String,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(rename = "isCancelled", default)]
    pub is_cancelled: bool,
}

/// Progress update for long-running operations.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProgressUpdate {
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub progress: f64,
    pub message: String,
}
