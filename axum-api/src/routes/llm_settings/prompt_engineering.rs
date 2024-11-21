use std::ops::Deref;

use crate::{models::llm::PromptConfig, utils::init::TOML_PATHS, Context, Result};
use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

// Need seperate struct because toml cant serialize Vec directly so we have to do it manually
#[derive(Serialize, Deserialize)]
struct Configs {
    configs: Vec<PromptConfig>, // configs name need to match the toml array name
}

pub async fn get_configs_handler() -> Result<Json<Vec<PromptConfig>>> {
    let toml = read_toml()?;
    Ok(Json(toml.configs))
}
pub fn get_configs_fn() -> Result<Vec<PromptConfig>> {
    let toml = read_toml()?;
    Ok(toml.configs)
}

pub async fn add_new_config(Json(config): Json<PromptConfig>) -> Result<StatusCode> {
    let mut toml = read_toml()?;
    toml.configs.push(config);
    save_toml(toml)?;
    Ok(StatusCode::OK)
}

pub async fn delete_config(id: String) -> Result<StatusCode> {
    let mut toml = read_toml()?;
    toml.configs
        .retain(|config| config.id != id.parse::<u32>().unwrap());
    save_toml(toml)?;
    Ok(StatusCode::OK)
}

pub async fn update_config(Json(config): Json<PromptConfig>) -> Result<StatusCode> {
    let mut toml = read_toml()?;
    let index = toml.configs.iter().position(|c| c.id == config.id).unwrap();
    toml.configs[index] = config;
    save_toml(toml)?;
    Ok(StatusCode::OK)
}

fn read_toml() -> Result<Configs> {
    let content = std::fs::read_to_string(TOML_PATHS.get().unwrap().llm.prompt_engineering.deref())
        .expect("Failed to read prompt-engineering configs from file");
    let toml: Configs = toml::from_str(&content)
        .expect("Failed to parse prompt-engineering configs from toml file");
    Ok(toml)
}
fn save_toml(toml: Configs) -> Result<()> {
    let content =
        toml::to_string(&toml).context("Failed to parse prompt-engineering config to toml")?;
    std::fs::write(
        TOML_PATHS.get().unwrap().llm.prompt_engineering.deref(),
        content,
    )
    .context("Failed to write prompt-engineering config to file")?;
    Ok(())
}
