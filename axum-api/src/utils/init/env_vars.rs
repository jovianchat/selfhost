use serde::Deserialize;
use std::{str::FromStr, sync::OnceLock};

// pub type Config = Arc<Configuration>;
pub static ENV_VARS: OnceLock<Configuration> = OnceLock::new();

#[derive(Deserialize, Debug)]
pub struct Configuration {
    /// The environment in which to run the application.
    pub env: Environment,

    /// The DSN for the database. Currently, only PostgreSQL is supported.
    pub db_dsn: String,
    /// The maximum number of connections for the PostgreSQL pool.
    pub db_pool_max_size: u32,
    /// JWT secret key
    pub jwt_secret: String,
}
impl Configuration {
    /// Sets the database DSN/Data Source Name.
    /// This method is used in tests to override the database DSN.
    pub fn set_dsn(&mut self, db_dsn: String) {
        self.db_dsn = db_dsn
    }
}

/// Creates a new configuration from environment variables.
pub fn set_configuration() -> crate::Result<()> {
    let env = env_var("APP_ENVIRONMENT")
            .parse::<Environment>()
            .expect("Unable to parse the value of the APP_ENVIRONMENT environment variable. Please make sure it is either \"development\" or \"production\".");

    let db_dsn = env_var("DATABASE_URL");

    let db_pool_max_size = env_var("DATABASE_POOL_MAX_SIZE")
            .parse::<u32>()
            .expect("Unable to parse the value of the DATABASE_POOL_MAX_SIZE environment variable. Please make sure it is a valid unsigned 32-bit integer.");

    let jwt_secret = env_var("JWT_SECRET");

    ENV_VARS
        .set(Configuration {
            env,
            db_dsn,
            db_pool_max_size,
            jwt_secret,
        })
        .expect("Failed to set environment variables");

    Ok(())
}

#[derive(Deserialize, Debug)]
pub enum Environment {
    Development,
    Production,
}
impl FromStr for Environment {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "development" => Ok(Environment::Development),
            "production" => Ok(Environment::Production),
            _ => Err(format!(
                "Invalid environment: {}. Please make sure it is either \"development\" or \"production\".",
                s
            )),
        }
    }
}

fn env_var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("Missing environment variable: {}", name))
}
