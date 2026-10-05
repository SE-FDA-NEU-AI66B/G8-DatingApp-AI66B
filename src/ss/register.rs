use crate::lib::share::register::{normalize_email, MessageBody, RegisterReq};
use actix_web::{post, web, HttpResponse};
use sqlx::PgPool;

fn msg(m: &str) -> MessageBody {
    MessageBody { message: m.to_string() }
}

/// OTP 6 chữ số, giữ cả số 0 đứng đầu ("004821").
pub fn generate_otp() -> String {
    use rand::Rng;
    format!("{:06}", rand::rng().random_range(0..1_000_000u32))
}

/// STUB (Sprint 2): chưa gửi mail thật, chỉ in ra log server.
fn send_otp_email_stub(email: &str, otp: &str) {
    println!("[stub-mail] to={email} otp={otp}");
}

#[post("/api/register")]
pub async fn register(pool: web::Data<PgPool>, body: web::Json<RegisterReq>) -> HttpResponse {
    // 1. Validate phía server (không tin client)
    let email = match normalize_email(&body.email) {
        Ok(e) => e,
        Err(m) => return HttpResponse::BadRequest().json(msg(m)),
    };

    // 2. Insert; UNIQUE sẽ chặn email trùng
    let otp = generate_otp();
    let res = sqlx::query(
        "INSERT INTO app_user (email, otp_code, otp_expires_at)
         VALUES ($1, $2, now() + interval '10 minutes')",
    )
    .bind(&email)
    .bind(&otp)
    .execute(pool.get_ref())
    .await;

    match res {
        Ok(_) => {
            send_otp_email_stub(&email, &otp);
            HttpResponse::Created().json(msg("Account created. We sent a 6-digit OTP to your email."))
        }
        // 3. Trùng email -> 409, không có dòng mới
        Err(sqlx::Error::Database(db)) if db.is_unique_violation() => {
            HttpResponse::Conflict().json(msg("This email is already registered"))
        }
        Err(e) => {
            eprintln!("register failed: {e}");
            HttpResponse::InternalServerError().json(msg("Something went wrong, please try again"))
        }
    }
}
