use crate::lib::share::{
    auth::{AuthError, LoginRequest},
    register::normalize_email,
};
use actix_web::{
    cookie::{Cookie, SameSite},
    web, HttpRequest, HttpResponse,
};
use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::{FromRow, PgPool};

const SESSION_COOKIE: &str = "datingapp_session";

#[derive(Debug, Clone, FromRow)]
pub struct AuthenticatedUser {
    pub role: String,
}

#[derive(Debug, FromRow)]
struct LoginUser {
    id: i64,
    otp_code: Option<String>,
    otp_expires_at: Option<time::OffsetDateTime>,
}

fn auth_error(status: actix_web::http::StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status).json(AuthError {
        message: message.to_string(),
    })
}

fn hash_token(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}

fn new_token() -> String {
    let mut bytes = [0_u8; 32];
    rand::rng().fill(&mut bytes);
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

pub async fn login(pool: web::Data<PgPool>, body: web::Json<LoginRequest>) -> HttpResponse {
    let email = match normalize_email(&body.email) {
        Ok(email) => email,
        Err(message) => return auth_error(actix_web::http::StatusCode::BAD_REQUEST, message),
    };

    let user = sqlx::query_as::<_, LoginUser>(
        "SELECT id, role, otp_code, otp_expires_at
         FROM app_user
         WHERE email = $1",
    )
    .bind(email)
    .fetch_optional(pool.get_ref())
    .await;

    let Some(user) = (match user {
        Ok(user) => user,
        Err(error) => {
            eprintln!("login lookup failed: {error}");
            return auth_error(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication service is temporarily unavailable.",
            );
        }
    }) else {
        return auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Invalid email or OTP.",
        );
    };

    let valid_otp = user.otp_code.as_deref() == Some(body.otp.trim())
        && user
            .otp_expires_at
            .is_some_and(|expires_at| expires_at > time::OffsetDateTime::now_utc());
    if !valid_otp {
        return auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Invalid email or OTP.",
        );
    }

    let token = new_token();
    let token_hash = hash_token(&token);
    let mut transaction = match pool.begin().await {
        Ok(transaction) => transaction,
        Err(error) => {
            eprintln!("login transaction failed: {error}");
            return auth_error(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Authentication service is temporarily unavailable.",
            );
        }
    };

    if let Err(error) = sqlx::query(
        "UPDATE app_user
         SET email_verified = TRUE, otp_code = NULL, otp_expires_at = NULL
         WHERE id = $1",
    )
    .bind(user.id)
    .execute(&mut *transaction)
    .await
    {
        eprintln!("login verification update failed: {error}");
        return auth_error(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication service is temporarily unavailable.",
        );
    }

    if let Err(error) = sqlx::query(
        "INSERT INTO auth_session (token_hash, user_id, expires_at)
         VALUES ($1, $2, now() + interval '8 hours')",
    )
    .bind(token_hash)
    .bind(user.id)
    .execute(&mut *transaction)
    .await
    {
        eprintln!("login session creation failed: {error}");
        return auth_error(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication service is temporarily unavailable.",
        );
    }

    if let Err(error) = transaction.commit().await {
        eprintln!("login transaction commit failed: {error}");
        return auth_error(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication service is temporarily unavailable.",
        );
    }

    HttpResponse::Ok()
        .cookie(
            Cookie::build(SESSION_COOKIE, token)
                .http_only(true)
                .secure(!cfg!(debug_assertions))
                .same_site(SameSite::Lax)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::hours(8))
                .finish(),
        )
        .finish()
}

pub async fn logout(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Some(cookie) = request.cookie(SESSION_COOKIE) {
        if let Err(error) = sqlx::query("DELETE FROM auth_session WHERE token_hash = $1")
            .bind(hash_token(cookie.value()))
            .execute(pool.get_ref())
            .await
        {
            eprintln!("logout failed: {error}");
            return auth_error(
                actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to end the session.",
            );
        }
    }

    HttpResponse::NoContent()
        .cookie(
            Cookie::build(SESSION_COOKIE, "")
                .http_only(true)
                .secure(!cfg!(debug_assertions))
                .same_site(SameSite::Lax)
                .path("/")
                .max_age(actix_web::cookie::time::Duration::ZERO)
                .finish(),
        )
        .finish()
}

pub async fn require_admin(
    request: &HttpRequest,
    pool: &PgPool,
) -> Result<AuthenticatedUser, HttpResponse> {
    let Some(cookie) = request.cookie(SESSION_COOKIE) else {
        return Err(auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Authentication is required.",
        ));
    };

    let user = sqlx::query_as::<_, AuthenticatedUser>(
        "SELECT u.role
         FROM auth_session s
         JOIN app_user u ON u.id = s.user_id
         WHERE s.token_hash = $1
           AND s.expires_at > now()",
    )
    .bind(hash_token(cookie.value()))
    .fetch_optional(pool)
    .await
    .map_err(|error| {
        eprintln!("admin authorization lookup failed: {error}");
        auth_error(
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            "Authentication service is temporarily unavailable.",
        )
    })?;

    let Some(user) = user else {
        return Err(auth_error(
            actix_web::http::StatusCode::UNAUTHORIZED,
            "Authentication is required.",
        ));
    };

    if user.role != "admin" {
        return Err(auth_error(
            actix_web::http::StatusCode::FORBIDDEN,
            "Administrator access is required.",
        ));
    }

    Ok(user)
}

pub async fn configure_admin(pool: &PgPool, email: Option<&str>) -> Result<(), sqlx::Error> {
    let Some(email) = email else {
        return Ok(());
    };

    sqlx::query("UPDATE app_user SET role = 'admin' WHERE email = $1")
        .bind(email.trim().to_lowercase())
        .execute(pool)
        .await?;
    Ok(())
}
