use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use bcrypt::verify;
use diesel::prelude::*;

use crate::{
    db::{models::User, schema::users, DbPool},
    errors::AuthError,
    handlers::AuthResponse,
    middleware::Claims,
};

#[derive(Clone)]
pub struct AuthService {
    pool: DbPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(pool: DbPool, jwt_secret: String) -> Self {
        Self { pool, jwt_secret }
    }

    pub fn authenticate(&self, username: &str, password: &str) -> Result<AuthResponse, AuthError> {
        let mut conn = self
            .pool
            .get()
            .map_err(AuthError::PoolError)?;

        let user = users::table
            .filter(users::name.eq(username))
            .first::<User>(&mut conn)
            .map_err(|_| AuthError::InvalidCredentials)?; // avoid leaking existence info

        let is_valid = verify(password, &user.password_hash)
            .map_err(AuthError::BcryptError)?;

        if !is_valid {
            return Err(AuthError::InvalidCredentials);
        }

        let expiration = Utc::now() + Duration::hours(24);

        let claims = Claims {
            sub: user.name,
            exp: expiration.timestamp() as usize,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
        .map_err(AuthError::JwtError)?;

        Ok(AuthResponse {
            token,
            expires_at: expiration,
        })
    }
}