use crate::models::{ModelInfo, ProviderConfig};
use crate::utils::model_inference::infer_model_info;
use serde_json::Value;

/// OpenAI-compatible provider.
///
/// Covers:
/// - OpenAI (api.openai.com)
/// - DeepSeek (api.deepseek.com)
/// - SiliconFlow (api.siliconflow.cn)
/// - OpenRouter (openrouter.ai)
///
/// All share the same /models listing (Bearer auth + data[].id) and
/// /chat/completions endpoint.
pub struct OpenAICompatProvider;

impl OpenAICompatProvider {
    /// List models from an OpenAI-compatible endpoint.
    /// GET {baseUrl}/models with Authorization: Bearer <key>
    pub fn list_models(config: &ProviderConfig) -> Result<Vec<ModelInfo>, String> {
        let key = config.api_key_effective();
        let base = config.base_url_normalized();

        let url = format!("{}/models", base);
        let req = ureq::get(&url);
        let req = if !key.is_empty() {
            req.set("Authorization", &format!("Bearer {}", key))
        } else {
            req
        };

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
                Some(infer_model_info(ModelInfo::new(
                    id.to_string(),
                    id.to_string(),
                )))
            })
            .collect();

        Ok(models)
    }

    /// Test connection by sending a minimal chat request.
    /// When useResponseApi is true, uses /responses endpoint with "input" format.
    /// Otherwise uses {chatPath} (default /chat/completions) with "messages" format.
    pub fn test_connection(
        config: &ProviderConfig,
        model_id: &str,
        use_stream: bool,
    ) -> Result<(), String> {
        let key = config.api_key_effective();
        let base = config.base_url_normalized();

        let (path, body) = if config.use_response_api == Some(true) {
            (
                "/responses".to_string(),
                serde_json::json!({
                    "model": model_id,
                    "input": [{"role": "user", "content": "hello"}],
                    "stream": use_stream
                }),
            )
        } else {
            (
                config.chat_path_effective().to_string(),
                serde_json::json!({
                    "model": model_id,
                    "messages": [{"role": "user", "content": "hello"}],
                    "stream": use_stream
                }),
            )
        };

        let url = format!("{}{}", base, path);

        let req = ureq::post(&url)
            .set("Content-Type", "application/json")
            .set(
                "Authorization",
                &format!("Bearer {}", key),
            );

        // OpenRouter default headers
        let req = Self::apply_openrouter_headers(config, req);

        let body_str = serde_json::to_string(&body)
            .map_err(|e| format!("Serialize body: {}", e))?;
        let resp = req
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

    fn apply_openrouter_headers(
        config: &ProviderConfig,
        req: ureq::Request,
    ) -> ureq::Request {
        let host = url_host(&config.base_url);
        if host.contains("openrouter.ai") {
            req.set("HTTP-Referer", "https://github.com/Chevey339/kelivo")
                .set("X-OpenRouter-Title", "Kelivo")
                .set("X-OpenRouter-Categories", "general-chat")
        } else {
            req
        }
    }
}

fn url_host(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
        .to_lowercase()
}
