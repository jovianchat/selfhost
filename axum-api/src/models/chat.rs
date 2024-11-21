use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

use super::cache::CacheState;
use crate::Result;

// Chat History
#[derive(Serialize, Deserialize, Clone)]
pub struct HistoryChatDetails {
    pub id: uuid::Uuid,
    pub title: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct UnstarredGroupedHistory {
    pub time_period: String,
    pub period_chats: Vec<HistoryChatDetails>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ChatHistory {
    pub unstarred_history: Vec<UnstarredGroupedHistory>,
    pub starred_history: Vec<HistoryChatDetails>,
}

// Each Chat
#[derive(Serialize, Deserialize, Clone)]
pub struct ChatDetails {
    pub id: uuid::Uuid,
    pub title: String,
    pub starred: bool,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub user_query: String,
    pub assistant_response: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Chat {
    pub details: ChatDetails,
    pub messages: Vec<ChatMessage>,
}

impl Chat {
    pub async fn fetch_chat(
        chat_id: &uuid::Uuid,
        cache: &CacheState,
        pool: &sqlx::PgPool,
    ) -> Result<Arc<RwLock<Self>>> {
        let chat_cache = cache.chat.clone();
        let chat = chat_cache.get(chat_id).await;
        match chat {
            Some(v) => Ok(v),
            None => {
                let rows = sqlx::query_as!(
                ChatMessage,
                r#"SELECT user_query, assistant_response FROM ai.chat_messages WHERE chat_id = $1 ORDER BY created_at ASC"#,
                chat_id
            )
            .fetch_all(pool)
            .await?;
                let chat_details = sqlx::query_as!(
                    ChatDetails,
                    r#"SELECT * FROM ai.chats WHERE id = $1"#,
                    chat_id
                )
                .fetch_one(pool)
                .await?;

                let chat = Arc::new(RwLock::new(Chat {
                    details: chat_details,
                    messages: rows,
                }));

                // Moka cache set
                let _ = chat_cache.insert(*chat_id, chat.clone()).await;

                Ok(chat)
            }
        }
    }
}
