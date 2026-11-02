import json
from urllib.error import HTTPError
from urllib.request import Request, urlopen

from screen.test_registration import app_url


def test_us02_t04_anonymous_user_cannot_create_study_date():
    request = Request(
        f"{app_url()}/api/study-dates",
        data=json.dumps(
            {"starts_at": "2099-01-01T12:00", "duration_min": 60}
        ).encode("utf-8"),
        headers={"Content-Type": "application/json"},
        method="POST",
    )

    try:
        response = urlopen(request, timeout=10)
    except HTTPError as error:
        response = error

    with response:
        assert response.status == 401
