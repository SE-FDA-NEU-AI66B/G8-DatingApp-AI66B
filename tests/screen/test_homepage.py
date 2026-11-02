from html.parser import HTMLParser
import os
from pathlib import Path
from urllib.request import urlopen


ENV_FILE = Path(__file__).resolve().parents[2] / ".env"


def homepage_url() -> str:
    configured_url = os.environ.get("TEST_APP_URL")
    if configured_url:
        return configured_url.rstrip("/") + "/"

    for line in ENV_FILE.read_text(encoding="utf-8").splitlines():
        if line.startswith("TEST_APP_URL="):
            return line.partition("=")[2].strip().strip("\"'").rstrip("/") + "/"

    raise AssertionError("TEST_APP_URL is missing from .env")


class HomepageParser(HTMLParser):
    def __init__(self):
        super().__init__()
        self.headings: list[str] = []
        self.links: list[tuple[str, str]] = []
        self._heading_level: str | None = None
        self._heading_text: list[str] = []
        self._link_href: str | None = None
        self._link_text: list[str] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, str | None]]):
        attributes = dict(attrs)
        if tag in {"h1", "h2"}:
            self._heading_level = tag
            self._heading_text = []
        elif tag == "a":
            self._link_href = attributes.get("href")
            self._link_text = []

    def handle_data(self, data: str):
        if self._heading_level:
            self._heading_text.append(data)
        if self._link_href is not None:
            self._link_text.append(data)

    def handle_endtag(self, tag: str):
        if tag == self._heading_level:
            self.headings.append(" ".join(" ".join(self._heading_text).split()))
            self._heading_level = None
        elif tag == "a" and self._link_href is not None:
            label = " ".join(" ".join(self._link_text).split())
            self.links.append((self._link_href, label))
            self._link_href = None


def test_homepage_renders_expected_content_and_actions():
    with urlopen(homepage_url(), timeout=5) as response:
        assert response.status == 200
        html = response.read().decode("utf-8")

    page = HomepageParser()
    page.feed(html)

    assert "Meet someone between classes." in page.headings
    assert any(
        href == "/userhome/study-dates" and "Find a study date" in label
        for href, label in page.links
    )
    assert any(
        href == "/register" and "Create your account" in label
        for href, label in page.links
    )
