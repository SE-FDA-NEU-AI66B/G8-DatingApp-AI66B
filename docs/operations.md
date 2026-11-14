# Operations and release guide

## 1. Run and release

The public URL is <https://www.dt-fwb.ipv64.de>. `cert/.cloudflared/config.yml`
routes that hostname through Cloudflare Tunnel to the application host at
`http://localhost:3100`. The application and tunnel run on that host; this
repository does not define a hosted application platform or automated deploy.

For development, follow [SETUP.md](SETUP.md) and run `cargo leptos watch`.
For a release, merge the reviewed pull request to `main` after its required CI
checks are green, then have the operator deploy that commit on the tunnel host
with `cargo leptos build --release` and restart the running app process. There
is no service unit or deploy script in this repository, so use the host's
existing process manager. At present, the repository has no active test/build
CI gate; see section 2 before treating a merge as verified.

If a release breaks, restore the previous known-good commit on the host and
rebuild/restart it, or revert the bad change in a pull request and deploy the
reverted `main` commit after checks pass. Confirm the public URL and admin
health page after rollback.

## 2. CI pipeline: install and test

The intended test workflow is stored at
`.github/workflows/ci.yml-not`, so GitHub Actions does not load it. Its steps
are described below for when the file is restored as `ci.yml`. No latest green
test run can be verified from this checkout. The
[Actions runs filtered to `main`](https://github.com/pham-ha-gif/H-iDB/actions?query=branch%3Amain)
page is the place to check; the active `eltonjohn.yml` workflow currently only
checks out the repository because its Cargo build/test commands are comments.
No measured duration is available; budget 5–10 minutes pending an active run.

| Workflow step | What it checks |
|---|---|
| Detect stack (Ubuntu) | Checks out the repository and detects Python project files to decide whether Python jobs run. |
| Python tests (Ubuntu) | Installs `uv`, runs `uv sync`, runs Ruff (currently warning-only), runs pytest with coverage and JUnit output, then always publishes a test summary. |
| Python tests (Windows) | Repeats dependency setup, Ruff, pytest, coverage, and the JUnit summary on Windows; Python 3.13 is installed for the summary step. |
| No secrets committed | Scans tracked text for obvious credential patterns and rejects committed `.env` files, SQL dumps, and database dumps. |

The workflow has no Rust/Leptos build or test step. The repository has no
branch-protection screenshot. Capture the real `main` rules from GitHub
Settings → Branches and add it as
`docs/evident/main-branch-protection.png`; do not treat the existing project
screenshots as branch-protection evidence.

## 3. Monitoring, logs, and health

Application diagnostics go to standard output/error: startup prints the
listening address, and handlers print database, authentication, and OAuth
failure summaries. Read them in the terminal for a foreground process or in
the host process manager's logs (for example, `journalctl -u <app-unit>` if
the host uses systemd). Cloudflare Tunnel has separate process logs. The repo
does not configure a log aggregation or metrics service.

The application handlers do not log passwords, session cookies, or raw
request bodies, and do not intentionally log emails or usernames. Keep log
access restricted and review database error details before sharing logs.

`GET /health` is a public readiness check for the host or tunnel monitor. It
runs `SELECT 1` with a two-second timeout and returns `200 ok` or `503
unhealthy`, without exposing database details. `GET /api/admin/health` still
requires an admin session and reports separate server/database status and
timestamps for the dashboard. Neither endpoint tests Microsoft sign-in, the
tunnel, or full user flows. Watch these two service signals:

- **Error rate:** fraction of HTTP responses that are 5xx; alert if it stays
  above 2% for five minutes.
- **Response time:** p95 request latency; alert if it stays above one second
  for five minutes.

These are proposed thresholds, not currently collected metrics. Until a
metrics tool is configured, check process logs and periodically verify the
public URL, `/health`, and the authenticated admin health view.

## 4. Known risks and technical debt

| Item | Layer | What could happen | Plan |
|---|---|---|---|
| Test workflow is named `ci.yml-not`; active Cargo workflow has build/test commands commented out. | infrastructure | A merge can appear successful without compiling or testing the Rust app. | accepted — this is a course/demo repository; restore a real CI gate before relying on releases. |
| Rust tests that need PostgreSQL are ignored unless `TEST_DATABASE_URL` is configured. | tests | CI can miss database authorization or query regressions. | accepted — local isolated database setup is required until CI provisions one. |
| Release is a manual build/restart on the Cloudflare Tunnel host. | infrastructure | A deploy can use the wrong commit or leave the app offline during restart. | accepted — there is no managed production platform or deploy runner in the repository. |
| No production backup or restore job is configured. | data | Host or disk loss could remove user and conversation data. | accepted — current database workflow is development-oriented; configure encrypted off-host backups before production use. |
| No metrics or log aggregation is configured. | infrastructure | Rising errors or latency may go unnoticed between manual checks. | accepted — current demo operations use process logs and manual health checks. |

## 5. Maintenance plan

| Area | What to do | Owner | How often |
|---|---|---|---|
| Bugs | Triage reports, reproduce, link a GitHub issue, and add a regression test. | Development team; Product Owner prioritizes | Weekly and before each release |
| Dependency updates | Review Rust and Python advisories, update lockfiles, and run build/tests. | Release operator and developer | Monthly; urgent advisories immediately |
| Database backups | Before production, configure encrypted off-host backups and perform a restore drill; development DB is recreated from the notebook. | Database operator | Daily backups; quarterly restore drill |
| Monitoring | Review 5xx rate, p95 latency, process logs, public URL, and admin health endpoint. | Release operator | After each deploy; metrics review weekly |
| Documentation | Recheck setup, endpoints, schema, runbook, and screenshots against the running system. | Change author; reviewer verifies | With every relevant PR; full review each sprint |

## 6. Lessons learned

1. The intended Python CI workflow was left with a `.yml-not` suffix, while
   the active Cargo workflow has its build/test commands commented out. We
   checked the actual workflow files instead of assuming their names meant
   they ran. Next time, verify a real successful Actions run on `main` before
   calling CI complete.
2. Sprint 3 review notes show that real-data and API-error screenshots and an
   independent setup walkthrough were still missing at wrap-up. The team
   added a review-evidence checklist. Next time, capture evidence during
   implementation and schedule the clean-machine walkthrough before the final
   week.
3. The hydration build passed on the host target, but `cargo leptos build`
   exposed a stale wasm standard library from a different nightly. We traced
   the misleading Serde import error to a compiler/target mismatch. Next time,
   verify the wasm target with the pinned project toolchain as part of the
   first build and keep toolchain components in sync.

## 7. Final demo (week 15)

**Demo scenario:** open the public URL, show the student registration and
Microsoft verification path, create a study date, send and accept a match
request, then exchange a chat message. Sign in with the configured admin
account to show dashboard statistics and the separate server/database health
indicators. Finish by showing how the release operator checks logs and rolls
back to the last good commit. This is a presentation script, not a claim that
each step has been captured as release evidence.
