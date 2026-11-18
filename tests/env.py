"""Load repository environment variables for Python test entry points."""

import os
from pathlib import Path


REPOSITORY_ROOT = Path(__file__).resolve().parents[1]


def load_test_env() -> None:
    env_file = REPOSITORY_ROOT / ".env"
    if not env_file.is_file():
        return

    for raw_line in env_file.read_text(encoding="utf-8").splitlines():
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        if line.startswith("export "):
            line = line[7:].lstrip()
        key, separator, value = line.partition("=")
        if not separator or not key.strip():
            continue

        value = value.strip()
        if len(value) >= 2 and value[0] == value[-1] and value[0] in "\"'":
            value = value[1:-1]
        elif " #" in value:
            value = value.partition(" #")[0].rstrip()

        os.environ.setdefault(key.strip(), value)
