pub mod http;
pub mod models;
mod routes;
pub mod utils;

pub use anyhow::Context;
pub use utils::error::Error;
pub type Result<T, E = Error> = std::result::Result<T, E>;
