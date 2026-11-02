import json
import os
from pathlib import Path
from urllib.error import HTTPError
from urllib.request import Request, urlopen


ENV_FILE = Path(__file__).resolve().parents[2] / ".env"


def app_url() -> str:
    configured_url = os.environ.get("TEST_APP_URL")
    if configured_url:
        return configured_url.rstrip("/")

    if ENV_FILE.exists():
        for line in ENV_FILE.read_text(encoding="utf-8").splitlines():
            if line.startswith("TEST_APP_URL="):
                return line.partition("=")[2].strip().strip("\"'").rstrip("/")

    raise AssertionError("TEST_APP_URL is missing from .env")


def get_status(path: str) -> tuple[int, str]:
    try:
        with urlopen(app_url() + path, timeout=5) as response:
            return response.status, response.read().decode("utf-8")
    except HTTPError as error:
        return error.code, error.read().decode("utf-8")


def post_status(path: str) -> tuple[int, str]:
    request = Request(app_url() + path, method="POST")
    try:
        with urlopen(request, timeout=5) as response:
            return response.status, response.read().decode("utf-8")
    except HTTPError as error:
        return error.code, error.read().decode("utf-8")


def test_us05_anonymous_user_cannot_read_or_respond_to_match_requests():
    # US05-T03: no session means no access to incoming requests or decisions.
    for path in (
        "/api/match-requests/incoming",
        "/api/match-requests/1/accept",
        "/api/match-requests/1/decline",
    ):
        status, _ = get_status(path) if path.endswith("incoming") else post_status(path)
        assert status == 401, f"{path} should reject an anonymous user"


def test_us12_anonymous_user_cannot_access_admin_console_or_data():
    # US12-T02: the page uses the auth gate; all admin data endpoints reject
    # anonymous requests without returning data.
    page_status, page = get_status("/admin_console")
    assert page_status == 200
    assert "Checking sign-in" in page or "Admin dashboard" not in page

    for path in ("/api/admin/access", "/api/admin/statistics", "/api/admin/health"):
        status, body = get_status(path)
        assert status == 401, f"{path} should reject an anonymous user"
        if body:
            try:
                payload = json.loads(body)
            except json.JSONDecodeError:
                continue
            assert "statistics" not in payload
            assert "database" not in payload
