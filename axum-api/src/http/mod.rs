use std::net::{Ipv6Addr, SocketAddr};

use axum::{middleware::from_fn, Router};
use sqlx::PgPool;
use tower::ServiceBuilder;

use middleware::layers;
pub mod middleware;
use crate::{models::cache::CacheState, routes, utils::telemetry};

#[derive(Clone)]
pub struct AppState {
    pub user: String,
    pub pg_pool: PgPool,
    pub cache: CacheState,
}

pub async fn serve(pg_pool: PgPool) {
    let cache = CacheState::new();
    let app_state = AppState {
        user: "admin".to_string(),
        pg_pool,
        cache,
    };

    let app = Router::new()
        .merge(routes::router_with_state())
        .with_state(app_state.clone())
        .merge(routes::router())
        .merge(middleware::router())
        .layer(
            ServiceBuilder::new()
                .layer(layers::request_id())
                .layer(telemetry::http_trace_layer())
                .layer(layers::propagate_request_id())
                .layer(layers::timeout())
                .layer(layers::cors())
                .layer(layers::normalize_path())
                .layer(from_fn(middleware::auth)),
        );

    let listen_address = SocketAddr::from((Ipv6Addr::UNSPECIFIED, 5000));
    let listener = tokio::net::TcpListener::bind(listen_address)
        .await
        .expect("Failed to bind Listen address");

    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server")
}
