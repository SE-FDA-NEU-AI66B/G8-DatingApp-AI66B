use super::auth;
use crate::lib::share::admin::{
    DashboardStatistics, HealthComponent, HealthStatus, StatisticsError,
};
use actix_web::{get, web, HttpRequest, HttpResponse};
use sqlx::{FromRow, PgPool};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

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
pub async fn dashboard_statistics(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
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

#[get("/api/admin/health")]
pub async fn health_status(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Err(response) = auth::require_admin(&request, pool.get_ref()).await {
        return response;
    }

    let checked_at = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .expect("RFC3339 formatting must be available");
    let database_healthy = sqlx::query("SELECT 1")
        .execute(pool.get_ref())
        .await
        .is_ok();

    HttpResponse::Ok().json(HealthStatus {
        server: HealthComponent {
            healthy: true,
            checked_at: checked_at.clone(),
        },
        database: HealthComponent {
            healthy: database_healthy,
            checked_at,
        },
    })
}

#[cfg(test)]
mod tests {
    use super::{DashboardStatisticsRow, DASHBOARD_QUERY};
    use crate::ss::migrate;
    use sqlx::postgres::PgPoolOptions;

    #[test]
    fn registered_users_metric_counts_verified_accounts_only() {
        assert!(DASHBOARD_QUERY.contains("email_verified = TRUE"));
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing to an isolated PostgreSQL database"]
    async fn dashboard_query_counts_fixture_rows() {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must point to an isolated PostgreSQL database");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&database_url)
            .await
            .expect("test database must be reachable");
        migrate::run(&pool)
            .await
            .expect("first migration must succeed");
        migrate::run(&pool)
            .await
            .expect("migration must be idempotent");

        let mut transaction = pool.begin().await.expect("fixture transaction must start");
        sqlx::query(
            "INSERT INTO app_user (email, email_verified)
             VALUES
                ('verified-dashboard-test@neu.edu.vn', TRUE),
                ('pending-dashboard-test@neu.edu.vn', FALSE)",
        )
        .execute(&mut *transaction)
        .await
        .expect("user fixtures must insert");

        let verified_id: i64 = sqlx::query_scalar(
            "SELECT id FROM app_user WHERE email = 'verified-dashboard-test@neu.edu.vn'",
        )
        .fetch_one(&mut *transaction)
        .await
        .expect("verified fixture must exist");
        let pending_id: i64 = sqlx::query_scalar(
            "SELECT id FROM app_user WHERE email = 'pending-dashboard-test@neu.edu.vn'",
        )
        .fetch_one(&mut *transaction)
        .await
        .expect("pending fixture must exist");
        sqlx::query(
            "INSERT INTO study_date_request (creator_id, starts_at, ends_at, duration_min, status)
             VALUES ($1, now() + interval '1 hour', now() + interval '2 hours', 60, 'active')",
        )
        .bind(verified_id)
        .execute(&mut *transaction)
        .await
        .expect("study-date fixture must insert");
        sqlx::query(
            "INSERT INTO report (reporter_id, reported_user_id, status)
             VALUES ($1, $2, 'pending')",
        )
        .bind(verified_id)
        .bind(pending_id)
        .execute(&mut *transaction)
        .await
        .expect("report fixture must insert");

        let row = sqlx::query_as::<_, DashboardStatisticsRow>(DASHBOARD_QUERY)
            .fetch_one(&mut *transaction)
            .await
            .expect("dashboard query must execute");
        assert_eq!(row.total_registered_users, 1);
        assert_eq!(row.active_study_date_requests, 1);
        assert_eq!(row.pending_reports, 1);

        transaction
            .rollback()
            .await
            .expect("fixtures must roll back");
    }

    #[tokio::test]
    #[ignore = "requires TEST_DATABASE_URL pointing to an isolated PostgreSQL database"]
    async fn dashboard_query_returns_error_when_database_is_closed() {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must point to an isolated PostgreSQL database");
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&database_url)
            .await
            .expect("test database must be reachable");
        pool.close().await;

        let result = sqlx::query_as::<_, DashboardStatisticsRow>(DASHBOARD_QUERY)
            .fetch_one(&pool)
            .await;
        assert!(result.is_err());
    }
}
