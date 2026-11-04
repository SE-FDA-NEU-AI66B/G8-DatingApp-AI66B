"""HTTP-level admin story tests backed by an isolated PostgreSQL database."""

import os
import subprocess
from pathlib import Path

import pytest


PROJECT_ROOT = Path(__file__).resolve().parents[1]


def _run_database_test(test_name: str) -> None:
    if not os.environ.get("TEST_DATABASE_URL"):
        pytest.skip("requires TEST_DATABASE_URL pointing to an isolated PostgreSQL database")
    subprocess.run(
        [
            "cargo",
            "test",
            "--lib",
            "--features",
            "ssr",
            "--",
            "--ignored",
            "--exact",
            test_name,
        ],
        cwd=PROJECT_ROOT,
        check=True,
    )


def test_us13_t01_health_reports_server_and_database_independently():
    _run_database_test(
        "ss::admin::tests::us13_t01_health_reports_server_and_database_independently"
    )


def test_us14_t01_statistics_count_verified_users_and_active_requests():
    _run_database_test(
        "ss::admin::tests::us14_t01_statistics_count_verified_users_and_active_requests"
    )
