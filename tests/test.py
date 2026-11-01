import marimo

__generated_with = "0.24.2"
app = marimo.App()


@app.cell
def _():
    import marimo as mo
    import sys
    from pathlib import Path

    sys.path.insert(0, str(Path(__file__).resolve().parent))
    from screen.test_homepage import test_homepage_has_neu_message_and_primary_actions

    return (test_homepage_has_neu_message_and_primary_actions,)


@app.cell
def _(test_homepage_has_neu_message_and_primary_actions):
    test_homepage_has_neu_message_and_primary_actions()
    return


if __name__ == "__main__":
    app.run()
