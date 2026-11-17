"""SQL definitions loaded and executed by the application database notebook.

This module only defines SQL and seed values. ``database/note.py`` owns the
database connection, recreation controls, transactions, and execution flow.
"""

APPLICATION_SCHEMA_SQL = """
DROP SCHEMA public CASCADE;
CREATE SCHEMA public;

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email TEXT NOT NULL UNIQUE,
    username TEXT UNIQUE,
    password_hash TEXT,
    email_verified BOOLEAN NOT NULL DEFAULT FALSE,
    role TEXT NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT users_email_lowercase CHECK (email = lower(email)),
    CONSTRAINT users_email_neu CHECK (
        email ~ '^[^@]+@([a-z0-9]([a-z0-9-]*[a-z0-9])?[.])*neu[.]edu[.]vn$'
    ),
    CONSTRAINT users_username_valid CHECK (
        username IS NULL OR username ~ '^[a-z0-9][a-z0-9._-]{1,30}[a-z0-9]$'
    ),
    CONSTRAINT users_valid_role CHECK (role IN ('user', 'admin'))
);

CREATE TABLE user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    display_name TEXT NOT NULL,
    bio TEXT NOT NULL DEFAULT '',
    cohort TEXT,
    interests TEXT NOT NULL DEFAULT '',
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT user_profiles_display_name_length CHECK (char_length(display_name) BETWEEN 1 AND 60),
    CONSTRAINT user_profiles_bio_length CHECK (char_length(bio) <= 500),
    CONSTRAINT user_profiles_cohort_length CHECK (cohort IS NULL OR char_length(cohort) <= 24),
    CONSTRAINT user_profiles_interests_length CHECK (char_length(interests) <= 300)
);

CREATE TABLE auth_cookies (
    token_hash TEXT PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT auth_cookies_sha256_token CHECK (token_hash ~ '^[0-9a-f]{64}$')
);

CREATE TABLE study_dates (
    id BIGSERIAL PRIMARY KEY,
    creator_id UUID NOT NULL REFERENCES users(id),
    starts_at TIMESTAMPTZ NOT NULL,
    ends_at TIMESTAMPTZ NOT NULL,
    duration_min INTEGER NOT NULL DEFAULT 60,
    status TEXT NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT study_dates_valid_status CHECK (
        status IN ('active', 'cancelled', 'expired', 'completed')
    ),
    CONSTRAINT study_dates_valid_duration CHECK (duration_min BETWEEN 1 AND 720),
    CONSTRAINT study_dates_valid_window CHECK (ends_at > starts_at),
    CONSTRAINT study_dates_duration_matches_window CHECK (
        ends_at = starts_at + duration_min * interval '1 minute'
    )
);

CREATE TABLE user_reports (
    id BIGSERIAL PRIMARY KEY,
    reporter_id UUID NOT NULL REFERENCES users(id),
    reported_user_id UUID NOT NULL REFERENCES users(id),
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT user_reports_valid_status CHECK (
        status IN ('pending', 'reviewed', 'dismissed')
    ),
    CONSTRAINT user_reports_not_self CHECK (reporter_id <> reported_user_id)
);

CREATE TABLE match_requests (
    id BIGSERIAL PRIMARY KEY,
    sender_id UUID NOT NULL REFERENCES users(id),
    receiver_id UUID NOT NULL REFERENCES users(id),
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    responded_at TIMESTAMPTZ,
    CONSTRAINT match_requests_valid_status CHECK (
        status IN ('pending', 'accepted', 'declined')
    ),
    CONSTRAINT match_requests_not_self CHECK (sender_id <> receiver_id),
    CONSTRAINT match_requests_unique_pair UNIQUE (sender_id, receiver_id),
    CONSTRAINT match_requests_response_state CHECK (
        (status = 'pending' AND responded_at IS NULL)
        OR (status IN ('accepted', 'declined') AND responded_at IS NOT NULL)
    )
);

CREATE TABLE conversations (
    id BIGSERIAL PRIMARY KEY,
    user_low UUID NOT NULL REFERENCES users(id),
    user_high UUID NOT NULL REFERENCES users(id),
    match_request_id BIGINT UNIQUE REFERENCES match_requests(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT conversations_ordered CHECK (user_low < user_high),
    CONSTRAINT conversations_unique_pair UNIQUE (user_low, user_high)
);

CREATE TABLE chat_messages (
    id BIGSERIAL PRIMARY KEY,
    conversation_id BIGINT NOT NULL REFERENCES conversations(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL REFERENCES users(id),
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT chat_messages_body_length CHECK (char_length(body) BETWEEN 1 AND 2000)
);

CREATE TABLE threads (
    id BIGSERIAL PRIMARY KEY,
    author_id UUID NOT NULL REFERENCES users(id),
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT threads_title_length CHECK (
        char_length(btrim(title)) BETWEEN 1 AND 120
    ),
    CONSTRAINT threads_body_length CHECK (
        char_length(btrim(body)) BETWEEN 1 AND 3000
    )
);

CREATE TABLE thread_comments (
    id BIGSERIAL PRIMARY KEY,
    thread_id BIGINT NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    author_id UUID NOT NULL REFERENCES users(id),
    body TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    CONSTRAINT thread_comments_body_length CHECK (
        char_length(btrim(body)) BETWEEN 1 AND 2000
    )
);

CREATE INDEX auth_cookies_user_id_idx ON auth_cookies (user_id);
CREATE INDEX study_dates_creator_id_idx ON study_dates (creator_id);
CREATE INDEX study_dates_active_window_idx
    ON study_dates (starts_at, ends_at) WHERE status = 'active';
CREATE INDEX user_reports_pending_idx
    ON user_reports (created_at) WHERE status = 'pending';
CREATE INDEX user_reports_reporter_id_idx ON user_reports (reporter_id);
CREATE INDEX user_reports_reported_user_id_idx ON user_reports (reported_user_id);
CREATE INDEX match_requests_receiver_status_idx
    ON match_requests (receiver_id, status, created_at DESC);
CREATE INDEX match_requests_sender_created_idx
    ON match_requests (sender_id, created_at DESC);
CREATE INDEX conversations_user_high_idx ON conversations (user_high);
CREATE INDEX chat_messages_conversation_created_idx
    ON chat_messages (conversation_id, created_at, id);
CREATE INDEX threads_created_idx ON threads (created_at DESC, id DESC);
CREATE INDEX thread_comments_thread_created_idx
    ON thread_comments (thread_id, created_at, id);
"""

