mod data_paths;
mod env_vars;
use crate::Result;

pub use data_paths::TOML_CONFIG_PATHS as TOML_PATHS;
pub use env_vars::ENV_VARS;

pub fn config() -> Result<()> {
    env_vars::set_configuration()?;
    data_paths::set_toml_paths_fn()?;

    Ok(())
}
