"""Python entry points for the database-independent US03/US04 Rust tests."""

import subprocess
from pathlib import Path


PROJECT_ROOT = Path(__file__).resolve().parents[1]


def _run_rust_test(test_name: str) -> None:
    subprocess.run(
        ["cargo", "test", "--lib", "--features", "ssr", "--", "--exact", test_name],
        cwd=PROJECT_ROOT,
        check=True,
    )


def test_us03_active_study_date_feed_contract():
    _run_rust_test(
        "ss::study_date::tests::us03_feed_includes_only_active_requests_overlapping_the_next_day"
    )


def test_us04_profile_request_state_contract():
    _run_rust_test(
        "ss::matching::tests::us04_profile_relations_show_request_state_and_hide_incomplete_accounts"
    )


def test_us04_rejects_a_self_match_request():
    _run_rust_test(
        "ss::matching::tests::us04_cannot_send_a_match_request_to_yourself"
    )