SAMPLE_USERS = (
    ("alex.demo@students.neu.edu.vn", "alex_demo"),
    ("linh.demo@demo.neu.edu.vn", "linh_demo"),
    ("minh.demo@neu.edu.vn", "minh_demo"),
)

UPSERT_SAMPLE_USER_SQL = """
INSERT INTO users (email, username, email_verified)
VALUES (%s, %s, TRUE)
ON CONFLICT (email) DO UPDATE SET
    username = EXCLUDED.username,
    password_hash = NULL,
    email_verified = TRUE
RETURNING id
"""

UPSERT_ADMIN_SEED_SQL = """
INSERT INTO users (email, username, email_verified, role)
VALUES (%s, %s, TRUE, 'user')
ON CONFLICT (email) DO UPDATE SET
    username = EXCLUDED.username,
    password_hash = NULL,
    email_verified = TRUE,
    role = 'user'
RETURNING id
"""

SEED_STUDY_DATES_SQL = """
INSERT INTO study_dates
    (creator_id, starts_at, ends_at, duration_min, status)
SELECT user_id, start_at, start_at + interval '60 minutes', 60, 'active'
FROM (VALUES
    (%s::uuid, date_trunc('day', now()) + interval '1 day' + interval '9 hours'),
    (%s::uuid, date_trunc('day', now()) + interval '1 day' + interval '14 hours'),
    (%s::uuid, date_trunc('day', now()) + interval '2 days' + interval '10 hours')
) AS samples(user_id, start_at)
WHERE NOT EXISTS (
    SELECT 1 FROM study_dates existing
    WHERE existing.creator_id = samples.user_id
      AND existing.starts_at = samples.start_at
)
"""

SEED_PENDING_REPORT_SQL = """
INSERT INTO user_reports (reporter_id, reported_user_id, status)
SELECT %s, %s, 'pending'
WHERE NOT EXISTS (
    SELECT 1 FROM user_reports
    WHERE reporter_id = %s AND reported_user_id = %s AND status = 'pending'
)
"""

SEED_ACCEPTED_MATCH_SQL = """
INSERT INTO match_requests (sender_id, receiver_id, status, responded_at)
VALUES (%s, %s, 'accepted', now())
ON CONFLICT (sender_id, receiver_id) DO UPDATE
SET status = 'accepted', responded_at = COALESCE(match_requests.responded_at, now())
RETURNING id
"""

SEED_PENDING_MATCH_SQL = """
INSERT INTO match_requests (sender_id, receiver_id, status)
VALUES (%s, %s, 'pending')
ON CONFLICT (sender_id, receiver_id) DO NOTHING
"""

SEED_CONVERSATION_SQL = """
INSERT INTO conversations (user_low, user_high, match_request_id)
VALUES (%s, %s, %s)
ON CONFLICT (user_low, user_high) DO NOTHING
"""
