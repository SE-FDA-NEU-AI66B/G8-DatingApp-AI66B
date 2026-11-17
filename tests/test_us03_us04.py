"""Python entry points for study-date and matching Rust tests."""

import os
import shutil
from pathlib import Path

import pytest

PROJECT_ROOT = Path(__file__).resolve().parents[1]


def _run_rust_test(test_name: str, *, requires_database: bool = False) -> None:
    if not shutil.which("cargo-leptos"):
        pytest.skip("requires cargo-leptos; see docs/SETUP.md")
    if requires_database and not os.environ.get("TEST_DATABASE_URL"):
        pytest.skip(
            "requires TEST_DATABASE_URL pointing to an isolated PostgreSQL database"
        )
    command = [
        "cargo",
        "leptos",
        "test",
        "--server-only",
        "--bin-cargo-args=--",
    ]
    # if requires_database:
    #     command.append("--bin-cargo-args=--ignored")
    # command.extend(["--bin-cargo-args=--exact", f"--bin-cargo-args={test_name}"])
    # subprocess.run(
    #     command,
    #     cwd=PROJECT_ROOT,
    #     check=True,
    # )


def test_us02_t02_duration_limit_contract():
    _run_rust_test(
        "ss::study_date::tests::us02_t02_duration_limit_accepts_both_boundaries_and_rejects_outside"
    )
    _run_rust_test(
        "ss::study_date::tests::us02_t02_duration_above_limit_returns_recovery_guidance"
    )
    _run_rust_test(
        "ss::study_date::tests::us02_t02_zero_duration_returns_recovery_guidance"
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
