use std::sync::Arc;

use axum::{extract::State, Json};
use chrono::Days;

use crate::http::AppState;
use crate::models::chat::{ChatHistory, HistoryChatDetails, UnstarredGroupedHistory};
use crate::Result;
pub struct Chat {
    id: uuid::Uuid,
    title: String,
    time_period: Option<String>,
}
pub async fn get_chat_history(State(state): State<AppState>) -> Result<Json<Arc<ChatHistory>>> {
    // Moka cache get
    let history_cache = state.cache.chat_history;
    let chat_history = history_cache.get(&state.user).await;
    match chat_history {
        Some(v) => Ok(Json(v)),
        None => {
            let unstarred_history = unstarred_history(&state.pg_pool).await?;
            let starred_history = starred_history(&state.pg_pool).await?;
            let all_chats = Arc::new(ChatHistory {
                unstarred_history,
                starred_history,
            });

            // Moka cache set
            let _ = history_cache.insert(state.user, all_chats.clone()).await;
            Ok(Json(all_chats))
        }
    }
}

async fn unstarred_history(pool: &sqlx::PgPool) -> Result<Vec<UnstarredGroupedHistory>> {
    let today_yesterday = sqlx::types::chrono::Utc::now()
        .checked_sub_days(Days::new(1))
        .unwrap();
    let past_7_days = sqlx::types::chrono::Utc::now()
        .checked_sub_days(Days::new(7))
        .unwrap();
    let past_30_days = sqlx::types::chrono::Utc::now()
        .checked_sub_days(Days::new(30))
        .unwrap();
    let chats = sqlx::query_as!(
        Chat,
        r#"SELECT CASE
                    WHEN updated_at >= $1 THEN 'Recent'
                    WHEN updated_at >= $2 AND updated_at < $1 THEN 'Past 7 days'
                    WHEN updated_at >= $2 AND updated_at < $3 THEN 'Past 30 days'
                    ELSE 'Older'
                END AS time_period,
                title,
                id
                FROM ai.chats
                WHERE starred = FALSE
                ORDER BY updated_at DESC
                    "#,
        today_yesterday,
        past_7_days,
        past_30_days
    )
    .fetch_all(pool)
    .await?;

    let grouped_history: Vec<UnstarredGroupedHistory> =
        chats.into_iter().fold(Vec::new(), |mut acc, chat| {
            if let Some(existing_time_period) = acc
                .iter_mut()
                .find(|h| h.time_period == chat.time_period.clone().unwrap())
            {
                existing_time_period.period_chats.push(HistoryChatDetails {
                    id: chat.id,
                    title: chat.title,
                });
            } else {
                acc.push(UnstarredGroupedHistory {
                    time_period: chat.time_period.unwrap(),
                    period_chats: vec![HistoryChatDetails {
                        id: chat.id,
                        title: chat.title,
                    }],
                });
            }
            acc
        });

    Ok(grouped_history)
}

async fn starred_history(pool: &sqlx::PgPool) -> Result<Vec<HistoryChatDetails>> {
    let chats = sqlx::query_as!(
        HistoryChatDetails,
        r#"SELECT title, id FROM ai.chats WHERE starred = TRUE ORDER BY updated_at DESC"#,
    )
    .fetch_all(pool)
    .await?;

    Ok(chats)
}
