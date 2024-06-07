use axum::{
    extract::{Request, State},

    middleware::Next,
    response::{ Response},
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{
    decode, encode, get_current_timestamp, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

use crate::AppState;

use super::{AppError, Ulid};

pub async fn protected(TypedHeader(bearer): TypedHeader<Authorization<Bearer>>) -> String {
    format!("Welcome to the protected area :)\nYour data:\n{bearer:?}",)
}

const SITE: &str  = "elerem.com";

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
struct JWTClaims {
    // Registered claims
    iss: String, // Issuer
    sub: String, // Subject
    aud: String, // Audience
    exp: i64,    // Expiration time
    iat: u64,    // Issued at
    jti: String, // JWT ID

    // Private claims
    pub rol: String, // User's role TODO: use an enum
}

impl JWTClaims {
    fn new(user_id: i32, user_role: String) -> Self {
        JWTClaims {
            iss: SITE.to_owned(),
            sub: user_id.to_string(),
            aud: SITE.to_owned(),
            exp: (Utc::now() + Duration::days(1)).timestamp(),
            iat: get_current_timestamp(),
            jti: Ulid::new().generate(&user_id),
            rol: user_role,
        }
    }
}

impl Display for JWTClaims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

pub async fn create_token(user_id: i32, user_role: String, state: &AppState)->Result<String, AppError>{
    let claims = JWTClaims::new(user_id, user_role);
    encode(&Header::default(), &claims, &state.keys.encoding).map_err(AppError::JWTEncodingError)
}

#[derive(Clone)]
pub struct UserRequest {
    id: i32,
    role: String,
}

pub async fn jwt_middleware(
    State(state): State<AppState>,
    TypedHeader(bearer): TypedHeader<Authorization<Bearer>>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let mut validation = Validation::default();
    validation.set_audience(&[SITE]);
    validation.set_issuer(&[SITE]);
    validation.reject_tokens_expiring_in_less_than = 86400u64;
    validation.set_required_spec_claims(&["exp", "nbf", "aud", "iss", "sub"]);

    let token_data = decode::<JWTClaims>(bearer.token(), &state.keys.decoding, &validation).map_err(AppError::JWTDecodingError)?;
    request.extensions_mut().insert(UserRequest {
        id: token_data.claims.sub.parse::<i32>().unwrap(),
        role: token_data.claims.rol,
    });
    Ok(next.run(request).await)
}

