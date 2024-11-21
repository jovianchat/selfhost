use super::{password, JWT};
use crate::Result;
use axum::{
    routing::{post, put},
    Json, Router,
};

pub fn router() -> Router {
    Router::new()
        .route("/sign-in", post(sign_in))
        .route("/new-access-token", post(new_access_token))
        .route("/new-refresh-token", post(new_refresh_token))
        .route("/change-password", put(change_password))
}

#[derive(serde::Serialize)]
pub struct AuthTokens {
    refresh: Option<Token>,
    access: Option<Token>,
}
#[derive(serde::Serialize)]
pub struct Token {
    token: String,
    expiration: usize,
}
pub async fn sign_in(Json(password): Json<String>) -> Result<Json<AuthTokens>> {
    let stored_hash = password::load_admin();
    password::verify(password, stored_hash).await?;

    let (refresh_token, refresh_token_expiration) =
        JWT::RefreshToken.encode("admin".to_string())?;
    let (access_token, access_token_expiration) = JWT::AccessToken.encode("admin".to_string())?;
    let auth_tokens = AuthTokens {
        refresh: Some(Token {
            token: refresh_token,
            expiration: refresh_token_expiration,
        }),
        access: Some(Token {
            token: access_token,
            expiration: access_token_expiration,
        }),
    };
    Ok(Json(auth_tokens))
    // Sign out on client side by just removing token
}

pub async fn new_access_token(Json(refresh_token): Json<String>) -> Result<Json<AuthTokens>> {
    let sub = JWT::decode(refresh_token)?;

    let (access_token, access_token_expiration) = JWT::AccessToken.encode(sub)?;
    let auth_tokens = AuthTokens {
        refresh: None,
        access: Some(Token {
            token: access_token,
            expiration: access_token_expiration,
        }),
    };
    Ok(Json(auth_tokens))
}
pub async fn new_refresh_token(Json(refresh_token): Json<String>) -> Result<Json<AuthTokens>> {
    let sub = JWT::decode(refresh_token)?;
    let (refresh_token, refresh_token_expiration) = JWT::RefreshToken.encode(sub)?;
    let auth_tokens = AuthTokens {
        refresh: Some(Token {
            token: refresh_token,
            expiration: refresh_token_expiration,
        }),
        access: None,
    };
    Ok(Json(auth_tokens))
}

pub async fn change_password(Json(password): Json<String>) -> Result<()> {
    let new_hash = password::hash(password).await?;

    password::save_admin(new_hash)?;
    Ok(())
}
