use chrono::{Utc, Duration};
use jsonwebtoken::{decode, encode, Algorithm, Decodingkey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use crate::errors::ErrorMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
} 

pub fn create_token (
    user_id: &str,
    secret: &[u8],
    expire_in_seconds: i64
) -> Result<String, jsonwebtoken::errors::Error> {
    if user_id.is_empty() {
        return Err(jsonwebtoken::errors::ErrorKind::InvalidToken.into());
    }    

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(expire_in_seconds)).timestamp() as usize;
    let claims = TokenClaims {
        sub: user_id.to_string(),
        iat,
        exp,
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn decode_token<T: Intro<String>>(token: T, secret: &[u8]) -> Result<String, HttpError> {
    let decoded = decode::<TokenClaims>(
        &token.into(),
        &Decodingkey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    );

    match decoded {
        Ok(token) => Ok(token.claims.sub),
        Err(_) => Err(HttpError::new(ErrorMessage::InvalidToken.to_string(), 401)),
    }
}