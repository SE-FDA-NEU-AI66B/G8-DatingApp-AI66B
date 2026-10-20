import marimo

__generated_with = "0.24.2"
app = marimo.App(width="full", auto_download=["html"])


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ## Setup
    """)
    return


@app.cell
def _():
    import os
    import marimo as mo
    # note if ur new to marimo dont worry about duckdb marimo auto useit if available;since both duckdb and and polars use arrow data format duckdb intergration with marimo can read all polars object and so is the otherway round
    import psycopg2
    import polars as pl  # arrow conpatable

    from psycopg2 import sql

    return mo, os, psycopg2


@app.cell
def _(os, psycopg2):
    PGPASS = os.environ.get("PGPASS")
    if PGPASS is None:
        from pathlib import Path

        env_file = Path(__file__).resolve().parents[1] / ".env"
        if env_file.exists():
            for line in env_file.read_text(encoding="utf-8").splitlines():
                if line.startswith("PGPASS="):
                    PGPASS = line.partition("=")[2].strip().strip("\"'")
                    break
    # coonect to the database
    try:
        uri = f"postgresql://postgres:{PGPASS}@localhost/userdb"
        conn = psycopg2.connect(
            host="localhost",  # Your database host
            database="userdb",  # Your database name
            user="postgres",  # Your database username
            password=PGPASS,  # Your database password
        )
        conn.autocommit = True
    except Exception as e:
        print(e)
    return conn, uri


@app.cell
def _(mo, uri):
    _df = mo.sql(
        f"""
        ATTACH '{uri}' AS userdb (TYPE postgres);
        """
    )
    return


@app.cell
def _(uri):
    uri
    return


@app.cell(hide_code=True)
def _(mo):
    app_database_heading = mo.md(r"""
    ## Application database
    Recreate the empty application schema here. This drops every table in the
    connected database's `public` schema, then creates the tables used by the app.
    """)
    recreate_database_button = mo.ui.run_button(label="Recreate application database")
    mo.vstack([app_database_heading, recreate_database_button])
    return (recreate_database_button,)


@app.cell(hide_code=True)
def _(conn, mo, recreate_database_button):
    if recreate_database_button.value:
        schema_sql = """
        DROP SCHEMA public CASCADE;
        CREATE SCHEMA public;

        CREATE TABLE users (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email TEXT NOT NULL UNIQUE,
            email_verified BOOLEAN NOT NULL DEFAULT FALSE,
            role TEXT NOT NULL DEFAULT 'user',
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            CONSTRAINT users_email_lowercase CHECK (email = lower(email)),
            CONSTRAINT users_email_neu CHECK (email LIKE '%@neu.edu.vn'),
            CONSTRAINT users_valid_role CHECK (role IN ('user', 'admin'))
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
        """
        try:
            conn.autocommit = False
            with conn.cursor() as _cursor:
                _cursor.execute(schema_sql)
            conn.commit()
            schema_status = mo.md("Application database recreated.")
        except Exception as error:
            conn.rollback()
            schema_status = mo.md(f"Database recreation failed: `{error}`")
        finally:
            conn.autocommit = True
    else:
        schema_status = mo.md("Click the button above to recreate the application database.")
    schema_status
    return


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Other
    """)
    return


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ###postgres
    """)
    return


@app.cell
def _(conn, mo, users):
    _df = mo.sql(
        f"""
        Select * from users
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        EXPLAIN (ANALYZE, BUFFERS) SELECT * FROM information_schema.tables WHERE table_schema = 'public'
        """,
        engine=conn
    )
    return


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Test
    """)
    return


if __name__ == "__main__":
    app.run()
