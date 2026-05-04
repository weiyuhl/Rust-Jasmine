use serde::{Deserialize, Serialize};

use super::provider_kind::ProviderKind;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub id: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(default)]
    #[serde(rename = "baseUrl")]
    pub base_url: String,
    #[serde(rename = "providerType")]
    pub provider_type: Option<ProviderKind>,
    #[serde(rename = "chatPath")]
    pub chat_path: Option<String>,
    #[serde(rename = "useResponseApi")]
    pub use_response_api: Option<bool>,
    #[serde(default)]
    pub models: Vec<String>,
    #[serde(default)]
    #[serde(rename = "modelOverrides")]
    pub model_overrides: serde_json::Map<String, serde_json::Value>,
}

impl ProviderConfig {
    pub fn effective_provider_kind(&self) -> ProviderKind {
        ProviderKind::classify(
            &self.id,
            self.provider_type.as_ref().map(|t| t.as_str()),
        )
    }

    pub fn base_url_normalized(&self) -> String {
        self.base_url.trim_end_matches('/').to_string()
    }

    pub fn api_key_effective(&self) -> &str {
        self.api_key.trim()
    }

    pub fn chat_path_effective(&self) -> &str {
        self.chat_path
            .as_deref()
            .unwrap_or("/chat/completions")
    }
}
