use serde_json::Value;

use crate::chat_protocol::ToolCallInfo;

/// A single delta from a streaming tool call.
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ToolCallDelta {
    pub index: i32,
    pub id: Option<String>,
    pub name: Option<String>,
    pub args_fragment: Option<String>,
}

/// Aggregates streaming tool call deltas into complete ToolCallInfo objects.
#[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct ToolCallAggregator {
    pending: Vec<PendingCall>,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct PendingCall {
    index: i32,
    id: String,
    name: String,
    args_buf: String,
}

impl ToolCallAggregator {
    pub fn new() -> Self {
        ToolCallAggregator { pending: Vec::new() }
    }

    /// Push a delta for a tool call. Updates or creates the pending entry.
    pub fn push(&mut self, delta: &ToolCallDelta) {
        let entry = self
            .pending
            .iter_mut()
            .find(|p| p.index == delta.index);

        if let Some(entry) = entry {
            if let Some(ref id) = delta.id {
                entry.id = id.clone();
            }
            if let Some(ref name) = delta.name {
                entry.name = name.clone();
            }
            if let Some(ref fragment) = delta.args_fragment {
                entry.args_buf.push_str(fragment);
            }
        } else {
            self.pending.push(PendingCall {
                index: delta.index,
                id: delta.id.clone().unwrap_or_default(),
                name: delta.name.clone().unwrap_or_default(),
                args_buf: delta.args_fragment.clone().unwrap_or_default(),
            });
        }
    }

    /// Push multiple deltas at once.
    pub fn push_all(&mut self, deltas: &[ToolCallDelta]) {
        for d in deltas {
            self.push(d);
        }
    }

    /// Returns all currently complete tool calls (those with a name and id).
    pub fn current_calls(&self) -> Vec<ToolCallInfo> {
        self.pending
            .iter()
            .filter(|p| !p.id.is_empty() && !p.name.is_empty())
            .map(|p| {
                let args: Value = serde_json::from_str(&p.args_buf)
                    .unwrap_or_else(|_| Value::Object(serde_json::Map::new()));
                ToolCallInfo {
                    id: p.id.clone(),
                    name: p.name.clone(),
                    arguments: args,
                    metadata: None,
                }
            })
            .collect()
    }

    /// Check if any pending calls have incomplete data.
    pub fn has_incomplete(&self) -> bool {
        self.pending.iter().any(|p| p.name.is_empty())
    }

    /// Clear all pending aggregations.
    pub fn reset(&mut self) {
        self.pending.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_single_tool_call() {
        let mut agg = ToolCallAggregator::new();
        agg.push(&ToolCallDelta {
            index: 0,
            id: Some("call_1".to_string()),
            name: Some("search".to_string()),
            args_fragment: Some(r#"{"query":"hello"}"#.to_string()),
        });
        let calls = agg.current_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].name, "search");
    }

    #[test]
    fn test_aggregate_fragmented_args() {
        let mut agg = ToolCallAggregator::new();
        agg.push(&ToolCallDelta {
            index: 0,
            id: Some("call_1".into()),
            name: Some("calc".into()),
            args_fragment: Some(r#"{"a":"#.into()),
        });
        agg.push(&ToolCallDelta {
            index: 0,
            id: None,
            name: None,
            args_fragment: Some("1}".into()),
        });
        let calls = agg.current_calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].arguments["a"], 1);
    }
}
