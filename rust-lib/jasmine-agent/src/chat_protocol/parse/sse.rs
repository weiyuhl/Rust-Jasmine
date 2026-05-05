use serde_json::Value;

/// A parsed SSE event extracted from a single line.
#[derive(Clone, Debug)]
pub struct SseEvent {
    /// The raw payload after the `data:` prefix.
    pub data: String,
    /// Whether this event signals stream completion (`[DONE]`).
    pub is_done: bool,
}

/// Parse a single SSE line. Returns `None` for empty lines or comments.
///
/// Supports:
/// - `data: {...}` → SseEvent with JSON payload
/// - `data: [DONE]` → SseEvent with is_done = true
/// - `event: ping` / `: heartbeat` → ignored (returns None)
pub fn parse_sse_line(line: &str) -> Option<SseEvent> {
    let line = line.trim();
    if line.is_empty() {
        return None;
    }

    // Skip comments and event-type lines without data
    if line.starts_with(':') {
        return None;
    }

    // Handle `data:` prefix
    if !line.starts_with("data:") {
        return None;
    }

    let data = line[5..].trim_start();

    // Check for [DONE] sentinel
    if data == "[DONE]" {
        return Some(SseEvent {
            data: String::new(),
            is_done: true,
        });
    }

    // Skip empty data
    if data.is_empty() {
        return None;
    }

    Some(SseEvent {
        data: data.to_string(),
        is_done: false,
    })
}

/// Detect the `[DONE]` sentinel in a raw line.
pub fn is_sse_done(line: &str) -> bool {
    let trimmed = line.trim();
    trimmed.starts_with("data:") && trimmed[5..].trim() == "[DONE]"
}

/// Extract and parse the JSON payload from a `data:` line.
pub fn extract_sse_json(line: &str) -> Result<Value, String> {
    let trimmed = line.trim();
    if !trimmed.starts_with("data:") {
        return Err("Not a data line".to_string());
    }
    let payload = trimmed[5..].trim_start();
    if payload.is_empty() || payload == "[DONE]" {
        return Err("Empty or DONE".to_string());
    }
    serde_json::from_str(payload).map_err(|e| format!("JSON parse error: {}", e))
}

/// Process a buffer of SSE text, splitting into lines and handling
/// buffered partial lines (returns remaining buffer after last newline).
pub fn process_sse_buffer(
    buffer: &mut String,
    text: &str,
) -> Vec<SseLine> {
    buffer.push_str(text);
    let mut results = Vec::new();
    while let Some(nl_pos) = buffer.find('\n') {
        let line = buffer[..nl_pos].trim_end_matches('\r').to_string();
        *buffer = buffer[nl_pos + 1..].to_string();
        if !line.is_empty() {
            if let Some(event) = parse_sse_line(&line) {
                results.push(SseLine::Event(event));
            }
        }
    }
    results
}

/// Result of processing an SSE buffer.
#[derive(Clone, Debug)]
pub enum SseLine {
    Event(SseEvent),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_data_line() {
        let event = parse_sse_line("data: {\"foo\":1}").unwrap();
        assert!(!event.is_done);
        assert_eq!(event.data, "{\"foo\":1}");
    }

    #[test]
    fn test_parse_done() {
        let event = parse_sse_line("data: [DONE]").unwrap();
        assert!(event.is_done);
    }

    #[test]
    fn test_skip_empty_and_comment() {
        assert!(parse_sse_line("").is_none());
        assert!(parse_sse_line(": heartbeat").is_none());
        assert!(parse_sse_line("event: ping").is_none());
    }

    #[test]
    fn test_buffer_processing() {
        let mut buf = String::new();
        let results = process_sse_buffer(
            &mut buf,
            "data: {\"a\":1}\ndata: [DONE]\n",
        );
        assert_eq!(results.len(), 2);
    }
}
