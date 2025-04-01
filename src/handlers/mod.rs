use chrono::Utc;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::services::auth::AuthService;
use crate::errors::AuthError;

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: chrono::DateTime<Utc>,
}

pub async fn login(
    req: web::Json<LoginRequest>,
    auth_service: web::Data<AuthService>,
) -> Result<impl Responder, AuthError> {
    let auth_response = auth_service.authenticate(&req.username, &req.password)?;
    Ok(HttpResponse::Ok().json(auth_response))
}