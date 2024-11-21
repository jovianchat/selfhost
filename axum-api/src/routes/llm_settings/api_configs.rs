use std::ops::Deref;

use crate::{models::llm::ApiConfig, utils::init::TOML_PATHS, Context, Result};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiConfigs {
    configs: Vec<ApiConfig>, // configs name need to match the toml array name
}

// Read all configurations and return them without exposing the API key
pub async fn get_configs_handler() -> Result<Json<Vec<ApiConfig>>> {
    let toml = read_toml()?;
    let configs: Vec<ApiConfig> = toml
        .configs
        .into_iter()
        .map(|config| config.without_api_key())
        .collect(); // Return configs without API key
    Ok(Json(configs))
}

pub fn get_configs_fn() -> Result<Vec<ApiConfig>> {
    let toml_content = read_toml()?;
    Ok(toml_content.configs)
}

// Add a new API config
pub async fn add_new_config(Json(config): Json<ApiConfig>) -> Result<StatusCode> {
    let mut toml = read_toml()?;
    toml.configs.push(config);
    save_toml(toml)?;
    Ok(StatusCode::OK)
}

// Update an existing API config
pub async fn update_config(Json(config): Json<ApiConfig>) -> Result<StatusCode> {
    let mut toml = read_toml()?;
    let index = config.id as usize;

    let mut updated_config = config;
    // Retain the old API key if the new one is empty
    if updated_config
        .secret_key
        .as_deref()
        .unwrap_or("")
        .is_empty()
    {
        updated_config.secret_key = toml.configs[index].secret_key.clone();
    }
    toml.configs[index] = updated_config;
    save_toml(toml)?;
    Ok(StatusCode::OK)
}

// Delete a configuration by ID
pub async fn delete_config(id: String) -> Result<StatusCode> {
    let mut toml = read_toml()?;
    toml.configs
        .retain(|config| config.id != id.parse::<u32>().unwrap());
    // Reassign IDs to ensure they are contiguous
    for (new_id, config) in toml.configs.iter_mut().enumerate() {
        config.id = new_id as u32; // Update the id based on new index
    }
    save_toml(toml)?;
    Ok(StatusCode::OK)
}

// Utility functions for reading and saving the TOML configs
fn read_toml() -> Result<ApiConfigs> {
    let content =
        std::fs::read_to_string(TOML_PATHS.get().unwrap().llm.api_and_models_config.deref())
            .expect("Failed to read llm-api-config configs from file");
    let toml: ApiConfigs =
        toml::from_str(&content).expect("Failed to parse llm-api-config configs from toml file");
    Ok(toml)
}

fn save_toml(toml: ApiConfigs) -> Result<()> {
    let content = toml::to_string(&toml).context("Failed to parse llm-api-config to toml")?;
    std::fs::write(
        TOML_PATHS.get().unwrap().llm.api_and_models_config.deref(),
        content,
    )
    .context("Failed to write llm-api-config to file")?;
    Ok(())
}
