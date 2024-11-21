use axum::Router;

mod auth;
pub mod layers;

pub use auth::middleware as auth;
pub use auth::JWT;

pub fn router() -> Router {
    Router::new().nest("/auth", auth::router())
}
