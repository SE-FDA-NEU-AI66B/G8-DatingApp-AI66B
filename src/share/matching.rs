use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const DAILY_LIMIT: i64 = 20;
pub const DAILY_LIMIT_MESSAGE: &str = "Daily limit of 20 requests reached";
pub const ALREADY_SENT_MESSAGE: &str = "Request already sent";

/// Fallback display name before a user creates a profile.
pub fn display_name(user_id: Uuid) -> String {
    format!("Student #{}", &user_id.simple().to_string()[..8])
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct StudyDateCard {
    pub id: i64,
    pub creator_id: Uuid,
    pub starts_at: String,
    pub ends_at: String,
    pub is_mine: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct CreateStudyDateRequest {
    pub starts_at: String,
    pub duration_min: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Relation {
    None,
    Pending,
    Incoming,
    Matched,
    #[serde(rename = "self")]
    SelfProfile,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProfileView {
    pub user_id: Uuid,
    pub relation: Relation,
    pub display_name: String,
    pub bio: String,
    pub cohort: Option<String>,
    pub interests: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct ProfileSearchResult {
    pub user_id: Uuid,
    pub display_name: String,
    pub cohort: Option<String>,
    pub bio: String,
    pub interests: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendMatchRequest {
    pub to_user_id: Uuid,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct IncomingRequest {
    pub id: i64,
    pub sender_id: Uuid,
    pub created_at: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct MatchedUser {
    pub user_id: Uuid,
    pub conversation_id: i64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn limit_message_matches_acceptance_criteria() {
        assert_eq!(DAILY_LIMIT, 20);
        assert_eq!(DAILY_LIMIT_MESSAGE, "Daily limit of 20 requests reached");
    }

    #[test]
    fn display_name_is_stable() {
        assert_eq!(
            display_name(Uuid::parse_str("00000000-0000-0000-0000-000000000007").unwrap()),
            "Student #00000000"
        );
    }
}
