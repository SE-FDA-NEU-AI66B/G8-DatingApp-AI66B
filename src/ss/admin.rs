use super::auth;
use crate::lib::share::admin::{
    ActivityLogEntry, CreateReportRequest, DashboardStatistics, HealthComponent, HealthStatus,
    ReportSummary, StatisticsError,
};
use actix_web::{get, post, web, HttpRequest, HttpResponse};
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

    #[post("/api/reports")]
    pub async fn create_report(
        request: HttpRequest,
        pool: web::Data<PgPool>,
        body: web::Json<CreateReportRequest>,
    ) -> HttpResponse {
        let reporter = match auth::require_user(&request, pool.get_ref()).await {
            Ok(user) => user,
            Err(response) => return response,
        };
        if reporter.id == body.reported_user_id || body.reason.trim().is_empty() {
            return HttpResponse::BadRequest().json(crate::lib::share::register::MessageBody {
                message: "A valid reported profile and reason are required.".to_string(),
            });
        }
        let result = sqlx::query(
            "INSERT INTO report (reporter_id, reported_user_id, reason)
             SELECT $1, $2, $3
             WHERE EXISTS (
                 SELECT 1 FROM app_user
                 WHERE id = $2 AND email_verified = TRUE AND is_hidden = FALSE
             )
               AND NOT EXISTS (
                 SELECT 1 FROM report
                 WHERE reporter_id = $1
                   AND reported_user_id = $2
                   AND created_at >= now() - interval '24 hours'
             )",
        )
        .bind(reporter.id)
        .bind(body.reported_user_id)
        .bind(body.reason.trim())
        .execute(pool.get_ref())
        .await;
        match result {
            Ok(result) if result.rows_affected() == 1 => {
                if let Err(error) = sqlx::query(
                    "INSERT INTO activity_log (actor_id, action, target_id)
                     VALUES ($1, 'report_submission', $2)",
                )
                .bind(reporter.id)
                .bind(body.reported_user_id)
                .execute(pool.get_ref())
                .await
                {
                    eprintln!("report activity log failed: {error}");
                }
                HttpResponse::Created().json(crate::lib::share::register::MessageBody {
                    message: "Report submitted.".to_string(),
                })
            }
            Ok(_) => HttpResponse::Conflict().json(crate::lib::share::register::MessageBody {
                message: "This profile was already reported by you in the last 24 hours.".to_string(),
            }),
            Err(error) => {
                eprintln!("create report failed: {error}");
                HttpResponse::InternalServerError().finish()
            }
        }
    }

    #[derive(Debug, FromRow)]
    struct ReportSummaryRow {
        id: i64,
        reported_user_id: i64,
        reason: String,
        report_count: i64,
        status: String,
    }

    #[derive(Debug, FromRow)]
    struct ActivityLogRow {
        action: String,
        target_id: Option<i64>,
        actor_id: Option<i64>,
        created_at: String,
    }

    #[get("/api/admin/reports")]
    pub async fn report_queue(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
        let _admin = match auth::require_admin(&request, pool.get_ref()).await {
            Ok(user) => user,
            Err(response) => return response,
        };
        let rows = sqlx::query_as::<_, ReportSummaryRow>(
            "SELECT MIN(r.id) AS id,
                    r.reported_user_id,
                    MIN(r.reason) AS reason,
                    COUNT(DISTINCT r.reporter_id) AS report_count,
                    MIN(r.status) AS status
             FROM report r
             JOIN app_user u ON u.id = r.reported_user_id
             WHERE r.status = 'pending'
               AND u.is_hidden = FALSE
               AND r.created_at >= now() - interval '24 hours'
             GROUP BY r.reported_user_id
             HAVING COUNT(DISTINCT r.reporter_id) >= 5
             ORDER BY MAX(r.created_at) DESC",
        )
        .fetch_all(pool.get_ref())
        .await;

        match rows {
            Ok(rows) => {
                let reports = rows
                    .into_iter()
                    .map(|row| ReportSummary {
                        id: row.id,
                        reported_user_id: row.reported_user_id,
                        reason: row.reason,
                        report_count: row.report_count,
                        status: row.status,
                    })
                    .collect::<Vec<_>>();
                HttpResponse::Ok().json(reports)
            }
            Err(error) => {
                eprintln!("admin report queue failed: {error}");
                HttpResponse::InternalServerError().json(crate::lib::share::register::MessageBody {
                    message: "Unable to load the report queue.".to_string(),
                })
            }
        }
    }

    async fn moderate_report(
        request: HttpRequest,
        pool: web::Data<PgPool>,
        report_id: web::Path<i64>,
        status: &'static str,
        hide_profile: bool,
    ) -> HttpResponse {
        let admin = match auth::require_admin(&request, pool.get_ref()).await {
            Ok(user) => user,
            Err(response) => return response,
        };
        let mut transaction = match pool.begin().await {
            Ok(transaction) => transaction,
            Err(error) => {
                eprintln!("moderation transaction failed: {error}");
                return HttpResponse::InternalServerError().finish();
            }
        };
        let target: Option<i64> = sqlx::query_scalar(
            "UPDATE report SET status = $1
             WHERE id = $2 AND status = 'pending'
             RETURNING reported_user_id",
        )
        .bind(status)
        .bind(report_id.into_inner())
        .fetch_optional(&mut *transaction)
        .await
        .unwrap_or(None);

        let Some(target) = target else {
            return HttpResponse::NotFound().json(crate::lib::share::register::MessageBody {
                message: "Report not found or already reviewed.".to_string(),
            });
        };
        if hide_profile {
            if let Err(error) = sqlx::query("UPDATE app_user SET is_hidden = TRUE WHERE id = $1")
                .bind(target)
                .execute(&mut *transaction)
                .await
            {
                eprintln!("hide profile failed: {error}");
                return HttpResponse::InternalServerError().finish();
            }
        }
        let action = if hide_profile {
            "moderation_hide_profile"
        } else {
            "moderation_dismiss_report"
        };
        if let Err(error) = sqlx::query(
            "INSERT INTO activity_log (actor_id, action, target_id) VALUES ($1, $2, $3)",
        )
        .bind(admin_id(&admin))
        .bind(action)
        .bind(target)
        .execute(&mut *transaction)
        .await
        {
            eprintln!("moderation activity log failed: {error}");
            return HttpResponse::InternalServerError().finish();
        }
        if let Err(error) = transaction.commit().await {
            eprintln!("moderation commit failed: {error}");
            return HttpResponse::InternalServerError().finish();
        }
        HttpResponse::Ok().json(crate::lib::share::register::MessageBody {
            message: "Report reviewed.".to_string(),
        })
    }

    fn admin_id(admin: &auth::AuthenticatedUser) -> i64 {
        admin.id
    }

    #[post("/api/admin/reports/{id}/hide")]
    pub async fn hide_report(
        request: HttpRequest,
        pool: web::Data<PgPool>,
        report_id: web::Path<i64>,
    ) -> HttpResponse {
        moderate_report(request, pool, report_id, "reviewed", true).await
    }

    #[post("/api/admin/reports/{id}/dismiss")]
    pub async fn dismiss_report(
        request: HttpRequest,
        pool: web::Data<PgPool>,
        report_id: web::Path<i64>,
    ) -> HttpResponse {
        moderate_report(request, pool, report_id, "dismissed", false).await
    }

    #[get("/api/admin/activity-log")]
    pub async fn activity_log(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
        if let Err(response) = auth::require_admin(&request, pool.get_ref()).await {
            return response;
        }
        match sqlx::query_as::<_, ActivityLogRow>(
            "SELECT action,
                    target_id,
                    actor_id,
                    to_char(created_at AT TIME ZONE 'Asia/Ho_Chi_Minh', 'YYYY-MM-DD HH24:MI:SS') AS created_at
             FROM activity_log
             ORDER BY created_at DESC, id DESC",
        )
        .fetch_all(pool.get_ref())
        .await
        {
            Ok(entries) => HttpResponse::Ok().json(
                entries
                    .into_iter()
                    .map(|entry| ActivityLogEntry {
                        action: entry.action,
                        target_id: entry.target_id,
                        actor_id: entry.actor_id,
                        created_at: entry.created_at,
                    })
                    .collect::<Vec<_>>(),
            ),
            Err(error) => {
                eprintln!("activity log query failed: {error}");
                HttpResponse::InternalServerError().finish()
            }
        }
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
