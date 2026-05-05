/// Vendor classification for OpenAI-compatible providers.
/// Determines special body adjustments needed per vendor.
#[derive(Clone, Debug, PartialEq)]
pub enum OpenAIVendor {
    Standard,
    Azure,
    DeepSeek,
    SiliconFlow,
    Mimo,
    LongCat,
    Kimi,
    OpenRouter,
    Grok,
    Zhipu,
    DashScope,
    ByteDance,
    XinLiu,
}

/// Classify the vendor based on provider config.
pub fn classify_vendor(provider_id: &str, base_url: &str, model_id: &str) -> OpenAIVendor {
    let host = base_url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
        .to_lowercase();
    let id_lower = provider_id.to_lowercase();
    let model_lower = model_id.to_lowercase();

    if host.contains("openai.azure.com") {
        return OpenAIVendor::Azure;
    }
    if host.contains("xiaomimimo") || model_lower.starts_with("mimo-") || model_lower.contains("/mimo-") {
        return OpenAIVendor::Mimo;
    }
    if host.contains("deepseek") || model_lower.contains("deepseek") {
        return OpenAIVendor::DeepSeek;
    }
    if host.contains("siliconflow") || id_lower.contains("siliconflow") {
        return OpenAIVendor::SiliconFlow;
    }
    if host.contains("longcat") {
        return OpenAIVendor::LongCat;
    }
    if host.contains("moonshot") || model_lower.contains("kimi") {
        return OpenAIVendor::Kimi;
    }
    if host.contains("openrouter.ai") {
        return OpenAIVendor::OpenRouter;
    }
    if model_lower.contains("grok") || host.contains("x.ai") || id_lower.contains("grok") {
        return OpenAIVendor::Grok;
    }
    if host.contains("bigmodel") || id_lower.contains("zhipu") || model_lower.contains("glm") {
        return OpenAIVendor::Zhipu;
    }
    if host.contains("dashscope") || id_lower.contains("aliyun") || model_lower.contains("qwen") {
        return OpenAIVendor::DashScope;
    }
    if host.contains("volces") || host.contains("ark") || id_lower.contains("bytedance") || model_lower.contains("doubao") {
        return OpenAIVendor::ByteDance;
    }
    if host.contains("iflow.cn") || host.contains("xinliu") {
        return OpenAIVendor::XinLiu;
    }
    OpenAIVendor::Standard
}

/// Get the completion tokens key name for the vendor (Azure/Mimo use max_completion_tokens).
pub fn completion_tokens_key(vendor: &OpenAIVendor) -> &'static str {
    match vendor {
        OpenAIVendor::Azure | OpenAIVendor::Mimo => "max_completion_tokens",
        _ => "max_tokens",
    }
}

/// Whether the vendor needs reasoning_content echoed in tool-call rounds.
pub fn needs_reasoning_echo(vendor: &OpenAIVendor) -> bool {
    matches!(vendor, OpenAIVendor::DeepSeek | OpenAIVendor::Mimo | OpenAIVendor::Kimi)
}

/// Whether OpenRouter should preserve reasoning_details across tool-calling turns.
pub fn preserve_reasoning_details(vendor: &OpenAIVendor, is_reasoning: bool) -> bool {
    *vendor == OpenAIVendor::OpenRouter && is_reasoning
}

/// Check if the Kimi K2.5 model needs special thinking body format.
pub fn is_kimi_k25_model(model_id: &str) -> bool {
    model_id.to_lowercase().contains("kimi-k2.5")
}

/// Check if a model is a Kimi thinking variant.
pub fn is_kimi_thinking_model(model_id: &str) -> bool {
    let lower = model_id.to_lowercase();
    lower.contains("kimi-k2-thinking") || lower.contains("kimi-k2.5")
}

/// Apply Kimi-specific body normalization.
pub fn normalize_kimi_body(body: &mut serde_json::Value, model_id: &str, is_reasoning: bool, thinking_budget: Option<i32>) {
    if !is_kimi_thinking_model(model_id) {
        return;
    }
    if let Some(obj) = body.as_object_mut() {
        obj.remove("reasoning_effort");
        if !is_reasoning {
            obj.remove("thinking");
            return;
        }
        if is_kimi_k25_model(model_id) {
            let disabled = thinking_budget.map(|b| b < 1024 && b != -1).unwrap_or(false);
            obj.insert(
                "thinking".to_string(),
                serde_json::json!({"type": if disabled { "disabled" } else { "enabled" }}),
            );
            obj.remove("temperature");
            obj.remove("top_p");
            obj.remove("n");
            obj.remove("presence_penalty");
            obj.remove("frequency_penalty");
        } else {
            obj.remove("thinking");
        }
    }
}

/// Apply Zhipu BigModel / Xiaomi MiMo thinking format.
pub fn apply_zhipu_mimo_thinking(body: &mut serde_json::Value, is_reasoning: bool, thinking_budget: Option<i32>) {
    if let Some(obj) = body.as_object_mut() {
        obj.remove("reasoning_effort");
        if is_reasoning {
            let off = thinking_budget.map(|b| b < 1024 && b != -1).unwrap_or(false);
            obj.insert(
                "thinking".to_string(),
                serde_json::json!({"type": if off { "disabled" } else { "enabled" }}),
            );
        } else {
            obj.remove("thinking");
        }
    }
}

