use std::env;
use url::Url;

use crate::errors::AppConfigError;

pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_port: u16,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppConfigError> {
        // Retrieve environment variables with logging for debugging
        let user = env::var("POSTGRES_USER").map_err(|e| {
            eprintln!("Error reading POSTGRES_USER: {:?}", e);
            AppConfigError::MissingEnvVar("POSTGRES_USER".to_string())
        })?;
        let password = env::var("POSTGRES_PASSWORD").map_err(|e| {
            eprintln!("Error reading POSTGRES_PASSWORD: {:?}", e);
            AppConfigError::MissingEnvVar("POSTGRES_PASSWORD".to_string())
        })?;
        let host = env::var("POSTGRES_HOST").unwrap_or_else(|_| {
            eprintln!("POSTGRES_HOST not set, defaulting to 'localhost'");
            "localhost".to_string()
        });
        let port = env::var("POSTGRES_PORT").unwrap_or_else(|_| {
            eprintln!("POSTGRES_PORT not set, defaulting to '5432'");
            "5432".to_string()
        });
        let db = env::var("POSTGRES_DB").map_err(|e| {
            eprintln!("Error reading POSTGRES_DB: {:?}", e);
            AppConfigError::MissingEnvVar("POSTGRES_DB".to_string())
        })?;

        // Validate user and password
        if user.is_empty() {
            return Err(AppConfigError::InvalidEnvVar(
                "POSTGRES_USER cannot be empty".to_string(),
            ));
        }
        if user.contains(|c: char| !c.is_alphanumeric() && c != '_') {
            return Err(AppConfigError::InvalidEnvVar(
                "POSTGRES_USER contains invalid characters (only alphanumeric and underscores allowed)".to_string(),
            ));
        }

        // Construct the database URL using the `url` crate for proper encoding
        let database_url = format!(
            "postgres://{}@{}:{}/{}",
            user, // User and password will be encoded below
            host,
            port,
            db
        );
        let mut url = Url::parse(&database_url).map_err(|e| {
            AppConfigError::InvalidDatabaseUrl(format!("Failed to parse database URL: {}", e))
        })?;
        url.set_password(Some(&password)).map_err(|_| {
            AppConfigError::InvalidDatabaseUrl("Failed to set password in database URL".to_string())
        })?;

        let database_url = url.to_string();
        if database_url.is_empty() {
            return Err(AppConfigError::InvalidDatabaseUrl(
                "Database URL is empty".to_string(),
            ));
        }
        Ok(AppConfig {
            database_url,
            jwt_secret: env::var("JWT_SECRET").map_err(|e| {
                eprintln!("Error reading JWT_SECRET: {:?}", e);
                AppConfigError::MissingEnvVar("JWT_SECRET".to_string())
            })?,
            server_port: env::var("SERVER_PORT")
                .map_err(|e| {
                    eprintln!("Error reading SERVER_PORT: {:?}", e);
                    AppConfigError::MissingEnvVar("SERVER_PORT".to_string())
                })?
                .parse()
                .map_err(|e| {
                    AppConfigError::InvalidEnvVar(format!("Invalid SERVER_PORT: {}", e))
                })
                .unwrap_or(8080),
        })
    }
}