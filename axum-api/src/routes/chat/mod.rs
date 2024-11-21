use axum::{
    routing::{get, patch, post},
    Router,
};

use crate::http::AppState;

mod chat_handlers;
mod chat_history;
mod new_chat;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/history", get(chat_history::get_chat_history))
        .route("/new", post(new_chat::create_new_chat))
        .route(
            "/:chat_id",
            get(chat_handlers::get_chat)
                .post(chat_handlers::append_chat)
                .delete(chat_handlers::delete_chat),
        )
        .route("/:chat_id/starred", patch(chat_handlers::star_chat))
        .route("/:chat_id/rename", patch(chat_handlers::rename_chat))
}
