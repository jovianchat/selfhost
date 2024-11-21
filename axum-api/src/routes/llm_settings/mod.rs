use axum::{routing::get, Router};

mod api_configs;
mod fav_model;
mod fav_models;
mod prompt_engineering;
pub use fav_models::get_selected_fn;

pub fn router() -> Router {
    Router::new()
        .route(
            "/prompt-engineering",
            get(prompt_engineering::get_configs_handler)
                .post(prompt_engineering::add_new_config)
                .delete(prompt_engineering::delete_config)
                .patch(prompt_engineering::update_config),
        )
        .route(
            "/api-config",
            get(api_configs::get_configs_handler)
                .post(api_configs::add_new_config)
                .delete(api_configs::delete_config)
                .patch(api_configs::update_config),
        )
        .route("/fav-model/:id", get(fav_model::get_handler))
        .route(
            "/fav-models",
            get(fav_models::get_handler).put(fav_models::update_handler),
        )
}
