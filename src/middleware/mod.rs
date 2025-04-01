use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// pub async fn validate_token(token: &str, secret: &str) -> Result<Claims, AuthError> {
//     let token_data = decode::<Claims> (
//         token,
//         &DecodingKey::from_secret(secret.as_ref()),
//         &Validation::default(),
//     ).map_err(AuthError::JwtError)?;

//     Ok(token_data.claims)
// }
