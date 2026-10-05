use super::auth;
use crate::lib::share::{matching::StudyDateCard, register::MessageBody};
use actix_web::{get, web, HttpRequest, HttpResponse};
use sqlx::PgPool;

// "Active" dùng cùng định nghĩa với dashboard admin: giao với [now, now+24h].
// Qualify `study_date_request.starts_at` trong ORDER BY để không sort theo alias dạng text.
pub const FEED_QUERY: &str = "SELECT
    id,
    creator_id,
    to_char(starts_at AT TIME ZONE 'Asia/Ho_Chi_Minh', 'YYYY-MM-DD HH24:MI') AS starts_at,
    to_char(ends_at   AT TIME ZONE 'Asia/Ho_Chi_Minh', 'YYYY-MM-DD HH24:MI') AS ends_at,
    (creator_id = $1) AS is_mine
FROM study_date_request
WHERE status = 'active'
  AND study_date_request.starts_at < now() + interval '24 hours'
  AND study_date_request.ends_at > now()
ORDER BY study_date_request.starts_at ASC, id ASC";

#[get("/api/study-dates")]
pub async fn list_study_dates(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    let me = match auth::require_user(&request, pool.get_ref()).await {
        Ok(u) => u,
        Err(r) => return r,
    };
    match sqlx::query_as::<_, StudyDateCard>(FEED_QUERY)
        .bind(me.id)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(cards) => HttpResponse::Ok().json(cards),
        Err(e) => {
            eprintln!("study date feed failed: {e}");
            HttpResponse::InternalServerError().json(MessageBody {
                message: "Something went wrong, please try again".into(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FEED_QUERY;

    #[test]
    fn feed_is_sorted_chronologically_on_the_real_column() {
        assert!(FEED_QUERY.contains("ORDER BY study_date_request.starts_at ASC"));
    }
}
