use serde::{Deserialize, Serialize};

/// Thông báo lỗi chính xác theo acceptance criteria.
pub const EMAIL_ERROR: &str = "Please use a valid @neu.edu.vn email address";
pub const NEU_DOMAIN: &str = "neu.edu.vn";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterReq {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageBody {
    pub message: String,
}

pub fn normalize_username(raw: &str) -> Result<String, &'static str> {
    let username = raw.trim().to_lowercase();
    let valid = (3..=32).contains(&username.len())
        && username.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || b"._-".contains(&byte)
        })
        && username
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphanumeric)
        && username
            .as_bytes()
            .last()
            .is_some_and(u8::is_ascii_alphanumeric);
    valid.then_some(username).ok_or("Username must be 3 to 32 characters and use letters, numbers, dots, underscores, or hyphens.")
}

/// Trim + lowercase, then accept `@neu.edu.vn` or any subdomain of it.
/// Trả về email đã chuẩn hóa, hoặc `EMAIL_ERROR`.
pub fn normalize_email(raw: &str) -> Result<String, &'static str> {
    let email = raw.trim().to_lowercase();

    let Some((local, domain)) = email.split_once('@') else {
        return Err(EMAIL_ERROR);
    };

    let local_ok = !local.is_empty()
        && local.len() <= 64
        && local
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '_' | '+' | '-'));

    let domain_ok = domain == NEU_DOMAIN
        || domain
            .strip_suffix(&format!(".{NEU_DOMAIN}"))
            .is_some_and(|subdomains| {
                !subdomains.is_empty()
                    && subdomains.split('.').all(|label| {
                        !label.is_empty()
                            && label.len() <= 63
                            && label.as_bytes()[0].is_ascii_alphanumeric()
                            && label.as_bytes()[label.len() - 1].is_ascii_alphanumeric()
                            && label
                                .bytes()
                                .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
                    })
            });

    if local_ok && domain_ok {
        Ok(email)
    } else {
        Err(EMAIL_ERROR)
    }
}
