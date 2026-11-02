import json
import os
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen


ENV_FILE = Path(__file__).resolve().parents[2] / ".env"


def app_url() -> str:
    configured_url = os.environ.get("TEST_APP_URL")
    if not configured_url:
        for line in ENV_FILE.read_text(encoding="utf-8").splitlines():
            if line.startswith("TEST_APP_URL="):
                configured_url = line.partition("=")[2].strip().strip("\"'")
                break

    if not configured_url:
        raise AssertionError("TEST_APP_URL is missing from .env")
    return configured_url.rstrip("/")


def submit_registration(email: str, username: str, password: str) -> tuple[int, dict]:
    request = Request(
        f"{app_url()}/api/register",
        data=json.dumps(
            {"email": email, "username": username, "password": password}
        ).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        response = urlopen(request, timeout=10)
    except HTTPError as error:
        response = error

    with response:
        return response.status, json.loads(response.read().decode("utf-8"))


def test_us01_t02_rejects_non_neu_email():
    status, body = submit_registration(
        "student@example.com", "test_student", "a-long-test-password"
    )

    assert status == 400
    assert body["message"]


def test_us01_t02_rejects_invalid_username():
    status, body = submit_registration(
        "student@neu.edu.vn", "ab", "a-long-test-password"
    )

    assert status == 400
    assert body["message"]


def test_us01_t02_rejects_short_password():
    status, body = submit_registration("student@neu.edu.vn", "test_student", "short")

    assert status == 400
    assert "at least 12 characters" in body["message"]
