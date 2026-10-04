use crate::lib::share::admin::{DashboardStatistics, StatisticsError};
use actix_web::{get, web, HttpResponse};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
struct DashboardStatisticsRow {
    total_registered_users: i64,
    active_study_date_requests: i64,
    pending_reports: i64,
}

#[get("/api/admin/statistics")]
pub async fn dashboard_statistics(pool: web::Data<PgPool>) -> HttpResponse {
    let result = sqlx::query_as::<_, DashboardStatisticsRow>(
        "SELECT
            (SELECT COUNT(*) FROM app_user) AS total_registered_users,
            -- Active means overlapping the current time and next 24 hours.
            (SELECT COUNT(*)
            FROM study_date_request
            WHERE status = 'active'
              AND starts_at < now() + interval '24 hours'
              AND ends_at > now()) AS active_study_date_requests,
            (SELECT COUNT(*) FROM report WHERE status = 'pending') AS pending_reports",
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(row) => HttpResponse::Ok().json(DashboardStatistics {
            total_registered_users: row.total_registered_users,
            active_study_date_requests: row.active_study_date_requests,
            pending_reports: row.pending_reports,
        }),
        Err(error) => {
            eprintln!("dashboard statistics query failed: {error}");
            HttpResponse::InternalServerError().json(StatisticsError {
                message: "Dashboard statistics are temporarily unavailable.".to_string(),
            })
        }
    }
}
