# Demo testing

The final demo is the admin console at `/admin_console`; this plan selects
US12–US14 from [traceability](traceability.md). The browser route is a Leptos
shell: authorization is checked by `GET /api/admin/health`, and dashboard data
comes from `GET /api/admin/statistics`. The routes return `401` without a
session and `403` for a signed-in non-admin. An admin receives health and
statistics JSON. The console renders "Admin console", "Registered users",
"Active study dates", "Pending reports", and the server/database health
components. Refresh requests current data; statistics failures show an error
and "Try again", not zero counts.

## Selected story checks

| Story / case | Steps and expected current behavior | Automated evidence |
|---|---|---|
| US12-T01 — Admin allowed | Sign in as an admin and load the console. Admin-only API calls succeed; the statistics endpoint returns three counts. | `test_us13_t01_health_reports_both_components_healthy` and `test_us14_t01_statistics_count_verified_users_and_active_requests` exercise authenticated admin endpoints. Both require `TEST_DATABASE_URL`. |
| US12-T02 — Anonymous denied | Without a session, request `/api/admin/access`, `/api/admin/statistics`, and `/api/admin/health`. Each returns `401` without admin data. `/admin_console` serves its client shell while the auth gate checks the session. | `test_us12_anonymous_user_cannot_access_admin_console_or_data` requires `TEST_APP_URL`. |
| US12-T03 — Role boundary | Repeat the API requests with a regular-user session: each returns `403`; an admin session is allowed. | The non-admin API case is not automated yet; follow-up [issue #95](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/95). |
| US13-T01 — Healthy services | As an admin, request `/api/admin/health` while PostgreSQL is reachable. Both `server.healthy` and `database.healthy` are `true`. | `test_us13_t01_health_reports_both_components_healthy` requires `TEST_DATABASE_URL`. |
| US13-T02 — Database unhealthy | The response must still report the server as healthy and the database as unhealthy; the component states are independent. | `test_us13_t02_database_failure_is_reported_separately_from_server_health` checks the response model. Forcing and asserting this state through the live endpoint remains in issue #95. |
| US14-T01 — Non-zero data | Seed verified/unverified users, active/out-of-window study dates, and pending/reviewed reports. The API counts only verified users, active dates in the next 24 hours, and pending reports. | `test_us14_t01_statistics_count_verified_users_and_active_requests` requires `TEST_DATABASE_URL`. |
| US14-T02 — Zero boundary | With all three counts at zero, zero is valid data and must not be shown as an API failure. Positive counts are covered by US14-T01. | `test_us14_t04_zero_statistics_are_valid_values` checks the serialized zero response model. |
| US14-T03 — Statistics error | A failed statistics request returns an error, and the UI presents an error with "Try again", not zeros. | `test_us14_t03_statistics_failure_has_an_error_message` checks the error model. The HTTP/UI failure and refresh path is in issue #95. |

These remaining issue #95 cases are deliberately not represented as passing:
regular-user authorization, a database failure through the health endpoint,
and statistics HTTP/UI failure and refresh. Microsoft OAuth is also a manual
check because it requires Entra configuration and an authorized account.

## Related business-rule boundary

The study-date form submits minutes to `POST /api/study-dates`; the API accepts
durations from 1 through 720 minutes and returns `422` outside that inclusive
range. The rejection messages explain what happened, why, and how to recover:

- Over the maximum: "This study date is longer than the 12-hour limit. Set the
  duration to 12 hours or less and try again."
- Under the minimum: "This study date is shorter than the 1-minute minimum.
  Choose a duration of at least 1 minute and try again."

`test_us02_t02_duration_limit_contract` runs the Rust validation and response
tests through a Python test entry point: 0 and 721 minutes are rejected, while
1 and 720 are accepted. It does not need a database or running server.

## Run from a clean checkout

Install `uv` and Rust/Cargo as described in [SETUP](SETUP.md), then run:

```sh
uv run --with pytest --no-project pytest -q
```

The command installs only pytest into an ephemeral environment. Tests needing
a running deployment skip unless `TEST_APP_URL` is set. PostgreSQL integration
tests skip unless `TEST_DATABASE_URL` points to a separate database whose
schema has been created for testing. They never initialize or mutate the
development database. Rust-backed unit tests skip if Cargo is not installed.
The test notebook can be opened with `uv run marimo edit tests/test.py`.

## CI run

`.github/workflows/ci.yml` provisions an isolated PostgreSQL service and runs
`uv run python database/init.py --recreate --seed` to create its schema and
sample data. The Marimo setup notebook imports the same functions for manual
database setup. CI then builds the Leptos application and starts it on port
3100. It waits for `/health` before running the complete pytest suite with
`TEST_APP_URL` and `TEST_DATABASE_URL` pointed at that local app and database.
The CI database is disposable; `--recreate` drops its public schema and must
only be used with an isolated database.

## Latest run

| Command | Result |
|---|---|
| `uv run --with pytest --no-project pytest -q --tb=no` | Failed: 7 failed, 9 passed in 0.05s. The seven live endpoint tests tried the `TEST_APP_URL` in `.env` (`0.0.0.0:3100`), where no app was running. Start the app first, or remove that local setting to let those tests skip. This is a local run, not the CI result. |
