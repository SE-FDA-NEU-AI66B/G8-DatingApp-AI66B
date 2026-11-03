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
    (SELECT COUNT(*) FROM users WHERE email_verified = TRUE) AS total_registered_users,
    -- Active means overlapping the current time and next 24 hours.
    (SELECT COUNT(*)
    FROM study_dates
    WHERE status = 'active'
      AND starts_at < now() + interval '24 hours'
      AND ends_at > now()) AS active_study_date_requests,
    (SELECT COUNT(*) FROM user_reports WHERE status = 'pending') AS pending_reports";

#[get("/api/admin/access")]
pub async fn admin_access(request: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
    if let Err(response) = auth::require_admin(&request, pool.get_ref()).await {
        return response;
    }

    HttpResponse::NoContent().finish()
}

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
    use super::{dashboard_statistics, health_status, DashboardStatisticsRow, DASHBOARD_QUERY};
    use actix_web::{cookie::Cookie, http::StatusCode, test, web, App};
    use rand::random;
    use sha2::{Digest, Sha256};
    use sqlx::{postgres::PgPoolOptions, PgPool};
    use uuid::Uuid;

    struct AdminFixture {
        pool: PgPool,
        user_id: Uuid,
        token: String,
    }

    async fn admin_fixture() -> AdminFixture {
        let database_url = std::env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must point to an isolated PostgreSQL database");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&database_url)
            .await
            .expect("test database must be reachable");
        let unique = random::<u64>();
        let email = format!("admin-test-{unique}@neu.edu.vn");
        let user_id: Uuid = sqlx::query_scalar(
            "INSERT INTO users (email, email_verified, role)
             VALUES ($1, TRUE, 'admin') RETURNING id",
        )
        .bind(email)
        .fetch_one(&pool)
        .await
        .expect("admin fixture user should be inserted");
        let token = format!("admin-test-session-{unique}");
        let token_hash = hex::encode(Sha256::digest(token.as_bytes()));
        sqlx::query(
            "INSERT INTO auth_cookies (token_hash, user_id, expires_at)
             VALUES ($1, $2, now() + interval '1 hour')",
        )
        .bind(token_hash)
        .bind(user_id)
        .execute(&pool)
        .await
        .expect("admin session fixture should be inserted");
        AdminFixture {
            pool,
            user_id,
            token,
        }
    }

    #[actix_web::test]
    #[ignore = "requires TEST_DATABASE_URL pointing to an isolated PostgreSQL database"]
    async fn us13_t01_health_reports_server_and_database_independently() {
        let fixture = admin_fixture().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(fixture.pool.clone()))
                .service(health_status),
        )
        .await;
        let request = test::TestRequest::get()
            .uri("/api/admin/health")
            .cookie(Cookie::new("datingapp_session", fixture.token.clone()))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(body["server"]["healthy"], true);
        assert_eq!(body["database"]["healthy"], true);

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(fixture.user_id)
            .execute(&fixture.pool)
            .await
            .expect("admin fixture should be removed");
    }

    #[actix_web::test]
    #[ignore = "requires TEST_DATABASE_URL pointing to an isolated PostgreSQL database"]
    async fn us14_t01_statistics_count_verified_users_and_active_requests() {
        let fixture = admin_fixture().await;
        let before = sqlx::query_as::<_, DashboardStatisticsRow>(DASHBOARD_QUERY)
            .fetch_one(&fixture.pool)
            .await;
        let before = before.expect("baseline dashboard statistics should be readable");
        let unique = random::<u64>();
        let verified_email = format!("verified-stats-{unique}@neu.edu.vn");
        let unverified_email = format!("unverified-stats-{unique}@neu.edu.vn");
        let verified_id: Uuid = sqlx::query_scalar(
            "INSERT INTO users (email, email_verified) VALUES ($1, TRUE) RETURNING id",
        )
        .bind(verified_email)
        .fetch_one(&fixture.pool)
        .await
        .expect("verified fixture should be inserted");
        let unverified_id: Uuid = sqlx::query_scalar(
            "INSERT INTO users (email, email_verified) VALUES ($1, FALSE) RETURNING id",
        )
        .bind(unverified_email)
        .fetch_one(&fixture.pool)
        .await
        .expect("unverified fixture should be inserted");
        sqlx::query(
            "INSERT INTO study_dates (creator_id, starts_at, ends_at, duration_min, status)
             VALUES ($1, now() + interval '1 hour', now() + interval '2 hours', 60, 'active'),
                    ($1, now() + interval '2 days', now() + interval '2 days 1 hour', 60, 'active'),
                    ($1, now() + interval '1 hour', now() + interval '2 hours', 60, 'cancelled')",
        )
        .bind(verified_id)
        .execute(&fixture.pool)
        .await
        .expect("study-date fixtures should be inserted");
        sqlx::query(
            "INSERT INTO user_reports (reporter_id, reported_user_id, status)
             VALUES ($1, $2, 'pending'), ($1, $2, 'reviewed')",
        )
        .bind(fixture.user_id)
        .bind(unverified_id)
        .execute(&fixture.pool)
        .await
        .expect("report fixtures should be inserted");

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(fixture.pool.clone()))
                .service(dashboard_statistics),
        )
        .await;
        let request = test::TestRequest::get()
            .uri("/api/admin/statistics")
            .cookie(Cookie::new("datingapp_session", fixture.token.clone()))
            .to_request();
        let response = test::call_service(&app, request).await;
        assert_eq!(response.status(), StatusCode::OK);
        let body: serde_json::Value = test::read_body_json(response).await;
        assert_eq!(
            body["total_registered_users"],
            before.total_registered_users + 1
        );
        assert_eq!(
            body["active_study_date_requests"],
            before.active_study_date_requests + 1
        );
        assert_eq!(body["pending_reports"], before.pending_reports + 1);

        sqlx::query("DELETE FROM study_dates WHERE creator_id = $1")
            .bind(verified_id)
            .execute(&fixture.pool)
            .await
            .expect("study-date fixtures should be removed");
        sqlx::query("DELETE FROM user_reports WHERE reporter_id = $1 AND reported_user_id = $2")
            .bind(fixture.user_id)
            .bind(unverified_id)
            .execute(&fixture.pool)
            .await
            .expect("report fixtures should be removed");
        sqlx::query("DELETE FROM users WHERE id = ANY($1)")
            .bind([verified_id, unverified_id, fixture.user_id])
            .execute(&fixture.pool)
            .await
            .expect("statistics fixture users should be removed");
    }
}
