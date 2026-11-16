use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[cfg_attr(not(target_arch = "wasm32"), derive(sqlx::FromRow))]
pub struct UserProfile {
    pub display_name: String,
    pub bio: String,
    pub cohort: Option<String>,
    pub interests: String,
}
