import marimo

__generated_with = "0.24.2"
app = marimo.App()


@app.cell
def _():
    import pytest

    pytest.main(["-q"])


if __name__ == "__main__":
    app.run()
