use axum::Router;

use crate::http::AppState;

mod chat;
mod llm_settings;

pub fn router_with_state() -> Router<AppState> {
    Router::new().nest("/chat", chat::router())
}

pub fn router() -> Router {
    Router::new()
        .route(
            "/server-health",
            axum::routing::get(|| async { "Server health check route hit" }),
        )
        .nest("/llm-settings", llm_settings::router())
}
