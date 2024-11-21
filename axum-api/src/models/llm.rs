use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct FavModel {
    pub id: u32,
    pub api: ApiConfig,
    pub model: String,
    pub prompt: PromptConfig,
}

#[derive(Serialize, Deserialize)]
pub struct PromptConfig {
    pub id: u32,
    pub name: String,
    pub max_tokens: u32,
    pub temperature: f32,
    pub system_prompt: String,
}
#[derive(Serialize, Deserialize)]
pub enum LlmSdk {
    OpenAI,
    Anthropic,
    OpenAICompatible,
}
#[derive(Serialize, Deserialize)]
pub struct ApiConfig {
    pub id: u32,
    pub name: String,
    pub endpoint_sdk: LlmSdk,
    // #[serde(skip_serializing_if = "should_skip_api_key")]
    pub secret_key: Option<String>,
    pub base_url: Option<String>,
    pub models: Option<Vec<String>>,
}

// // Custom serialization function to skip the API key or return an empty string
// fn should_skip_api_key(api_key: &Option<String>) -> bool {
//     api_key.is_none() || api_key.as_deref() == Some("")
// }
impl ApiConfig {
    pub fn without_api_key(mut self) -> Self {
        // self.secret_key = Some(String::new()); // Set api_key to an empty string
        self.secret_key = None; // Set api_key to an empty string
        self
    }
}
