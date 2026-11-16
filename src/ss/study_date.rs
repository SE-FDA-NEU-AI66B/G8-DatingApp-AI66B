use super::auth;
use crate::lib::share::{
    matching::{CreateStudyDateRequest, StudyDateCard},
    register::MessageBody,
};
use actix_web::{web, HttpRequest, HttpResponse};
use sqlx::PgPool;

pub const MAX_DURATION_MIN: i32 = 720;
pub const MAX_DURATION_ERROR: &str = "Maximum study date duration is 12 hours";

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

const CREATE_QUERY: &str = "INSERT INTO study_date_request
    (creator_id, starts_at, ends_at, duration_min, status)
VALUES (
    $1,
    $2::timestamp AT TIME ZONE 'Asia/Ho_Chi_Minh',
    ($2::timestamp AT TIME ZONE 'Asia/Ho_Chi_Minh') + ($3 * interval '1 minute'),
    $3,
    'active'
)
RETURNING
    id,
    creator_id,
    to_char(starts_at AT TIME ZONE 'Asia/Ho_Chi_Minh', 'YYYY-MM-DD HH24:MI') AS starts_at,
    to_char(ends_at AT TIME ZONE 'Asia/Ho_Chi_Minh', 'YYYY-MM-DD HH24:MI') AS ends_at,
    TRUE AS is_mine";

fn validate_duration(duration_min: i32) -> Result<(), &'static str> {
    if duration_min > MAX_DURATION_MIN {
        Err(MAX_DURATION_ERROR)
    } else if duration_min < 1 {
        Err("Study date duration must be at least 1 minute")
    } else {
        Ok(())
    }
}

fn duration_validation_response(duration_min: i32) -> Option<HttpResponse> {
    validate_duration(duration_min).err().map(|message| {
        HttpResponse::UnprocessableEntity().json(MessageBody {
            message: message.to_string(),
        })
    })
}

fn valid_local_datetime(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 16
        || bytes[4] != b'-'
        || bytes[7] != b'-'
        || bytes[10] != b'T'
        || bytes[13] != b':'
        || ![0..4, 5..7, 8..10, 11..13, 14..16]
            .iter()
            .all(|range| bytes[range.clone()].iter().all(u8::is_ascii_digit))
    {
        return false;
    }

    let Some(year) = value[0..4].parse::<u32>().ok() else {
        return false;
    };
    let Some(month) = value[5..7].parse::<u32>().ok() else {
        return false;
    };
    let Some(day) = value[8..10].parse::<u32>().ok() else {
        return false;
    };
    let Some(hour) = value[11..13].parse::<u32>().ok() else {
        return false;
    };
    let Some(minute) = value[14..16].parse::<u32>().ok() else {
        return false;
    };

    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year % 400 == 0 || (year % 4 == 0 && year % 100 != 0) => 29,
        2 => 28,
        _ => return false,
    };

    year >= 1 && day >= 1 && day <= days_in_month && hour < 24 && minute < 60
}

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

pub async fn create_study_date(
    request: HttpRequest,
    pool: web::Data<PgPool>,
    body: web::Json<CreateStudyDateRequest>,
) -> HttpResponse {
    let me = match auth::require_user(&request, pool.get_ref()).await {
        Ok(u) => u,
        Err(r) => return r,
    };

    if let Some(response) = duration_validation_response(body.duration_min) {
        return response;
    }
    if !valid_local_datetime(&body.starts_at) {
        return HttpResponse::UnprocessableEntity().json(MessageBody {
            message: "Choose a valid start date and time".to_string(),
        });
    }

    match sqlx::query_as::<_, StudyDateCard>(CREATE_QUERY)
        .bind(me.id)
        .bind(&body.starts_at)
        .bind(body.duration_min)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(card) => HttpResponse::Created().json(card),
        Err(error) => {
            eprintln!("study date creation failed: {error}");
            HttpResponse::InternalServerError().json(MessageBody {
                message: "Something went wrong, please try again".into(),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        duration_validation_response, valid_local_datetime, validate_duration, CREATE_QUERY,
        FEED_QUERY, MAX_DURATION_ERROR,
    };
    use actix_web::{web, App};
    use sqlx::postgres::PgPoolOptions;

    #[test]
    fn feed_is_sorted_chronologically_on_the_real_column() {
        assert!(FEED_QUERY.contains("ORDER BY study_date_request.starts_at ASC"));
    }

    #[test]
    fn three_hour_duration_is_valid() {
        assert_eq!(validate_duration(180), Ok(()));
    }

    #[test]
    fn fourteen_hour_duration_is_rejected_with_the_required_message() {
        assert_eq!(validate_duration(14 * 60), Err(MAX_DURATION_ERROR));
    }

    #[actix_web::test]
    async fn fourteen_hour_duration_returns_422_with_the_required_message() {
        assert!(duration_validation_response(180).is_none());
        let response = duration_validation_response(14 * 60)
            .expect("a duration above the limit must return an error response");
        assert_eq!(
            response.status(),
            actix_web::http::StatusCode::UNPROCESSABLE_ENTITY
        );
        let body = actix_web::body::to_bytes(response.into_body())
            .await
            .expect("error response body should be readable");
        assert!(String::from_utf8(body.to_vec())
            .expect("JSON response body should be UTF-8")
            .contains(MAX_DURATION_ERROR));
    }

    #[test]
    fn create_query_persists_duration_and_derives_end_time() {
        assert!(CREATE_QUERY.contains("duration_min"));
        assert!(CREATE_QUERY.contains(
            "($2::timestamp AT TIME ZONE 'Asia/Ho_Chi_Minh') + ($3 * interval '1 minute')"
        ));
    }

    #[test]
    fn local_datetime_validation_checks_calendar_and_time() {
        assert!(valid_local_datetime("2026-10-05T20:00"));
        assert!(!valid_local_datetime("2026-02-29T20:00"));
        assert!(!valid_local_datetime("2026-10-05T25:00"));
    }

    #[actix_web::test]
    async fn create_without_session_returns_unauthorized() {
        let pool = PgPoolOptions::new()
            .connect_lazy("postgres://unused:unused@localhost/unused")
            .expect("lazy pool should not connect");
        let app = actix_web::test::init_service(App::new().app_data(web::Data::new(pool)).service(
            web::resource("/api/study-dates").route(web::post().to(super::create_study_date)),
        ))
        .await;
        let request = actix_web::test::TestRequest::post()
            .uri("/api/study-dates")
            .set_json(crate::lib::share::matching::CreateStudyDateRequest {
                starts_at: "2026-10-05T20:00".to_string(),
                duration_min: 180,
            })
            .to_request();
        let response = actix_web::test::call_service(&app, request).await;
        assert_eq!(response.status(), actix_web::http::StatusCode::UNAUTHORIZED);
    }
}
