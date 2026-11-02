from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
ADMIN_HANDLER = (ROOT / "src/ss/admin.rs").read_text(encoding="utf-8")
ADMIN_PAGE = (ROOT / "src/app/admin.rs").read_text(encoding="utf-8")


def test_us13_t01_health_reports_server_and_database_independently():
    health_response = ADMIN_HANDLER.split("pub async fn health_status", 1)[1]

    assert "healthy: true" in health_response
    assert "healthy: database_healthy" in health_response
    assert 'sqlx::query("SELECT 1")' in health_response


def test_us13_t02_database_query_failure_is_reported_as_unhealthy():
    health_response = ADMIN_HANDLER.split("pub async fn health_status", 1)[1]

    assert ".is_ok();" in health_response
    assert "database: HealthComponent" in health_response
    assert "healthy: database_healthy" in health_response


def test_us14_t01_statistics_count_verified_users_and_active_requests():
    query = ADMIN_HANDLER.split("const DASHBOARD_QUERY: &str =", 1)[1].split(";", 1)[0]

    assert "email_verified = TRUE" in query
    assert "status = 'active'" in query
    assert "starts_at < now() + interval '24 hours'" in query
    assert "ends_at > now()" in query
    assert "status = 'pending'" in query


def test_us14_t03_t04_dashboard_distinguishes_errors_from_zero_counts():
    assert 'if let Some(message) = error.get()' in ADMIN_PAGE
    assert 'if let Some(value) = statistics.get()' in ADMIN_PAGE
    assert "value.total_registered_users" in ADMIN_PAGE
    assert "value.active_study_date_requests" in ADMIN_PAGE
    assert "value.pending_reports" in ADMIN_PAGE
    assert '"Dashboard statistics are unavailable."' in ADMIN_PAGE
