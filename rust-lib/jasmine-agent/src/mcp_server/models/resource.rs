use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Resource definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Resource {
    pub uri: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Resource template definition.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceTemplate {
    #[serde(rename = "uriTemplate")]
    pub uri_template: String,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// Resource content info.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceContentInfo {
    pub uri: String,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
}

/// Read resource result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadResourceResult {
    pub contents: Vec<ResourceContentInfo>,
}

/// Read resource request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReadResourceRequest {
    pub uri: String,
}

/// List resources result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListResourcesResult {
    pub resources: Vec<Resource>,
}

/// Resources list changed notification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourcesListChangedNotification {}

/// Resource updated notification.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceUpdatedNotification {
    pub uri: String,
}

/// Cached resource for performance.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CachedResource {
    pub uri: String,
    pub content: ReadResourceResult,
    #[serde(rename = "cachedAt")]
    pub cached_at: String,
    #[serde(rename = "maxAgeSeconds")]
    pub max_age_seconds: u64,
}
