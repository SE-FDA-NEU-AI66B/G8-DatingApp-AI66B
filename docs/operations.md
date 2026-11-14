# Operations guide

## Run

For local development, follow [SETUP.md](SETUP.md). Start PostgreSQL with
Docker Compose, recreate the development schema through `database/note.py`,
then run `cargo leptos watch`.

## Monitor

Public readiness is `GET /health`. It runs `SELECT 1` with a two-second
timeout and returns `200 ok` or `503 unhealthy`. Admin-only
`GET /api/admin/health` reports server and database status. Neither endpoint
checks Microsoft sign-in or the Cloudflare Tunnel.

The app writes startup and error summaries to stdout/stderr. Read them in the
foreground terminal or the self-host's process manager; Cloudflare Tunnel has
separate logs. App handlers do not log passwords, session cookies, or raw
request bodies. No metrics or log aggregation service is configured. Once
metrics are available, track 5xx rate and p95 response time; until then check
logs, the public URL, `/health`, and admin health after deploys.

## Database and deployment limits

PostgreSQL is self-hosted on the app host. The Compose port binds to loopback;
the app connects through `DATABASE_URL`. Development schema creation belongs
to the notebook, not server startup. The repository does not configure
production backups, a managed database, or automated deployment. See the
[release guide](release.md) for deploy and rollback steps.
