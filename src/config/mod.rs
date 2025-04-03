use std::env;
use url::Url;
use validator::Validate;

use crate::errors::AppConfigError;

#[derive(Debug, Validate)]
pub struct AppConfig {
    #[validate(url)]
    pub database_url: String,

    #[validate(length(min = 32))]
    pub jwt_secret: String,
    
    #[validate(range(min = 1024, max = 65535))]
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppConfigError> {
        let user = env::var("POSTGRES_USER")?;
        let password = env::var("POSTGRES_PASSWORD")?;
        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".to_string());
        let db = env::var("POSTGRES_DB")?;

        let database_url = format!("postgres://{}:{}@{}:{}/{}", user, password, host, port, db);
        let url = Url::parse(&database_url).map_err(|e| AppConfigError::InvalidDatabaseUrl(e.to_string()))?;

        let config = AppConfig {
            database_url: url.to_string(),
            jwt_secret: env::var("JWT_SECRET")?,
            server_port: env::var("SERVER_PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|e| AppConfigError::InvalidEnvVar(format!("SERVER_PORT: {}", e)))?,
        };

        config.validate()?;
        Ok(config)
    }
}