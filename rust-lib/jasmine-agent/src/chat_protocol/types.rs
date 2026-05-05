use serde::{Deserialize, Serialize};
use serde_json::Value;

/// SSE stream output chunk — mirrors Kelivo's ChatStreamChunk.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatStreamChunk {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<String>,
    #[serde(rename = "isDone")]
    pub is_done: bool,
    #[serde(rename = "totalTokens")]
    pub total_tokens: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<TokenUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<ToolCallInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_results: Option<Vec<ToolResultInfo>>,
}

impl ChatStreamChunk {
    pub fn content_only(text: &str) -> Self {
        ChatStreamChunk {
            content: text.to_string(),
            reasoning: None,
            is_done: false,
            total_tokens: 0,
            usage: None,
            tool_calls: None,
            tool_results: None,
        }
    }
}

/// Complete tool call info — mirrors Kelivo's ToolCallInfo.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallInfo {
    pub id: String,
    pub name: String,
    pub arguments: Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Tool call result info — mirrors Kelivo's ToolResultInfo.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultInfo {
    pub id: String,
    pub name: String,
    pub arguments: Value,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
}

/// Token usage with merge logic — mirrors Kelivo's TokenUsage.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TokenUsage {
    #[serde(rename = "promptTokens", default)]
    pub prompt_tokens: i32,
    #[serde(rename = "completionTokens", default)]
    pub completion_tokens: i32,
    #[serde(rename = "cachedTokens", default)]
    pub cached_tokens: i32,
    #[serde(rename = "totalTokens", default)]
    pub total_tokens: i32,
}

impl TokenUsage {
    /// Merge another TokenUsage into this one (takes max of each field).
    pub fn merge(&mut self, other: &TokenUsage) {
        if other.prompt_tokens > 0 {
            self.prompt_tokens = other.prompt_tokens;
        }
        if other.completion_tokens > 0 {
            self.completion_tokens = other.completion_tokens;
        }
        if other.cached_tokens > 0 {
            self.cached_tokens = other.cached_tokens;
        }
        let split = self.prompt_tokens + self.completion_tokens;
        if other.total_tokens > 0 {
            self.total_tokens = other.total_tokens;
        }
        if split > 0 && split > self.total_tokens {
            self.total_tokens = split;
        }
    }
}
