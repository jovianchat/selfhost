use axum::{extract::State, Json};

use crate::http::AppState;
use crate::models::chat::ChatDetails;
use crate::Result;

// #[axum::debug_handler]
pub async fn create_new_chat(
    State(state): State<AppState>,
    title: String,
) -> Result<Json<ChatDetails>> {
    let pool = &state.pg_pool;

    let details = sqlx::query_as!(
        ChatDetails,
        r#"INSERT INTO ai.chats (title) VALUES ($1) RETURNING *"#,
        title
    )
    .fetch_one(pool)
    .await?;

    // Moka cache invalidate
    let _ = state.cache.chat_history.invalidate(&state.user).await;

    Ok(Json(details))
}
