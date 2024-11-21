use axum::{extract::Path, Json};

use crate::{
    models::llm::{ApiConfig, FavModel, PromptConfig},
    Result,
};

pub async fn get_handler(Path(id): Path<u32>) -> Result<Json<FavModel>> {
    let llm_cfg = super::get_selected_fn(id)?;
    // Return api key base url model name and prompt
    let selected_fav_model = FavModel {
        id,
        api: ApiConfig {
            id: llm_cfg.0.id,
            name: llm_cfg.0.name,
            endpoint_sdk: llm_cfg.0.endpoint_sdk,
            secret_key: Some(llm_cfg.0.secret_key.unwrap()),
            base_url: Some(llm_cfg.0.base_url.unwrap()),
            models: None,
        },
        model: llm_cfg.1,
        prompt: PromptConfig {
            id: llm_cfg.2.id,
            name: llm_cfg.2.name,
            max_tokens: llm_cfg.2.max_tokens,
            temperature: llm_cfg.2.temperature,
            system_prompt: llm_cfg.2.system_prompt,
        },
    };
    Ok(Json(selected_fav_model))
}
