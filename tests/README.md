# Tests

Run the clean-checkout suite with:

```sh
uv run --with pytest --no-project pytest -q
```

Tests for live routes require `TEST_APP_URL`; PostgreSQL integration tests
require `TEST_DATABASE_URL` pointed at an isolated test database. Both groups
skip when their configuration is absent. Rust tests are referenced by Python
tests in this directory, as required by [RULES.md](RULES.md).

For the selected demo stories and the latest run result, see
[docs/testing.md](../docs/testing.md).
