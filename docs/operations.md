# Operations guide

## 1. Run and release

URL: <https://www.dt-fwb.ipv64.de>, routed by Cloudflare Tunnel to
`localhost:3100` (`cert/.cloudflared/config.yml`). App and tunnel run on the
host; deployment is manual.

Develop with [SETUP.md](SETUP.md) and `cargo leptos watch`. Release by merging
to `main` after CI passes, then build with `cargo leptos build --release` and
restart the app through the host's process manager. No deploy script or service
unit is defined here; the current workflows do not provide an active build/test
gate (section 2).

Rollback by rebuilding/restarting the previous good commit, or revert the
change and deploy `main`. Verify the public URL and admin health page.

## 2. CI pipeline: install and test

`.github/workflows/ci.yml` is the active build and test workflow. No latest
green run or measured duration is recorded here; check the [main Actions
runs](https://github.com/pham-ha-gif/H-iDB/actions?query=branch%3Amain).

| Workflow step | What it checks |
| --- | --- |
| PostgreSQL service | Starts an isolated PostgreSQL 17 database and waits for readiness. |
| Python setup | Installs Python 3.14 and dependencies from `uv.lock`. |
| Database setup | Executes the schema recreation and sample-data actions in `database/note.py` against the isolated CI database. |
| Rust build | Installs nightly Rust and `cargo-leptos`, then builds the Leptos app. |
| Start and health | Starts the app on port 3100 and waits for `/health`. |
| Pytest | Runs Python tests against the live local app and isolated database; publishes test totals and displays app logs if a step fails. |

There is no Rust/Leptos step or branch-protection screenshot. Capture the
actual `main` rules from GitHub Settings → Branches at
`docs/evident/main-branch-protection.png`.

## 3. Monitoring, logs, and health

App diagnostics go to stdout/stderr: startup address and database,
authentication, and OAuth failure summaries. Read foreground output or host
process logs (for example, `journalctl -u <app-unit>`); Tunnel logs are
separate. No log aggregation or metrics service is configured.

Handlers do not log passwords, session cookies, raw request bodies, or
intentionally log emails/usernames. Restrict log access; review database errors
before sharing logs.

`GET /health` runs `SELECT 1` (2-second timeout): `200 ok` or `503 unhealthy`.
`GET /api/admin/health` requires an admin session and shows server/database
status and timestamps. Neither checks sign-in, tunnel, or full user flows.
Watch:

- **Error rate:** 5xx responses; alert above 2% for five minutes.
- **Response time:** p95 latency; alert above one second for five minutes.

Thresholds are proposed, not measured. Until metrics are configured, review
logs and check the URL, `/health`, and admin health view.

## 4. Known risks and technical debt

| Item                                                                                              | Layer          | What could happen                                                          | Plan                                                                                                                      |
| ------------------------------------------------------------------------------------------------- | -------------- | -------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Item | Layer | What could happen | Plan |
| --- | --- | --- | --- |
| CI config is inactive; Cargo checks are commented out. | infrastructure | Rust regressions reach `main`. | accepted — enable CI before production use. |
| PostgreSQL tests need `TEST_DATABASE_URL`. | tests | CI misses database regressions. | accepted — CI needs an isolated database. |
| Deployment is manual on the Tunnel host. | infrastructure | Wrong commit or downtime. | accepted — no deploy runner is configured. |
| No production backup/restore job. | data | Host loss removes user data. | accepted — configure encrypted off-host backups before production. |
| No metrics or log aggregation. | infrastructure | Errors/latency go unnoticed. | accepted — demo uses manual checks. |

## 5. Maintenance plan

| Area               | What to do                                                                                                                          | Owner                                       | How often                                       |
| ------------------ | ----------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------- | ----------------------------------------------- |
| Area | What to do | Owner | How often |
| --- | --- | --- | --- |
| Bugs | Triage, reproduce, file issue, add regression test. | Dev team; Product Owner prioritizes | Weekly; before release |
| Dependencies | Review advisories, update lockfiles, build/test. | Developer and release operator | Monthly; urgent fixes immediately |
| Backups | Configure encrypted off-host backup; test restore. Dev DB is notebook-recreated. | Database operator | Daily; quarterly restore drill |
| Monitoring | Review errors, latency, logs, URL, health endpoints. | Release operator | After deploy; metrics weekly |
| Docs | Verify setup, API, schema, runbook, screenshots. | Author; reviewer verifies | Relevant PRs; full sprint review |

## 6. Lessons learned

1. CI files looked active but checks were disabled. We inspected the workflow
   contents; next time, verify a green `main` run before claiming CI works.
2. Review evidence and an independent setup walkthrough were missing late in
   the project. We added an evidence checklist; next time, capture evidence as
   we work and schedule a clean-machine setup early.
3. A stale wasm standard library caused a misleading Serde error. We found the
   toolchain mismatch; next time, verify wasm targets during the first build.
