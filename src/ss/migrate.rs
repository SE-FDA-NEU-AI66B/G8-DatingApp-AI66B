use sqlx::PgPool;

const SCHEMA_STATEMENTS: &[&str] = &[
    "CREATE EXTENSION IF NOT EXISTS pgcrypto",
    "CREATE TABLE IF NOT EXISTS users (
        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
        email TEXT NOT NULL UNIQUE,
        username TEXT UNIQUE,
        password_hash TEXT,
        email_verified BOOLEAN NOT NULL DEFAULT FALSE,
        role TEXT NOT NULL DEFAULT 'user',
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS user_profiles (
        user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
        display_name TEXT NOT NULL,
        bio TEXT NOT NULL DEFAULT '',
        cohort TEXT,
        interests TEXT NOT NULL DEFAULT '',
        updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS auth_cookies (
        token_hash TEXT PRIMARY KEY,
        user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
        expires_at TIMESTAMPTZ NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS study_dates (
        id BIGSERIAL PRIMARY KEY,
        creator_id UUID NOT NULL REFERENCES users(id),
        starts_at TIMESTAMPTZ NOT NULL,
        ends_at TIMESTAMPTZ NOT NULL,
        duration_min INTEGER NOT NULL DEFAULT 60,
        status TEXT NOT NULL DEFAULT 'active',
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS user_reports (
        id BIGSERIAL PRIMARY KEY,
        reporter_id UUID NOT NULL REFERENCES users(id),
        reported_user_id UUID NOT NULL REFERENCES users(id),
        status TEXT NOT NULL DEFAULT 'pending',
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS match_requests (
        id BIGSERIAL PRIMARY KEY,
        sender_id UUID NOT NULL REFERENCES users(id),
        receiver_id UUID NOT NULL REFERENCES users(id),
        status TEXT NOT NULL DEFAULT 'pending',
        created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
        responded_at TIMESTAMPTZ
    )",
    "CREATE TABLE IF NOT EXISTS conversations (
        id BIGSERIAL PRIMARY KEY,
        user_low UUID NOT NULL REFERENCES users(id),
        user_high UUID NOT NULL REFERENCES users(id),
        match_request_id BIGINT UNIQUE REFERENCES match_requests(id),
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS chat_messages (
        id BIGSERIAL PRIMARY KEY,
        conversation_id BIGINT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
        sender_id UUID NOT NULL REFERENCES users(id),
        body TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS threads (
        id BIGSERIAL PRIMARY KEY,
        author_id UUID NOT NULL REFERENCES users(id),
        title TEXT NOT NULL,
        body TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
    "CREATE TABLE IF NOT EXISTS thread_comments (
        id BIGSERIAL PRIMARY KEY,
        thread_id BIGINT NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
        author_id UUID NOT NULL REFERENCES users(id),
        body TEXT NOT NULL,
        created_at TIMESTAMPTZ NOT NULL DEFAULT now()
    )",
];

pub async fn run(pool: &PgPool) -> Result<(), sqlx::Error> {
    for statement in SCHEMA_STATEMENTS {
        sqlx::query(*statement).execute(pool).await?;
    }

    for statement in [
        "CREATE INDEX IF NOT EXISTS auth_cookies_user_id_idx ON auth_cookies (user_id)",
        "CREATE INDEX IF NOT EXISTS study_dates_creator_id_idx ON study_dates (creator_id)",
        "CREATE INDEX IF NOT EXISTS user_reports_pending_idx ON user_reports (created_at)",
        "CREATE INDEX IF NOT EXISTS match_requests_receiver_status_idx ON match_requests (receiver_id, status, created_at DESC)",
        "CREATE INDEX IF NOT EXISTS match_requests_sender_created_idx ON match_requests (sender_id, created_at DESC)",
        "CREATE INDEX IF NOT EXISTS conversations_user_high_idx ON conversations (user_high)",
        "CREATE INDEX IF NOT EXISTS chat_messages_conversation_created_idx ON chat_messages (conversation_id, created_at, id)",
        "CREATE INDEX IF NOT EXISTS threads_created_idx ON threads (created_at DESC, id DESC)",
        "CREATE INDEX IF NOT EXISTS thread_comments_thread_created_idx ON thread_comments (thread_id, created_at, id)",
    ] {
        sqlx::query(statement).execute(pool).await?;
    }

    Ok(())
}
