use super::auth;
use crate::lib::share::{chat::*, register::MessageBody};
use actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse};
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

async fn authorize(request: &HttpRequest, pool: &PgPool, id: i64) -> Result<bool, HttpResponse> {
    let me = auth::require_user(request, pool).await?;
    sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM conversations WHERE id = $1 AND (user_low = $2 OR user_high = $2))")
        .bind(id).bind(me.id).fetch_one(pool).await
        .map_err(|e| db_error("authorize conversation", e))
}

#[get("/api/conversations")]
pub async fn conversations(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let me = match auth::require_user(&request, pool.get_ref()).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    let result = sqlx::query_as::<_, ConversationCard>(
        "SELECT c.id AS conversation_id, other.id AS user_id,
                COALESCE(other.username, split_part(other.email, '@', 1)) AS display_name,
                latest.body AS last_message, latest.created_at::text AS last_message_at
         FROM conversations c
         JOIN users other ON other.id = CASE WHEN c.user_low = $1 THEN c.user_high ELSE c.user_low END
         LEFT JOIN LATERAL (SELECT body, created_at FROM chat_messages WHERE conversation_id = c.id ORDER BY id DESC LIMIT 1) latest ON TRUE
         WHERE c.user_low = $1 OR c.user_high = $1
         ORDER BY COALESCE(latest.created_at, c.created_at) DESC, c.id DESC")
        .bind(me.id).fetch_all(pool.get_ref()).await;
    match result {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => db_error("list conversations", e),
    }
}

#[get("/api/conversations/{id}/messages")]
pub async fn messages(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let id = path.into_inner();
    match authorize(&request, pool.get_ref(), id).await {
        Err(r) => r,
        Ok(false) => error(StatusCode::NOT_FOUND, "Conversation not found"),
        Ok(true) => match sqlx::query_as::<_, ChatMessage>(
            "SELECT m.id, m.conversation_id, m.sender_id,
                    COALESCE(u.username, split_part(u.email, '@', 1)) AS sender_name,
                    m.body, m.created_at::text AS created_at
             FROM chat_messages m JOIN users u ON u.id = m.sender_id
             WHERE m.conversation_id = $1 ORDER BY m.id ASC LIMIT 500",
        )
        .bind(id)
        .fetch_all(pool.get_ref())
        .await
        {
            Ok(rows) => HttpResponse::Ok().json(rows),
            Err(e) => db_error("load messages", e),
        },
    }
}

#[post("/api/conversations/{id}/messages")]
pub async fn send_message(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
    body: web::Json<SendMessage>,
) -> HttpResponse {
    let id = path.into_inner();
    let me = match auth::require_user(&request, pool.get_ref()).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    let body_text = body.body.trim();
    if body_text.is_empty() || body_text.chars().count() > 2000 {
        return error(
            StatusCode::BAD_REQUEST,
            "Messages must contain 1 to 2000 characters",
        );
    }
    let result = sqlx::query_as::<_, ChatMessage>(
        "INSERT INTO chat_messages (conversation_id, sender_id, body)
         SELECT c.id, $2, $3 FROM conversations c
         WHERE c.id = $1 AND (c.user_low = $2 OR c.user_high = $2)
         RETURNING id, conversation_id, sender_id, $4::text AS sender_name, body, created_at::text AS created_at")
        .bind(id).bind(me.id).bind(body_text).bind(me.username.unwrap_or_else(|| me.email.split('@').next().unwrap_or("student").to_owned()))
        .fetch_optional(pool.get_ref()).await;
    match result {
        Ok(Some(row)) => HttpResponse::Created().json(row),
        Ok(None) => error(StatusCode::NOT_FOUND, "Conversation not found"),
        Err(e) => db_error("send message", e),
    }
}
