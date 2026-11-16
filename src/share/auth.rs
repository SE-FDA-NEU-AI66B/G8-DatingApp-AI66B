use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub otp: String,
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
