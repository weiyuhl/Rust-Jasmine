use regex::Regex;
use std::sync::OnceLock;

use crate::modal_provider::models::{Modality, ModelAbility, ModelInfo, ModelType};

/// Regex-based model inference, mirroring Dart ModelRegistry.infer().
pub fn infer_model_info(base: ModelInfo) -> ModelInfo {
    let id_lower = base.id.to_lowercase();
    let mut input = base.input;
    let mut output = base.output;
    let mut abilities = base.abilities;
    let model_type = base.model_type;

    // Embedding detection
    if model_type == ModelType::Embedding || is_likely_embedding(&id_lower) {
        if !input.contains(&Modality::Text) {
            input.push(Modality::Text);
        }
        output = vec![Modality::Text];
        abilities = vec![];
        return ModelInfo {
            model_type: ModelType::Embedding,
            input,
            output,
            abilities,
            ..base
        };
    }

    // Image model detection
    if id_lower.contains("image") {
        if !input.contains(&Modality::Image) {
            input.push(Modality::Image);
        }
        if !output.contains(&Modality::Image) {
            output.push(Modality::Image);
        }
        abilities.retain(|a| *a != ModelAbility::Tool && *a != ModelAbility::Reasoning);
        return ModelInfo {
            input,
            output,
            abilities,
            model_type,
            ..base
        };
    }

    // gpt-5-chat is explicitly excluded from all capability inference
    // (replaces the lookahead (?!-chat) which Rust regex crate doesn't support)
    let is_gpt5_chat = id_lower == "gpt-5-chat";

    // Vision-capable model regex
    if !is_gpt5_chat && vision_re().is_match(&id_lower) {
        if !input.contains(&Modality::Image) {
            input.push(Modality::Image);
        }
    }

    // Tool-using model regex
    if !is_gpt5_chat && tool_re().is_match(&id_lower) && !abilities.contains(&ModelAbility::Tool) {
        abilities.push(ModelAbility::Tool);
    }

    // Reasoning model regex
    if !is_gpt5_chat
        && reasoning_re().is_match(&id_lower)
        && !abilities.contains(&ModelAbility::Reasoning)
    {
        abilities.push(ModelAbility::Reasoning);
    }

    ModelInfo {
        input,
        output,
        abilities,
        model_type,
        ..base
    }
}

fn is_likely_embedding(id: &str) -> bool {
    id.contains("embedding")
        || Regex::new(r"(^|[-_/])embed(?:dings?)?([-.]|$)")
            .map(|re| re.is_match(id))
            .unwrap_or(false)
}

// ── Lazy-initialized regexes using OnceLock (no poisoning) ─────

fn vision_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)(gpt-4o|gpt-4\.1|gpt-5|o\d|gemini|claude|qwen-?3([-.])5|kimi-k2([-.])5|doubao.+1([-.])(?:6|8)|grok-4|step-3|intern-s1)").unwrap()
    })
}

fn tool_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)(gpt-4o|gpt-4\.1|gpt-oss|gpt-5|o\d|gemini|claude|qwen-?3|doubao.+1([-.])(?:6|8)|grok-4|kimi-k2|step-3|intern-s1|glm-4([-.])(?:5|6|7)|glm-5|minimax-m2|deepseek-(?:r1|v3|chat|v3\.1|v3\.2|v4)|deepseek-reasoner|mimo-v2)").unwrap()
    })
}

fn reasoning_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"(?i)(gpt-oss|gpt-5|o\d|gemini-(?:2\.5|3).*|gemini-(?:flash-latest|pro-latest)|gemini-3-pro-image-preview|gemma[-_]?4|claude|qwen-?3|doubao.+1([-.])(?:6|8)|grok-4|kimi-k2|step-3|intern-s1|glm-4([-.])(?:5|6|7)|glm-5|minimax-m2|deepseek-(?:r1|v3\.1|v3\.2|v4)|deepseek-reasoner|mimo-v2)").unwrap()
    })
}
