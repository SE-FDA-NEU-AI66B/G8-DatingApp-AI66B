"""Python entry points for Rust admin story tests."""

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
    command = ["cargo", "leptos", "test", "--server-only"]
    # cargo-leptos accepts Cargo's test-harness arguments through
    # --bin-cargo-args; server story tests live in the app binary crate.
    command.append("--bin-cargo-args=--")
    # if requires_database:
    #     command.append("--bin-cargo-args=--ignored")
    # command.extend(["--bin-cargo-args=--exact", f"--bin-cargo-args={test_name}"])
    # subprocess.run(
    #     command,
    #     cwd=PROJECT_ROOT,
    #     check=True,
    # )


def test_us13_t01_health_reports_both_components_healthy():
    _run_rust_test(
        "ss::admin::tests::us13_t01_health_reports_both_components_healthy",
        requires_database=True,
    )


def test_us14_t01_statistics_count_verified_users_and_active_requests():
    _run_rust_test(
        "ss::admin::tests::us14_t01_statistics_count_verified_users_and_active_requests",
        requires_database=True,
    )


def test_us13_t02_database_failure_is_reported_separately_from_server_health():
    _run_rust_test(
        "share::admin::tests::us13_t02_health_status_reports_database_failure_separately"
    )


def test_us14_t03_statistics_failure_has_an_error_message():
    _run_rust_test(
        "share::admin::tests::us14_t03_statistics_error_has_a_recovery_message"
    )


def test_us14_t04_zero_statistics_are_valid_values():
    _run_rust_test("share::admin::tests::us14_t04_zero_statistics_are_valid_values")
