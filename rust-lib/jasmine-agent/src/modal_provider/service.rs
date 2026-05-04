use crate::modal_provider::models::{ModelInfo, ProviderConfig, ProviderKind};
use crate::modal_provider::providers::claude::ClaudeProvider;
use crate::modal_provider::providers::openai_compat::OpenAICompatProvider;
use crate::modal_provider::utils::model_inference::infer_model_info;

// ── 模型供应商业务 API ───────────────────────────────────────────

/// 从配置中的 modelOverrides 解析上游模型 ID（apiModelId）。
/// 如果 modelOverrides 里没有为 model_id 指定 apiModelId，则返回原 model_id。
pub fn modal_provider_resolve_api_model_id(
    config_json: String,
    model_id: String,
) -> Result<String, String> {
    let config: ProviderConfig =
        serde_json::from_str(&config_json).map_err(|e| format!("Invalid config JSON: {}", e))?;
    Ok(modal_provider_resolve_api_model_id_from_config(
        &config, &model_id,
    ))
}

/// 根据 provider id 和显式类型分类供应商类型。
/// 返回 JSON 格式的 ProviderKind 枚举值（"openai" | "claude"）。
pub fn modal_provider_classify_provider_kind(
    provider_id: String,
    explicit_type: Option<String>,
) -> String {
    let kind = ProviderKind::classify(&provider_id, explicit_type.as_deref());
    serde_json::to_string(&kind).unwrap_or_else(|_| r#""openai""#.to_string())
}

/// 获取已知供应商的默认 Base URL（仅基于 provider id 名推断）。
/// 这部分是硬编码的预设值，和 Kelivo 原实现一致。
pub fn modal_provider_default_base_url(provider_id: String) -> String {
    modal_provider_default_base_url_for(&provider_id).to_string()
}

/// 获取供应商的默认 HTTP 请求头（如 OpenRouter 的标识头）。
/// 返回 JSON map。
pub fn modal_provider_get_provider_default_headers(config_json: String) -> Result<String, String> {
    let config: ProviderConfig =
        serde_json::from_str(&config_json).map_err(|e| format!("Invalid config JSON: {}", e))?;
    let headers = modal_provider_default_provider_headers(&config);
    serde_json::to_string(&headers).map_err(|e| format!("Serialize headers: {}", e))
}

/// 从 API 获取模型列表。
/// 向 `{baseUrl}/models` 发送 GET 请求，解析并推断每个模型的能力标签。
/// 返回 JSON 数组的 ModelInfo。
///
/// SiliconFlow 无 key 时限制为 2 个免费模型（硬编码白名单，与 Kelivo 一致）。
pub fn modal_provider_list_models(
    config_json: String,
    siliconflow_fallback_key: Option<String>,
) -> Result<String, String> {
    let config: ProviderConfig =
        serde_json::from_str(&config_json).map_err(|e| format!("Invalid config JSON: {}", e))?;

    // SiliconFlow 无 key → 只返回 2 个免费模型
    if config.id == "SiliconFlow"
        && config.api_key.trim().is_empty()
        && is_siliconflow_host(&config)
    {
        let fallback = siliconflow_fallback_key
            .map(|k| k.trim().to_string())
            .unwrap_or_default();
        if fallback.is_empty() {
            // 无 key 也无 fallback：返回硬编码的 2 个免费模型（经 infer 补齐能力标签）
            let free_models = vec![
                infer_model_info(ModelInfo::new(
                    "THUDM/GLM-4-9B-0414".to_string(),
                    "THUDM/GLM-4-9B-0414".to_string(),
                )),
                infer_model_info(ModelInfo::new(
                    "Qwen/Qwen3-8B".to_string(),
                    "Qwen/Qwen3-8B".to_string(),
                )),
            ];
            return serde_json::to_string(&free_models)
                .map_err(|e| format!("Serialize models: {}", e));
        }
    }

    let models = match config.effective_provider_kind() {
        ProviderKind::OpenAI => OpenAICompatProvider::list_models(&config)?,
        ProviderKind::Claude => ClaudeProvider::list_models(&config)?,
    };

    serde_json::to_string(&models).map_err(|e| format!("Serialize models: {}", e))
}

/// 测试供应商连接。
/// 向对应 API 端点发送最小聊天请求，验证凭证和模型是否可达。
pub fn modal_provider_test_connection(
    config_json: String,
    model_id: String,
    use_stream: bool,
    siliconflow_fallback_key: Option<String>,
) -> Result<(), String> {
    let mut config: ProviderConfig =
        serde_json::from_str(&config_json).map_err(|e| format!("Invalid config JSON: {}", e))?;

    // 解析 apiModelId（如果 modelOverrides 中指定了上游模型 ID）
    let upstream_id = modal_provider_resolve_api_model_id_from_config(&config, &model_id);

    // SiliconFlow 免费模型 fallback key
    if config.id == "SiliconFlow"
        && config.api_key.trim().is_empty()
        && is_siliconflow_host(&config)
    {
        if let Some(ref fallback) = siliconflow_fallback_key {
            let m = upstream_id.to_lowercase();
            let allowed = m == "thudm/glm-4-9b-0414" || m == "qwen/qwen3-8b";
            if allowed && !fallback.trim().is_empty() {
                config.api_key = fallback.trim().to_string();
            }
        }
    }

    match config.effective_provider_kind() {
        ProviderKind::OpenAI => {
            OpenAICompatProvider::test_connection(&config, &upstream_id, use_stream)
        }
        ProviderKind::Claude => ClaudeProvider::test_connection(&config, &upstream_id, use_stream),
    }
}

/// 校验 ProviderConfig 配置合法性（无网络调用）。
/// 返回 Ok(()) 或错误信息字符串。
pub fn modal_provider_validate_provider_config(config_json: String) -> Result<(), String> {
    let config: ProviderConfig =
        serde_json::from_str(&config_json).map_err(|e| format!("Invalid config JSON: {}", e))?;

    let mut errors: Vec<String> = Vec::new();

    if config.name.trim().is_empty() {
        errors.push("name is required".to_string());
    }
    if config.base_url.trim().is_empty() {
        errors.push("baseUrl is required".to_string());
    }
    if !config.base_url.starts_with("http://") && !config.base_url.starts_with("https://") {
        errors.push("baseUrl must start with http:// or https://".to_string());
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors.join("; "))
    }
}

