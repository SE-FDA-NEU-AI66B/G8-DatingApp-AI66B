import marimo

__generated_with = "0.24.0"
app = marimo.App(width="full", auto_download=["html"])


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
    # note if ur new to marimo dont worry about duckdb marimo auto useit if available;since both duckdb and and polars use arrow data format duckdb intergration with marimo can read all polars object and so is the otherway round
    import psycopg2
    import polars as pl  # arrow conpatable
    from psycopg2 import sql

    return mo, os, pl, psycopg2


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
    # Other
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


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        --DROP TABLE if  exists userdb.public.cookielogin;
        CREATE TABLE if Not exists userdb.public.cookielogin (
            cookie  BYTEA,
            userid UHUGEINT,
            start DATETIME,
        );
        """
    )
    return


@app.cell(hide_code=True)
def _(mo):
    _df = mo.sql(
        f"""
        INSERT INTO userdb.public.cookielogin (cookie,userid,start)
        VALUES (from_hex('AA'),3,now())
        """
    )
    return


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        EXPLAIN ANALYZE SELECT * FROM userdb.information_schema.tables WHERE table_schema = 'public'
        """
    )
    return


@app.cell
def _(mo):
    aa = mo.sql(
        f"""
        SELECT hex(cookie) as hex, * FROM userdb.public.cookielogin
        """
    )
    return


@app.cell
def _(pl, uri):
    pl.read_database_uri("select * from cookielogin",uri=uri)
    return


@app.cell(hide_code=True)
def _(conn, mo):
    a = mo.sql(
        f"""
        select cookie from public.cookielogin Limit 3
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


@app.cell
def _(mo):
    _df = mo.sql(
        f"""
        CREATE TABLE if Not exists weather (
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
        CREATE TABLE IF Not EXISTS cities (
            name VARCHAR,
            lat  DECIMAL,
            lon  DECIMAL
        );
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        INSERT INTO weather
        VALUES ('San Francisco', 46, 50, 0.25, '1994-11-27');
        """
    )
    return


@app.cell
def _(cities, mo):
    _df = mo.sql(
        f"""
        INSERT INTO cities
        VALUES ('San Francisco', -194.0, 53.0);
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        INSERT INTO weather (city, temp_lo, temp_hi, prcp, date)
        VALUES ('San Francisco', 43, 57, 0.0, '1994-11-29');
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        INSERT INTO weather (date, city, temp_hi, temp_lo)
        VALUES ('1994-11-29', 'Hayward', 54, 37);
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        SELECT max(temp_lo)
        FROM weather;
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        SELECT city
        FROM weather
        WHERE temp_lo = (SELECT max(temp_lo) FROM weather);
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        SELECT city, max(temp_lo)
        FROM weather
        GROUP BY city;
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        SELECT city, max(temp_lo)
        FROM weather
        GROUP BY city
        HAVING max(temp_lo) < 40;
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        SELECT city, max(temp_lo)
        FROM weather
        WHERE city LIKE 'S%'            -- (1)
        GROUP BY city
        HAVING max(temp_lo) < 40;
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        UPDATE weather
        SET temp_hi = temp_hi - 2,  temp_lo = temp_lo - 2
        WHERE date > '1994-11-28';
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        SELECT city, (temp_hi + temp_lo) / 2 AS temp_avg, date
        FROM weather;
        """
    )
    return


@app.cell
def _(mo, weather):
    _df = mo.sql(
        f"""
        select * from weather
        """
    )
    return


@app.cell
def _(cities, mo):
    _df = mo.sql(
        f"""
        select * from cities
        """
    )
    return


if __name__ == "__main__":
    app.run()
