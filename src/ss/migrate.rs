use sqlx::PgPool;

const CREATE_APP_USER: &str = "
CREATE TABLE IF NOT EXISTS app_user (
    id              BIGSERIAL PRIMARY KEY,
    email           TEXT NOT NULL UNIQUE,
    email_verified  BOOLEAN NOT NULL DEFAULT FALSE,
    otp_code        CHAR(6),
    otp_expires_at  TIMESTAMPTZ,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT email_is_lowercase CHECK (email = lower(email)),
    CONSTRAINT email_is_neu CHECK (email LIKE '%@neu.edu.vn')
)";

const CREATE_STUDY_DATE_REQUEST: &str = "
CREATE TABLE IF NOT EXISTS study_date_request (
    id              BIGSERIAL PRIMARY KEY,
    creator_id      BIGINT NOT NULL REFERENCES app_user(id),
    starts_at       TIMESTAMPTZ NOT NULL,
    ends_at         TIMESTAMPTZ NOT NULL,
    status          TEXT NOT NULL DEFAULT 'active',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT study_date_request_valid_status
        CHECK (status IN ('active', 'cancelled', 'expired', 'completed')),
    CONSTRAINT study_date_request_valid_window
        CHECK (ends_at > starts_at)
)";

const CREATE_REPORT: &str = "
CREATE TABLE IF NOT EXISTS report (
    id                BIGSERIAL PRIMARY KEY,
    reporter_id       BIGINT NOT NULL REFERENCES app_user(id),
    reported_user_id  BIGINT NOT NULL REFERENCES app_user(id),
    status            TEXT NOT NULL DEFAULT 'pending',
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT report_valid_status
        CHECK (status IN ('pending', 'reviewed', 'dismissed')),
    CONSTRAINT report_not_self
        CHECK (reporter_id <> reported_user_id)
)";

const CREATE_DASHBOARD_INDEXES: &str = "
CREATE INDEX IF NOT EXISTS study_date_request_active_idx
    ON study_date_request (starts_at, ends_at)
    WHERE status = 'active';
CREATE INDEX IF NOT EXISTS report_pending_idx
    ON report (status)
    WHERE status = 'pending'";

pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_APP_USER).execute(pool).await?;
    sqlx::query(CREATE_STUDY_DATE_REQUEST).execute(pool).await?;
    sqlx::query(CREATE_REPORT).execute(pool).await?;
    sqlx::query(CREATE_DASHBOARD_INDEXES).execute(pool).await?;
    Ok(())
}
