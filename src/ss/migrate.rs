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

pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_APP_USER).execute(pool).await?;
    Ok(())
}