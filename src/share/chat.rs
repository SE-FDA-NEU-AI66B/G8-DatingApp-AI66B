use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct ConversationCard {
    pub conversation_id: i64,
    pub user_id: Uuid,
    pub display_name: String,
    pub last_message: Option<String>,
    pub last_message_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct ChatMessage {
    pub id: i64,
    pub conversation_id: i64,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub body: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendMessage {
    pub body: String,
}
