use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Base content type for MCP (2025-03-26 compliant).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Content {
    #[serde(rename = "text")]
    Text(TextContent),
    #[serde(rename = "image")]
    Image(ImageContent),
    #[serde(rename = "resource")]
    Resource(ResourceContent),
}

impl Content {
    pub fn text(text: String) -> Self {
        Content::Text(TextContent {
            text,
            annotations: None,
        })
    }
}

/// Text content representation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextContent {
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Value>,
}

/// Image content representation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ImageContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Value>,
}

/// Resource content representation.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResourceContent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blob: Option<String>,
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Value>,
    /// Nested resource object (2025 format).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<Box<ResourceContent>>,
}
