use crate::{Error, Result};
use axum::{extract::Request, http::header, middleware::Next, response::Response};

mod jwt;
pub mod password;
mod routes;

pub use jwt::JWT;
pub use routes::router;

pub async fn middleware(req: Request, next: Next) -> Result<Response> {
    let public_route = ["/auth/sign-in", "/auth/new-access-token"];
    if public_route.contains(&req.uri().path()) {
        return Ok(next.run(req).await);
    }

    let auth_header = req.headers().get(header::AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header.to_str().map_err(|_| Error::Unauthorized)?,
        None => return Err(Error::Unauthorized),
    };
    let access_token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);
    let _sub = JWT::decode(access_token.to_string())?;
    // if token_data.claims.sub != "admin" {
    //     return Err(Error::Unauthorized);
    // }
    // req.extensions_mut().insert(current_user);
    Ok(next.run(req).await)
}
