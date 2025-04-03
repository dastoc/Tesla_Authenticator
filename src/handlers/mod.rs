use chrono::Utc;
use actix_web::{web, HttpResponse, Responder};
use tracing::instrument;
use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::services::auth::AuthService;
use crate::errors::AuthError;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 50))]
    username: String,

    #[validate(length(min = 8))]
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: chrono::DateTime<Utc>,
}

#[instrument(skip(auth_service))]
pub async fn login(
    req: web::Json<LoginRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<impl Responder, AuthError> {
    req.validate().map_err(|e| AuthError::ValidationError(e.to_string()))?;
    let auth_response = auth_service.authenticate(&req.username, &req.password).await?;
    Ok(HttpResponse::Ok().json(auth_response))
}