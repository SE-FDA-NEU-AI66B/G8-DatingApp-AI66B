use super::auth;
use crate::lib::share::{matching::*, register::MessageBody};
use actix_web::{get, http::StatusCode, post, web, HttpRequest, HttpResponse};
use sqlx::PgPool;

fn reply(status: StatusCode, message: &str) -> HttpResponse {
    HttpResponse::build(status).json(MessageBody { message: message.to_string() })
}

fn db_error(context: &str, error: sqlx::Error) -> HttpResponse {
    eprintln!("{context}: {error}");
    reply(StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong, please try again")
}

macro_rules! current_user {
    ($req:expr, $pool:expr) => {
        match auth::require_user(&$req, $pool.get_ref()).await {
            Ok(user) => user,
            Err(response) => return response,
        }
    };
}

const NOT_PENDING: &str = "Request not found or already handled";

// ---------- US04 ----------

const RELATION_QUERY: &str = "SELECT CASE
    WHEN u.id = $1 THEN 'self'
    WHEN EXISTS (SELECT 1 FROM match_request m WHERE m.status = 'accepted'
                 AND ((m.sender_id = $1 AND m.receiver_id = u.id)
                   OR (m.sender_id = u.id AND m.receiver_id = $1))) THEN 'matched'
    WHEN EXISTS (SELECT 1 FROM match_request m
                 WHERE m.sender_id = $1 AND m.receiver_id = u.id) THEN 'pending'
    WHEN EXISTS (SELECT 1 FROM match_request m
                 WHERE m.sender_id = u.id AND m.receiver_id = $1
                   AND m.status = 'pending') THEN 'incoming'
    ELSE 'none' END
FROM app_user u
WHERE u.id = $2 AND u.email_verified";

fn relation_from_db(value: &str) -> Relation {
    match value {
        "self" => Relation::SelfProfile,
        "matched" => Relation::Matched,
        "pending" => Relation::Pending,
        "incoming" => Relation::Incoming,
        _ => Relation::None,
    }
}

#[get("/api/profiles/{id}")]
pub async fn get_profile(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let me = current_user!(request, pool);
    let target = path.into_inner();
    match sqlx::query_scalar::<_, String>(RELATION_QUERY)
        .bind(me.id)
        .bind(target)
        .fetch_optional(pool.get_ref())
        .await
    {
        Ok(Some(rel)) => HttpResponse::Ok().json(ProfileView {
            user_id: target,
            relation: relation_from_db(&rel),
        }),
        Ok(None) => reply(StatusCode::NOT_FOUND, "Profile not found"),
        Err(e) => db_error("get profile", e),
    }
}

async fn try_send(pool: &PgPool, me: i64, to: i64) -> Result<HttpResponse, sqlx::Error> {
    if me == to {
        return Ok(reply(StatusCode::BAD_REQUEST, "You cannot send a request to yourself"));
    }
    let mut tx = pool.begin().await?;
    // Tuần tự hóa các request của cùng một người gửi để giới hạn 20/ngày không bị vượt khi gửi song song.
    sqlx::query("SELECT pg_advisory_xact_lock($1)")
        .bind(me)
        .execute(&mut *tx)
        .await?;

    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM app_user WHERE id = $1 AND email_verified)",
    )
    .bind(to)
    .fetch_one(&mut *tx)
    .await?;
    if !exists {
        return Ok(reply(StatusCode::NOT_FOUND, "Profile not found"));
    }

    let existing: Vec<(i64, String)> = sqlx::query_as(
        "SELECT sender_id, status FROM match_request
         WHERE (sender_id = $1 AND receiver_id = $2)
            OR (sender_id = $2 AND receiver_id = $1)",
    )
    .bind(me)
    .bind(to)
    .fetch_all(&mut *tx)
    .await?;
    // BR4: mỗi cặp chỉ một quyết định → gửi lại (kể cả đã bị từ chối) bị bỏ qua.
    if existing.iter().any(|(sender, _)| *sender == me) {
        return Ok(reply(StatusCode::CONFLICT, ALREADY_SENT_MESSAGE));
    }
    if existing.iter().any(|(sender, status)| *sender == to && status != "declined") {
        return Ok(reply(StatusCode::CONFLICT, "This user has already sent you a request"));
    }

    let sent_today: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM match_request
         WHERE sender_id = $1
           AND created_at >= (date_trunc('day', now() AT TIME ZONE 'Asia/Ho_Chi_Minh')
                              AT TIME ZONE 'Asia/Ho_Chi_Minh')",
    )
    .bind(me)
    .fetch_one(&mut *tx)
    .await?;
    if sent_today >= DAILY_LIMIT {
        return Ok(reply(StatusCode::TOO_MANY_REQUESTS, DAILY_LIMIT_MESSAGE));
    }

    sqlx::query("INSERT INTO match_request (sender_id, receiver_id) VALUES ($1, $2)")
        .bind(me)
        .bind(to)
        .execute(&mut *tx)
        .await?;
    tx.commit().await?;
    Ok(reply(StatusCode::CREATED, "Match request sent"))
}

