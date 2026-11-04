"""Python entry points for US03/US04 Rust tests."""

import os
import subprocess
from pathlib import Path

import pytest


PROJECT_ROOT = Path(__file__).resolve().parents[1]


def _run_rust_test(test_name: str, *, requires_database: bool = False) -> None:
    if requires_database and not os.environ.get("TEST_DATABASE_URL"):
        pytest.skip("requires TEST_DATABASE_URL pointing to an isolated PostgreSQL database")
    command = ["cargo", "test", "--lib", "--features", "ssr"]
    if requires_database:
        command.append("--")
        command.append("--ignored")
    else:
        command.append("--")
    command.extend(["--exact", test_name])
    subprocess.run(
        command,
        cwd=PROJECT_ROOT,
        check=True,
    )


def test_us03_active_study_date_feed_contract():
    _run_rust_test(
        "ss::study_date::tests::us03_feed_includes_only_active_requests_overlapping_the_next_day"
    )


def test_us04_profile_request_state_contract():
    _run_rust_test(
        "ss::matching::tests::us04_profile_relations_show_request_state_and_hide_incomplete_accounts",
        requires_database=True,
    )


def test_us04_rejects_a_self_match_request():
    _run_rust_test(
        "ss::matching::tests::us04_cannot_send_a_match_request_to_yourself",
        requires_database=True,
    )
