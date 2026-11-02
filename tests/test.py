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
    from screen.test_us05_us12 import (
        test_us05_anonymous_user_cannot_read_or_respond_to_match_requests,
        test_us12_anonymous_user_cannot_access_admin_console_or_data,
    )

    return (
        test_homepage_renders_expected_content_and_actions,
        test_us05_anonymous_user_cannot_read_or_respond_to_match_requests,
        test_us12_anonymous_user_cannot_access_admin_console_or_data,
    )


@app.cell
def _(
    test_homepage_renders_expected_content_and_actions,
    test_us05_anonymous_user_cannot_read_or_respond_to_match_requests,
    test_us12_anonymous_user_cannot_access_admin_console_or_data,
):
    test_homepage_renders_expected_content_and_actions()
    test_us05_anonymous_user_cannot_read_or_respond_to_match_requests()
    test_us12_anonymous_user_cannot_access_admin_console_or_data()
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
