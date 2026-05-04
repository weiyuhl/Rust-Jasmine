use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ProviderKind {
    #[serde(rename = "openai")]
    OpenAI,
    #[serde(rename = "claude")]
    Claude,
}

impl ProviderKind {
    pub fn classify(key: &str, explicit_type: Option<&str>) -> Self {
        if let Some(et) = explicit_type {
            return match et.to_lowercase().as_str() {
                "claude" => ProviderKind::Claude,
                _ => ProviderKind::OpenAI,
            };
        }
        let k = key.to_lowercase();
        if k.contains("claude") || k.contains("anthropic") {
            ProviderKind::Claude
        } else {
            ProviderKind::OpenAI
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ProviderKind::OpenAI => "openai",
            ProviderKind::Claude => "claude",
        }
    }
}
