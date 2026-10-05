use sqlx::PgPool;

const CREATE_APP_USER: &str = "
CREATE TABLE IF NOT EXISTS app_user (
    id              BIGSERIAL PRIMARY KEY,
    email           TEXT NOT NULL UNIQUE,
    email_verified  BOOLEAN NOT NULL DEFAULT FALSE,
    otp_code        CHAR(6),
    otp_expires_at  TIMESTAMPTZ,
    role            TEXT NOT NULL DEFAULT 'user',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT email_is_lowercase CHECK (email = lower(email)),
    CONSTRAINT email_is_neu CHECK (email LIKE '%@neu.edu.vn')
)";

const CREATE_AUTH_SESSIONS: &str = "
CREATE TABLE IF NOT EXISTS auth_session (
    token_hash  TEXT PRIMARY KEY,
    user_id     BIGINT NOT NULL REFERENCES app_user(id) ON DELETE CASCADE,
    expires_at  TIMESTAMPTZ NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT now()
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

const CREATE_MATCH_REQUEST: &str = "
CREATE TABLE IF NOT EXISTS match_request (
    id            BIGSERIAL PRIMARY KEY,
    sender_id     BIGINT NOT NULL REFERENCES app_user(id),
    receiver_id   BIGINT NOT NULL REFERENCES app_user(id),
    status        TEXT NOT NULL DEFAULT 'pending',
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now(),
    responded_at  TIMESTAMPTZ,
    CONSTRAINT match_request_valid_status CHECK (status IN ('pending', 'accepted', 'declined')),
    CONSTRAINT match_request_not_self CHECK (sender_id <> receiver_id),
    CONSTRAINT match_request_unique_pair UNIQUE (sender_id, receiver_id)
)";

const CREATE_CHAT_ROOM: &str = "
CREATE TABLE IF NOT EXISTS chat_room (
    id                BIGSERIAL PRIMARY KEY,
    user_low          BIGINT NOT NULL REFERENCES app_user(id),
    user_high         BIGINT NOT NULL REFERENCES app_user(id),
    match_request_id  BIGINT UNIQUE REFERENCES match_request(id),
    created_at        TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT chat_room_ordered CHECK (user_low < user_high),
    CONSTRAINT chat_room_unique_pair UNIQUE (user_low, user_high)
)";

const CREATE_MATCH_INDEXES: &str = "
CREATE INDEX IF NOT EXISTS match_request_receiver_pending_idx
    ON match_request (receiver_id) WHERE status = 'pending';
CREATE INDEX IF NOT EXISTS match_request_sender_day_idx
    ON match_request (sender_id, created_at)";

pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(CREATE_APP_USER).execute(pool).await?;
    sqlx::query("ALTER TABLE app_user ADD COLUMN IF NOT EXISTS role TEXT NOT NULL DEFAULT 'user'")
        .execute(pool)
        .await?;
    sqlx::query(CREATE_STUDY_DATE_REQUEST).execute(pool).await?;
    sqlx::query(CREATE_REPORT).execute(pool).await?;
    sqlx::query(CREATE_AUTH_SESSIONS).execute(pool).await?;
    sqlx::query(CREATE_DASHBOARD_INDEXES).execute(pool).await?;
    sqlx::query(CREATE_MATCH_REQUEST).execute(pool).await?;
    sqlx::query(CREATE_CHAT_ROOM).execute(pool).await?;
    sqlx::query(CREATE_MATCH_INDEXES).execute(pool).await?;
    Ok(())
}
