import marimo

__generated_with = "0.25.2"
app = marimo.App(width="full", auto_download=["html"])


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    ## Setup
    """)
    return


@app.cell(hide_code=True)
def _():
    import os
    import marimo as mo
    import psycopg2
    import os
    from dotenv import load_dotenv
    from database.init import initialize_sample_data, recreate_application_database

    load_dotenv()
    return initialize_sample_data, mo, os, psycopg2, recreate_application_database


@app.cell
def _(os):
    os.environ.get("PGPASS")
    return


@app.cell
def _(os, psycopg2):
    PGPASS = os.environ.get("PGPASS")
    PGHOST = os.environ.get("PGHOST", "localhost")
    PGDATABASE = os.environ.get("PGDATABASE", "userdb")
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
        uri = f"postgresql://postgres:{PGPASS}@{PGHOST}/{PGDATABASE}"
        conn = psycopg2.connect(
            host=PGHOST,
            database=PGDATABASE,
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
    The sample-data button below adds reusable demo records without resetting the schema.
    """)
    recreate_database_button = mo.ui.run_button(label="Recreate application database")
    initialize_sample_data_button = mo.ui.run_button(label="Initialize sample data")
    mo.vstack([app_database_heading, recreate_database_button, initialize_sample_data_button])
    return initialize_sample_data_button, recreate_database_button


@app.cell(hide_code=True)
def _(conn, initialize_sample_data, initialize_sample_data_button, mo, os, recreate_application_database, recreate_database_button):
    if recreate_database_button.value or os.environ.get("RECREATE_DATABASE") == "1":
        try:
            recreate_application_database(conn)
            schema_status = mo.md("Application database recreated.")
        except Exception as error:
            schema_status = mo.md(f"Database recreation failed: `{error}`")
    else:
        schema_status = mo.md("Click the button above to recreate the application database.")
    schema_status
    return


@app.cell(hide_code=True)
def _(conn, initialize_sample_data, initialize_sample_data_button, mo, os):
    if initialize_sample_data_button.value or os.environ.get("INITIALIZE_SAMPLE_DATA") == "1":
        try:
            initialize_sample_data(conn)
            sample_data_status = mo.md(
                "Demo users, study dates, one pending report, an accepted match, and a conversation are ready. Demo users have no password login; configure ADMIN_EMAIL to grant an administrator role."
            )
        except Exception as error:
            sample_data_status = mo.md(f"Sample data initialization failed: `{error}`")
    else:
        sample_data_status = mo.md("Click **Initialize sample data** to add demo records.")
    sample_data_status
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
def _(conn, mo, os, users):
    if os.environ.get("CI_DATABASE_SETUP_ONLY") != "1":
        _df = mo.sql(
            f"""
            Select * from users
            """,
            engine=conn
        )
    return


@app.cell
def _(conn, mo, os, users):
    if os.environ.get("CI_DATABASE_SETUP_ONLY") != "1":
        _df = mo.sql(
            f"""
            DELETE FROM users
            WHERE username='quang';
            """,
            engine=conn
        )
    return


@app.cell
def _(conn, mo, os):
    if os.environ.get("CI_DATABASE_SETUP_ONLY") != "1":
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
