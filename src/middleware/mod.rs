use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::rc::Rc;
use tracing::instrument;

use crate::errors::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub struct JwtAuth {
    secret: String,
}

impl JwtAuth {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = JwtAuthMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware {
            service: Rc::new(service), // Wrap service in Rc to share it
            secret: self.secret.clone(),
        })
    }
}

pub struct JwtAuthMiddleware<S> {
    service: Rc<S>,
    secret: String,
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    #[instrument(skip(self, req))]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = self.secret.clone(); // Clone the secret for the async block
        let service = Rc::clone(&self.service); // Clone the Rc pointer, not the service itself

        Box::pin(async move {
            let auth_header = match req.headers().get("Authorization") {
                Some(header) => header,
                None => return Err(AuthError::InvalidCredentials.into()),
            };

            let token = match auth_header.to_str().ok().and_then(|s| s.strip_prefix("Bearer ")) {
                Some(t) => t,
                None => return Err(AuthError::InvalidCredentials.into()),
            };

            let claims = match decode::<Claims>(
                token,
                &DecodingKey::from_secret(secret.as_ref()),
                &Validation::default(),
            ) {
                Ok(data) => data.claims,
                Err(_) => return Err(AuthError::InvalidCredentials.into()),
            };

            let now = chrono::Utc::now().timestamp() as usize;
            if claims.exp < now {
                return Err(AuthError::InvalidCredentials.into());
            }

            service.call(req).await
        })
    }
}