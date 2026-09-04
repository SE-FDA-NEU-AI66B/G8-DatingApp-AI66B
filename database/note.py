import marimo

__generated_with = "0.24.0"
app = marimo.App(width="full", auto_download=["html"], sql_output="native")


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Setup
    """)
    return


@app.cell
def _():
    import os

    import marimo as mo
    import polars as pl
    import psycopg2

    return mo, os, pl, psycopg2


@app.cell
def _(os, psycopg2):
    PGPASS = os.environ.get("PGPASS")
    # coonect to the database
    try:
        uri = f"postgresql://postgres:{PGPASS}@localhost/"
        conn = psycopg2.connect(
            host="localhost",  # Your database host
            database="userdb",  # Your database name
            user="postgres",  # Your database username
            password=PGPASS,  # Your database password
        )
        conn.autocommit = True
        cur = conn.cursor()
    except Exception as e:
        print(e)
    return conn, uri


@app.cell
def _(mo, uri):
    _df = mo.sql(
        f"""
        ATTACH '{uri}' AS database (TYPE postgres);
        """
    )
    return


@app.cell(hide_code=True)
def _(mo):
    mo.md(r"""
    # Other
    """)
    return


@app.cell
def _(pl, uri):
    pl.read_database_uri(query="select * from weather", uri=uri)
    return


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        CREATE TABLE If Not EXISTS database.weather (
            city    VARCHAR,
            temp_lo INTEGER, -- minimum temperature on a day
            temp_hi INTEGER, -- maximum temperature on a day
            prcp    FLOAT,
            date    DATE
        );
        """
    )
    return


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        INSERT INTO database.public.weather
        VALUES ('San Francisco', 46, 50, 0.25, '1994-11-27');
        """
    )
    return


@app.cell
def _(conn, mo, pg_database):
    _df = mo.sql(
        f"""
        SELECT * FROM pg_database;
        """,
        engine=conn
    )
    return


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        Select * from database.information_schema.tables
        """
    )
    return


@app.cell(hide_code=True)
def _(mo):
    _df = mo.sql(
        f"""
        CREATE TABLE database.userdb.public.weather (
            city    VARCHAR,
            temp_lo INTEGER, -- minimum temperature on a day
            temp_hi INTEGER, -- maximum temperature on a day
            prcp    FLOAT,
            date    DATE
        );
        """
    )
    return


@app.cell(hide_code=True)
def _(conn, mo):
    _df = mo.sql(
        f"""
        Select * from database.information_schema
        """,
        engine=conn
    )
    return


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        Select * from database.public.weather
        """
    )
    return


@app.cell
def _():
    print("fsadasdff")
    return


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        CREATE TABLE if not exists weather (
            city    VARCHAR,
            temp_lo INTEGER, -- minimum temperature on a day
            temp_hi INTEGER, -- maximum temperature on a day
            prcp    FLOAT,
            date    DATE
        );
        """
    )
    return


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        CREATE TABLE cities (
            name VARCHAR,
            lat  DECIMAL,
            lon  DECIMAL
        );
        """
    )
    return


@app.cell
def _(mo):
    _df = mo.sql("""
        _df = mo.sql(
            f\"""
            INSERT INTO weather
            VALUES ('San Francisco', 46, 50, 0.25, '1994-11-27');
            \"""
        )
        """)
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        select * from weather
        """
    )
    return


app._unparsable_cell(
    r"""
    "cell.cellActions" = "Ctrl-Shift-p"
    """,
    name="_"
)


@app.cell
def _():
    for i in range(1000000):
        print(i)
    return


@app.cell
def _(a):
    b = a + 3
    print(b)
    return


if __name__ == "__main__":
    app.run()
