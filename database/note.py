import marimo

__generated_with = "0.24.0"
app = marimo.App(width="medium", auto_download=["html"])


@app.cell
def _():
    import os
    import psycopg2
    import sqlglot
    import polars as pl
    import marimo as mo
    import connectorx as cx

    return mo, os, pl, psycopg2


@app.cell
def _(os, psycopg2):
    PGPASS = os.environ.get("PGPASS")
    # coonect to the database
    try:
        uri = f"postgresql://postgres:{PGPASS}@localhost/"
        conn = psycopg2.connect(
            host="localhost",  # Your database host
            #database="dvdrental",  # Your database name
            user="postgres",  # Your database username
            password=PGPASS,  # Your database password
        )
        conn.autocommit=True
        cur = conn.cursor()
    except Exception as e:
        print(e)
    return conn, uri


@app.cell
def _(uri):
    uri
    return


@app.cell(hide_code=True)
def _(mo, uri):
    _df = mo.sql(
        f"""
        ATTACH '{uri}' AS database (TYPE postgres)
        """
    )
    return


@app.cell(hide_code=True)
def _(mo, uri):
    _df = mo.sql(
        f"""
        ATTACH '{uri}/information_schema' AS database2 (TYPE postgres)
        """
    )
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


@app.cell(hide_code=True)
def _(conn, mo):
    _df = mo.sql(
        f"""
        Select * from postgres.public.weather
        """,
        engine=conn
    )
    return


@app.cell
def _(conn, mo):
    _df = mo.sql(
        f"""
        Select * from postgres.information_schema.tables
        """,
        engine=conn
    )
    return


@app.cell(hide_code=True)
def _(mo):
    _df = mo.sql(
        f"""
        CREATE TABLE weather (
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
def _(mo):
    _df = mo.sql(
        f"""
        Select * from database.information_schema
        """
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


if __name__ == "__main__":
    app.run()
