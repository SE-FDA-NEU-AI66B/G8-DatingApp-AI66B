use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct ThreadCard {
    pub id: i64,
    pub author_id: Uuid,
    pub author_name: String,
    pub title: String,
    pub body: String,
    pub created_at: String,
    pub comments_count: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateThread {
    pub title: String,
    pub body: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct ThreadComment {
    pub id: i64,
    pub thread_id: i64,
    pub author_id: Uuid,
    pub author_name: String,
    pub body: String,
    pub created_at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateThreadComment {
    pub body: String,
}
