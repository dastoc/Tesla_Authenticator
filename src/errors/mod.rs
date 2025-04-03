use actix_web::{HttpResponse, ResponseError};
use diesel::result;
use r2d2::Error as PoolError;
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Debug, Error)]
pub enum AppConfigError {
    #[error("Missing environment variable: {0}")]
    MissingEnvVar(String),

    #[error("Invalid environment variable: {0}")]
    InvalidEnvVar(String),

    #[error("Invalid database URL: {0}")]
    InvalidDatabaseUrl(String),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),
}

impl From<std::env::VarError> for AppConfigError {
    fn from(err: std::env::VarError) -> Self {
        AppConfigError::MissingEnvVar(err.to_string())
    }
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] result::Error),

    #[error("Database pool error: {0}")]
    PoolError(#[from] PoolError),

    #[error("Bcrypt error: {0}")]
    BcryptError(#[from] BcryptError),

    #[error("JWT error: {0}")]
    JwtError(#[from] JwtError),

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("Config error: {0}")]
    ConfigError(#[from] AppConfigError),

    #[error("Validation error: {0}")]
    ValidationError(String),
}



// Return HTTP response for AuthError
impl ResponseError for AuthError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AuthError::InvalidCredentials => {
                HttpResponse::Unauthorized().body(self.to_string())
            },
            AuthError::ConfigError(e) => {
                HttpResponse::InternalServerError().json(e.to_string())
            },
            _ => HttpResponse::InternalServerError().body(self.to_string()),
        }
    }
}