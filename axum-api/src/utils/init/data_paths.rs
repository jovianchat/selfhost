use anyhow::{Context, Result};
use std::{io::Write, sync::OnceLock};

pub static TOML_CONFIG_PATHS: OnceLock<Paths> = OnceLock::new();

#[derive(Debug)]
pub struct Paths {
    pub config: Config,
    pub llm: Llm,
}

#[derive(Debug)]
pub struct Config {
    pub password_hash: String,
}

#[derive(Debug)]
pub struct Llm {
    pub api_and_models_config: String,
    pub prompt_engineering: String,
    pub selected_model_and_prompt: String,
}

pub fn set_toml_paths_fn() -> Result<()> {
    let config = Config {
            password_hash: create_toml("/app/.data/config/admin-user/pass.toml", Some("password_hash = \"$argon2id$v=19$m=19456,t=2,p=1$rcqhKow+Gkq2C1CQMRjdBA$iVoDKImduIYsD3XcaAIMvKeM0JYl9yEaLcx844A7ESg\""))?,
        };
    let llm = Llm {
        api_and_models_config: create_toml(
            "/app/.data/llm/api_and_models-config.toml",
            Some("configs = []"),
        )?,
        prompt_engineering: create_toml(
            "/app/.data/llm/prompt-engineering.toml",
            Some("configs = []"),
        )?,
        selected_model_and_prompt: create_toml(
            "/app/.data/llm/fav-models.toml",
            Some("configs = []"),
        )?,
    };
    TOML_CONFIG_PATHS.set(Paths { config, llm }).unwrap();

    Ok(())
}

fn create_toml(raw_path: &str, default: Option<&str>) -> Result<String> {
    let path = std::path::Path::new(raw_path);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create parent directory: {}", parent.display()))?;
    }
    // Check if the file exists or create a new one with a default value
    if !path.exists() {
        match default {
            Some(default) => {
                let mut file = std::fs::File::create(path).with_context(|| {
                    format!("Failed to create file at path: {}", path.display())
                })?;
                file.write_all(default.as_bytes()).with_context(|| {
                    format!("Failed to write default value to file: {}", path.display())
                })?;
            }
            None => {
                std::fs::File::create(path).with_context(|| {
                    format!("Failed to create file at path: {}", path.display())
                })?;
            }
        }
    }

    // Return path as a string
    Ok(raw_path.to_string())
}
