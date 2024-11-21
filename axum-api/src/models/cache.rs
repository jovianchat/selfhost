use std::sync::{Arc, RwLock};

use moka::{future::Cache, Entry};
use uuid::Uuid;

use super::chat::{Chat, ChatHistory, ChatMessage};

#[derive(Clone)]
pub struct CacheState {
    pub chat_history: Cache<String, Arc<ChatHistory>>,
    pub chat: Cache<Uuid, Arc<RwLock<Chat>>>,
}
impl CacheState {
    pub fn new() -> Self {
        Self {
            chat_history: Cache::new(1),
            chat: Cache::new(1000),
        }
    }
}
impl Default for CacheState {
    fn default() -> Self {
        Self::new()
    }
}

pub async fn append_to_cached_chat(
    cache_state: &CacheState,
    key: &Uuid,
    value: ChatMessage,
    pool: &sqlx::PgPool,
) -> Entry<Uuid, Arc<RwLock<Chat>>> {
    let cache = cache_state.chat.clone();
    cache
        .entry_by_ref(key)
        .and_upsert_with(|maybe_entry| async {
            if let Some(entry) = maybe_entry {
                // The entry exists, append the value to the Vec.
                let v = entry.into_value();
                v.write().unwrap().messages.push(value);
                v
            } else {
                let chat = Chat::fetch_chat(key, cache_state, pool).await.unwrap();
                chat.write().unwrap().messages.push(value);
                chat
            }
        })
        .await
}
