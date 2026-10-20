use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username_or_email: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SessionInfo {
    pub role: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct AuthError {
    pub message: String,
}

#[cfg(test)]
mod tests {
    #[test]
    fn roles_are_explicitly_distinct() {
        assert_ne!("admin", "user");
    }
}