/// Sanitize GPT-5 sampling parameters (temperature, top_p, logprobs) based on reasoning effort.
pub fn sanitize_gpt5_sampling(body: &mut serde_json::Value, model_id: &str, effort: &str) {
    let lower = model_id.to_lowercase();
    if !lower.starts_with("gpt-5") {
        return;
    }
    let has_sampling = body.get("temperature").is_some()
        || body.get("top_p").is_some()
        || body.get("logprobs").is_some();
    if !has_sampling {
        return;
    }
    let allowed = allows_sampling_for_gpt5(model_id, effort);
    if let Some(obj) = body.as_object_mut() {
        if !allowed {
            obj.remove("temperature");
            obj.remove("top_p");
            obj.remove("logprobs");
        }
    }
}

/// Check if GPT-5 model allows sampling params with given reasoning effort.
fn allows_sampling_for_gpt5(model_id: &str, effort: &str) -> bool {
    let lower = model_id.to_lowercase();
    if !lower.starts_with("gpt-5.2") && !lower.starts_with("gpt-5.4") {
        return true; // not a restricted variant
    }
    !matches!(effort, "low" | "medium" | "high" | "xhigh" | "max")
}

/// Apply reasoning effort normalization for OpenAI models.
pub fn normalize_reasoning_effort(effort: &str, model_id: &str) -> String {
    let effort = effort.trim().to_lowercase();
    if effort == "auto" || effort == "off" {
        return effort;
    }
    let lower = model_id.to_lowercase();

    if lower.starts_with("o4-mini") || lower.starts_with("o3") || lower.starts_with("o1") {
        match effort.as_str() {
            "max" => "max".into(),
            "xhigh" | "high" => "high".into(),
            "medium" => "medium".into(),
            "low" => "low".into(),
            _ => "auto".into(),
        }
    } else if lower.starts_with("gpt-oss") || lower.starts_with("gpt-5") {
        match effort.as_str() {
            "max" | "xhigh" | "high" => "high".into(),
            "medium" => "medium".into(),
            "low" => "low".into(),
            _ => effort,
        }
    } else {
        match effort.as_str() {
            "max" | "xhigh" => "high".into(),
            _ => effort,
        }
    }
}

/// Build Grok search parameters for Chat Completions body.
pub fn apply_grok_search(body: &mut serde_json::Value) {
    if let Some(obj) = body.as_object_mut() {
        obj.insert(
            "search_parameters".to_string(),
            serde_json::json!({"mode": "auto", "return_citations": true}),
        );
    }
}

/// Apply DashScope (Aliyun) built-in search injection for Chat Completions.
pub fn apply_dashscope_chat_search(body: &mut serde_json::Value, _model_id: &str, search_options: Option<&serde_json::Value>) {
    if let Some(obj) = body.as_object_mut() {
        obj.insert("enable_search".to_string(), serde_json::Value::Bool(true));
        if let Some(opts) = search_options {
            if opts.as_object().map(|o| !o.is_empty()).unwrap_or(false) {
                obj.insert("search_options".to_string(), opts.clone());
            } else {
                obj.remove("search_options");
            }
        }
    }
}

/// Apply DashScope thinking override (Responses API format).
pub fn apply_dashscope_responses_reasoning(
    body: &mut serde_json::Value,
    is_reasoning: bool,
    thinking_budget: Option<i32>,
    search_enabled: bool,
    model_id: &str,
) {
    if let Some(obj) = body.as_object_mut() {
        obj.remove("reasoning");
        if !is_reasoning {
            obj.remove("enable_thinking");
            return;
        }
        let off = thinking_budget.map(|b| b < 1024 && b != -1).unwrap_or(false);
        let force = search_enabled && model_id.to_lowercase().starts_with("qwen3-max");
        obj.insert(
            "enable_thinking".to_string(),
            serde_json::Value::Bool(force || !off),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_deepseek() {
        let v = classify_vendor("", "https://api.deepseek.com/v1", "");
        assert_eq!(v, OpenAIVendor::DeepSeek);
    }

    #[test]
    fn test_classify_openrouter() {
        let v = classify_vendor("openrouter", "https://openrouter.ai/api/v1", "");
        assert_eq!(v, OpenAIVendor::OpenRouter);
    }

    #[test]
    fn test_classify_standard() {
        let v = classify_vendor("", "https://api.openai.com/v1", "gpt-4o");
        assert_eq!(v, OpenAIVendor::Standard);
    }

    #[test]
    fn test_gpt5_sampling_restricted() {
        let mut body = serde_json::json!({"temperature": 0.7});
        sanitize_gpt5_sampling(&mut body, "gpt-5.2", "high");
        assert!(body.get("temperature").is_none());
    }

    #[test]
    fn test_gpt5_sampling_allowed() {
        let mut body = serde_json::json!({"temperature": 0.7});
        sanitize_gpt5_sampling(&mut body, "gpt-5.2", "auto");
        assert!(body.get("temperature").is_some());
    }

    #[test]
    fn test_reasoning_effort_o_series() {
        let e = normalize_reasoning_effort("max", "o3");
        assert_eq!(e, "max");
        let e = normalize_reasoning_effort("xhigh", "o4-mini");
        assert_eq!(e, "high");
    }
}
