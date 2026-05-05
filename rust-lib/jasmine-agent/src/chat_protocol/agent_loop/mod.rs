pub mod openai_loop;
pub mod claude_loop;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Event emitted by the agent loop to Dart.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum AgentEvent {
    /// Text content delta from the model.
    Content {
        text: String,
    },
    /// Reasoning/thinking delta.
    Reasoning {
        text: String,
    },
    /// Image output from the model.
    Image {
        url: String,
    },
    /// Tool calls requested by the model (batch).
    ToolCalls {
        calls: Vec<ToolCallInfo>,
    },
    /// Results after tool execution.
    ToolResults {
        results: Vec<ToolResultInfo>,
    },
    /// Token usage update.
    Usage {
        prompt_tokens: i32,
        completion_tokens: i32,
        total_tokens: i32,
    },
    /// Stream finished (no more tool calls).
    Done,
    /// Error occurred.
    Error {
        message: String,
    },
}

/// A single tool call.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolCallInfo {
    pub id: String,
    pub name: String,
    pub arguments: Value,
}

/// A tool result.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ToolResultInfo {
    pub id: String,
    pub name: String,
    pub arguments: Value,
    pub content: String,
}

/// Accumulated tool calls during streaming.
#[derive(Clone, Debug, Default)]
pub struct ToolAccumulator {
    entries: Vec<ToolCallAccEntry>,
}

#[derive(Clone, Debug, Default)]
struct ToolCallAccEntry {
    index: i32,
    id: String,
    name: String,
    args_buf: String,
}

impl ToolAccumulator {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn push(&mut self, index: i32, id: Option<&str>, name: Option<&str>, args_fragment: Option<&str>) {
        if let Some(entry) = self.entries.iter_mut().find(|e| e.index == index) {
            if let Some(id) = id { entry.id = id.to_string(); }
            if let Some(name) = name { entry.name = name.to_string(); }
            if let Some(frag) = args_fragment { entry.args_buf.push_str(frag); }
        } else {
            self.entries.push(ToolCallAccEntry {
                index,
                id: id.unwrap_or("").to_string(),
                name: name.unwrap_or("").to_string(),
                args_buf: args_fragment.unwrap_or("").to_string(),
            });
        }
    }

    pub fn finalize(&self) -> Vec<ToolCallInfo> {
        self.entries
            .iter()
            .filter(|e| !e.name.is_empty())
            .map(|e| ToolCallInfo {
                id: if e.id.is_empty() {
                    format!("call_{}", e.index)
                } else {
                    e.id.clone()
                },
                name: e.name.clone(),
                arguments: serde_json::from_str(&e.args_buf)
                    .unwrap_or(Value::Object(serde_json::Map::new())),
            })
            .collect()
    }

    pub fn has_calls(&self) -> bool {
        !self.entries.is_empty()
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}
