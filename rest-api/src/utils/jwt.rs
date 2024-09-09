use super::app_error::AppError;
use axum::{
    body::Body,
    http::{HeaderMap, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use tracing::{debug, error};

#[derive(Serialize, Deserialize)]
pub struct Claims {
    exp: usize,
    username: String,
}

lazy_static! {
    static ref SECRET_KEY: String = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
}

pub fn create_token(username: String) -> Result<String, AppError> {
    let now = chrono::Utc::now();
    let expires_at = now + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, username };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(SECRET_KEY.as_bytes());

    encode(&token_header, &claims, &key).map_err(|err| {
        error!("Error creating token: {:?}", err);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error, please try again later",
        )
    })
}

pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        &token.replace("Bearer ", ""),
        &DecodingKey::from_secret(SECRET_KEY.as_bytes()),
        &Validation::new(jsonwebtoken::Algorithm::HS256),
    )
    .map_err(|err| match err.kind() {
        jsonwebtoken::errors::ErrorKind::InvalidToken
        | jsonwebtoken::errors::ErrorKind::InvalidSignature
        | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            AppError::new(StatusCode::UNAUTHORIZED, "not authenticated!")
        }
        _ => {
            error!("Error validating token: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error validating token")
        }
    })
    .and_then(|decoded| {
        if chrono::Utc::now().timestamp() > decoded.claims.exp as i64 {
            Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "not authenticated!",
            ))
        } else {
            Ok(decoded.claims)
        }
    })
}

pub async fn authenticate(
    headers: HeaderMap,
    request: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    if let Some(value) = headers.get("Authorization") {
        let token = value.to_str().map_err(|err| {
            error!("Error extracting token from headers: {:?}", err);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error reading token")
        })?;

        let claim = validate_token(token)?;

        debug!("Authenticated user: {}", claim.username);

        if claim.exp < (chrono::Utc::now().timestamp() as usize) {
            return Err(AppError::new(StatusCode::UNAUTHORIZED, "Token has expired"));
        }

        Ok(next.run(request).await)
    } else {
        Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Not authenticated!",
        ))
    }
}
