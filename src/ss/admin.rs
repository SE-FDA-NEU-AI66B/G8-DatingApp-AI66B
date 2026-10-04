use super::auth;
use crate::lib::share::admin::{DashboardStatistics, StatisticsError};
use actix_web::{get, web, HttpRequest, HttpResponse};
use sqlx::{FromRow, PgPool};

#[derive(Debug, FromRow)]
struct DashboardStatisticsRow {
    total_registered_users: i64,
    active_study_date_requests: i64,
    pending_reports: i64,
}

const DASHBOARD_QUERY: &str = "SELECT
    (SELECT COUNT(*) FROM app_user WHERE email_verified = TRUE) AS total_registered_users,
    -- Active means overlapping the current time and next 24 hours.
    (SELECT COUNT(*)
    FROM study_date_request
    WHERE status = 'active'
      AND starts_at < now() + interval '24 hours'
      AND ends_at > now()) AS active_study_date_requests,
    (SELECT COUNT(*) FROM report WHERE status = 'pending') AS pending_reports";

#[get("/api/admin/statistics")]
pub async fn dashboard_statistics(
    request: HttpRequest,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    if let Err(response) = auth::require_admin(&request, pool.get_ref()).await {
        return response;
    }

    let result = sqlx::query_as::<_, DashboardStatisticsRow>(DASHBOARD_QUERY)
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

#[cfg(test)]
mod tests {
    use super::DASHBOARD_QUERY;

    #[test]
    fn registered_users_metric_counts_verified_accounts_only() {
        assert!(DASHBOARD_QUERY.contains("email_verified = TRUE"));
    }
}
