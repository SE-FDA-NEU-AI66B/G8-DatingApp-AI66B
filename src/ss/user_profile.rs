use super::auth;
use crate::lib::share::{register::MessageBody, user_profile::UserProfile};
use actix_web::{get, http::StatusCode, put, web, HttpRequest, HttpResponse};
use sqlx::PgPool;

fn error(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status).json(MessageBody {
        message: message.to_owned(),
    })
}

fn db_error(context: &str, e: sqlx::Error) -> HttpResponse {
    eprintln!("{context}: {e}");
    error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong, please try again",
    )
}

#[get("/api/user-profile")]
pub async fn get_user_profile(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let user = match auth::require_user(&request, pool.get_ref()).await {
        Ok(user) => user,
        Err(response) => return response,
    };
    let result = sqlx::query_as::<_, UserProfile>(
        "SELECT COALESCE(p.display_name, u.username, split_part(u.email, '@', 1)) AS display_name,
                COALESCE(p.bio, '') AS bio, p.cohort,
                COALESCE(p.interests, '') AS interests
         FROM users u LEFT JOIN user_profiles p ON p.user_id = u.id WHERE u.id = $1",
    )
    .bind(user.id)
    .fetch_optional(pool.get_ref())
    .await;
    match result {
        Ok(Some(profile)) => HttpResponse::Ok().json(profile),
        Ok(None) => error(StatusCode::NOT_FOUND, "Account not found"),
        Err(e) => db_error("load user profile", e),
    }
}

#[put("/api/user-profile")]
pub async fn save_user_profile(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<UserProfile>,
) -> HttpResponse {
    let user = match auth::require_user(&request, pool.get_ref()).await {
        Ok(user) => user,
        Err(response) => return response,
    };
    let profile = UserProfile {
        display_name: body.display_name.trim().to_owned(),
        bio: body.bio.trim().to_owned(),
        cohort: body
            .cohort
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_owned),
        interests: body.interests.trim().to_owned(),
    };
    if profile.display_name.is_empty()
        || profile.display_name.chars().count() > 60
        || profile.bio.chars().count() > 500
        || profile
            .cohort
            .as_ref()
            .is_some_and(|value| value.chars().count() > 24)
        || profile.interests.chars().count() > 300
    {
        return error(
            StatusCode::BAD_REQUEST,
            "Display name is required (up to 60 characters); bio, cohort, or interests exceed their limits.",
        );
    }
    let result = sqlx::query_as::<_, UserProfile>(
        "INSERT INTO user_profiles (user_id, display_name, bio, cohort, interests, updated_at)
         VALUES ($1, $2, $3, $4, $5, now())
         ON CONFLICT (user_id) DO UPDATE SET display_name = EXCLUDED.display_name,
             bio = EXCLUDED.bio, cohort = EXCLUDED.cohort,
             interests = EXCLUDED.interests, updated_at = now()
         RETURNING display_name, bio, cohort, interests",
    )
    .bind(user.id)
    .bind(&profile.display_name)
    .bind(&profile.bio)
    .bind(&profile.cohort)
    .bind(&profile.interests)
    .fetch_one(pool.get_ref())
    .await;
    match result {
        Ok(saved) => HttpResponse::Ok().json(saved),
        Err(e) => db_error("save user profile", e),
    }
}
