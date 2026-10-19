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


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ## Application database creation
    Run this notebook cell before starting the Rust server.
    """)
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE TABLE IF NOT EXISTS app_user (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email TEXT NOT NULL UNIQUE,
            email_verified BOOLEAN NOT NULL DEFAULT FALSE,
            otp_code CHAR(6),
            otp_expires_at TIMESTAMPTZ,
            role TEXT NOT NULL DEFAULT 'user',
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            CONSTRAINT email_is_lowercase CHECK (email = lower(email)),
            CONSTRAINT email_is_neu CHECK (email LIKE '%%@neu.edu.vn')
        )
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE TABLE IF NOT EXISTS study_date_request (
            id BIGSERIAL PRIMARY KEY,
            creator_id UUID NOT NULL REFERENCES app_user(id),
            starts_at TIMESTAMPTZ NOT NULL,
            ends_at TIMESTAMPTZ NOT NULL,
            duration_min INTEGER NOT NULL DEFAULT 60,
            status TEXT NOT NULL DEFAULT 'active',
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            CONSTRAINT study_date_request_valid_status
                CHECK (status IN ('active', 'cancelled', 'expired', 'completed')),
            CONSTRAINT study_date_request_valid_duration
                CHECK (duration_min BETWEEN 1 AND 720),
            CONSTRAINT study_date_request_valid_window CHECK (ends_at > starts_at)
        )
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        ALTER TABLE study_date_request ADD COLUMN IF NOT EXISTS duration_min INTEGER
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo, study_date_request):
    _df = mo.sql(
        f"""
        UPDATE study_date_request
        SET duration_min = GREATEST(1, CEIL(EXTRACT(EPOCH FROM (ends_at - starts_at)) / 60)::INTEGER)
        WHERE duration_min IS NULL
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        ALTER TABLE study_date_request
        ALTER COLUMN duration_min SET DEFAULT 60,
        ALTER COLUMN duration_min SET NOT NULL
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        DO $$
        BEGIN
            IF NOT EXISTS (
                SELECT 1 FROM pg_constraint
                WHERE conname = 'study_date_request_valid_duration'
            ) THEN
                ALTER TABLE study_date_request
                ADD CONSTRAINT study_date_request_valid_duration
                CHECK (duration_min BETWEEN 1 AND 720) NOT VALID;
            END IF;
        END $$
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE TABLE IF NOT EXISTS report (
            id BIGSERIAL PRIMARY KEY,
            reporter_id UUID NOT NULL REFERENCES app_user(id),
            reported_user_id UUID NOT NULL REFERENCES app_user(id),
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            CONSTRAINT report_valid_status CHECK (status IN ('pending', 'reviewed', 'dismissed')),
            CONSTRAINT report_not_self CHECK (reporter_id <> reported_user_id)
        )
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE TABLE IF NOT EXISTS auth_session (
            token_hash TEXT PRIMARY KEY,
            user_id UUID NOT NULL REFERENCES app_user(id) ON DELETE CASCADE,
            expires_at TIMESTAMPTZ NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT now()
        )
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE INDEX IF NOT EXISTS study_date_request_active_idx
        ON study_date_request (starts_at, ends_at) WHERE status = 'active'
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE INDEX IF NOT EXISTS report_pending_idx
        ON report (status) WHERE status = 'pending'
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE TABLE IF NOT EXISTS match_request (
            id BIGSERIAL PRIMARY KEY,
            sender_id UUID NOT NULL REFERENCES app_user(id),
            receiver_id UUID NOT NULL REFERENCES app_user(id),
            status TEXT NOT NULL DEFAULT 'pending',
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            responded_at TIMESTAMPTZ,
            CONSTRAINT match_request_valid_status CHECK (status IN ('pending', 'accepted', 'declined')),
            CONSTRAINT match_request_not_self CHECK (sender_id <> receiver_id),
            CONSTRAINT match_request_unique_pair UNIQUE (sender_id, receiver_id)
        )
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE TABLE IF NOT EXISTS chat_room (
            id BIGSERIAL PRIMARY KEY,
            user_low UUID NOT NULL REFERENCES app_user(id),
            user_high UUID NOT NULL REFERENCES app_user(id),
            match_request_id BIGINT UNIQUE REFERENCES match_request(id),
            created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
            CONSTRAINT chat_room_ordered CHECK (user_low < user_high),
            CONSTRAINT chat_room_unique_pair UNIQUE (user_low, user_high)
        )
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE INDEX IF NOT EXISTS match_request_receiver_pending_idx
        ON match_request (receiver_id) WHERE status = 'pending'
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        CREATE INDEX IF NOT EXISTS match_request_sender_day_idx ON match_request (sender_id, created_at)
        """,
        engine=conn
    )
    return


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ##OTHER
    """)
    return


@app.cell
def _(app_user, conn, mo):
    _df = mo.sql(
        f"""
        Select * from app_user
        """,
        engine=conn
    )
    return


@app.cell(hide_code=True)
def _(mo):
    drop_warning = mo.md(r"""
    ## Drop application tables
    **Destructive:** activating this button permanently deletes all six application
    tables and their data. It does not drop unrelated notebook demo tables.
    """)
    drop_application_tables = mo.ui.run_button(label="Drop all application tables")
    mo.vstack([drop_warning, drop_application_tables])
    return (drop_application_tables,)


@app.cell(hide_code=True)
def _(conn, drop_application_tables, mo):
    if drop_application_tables.value:
        with conn.cursor() as _cursor:
            _cursor.execute(
                """
                DROP TABLE IF EXISTS
                    chat_room,
                    match_request,
                    auth_session,
                    report,
                    study_date_request,
                    app_user
                CASCADE
                """
            )
        drop_status = mo.md("Application tables dropped.")
    else:
        drop_status = mo.md("Click the button above to drop the application tables.")
    drop_status
    return


@app.cell(hide_code=True)
def _(mo):
    seed_heading = mo.md(r"""
    ## Load seed data
    Activating this button runs `database/seed.sql` against the connected PostgreSQL database.
    """)
    load_seed_button = mo.ui.run_button(label="Load seed.sql")
    mo.vstack([seed_heading, load_seed_button])
    return (load_seed_button,)


@app.cell(hide_code=True)
def _(conn, load_seed_button, mo):
    if load_seed_button.value:
        from pathlib import Path

        seed_sql = Path(__file__).with_name("seed.sql").read_text(encoding="utf-8")
        with conn.cursor() as _cursor:
            _cursor.execute(seed_sql)
        seed_status = mo.md("Seed data loaded.")
    else:
        seed_status = mo.md("Click the button above to load seed.sql.")
    seed_status
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
def _(conn, mo):
    _df = mo.sql(
        f"""
        Select * from public.cookielogin
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
