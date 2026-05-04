use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ModelType {
    #[serde(rename = "chat")]
    Chat,
    #[serde(rename = "embedding")]
    Embedding,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum Modality {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "image")]
    Image,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum ModelAbility {
    #[serde(rename = "tool")]
    Tool,
    #[serde(rename = "reasoning")]
    Reasoning,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "type")]
    pub model_type: ModelType,
    #[serde(default = "default_text_input")]
    pub input: Vec<Modality>,
    #[serde(default = "default_text_output")]
    pub output: Vec<Modality>,
    #[serde(default)]
    pub abilities: Vec<ModelAbility>,
}

fn default_text_input() -> Vec<Modality> {
    vec![Modality::Text]
}

fn default_text_output() -> Vec<Modality> {
    vec![Modality::Text]
}

impl ModelInfo {
    pub fn new(id: String, display_name: String) -> Self {
        ModelInfo {
            id,
            display_name,
            model_type: ModelType::Chat,
            input: vec![Modality::Text],
            output: vec![Modality::Text],
            abilities: vec![],
        }
    }
}
