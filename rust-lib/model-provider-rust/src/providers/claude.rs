use crate::models::{ModelInfo, ProviderConfig};
use crate::utils::model_inference::infer_model_info;
use serde_json::Value;

/// Anthropic Claude provider.
///
/// Uses the Messages API with:
/// - x-api-key header (not Bearer)
/// - anthropic-version: 2023-06-01
/// - GET {baseUrl}/models returns { data: [{ id, display_name }] }
pub struct ClaudeProvider;

impl ClaudeProvider {
    pub const ANTHROPIC_VERSION: &str = "2023-06-01";

    /// List models from a Claude-compatible endpoint.
    /// GET {baseUrl}/models with x-api-key + anthropic-version headers.
    pub fn list_models(config: &ProviderConfig) -> Result<Vec<ModelInfo>, String> {
        let key = config.api_key_effective();
        let base = config.base_url_normalized();

        let url = format!("{}/models", base);
        let mut req = ureq::get(&url).set("anthropic-version", Self::ANTHROPIC_VERSION);

        if !key.is_empty() {
            req = req.set("x-api-key", key);
        }

        let resp = req.call().map_err(|e| format!("HTTP request failed: {}", e))?;
        if resp.status() < 200 || resp.status() >= 300 {
            return Err(format!(
                "HTTP {}: {}",
                resp.status(),
                resp.into_string().unwrap_or_default()
            ));
        }

        let body = resp.into_string().map_err(|e| format!("Read body: {}", e))?;
        let parsed: Value =
            serde_json::from_str(&body).map_err(|e| format!("Parse JSON: {}", e))?;

        let data = parsed["data"]
            .as_array()
            .ok_or_else(|| format!("Unexpected response format: missing 'data' array"))?;

        let models: Vec<ModelInfo> = data
            .iter()
            .filter_map(|entry| {
                let id = entry["id"].as_str()?;
                let display_name = entry["display_name"]
                    .as_str()
                    .unwrap_or(id);
                Some(infer_model_info(ModelInfo::new(
                    id.to_string(),
                    display_name.to_string(),
                )))
            })
            .collect();

        Ok(models)
    }

    /// Test connection by sending a minimal Messages API request.
    /// POST {baseUrl}/messages with a minimal messages body.
    pub fn test_connection(
        config: &ProviderConfig,
        model_id: &str,
        use_stream: bool,
    ) -> Result<(), String> {
        let key = config.api_key_effective();
        let base = config.base_url_normalized();
        let url = format!("{}/messages", base);

        let body = serde_json::json!({
            "model": model_id,
            "max_tokens": 8,
            "messages": [
                {"role": "user", "content": "hello"}
            ],
            "stream": use_stream
        });

        let body_str = serde_json::to_string(&body)
            .map_err(|e| format!("Serialize body: {}", e))?;
        let resp = ureq::post(&url)
            .set("x-api-key", key)
            .set("anthropic-version", Self::ANTHROPIC_VERSION)
            .set("Content-Type", "application/json")
            .send_string(&body_str)
            .map_err(|e| format!("HTTP request failed: {}", e))?;

        if resp.status() < 200 || resp.status() >= 300 {
            return Err(format!(
                "HTTP {}: {}",
                resp.status(),
                resp.into_string().unwrap_or_default()
            ));
        }

        // For streaming, verify content-type suggests event-stream
        if use_stream {
            let content_type = resp
                .header("content-type")
                .unwrap_or("");
            if !content_type.contains("text/event-stream") {
                let body_text = resp.into_string().unwrap_or_default();
                if body_text.is_empty() {
                    return Err("Stream response expected but not received".to_string());
                }
            }
        }

        Ok(())
    }
}
