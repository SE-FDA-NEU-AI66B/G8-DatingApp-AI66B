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
    from test_us03_us04 import (
        test_us03_active_study_date_feed_contract,
        test_us04_rejects_a_self_match_request,
        test_us04_profile_request_state_contract,
    )

    return (
        test_homepage_renders_expected_content_and_actions,
        test_us03_active_study_date_feed_contract,
        test_us04_rejects_a_self_match_request,
        test_us04_profile_request_state_contract,
    )


@app.cell
def _(
    test_homepage_renders_expected_content_and_actions,
    test_us03_active_study_date_feed_contract,
    test_us04_rejects_a_self_match_request,
    test_us04_profile_request_state_contract,
):
    test_homepage_renders_expected_content_and_actions()
    test_us03_active_study_date_feed_contract()
    test_us04_rejects_a_self_match_request()
    test_us04_profile_request_state_contract()
    return


@app.cell
def _():
    import pytest

    pytest.main(["tests"])
    return


if __name__ == "__main__":
    app.run()