#[post("/api/match-requests")]
pub async fn send_request(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<SendMatchRequest>,
) -> HttpResponse {
    let me = current_user!(request, pool);
    match try_send(pool.get_ref(), me.id, body.to_user_id).await {
        Ok(response) => response,
        Err(sqlx::Error::Database(db)) if db.is_unique_violation() => {
            reply(StatusCode::CONFLICT, ALREADY_SENT_MESSAGE)
        }
        Err(e) => db_error("send match request", e),
    }
}

// ---------- US05 ----------

#[get("/api/match-requests/incoming")]
pub async fn incoming_requests(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let me = current_user!(request, pool);
    match sqlx::query_as::<_, IncomingRequest>(
        "SELECT id, sender_id,
                to_char(created_at AT TIME ZONE 'Asia/Ho_Chi_Minh', 'YYYY-MM-DD HH24:MI') AS created_at
         FROM match_request
         WHERE receiver_id = $1 AND status = 'pending'
         ORDER BY match_request.created_at DESC, id DESC",
    )
    .bind(me.id)
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => db_error("incoming requests", e),
    }
}

#[get("/api/matches")]
pub async fn matched_users(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let me = current_user!(request, pool);
    match sqlx::query_as::<_, MatchedUser>(
        "SELECT CASE WHEN user_low = $1 THEN user_high ELSE user_low END AS user_id,
                id AS chat_room_id
         FROM chat_room
         WHERE user_low = $1 OR user_high = $1
         ORDER BY created_at DESC, id DESC",
    )
    .bind(me.id)
    .fetch_all(pool.get_ref())
    .await
    {
        Ok(rows) => HttpResponse::Ok().json(rows),
        Err(e) => db_error("matched users", e),
    }
}

async fn try_accept(pool: &PgPool, me: i64, id: i64) -> Result<HttpResponse, sqlx::Error> {
    let mut tx = pool.begin().await?;
    let sender: Option<i64> = sqlx::query_scalar(
        "UPDATE match_request SET status = 'accepted', responded_at = now()
         WHERE id = $1 AND receiver_id = $2 AND status = 'pending'
         RETURNING sender_id",
    )
    .bind(id)
    .bind(me)
    .fetch_optional(&mut *tx)
    .await?;
    let Some(sender) = sender else {
        return Ok(reply(StatusCode::NOT_FOUND, NOT_PENDING));
    };
    let (low, high) = if sender < me { (sender, me) } else { (me, sender) };
    let room: i64 = sqlx::query_scalar(
        "INSERT INTO chat_room (user_low, user_high, match_request_id)
         VALUES ($1, $2, $3)
         ON CONFLICT (user_low, user_high)
         DO UPDATE SET match_request_id = chat_room.match_request_id
         RETURNING id",
    )
    .bind(low)
    .bind(high)
    .bind(id)
    .fetch_one(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(HttpResponse::Ok().json(MatchedUser { user_id: sender, chat_room_id: room }))
}

#[post("/api/match-requests/{id}/accept")]
pub async fn accept_request(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let me = current_user!(request, pool);
    match try_accept(pool.get_ref(), me.id, path.into_inner()).await {
        Ok(response) => response,
        Err(e) => db_error("accept match request", e),
    }
}

#[post("/api/match-requests/{id}/decline")]
pub async fn decline_request(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    path: web::Path<i64>,
) -> HttpResponse {
    let me = current_user!(request, pool);
    match sqlx::query_scalar::<_, i64>(
        "UPDATE match_request SET status = 'declined', responded_at = now()
         WHERE id = $1 AND receiver_id = $2 AND status = 'pending'
         RETURNING id",
    )
    .bind(path.into_inner())
    .bind(me.id)
    .fetch_optional(pool.get_ref())
    .await
    {
        Ok(Some(_)) => reply(StatusCode::OK, "Request declined"),
        Ok(None) => reply(StatusCode::NOT_FOUND, NOT_PENDING),
        Err(e) => db_error("decline match request", e),
    }
}
