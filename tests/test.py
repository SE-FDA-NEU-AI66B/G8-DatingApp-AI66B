import marimo

__generated_with = "0.24.2"
app = marimo.App()


@app.cell
def _():
    import marimo as mo
    import sys
    from pathlib import Path

    sys.path.insert(0, str(Path(__file__).resolve().parent))
    from screen.test_homepage import test_homepage_renders_expected_content_and_actions
    from screen.test_registration import (
        test_us01_t02_rejects_invalid_username,
        test_us01_t02_rejects_non_neu_email,
        test_us01_t02_rejects_short_password,
    )
    from screen.test_study_date import (
        test_us02_t04_anonymous_user_cannot_create_study_date,
    )

    return (
        test_homepage_renders_expected_content_and_actions,
        test_us01_t02_rejects_invalid_username,
        test_us01_t02_rejects_non_neu_email,
        test_us01_t02_rejects_short_password,
        test_us02_t04_anonymous_user_cannot_create_study_date,
    )


@app.cell
def _(test_homepage_renders_expected_content_and_actions):
    test_homepage_renders_expected_content_and_actions()
    return


@app.cell
def _(
    test_us01_t02_rejects_invalid_username,
    test_us01_t02_rejects_non_neu_email,
    test_us01_t02_rejects_short_password,
):
    test_us01_t02_rejects_non_neu_email()
    test_us01_t02_rejects_invalid_username()
    test_us01_t02_rejects_short_password()
    return


@app.cell
def _(test_us02_t04_anonymous_user_cannot_create_study_date):
    test_us02_t04_anonymous_user_cannot_create_study_date()
    return


@app.cell
def _():
    import pytest

    pytest.main(["tests"])
    return


if __name__ == "__main__":
    app.run()
