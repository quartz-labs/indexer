#[derive(Debug)]
pub enum ConfigError {
    EnvLoadError(dotenvy::Error),
    MissingVar(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::EnvLoadError(e) => write!(f, "Failed to load .env file: {}", e),
            ConfigError::MissingVar(var) => write!(f, "Missing environment variable: {}", var),
        }
    }
}

impl std::error::Error for ConfigError {}
