use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::BufRead;
use std::time::Duration;

use crate::chat_protocol::parse::sse::{parse_sse_line, SseEvent};

/// MCP transport type.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum McpTransportType {
    #[serde(rename = "sse")]
    Sse,
    #[serde(rename = "http")]
    StreamableHttp,
    #[serde(rename = "stdio")]
    Stdio,
}

/// Transport configuration for connecting to an MCP server.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpTransportConfig {
    pub transport_type: McpTransportType,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub timeout_ms: u64,
}

/// Result of reading one SSE event from a transport.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransportReadResult {
    pub event: Option<SseEvent>,
    pub error: Option<String>,
}

/// Send a JSON-RPC message via HTTP POST to the MCP server.
pub fn send_http_message(
    config: &McpTransportConfig,
    message: &str,
) -> Result<String, String> {
    let url = config.url.trim_end_matches('/');

    let mut request = ureq::post(url)
        .set("Content-Type", "application/json")
        .set("Accept", "application/json, text/event-stream")
        .timeout(Duration::from_millis(config.timeout_ms));

    for (k, v) in &config.headers {
        request = request.set(k, v);
    }

    let response = request
        .send_string(message)
        .map_err(|e| format!("HTTP send error: {}", e))?;

    let status = response.status();
    if status < 200 || status >= 300 {
        return Err(format!("HTTP {}: {}", status, response.into_string().unwrap_or_default()));
    }

    response
        .into_string()
        .map_err(|e| format!("Read body error: {}", e))
}

/// Read an SSE stream from an HTTP response body.
/// Returns parsed SSE events from the response.
pub fn read_sse_stream(body: &str) -> Vec<SseEvent> {
    let mut events = Vec::new();
    let mut buffer = String::new();

    for line in body.lines() {
        let line = line.trim_end_matches('\r');
        if line.is_empty() && !buffer.is_empty() {
            if let Some(event) = parse_sse_line(&buffer) {
                events.push(event);
            }
            buffer.clear();
        } else {
            buffer.push_str(line);
            buffer.push('\n');
        }
    }

    // Process remaining buffer
    if !buffer.is_empty() {
        if let Some(event) = parse_sse_line(buffer.trim()) {
            events.push(event);
        }
    }

    events
}

/// Connect via SSE transport and read the endpoint URL from the initial SSE response.
pub fn connect_sse(config: &McpTransportConfig) -> Result<String, String> {
    let url = config.url.trim_end_matches('/');

    let mut request = ureq::get(url)
        .set("Accept", "text/event-stream")
        .set("Cache-Control", "no-cache")
        .timeout(Duration::from_millis(config.timeout_ms));

    for (k, v) in &config.headers {
        request = request.set(k, v);
    }

    let response = request
        .call()
        .map_err(|e| format!("SSE connect error: {}", e))?;

    let status = response.status();
    if status < 200 || status >= 300 {
        return Err(format!("SSE HTTP {}: {}", status, response.into_string().unwrap_or_default()));
    }

    // Read the first few lines to get the endpoint
    let body = response
        .into_string()
        .map_err(|e| format!("SSE read error: {}", e))?;

    // Look for endpoint event in SSE stream
    for line in body.lines() {
        let line = line.trim();
        if line.starts_with("data:") {
            let data = line[5..].trim();
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(data) {
                if let Some(endpoint) = json.get("endpoint").and_then(|v| v.as_str()) {
                    return Ok(endpoint.to_string());
                }
            }
        }
    }

    // If no endpoint found, use the base URL
    Ok(url.to_string())
}

/// Send a JSON-RPC message via SSE and read the response.
pub fn send_and_read_sse(
    config: &McpTransportConfig,
    message: &str,
) -> Result<Vec<SseEvent>, String> {
    let url = config.url.trim_end_matches('/');

    let mut request = ureq::post(url)
        .set("Content-Type", "application/json")
        .set("Accept", "text/event-stream")
        .timeout(Duration::from_millis(config.timeout_ms));

    for (k, v) in &config.headers {
        request = request.set(k, v);
    }

    let response = request
        .send_string(message)
        .map_err(|e| format!("SSE send error: {}", e))?;

    let status = response.status();
    if status < 200 || status >= 300 {
        return Err(format!("SSE HTTP {}: {}", status, response.into_string().unwrap_or_default()));
    }

    let content_type = response
        .header("content-type")
        .unwrap_or_default()
        .to_lowercase();

    let body = response
        .into_string()
        .map_err(|e| format!("SSE read error: {}", e))?;

    if content_type.contains("text/event-stream") {
        Ok(read_sse_stream(&body))
    } else {
        // JSON response
        Ok(vec![SseEvent {
            data: body,
            is_done: false,
        }])
    }
}

/// Build SSE event stream body for post-response handling.
pub fn build_sse_events(body: &str) -> Vec<serde_json::Value> {
    let mut events = Vec::new();
    for event in read_sse_stream(body) {
        if event.is_done {
            continue;
        }
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&event.data) {
            events.push(json);
        }
    }
    events
}
