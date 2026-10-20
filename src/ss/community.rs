use super::auth;
use crate::lib::share::{community::*, register::MessageBody};
use actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse};
use sqlx::PgPool;

fn error(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status).json(MessageBody {
        message: message.to_owned(),
    })
}

fn db_error(context: &str, e: sqlx::Error) -> HttpResponse {
    let missing_table = matches!(
        &e,
        sqlx::Error::Database(database_error)
            if database_error.code().as_deref() == Some("42P01")
    );
    eprintln!("{context}: {e}");
    if missing_table {
        return error(
            StatusCode::INTERNAL_SERVER_ERROR,
            "The community database tables are missing. Recreate the development database from database/note.py, then restart the app.",
        );
    }
    error(
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong, please try again",
    )
}

fn valid_text(value: &str, max: usize) -> bool {
    let value = value.trim();
    !value.is_empty() && value.chars().count() <= max
}

#[get("/api/threads")]
pub async fn list_threads(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Err(response) = auth::require_user(&request, pool.get_ref()).await {
        return response;
    }
    let result = sqlx::query_as::<_, ThreadCard>(
        "SELECT t.id, t.author_id,
                COALESCE(NULLIF(btrim(p.display_name), ''), NULLIF(btrim(u.username), ''), split_part(u.email, '@', 1)) AS author_name,
                t.title, t.body, t.created_at::text AS created_at,
                (SELECT count(*) FROM thread_comments c WHERE c.thread_id = t.id) AS comments_count
         FROM threads t
         JOIN users u ON u.id = t.author_id
         LEFT JOIN user_profiles p ON p.user_id = u.id
         ORDER BY t.created_at DESC, t.id DESC
         LIMIT 100",
    )
    .fetch_all(pool.get_ref())
    .await;
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => db_error("list threads", e),
    }
}

#[post("/api/threads")]
pub async fn create_thread(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateThread>,
) -> HttpResponse {
    let user = match auth::require_user(&request, pool.get_ref()).await {
        Ok(user) => user,
        Err(response) => return response,
    };
    let title = body.title.trim();
    let content = body.body.trim();
    if !valid_text(title, 120) || !valid_text(content, 3000) {
        return error(
            StatusCode::BAD_REQUEST,
            "Title must be 1 to 120 characters and body must be 1 to 3000 characters.",
        );
    }
    let result = sqlx::query_as::<_, ThreadCard>(
        "INSERT INTO threads (author_id, title, body)
         SELECT u.id, $2, $3 FROM users u WHERE u.id = $1
         RETURNING id, author_id,
                   (SELECT COALESCE(NULLIF(btrim(p.display_name), ''), NULLIF(btrim(u.username), ''), split_part(u.email, '@', 1))
                    FROM users u LEFT JOIN user_profiles p ON p.user_id = u.id WHERE u.id = threads.author_id) AS author_name,
                   title, body, created_at::text AS created_at, 0::bigint AS comments_count",
    )
    .bind(user.id)
    .bind(title)
    .bind(content)
    .fetch_one(pool.get_ref())
    .await;
    match result {
        Ok(thread) => HttpResponse::Created().json(thread),
        Err(e) => db_error("create thread", e),
    }
}

#[get("/api/threads/{id}/comments")]
pub async fn list_thread_comments(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    if let Err(response) = auth::require_user(&request, pool.get_ref()).await {
        return response;
    }
    let thread_id = path.into_inner();
    let exists =
        match sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1)")
            .bind(thread_id)
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(exists) => exists,
            Err(e) => return db_error("check thread for comments", e),
        };
    if !exists {
        return error(StatusCode::NOT_FOUND, "Thread not found");
    }
    let result = sqlx::query_as::<_, ThreadComment>(
        "SELECT c.id, c.thread_id, c.author_id,
                COALESCE(NULLIF(btrim(p.display_name), ''), NULLIF(btrim(u.username), ''), split_part(u.email, '@', 1)) AS author_name,
                c.body, c.created_at::text AS created_at
         FROM thread_comments c JOIN users u ON u.id = c.author_id
         LEFT JOIN user_profiles p ON p.user_id = u.id
         WHERE c.thread_id = $1
         ORDER BY c.created_at ASC, c.id ASC",
    )
    .bind(thread_id)
    .fetch_all(pool.get_ref())
    .await;
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => db_error("list thread comments", e),
    }
}

#[post("/api/threads/{id}/comments")]
pub async fn create_thread_comment(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
    body: web::Json<CreateThreadComment>,
) -> HttpResponse {
    let user = match auth::require_user(&request, pool.get_ref()).await {
        Ok(user) => user,
        Err(response) => return response,
    };
    let thread_id = path.into_inner();
    let content = body.body.trim();
    if !valid_text(content, 2000) {
        return error(
            StatusCode::BAD_REQUEST,
            "Comment body must be 1 to 2000 characters.",
        );
    }
    let exists =
        match sqlx::query_scalar::<_, bool>("SELECT EXISTS(SELECT 1 FROM threads WHERE id = $1)")
            .bind(thread_id)
            .fetch_one(pool.get_ref())
            .await
        {
            Ok(exists) => exists,
            Err(e) => return db_error("check thread for comment", e),
        };
    if !exists {
        return error(StatusCode::NOT_FOUND, "Thread not found");
    }
    let result = sqlx::query_as::<_, ThreadComment>(
        "INSERT INTO thread_comments (thread_id, author_id, body)
         SELECT $1, u.id, $3 FROM users u WHERE u.id = $2
         RETURNING id, thread_id, author_id,
                   (SELECT COALESCE(NULLIF(btrim(p.display_name), ''), NULLIF(btrim(u.username), ''), split_part(u.email, '@', 1))
                    FROM users u LEFT JOIN user_profiles p ON p.user_id = u.id WHERE u.id = thread_comments.author_id) AS author_name,
                   body, created_at::text AS created_at",
    )
    .bind(thread_id)
    .bind(user.id)
    .bind(content)
    .fetch_one(pool.get_ref())
    .await;
    match result {
        Ok(comment) => HttpResponse::Created().json(comment),
        Err(e) => db_error("create thread comment", e),
    }
}
