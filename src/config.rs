use std::env;
use dotenvy::dotenv;
use crate::errors::ConfigError;

#[derive(Debug)]
pub struct Config {
    pub rpc_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        dotenv().map_err(ConfigError::EnvLoadError)?;
        
        Ok(Config {
            rpc_url: env::var("RPC_URL")
                .map_err(|_| ConfigError::MissingVar("RPC_URL".to_string()))?,
            redis_url: env::var("REDIS_URL")
                .map_err(|_| ConfigError::MissingVar("REDIS_URL".to_string()))?,
        })
    }
}