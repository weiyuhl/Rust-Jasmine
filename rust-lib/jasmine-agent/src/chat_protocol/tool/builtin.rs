use serde_json::json;

/// Built-in tool name constants (mirrors Kelivo's BuiltInToolNames).
pub mod tool_names {
    pub const SEARCH: &str = "search";
    pub const CODE_EXECUTION: &str = "code_execution";
    pub const CODE_INTERPRETER: &str = "code_interpreter";
    pub const IMAGE_GENERATION: &str = "image_generation";
}

/// Check whether a model supports OpenAI Responses built-in web search.
pub fn is_openai_builtin_search_supported(model_id: &str) -> bool {
    let m = model_id.to_lowercase();
    m.starts_with("gpt-4o")
        || m.starts_with("gpt-4.1")
        || m.starts_with("o4-mini")
        || m == "o3"
        || m.starts_with("o3-")
}

/// Check if a provider is DashScope (Aliyun).
pub fn is_dashscope_provider(base_url: &str) -> bool {
    let host = extract_host(base_url);
    host == "dashscope.aliyuncs.com"
}

/// Check if a model is a Grok variant.
pub fn is_grok_model(model_id: &str) -> bool {
    model_id.to_lowercase().contains("grok")
}

/// Check whether a Claude model supports built-in web search.
pub fn is_claude_builtin_search_supported(model_id: &str) -> bool {
    let m = model_id.to_lowercase();
    if m.contains("mythos") { return true; }
    matches!(
        m.as_str(),
        "claude-opus-4-7" | "claude-opus-4-6"
        | "claude-sonnet-4-5-20250929" | "claude-sonnet-4-20250514"
        | "claude-3-7-sonnet-20250219" | "claude-haiku-4-5-20251001"
        | "claude-3-5-haiku-latest" | "claude-sonnet-4-6"
        | "claude-opus-4-1-20250805" | "claude-opus-4-20250514"
    )
}

/// Get the Claude built-in search tool type string.
pub fn claude_builtin_search_tool_type(is_dynamic: bool) -> &'static str {
    if is_dynamic {
        "web_search_20250305"
    } else {
        "web_search_20260209"
    }
}

/// Build a web_search tool definition for Chat Completions / Responses API.
pub fn build_web_search_tool(use_preview: bool) -> serde_json::Value {
    let search_type = if use_preview {
        "web_search_preview"
    } else {
        "web_search"
    };
    json!({"type": search_type})
}

fn extract_host(url: &str) -> &str {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openai_search_supported() {
        assert!(is_openai_builtin_search_supported("gpt-4o"));
        assert!(is_openai_builtin_search_supported("gpt-4.1"));
        assert!(!is_openai_builtin_search_supported("gpt-3.5-turbo"));
    }

    #[test]
    fn test_claude_search_supported() {
        assert!(is_claude_builtin_search_supported("claude-opus-4-7"));
        assert!(!is_claude_builtin_search_supported("claude-2"));
    }
}