/// 为指定 provider id 创建默认 ProviderConfig（JSON 模板）。
/// 对齐 Kelivo 的 ProviderConfig.defaultsFor()。
/// 返回 JSON 格式的 ProviderConfig，供 Flutter 侧初始化供应商用。
pub fn modal_provider_create_default_config(
    provider_id: String,
    display_name: Option<String>,
) -> Result<String, String> {
    let name = display_name.unwrap_or_else(|| provider_id.clone());
    let kind = ProviderKind::classify(&provider_id, None);
    let base_url = modal_provider_default_base_url_for(&provider_id).to_string();
    let mut enabled = false;
    let lower = provider_id.to_lowercase();
    if lower.contains("openai") || lower.contains("silicon") || lower.contains("openrouter") {
        enabled = true;
    }

    let models: Vec<String> = vec![];
    let model_overrides = serde_json::Map::new();

    let config = match kind {
        ProviderKind::Claude => ProviderConfig {
            id: provider_id,
            enabled,
            name,
            api_key: String::new(),
            base_url,
            provider_type: Some(ProviderKind::Claude),
            chat_path: None,
            use_response_api: None,
            models,
            model_overrides,
        },
        ProviderKind::OpenAI => ProviderConfig {
            id: provider_id,
            enabled,
            name,
            api_key: String::new(),
            base_url,
            provider_type: Some(ProviderKind::OpenAI),
            chat_path: Some("/chat/completions".to_string()),
            use_response_api: Some(false),
            models,
            model_overrides,
        },
    };

    serde_json::to_string(&config).map_err(|e| format!("Serialize config: {}", e))
}

// ── 内部辅助函数 ──────────────────────────────────────────────────

/// 从 modelOverrides 中解析 apiModelId（上游 / vendor model id）。
/// 对齐 Kelivo 的 _apiModelId / resolveApiModelIdOverride 逻辑。
fn modal_provider_resolve_api_model_id_from_config(
    config: &ProviderConfig,
    model_id: &str,
) -> String {
    // modelOverrides 是一个 Map<String, serde_json::Value>
    if let Some(override_map) = config.model_overrides.get(model_id) {
        // 先查 apiModelId，再查 api_model_id（兼容两种命名）
        if let Some(v) = override_map.get("apiModelId") {
            if let Some(s) = v.as_str() {
                let trimmed = s.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
        if let Some(v) = override_map.get("api_model_id") {
            if let Some(s) = v.as_str() {
                let trimmed = s.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
    }
    model_id.to_string()
}

fn is_siliconflow_host(config: &ProviderConfig) -> bool {
    let host = url_host(&config.base_url);
    host.contains("siliconflow")
}

fn url_host(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
        .to_lowercase()
}

/// 已知供应商的预设 Base URL（硬编码，和 Kelivo _defaultBase 一致）。
fn modal_provider_default_base_url_for(key: &str) -> &'static str {
    let k = key.to_lowercase();
    if k.contains("openrouter") {
        "https://openrouter.ai/api/v1"
    } else if k.contains("silicon") {
        "https://api.siliconflow.cn/v1"
    } else if k.contains("deepseek") {
        "https://api.deepseek.com/v1"
    } else if k.contains("claude") || k.contains("anthropic") {
        "https://api.anthropic.com/v1"
    } else if k.contains("openai") {
        "https://api.openai.com/v1"
    } else if k.contains("aihubmix") {
        "https://aihubmix.com/v1"
    } else if k.contains("tensdaq") {
        "https://tensdaq-api.x-aio.com/v1"
    } else if k.contains("kelivoin") {
        "https://text.pollinations.ai/openai"
    } else if regex_find(&k, r"qwen|aliyun|dashscope") {
        "https://dashscope.aliyuncs.com/compatible-mode/v1"
    } else if regex_find(&k, r"bytedance|doubao|volces|ark") {
        "https://ark.cn-beijing.volces.com/api/v3"
    } else if k.contains("grok") || k.contains("x.ai") || k.contains("xai") {
        "https://api.x.ai/v1"
    } else if regex_find(&k, r"zhipu|智谱|glm") {
        "https://open.bigmodel.cn/api/paas/v4"
    } else {
        "https://api.openai.com/v1"
    }
}

fn regex_find(text: &str, pattern: &str) -> bool {
    regex::Regex::new(pattern)
        .map(|re| re.is_match(text))
        .unwrap_or(false)
}

/// OpenRouter 等供应商需要的默认请求头。
fn modal_provider_default_provider_headers(
    config: &ProviderConfig,
) -> serde_json::Map<String, serde_json::Value> {
    let mut headers = serde_json::Map::new();
    let host = url_host(&config.base_url);
    if host.contains("openrouter.ai") {
        headers.insert(
            "HTTP-Referer".to_string(),
            serde_json::Value::String("https://github.com/Chevey339/kelivo".to_string()),
        );
        headers.insert(
            "X-OpenRouter-Title".to_string(),
            serde_json::Value::String("Kelivo".to_string()),
        );
        headers.insert(
            "X-OpenRouter-Categories".to_string(),
            serde_json::Value::String("general-chat".to_string()),
        );
    }
    headers
}
