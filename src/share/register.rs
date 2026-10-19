use serde::{Deserialize, Serialize};

/// Thông báo lỗi chính xác theo acceptance criteria.
pub const EMAIL_ERROR: &str = "Please use a valid @neu.edu.vn email address";
pub const NEU_DOMAIN: &str = "neu.edu.vn";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterReq {
    pub email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MessageBody {
    pub message: String,
}

/// Trim + lowercase, rồi kiểm tra đúng dạng `<local>@neu.edu.vn`.
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

    if local_ok && domain == NEU_DOMAIN {
        Ok(email)
    } else {
        Err(EMAIL_ERROR)
    }
}
