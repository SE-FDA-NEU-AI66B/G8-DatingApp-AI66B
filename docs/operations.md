# Operations

## 1. How to run and release

The production platform is Render Web Service. The URL is recorded as the
first line of `README.md` after the service is created. Render deploys a
merge to `main` after CI is green. PostgreSQL is provided by Neon or Supabase
through `DATABASE_URL`; secrets stay in Render environment settings.

To roll back, select the last good commit in Render and redeploy it. If the
rollback needs to be permanent, revert the breaking merge and let CI deploy
the revert.

## 2. The CI pipeline

`ci.yml` checks out the repository, runs the Python tests, discovers the
Playwright tests, and scans tracked files for credentials and database dumps.
The Rust release build must also pass before the first production release.
A normal run should take under 10 minutes. The latest green `main` run is
recorded here: `<add GitHub Actions run URL>`.

Branch protection for `main` must require a pull request, one approval, and a
green CI check. Save the settings screenshot with the final submission.

## 3. Monitoring

Application errors are written to the platform logs with operation context.
Logs never contain passwords, tokens, database URLs, or personal profile data.
Render exposes the service logs in its dashboard.

`GET /health` runs `SELECT 1`; it returns 200 when the app can reach
PostgreSQL and 503 otherwise. Watch error rate and response time. A failed
health check or a sudden increase in either metric triggers a log review and,
if needed, rollback.

## 4. Known risks and technical debt

| Item | Layer | What could happen | Plan |
|---|---|---|---|
| Free service sleeps | infrastructure | First request is slow | accepted: warn demo reviewers |
| No external alerting | infrastructure | Outage is noticed late | #145: add production monitoring |
| OTP mail stub | code | Production login is unsuitable | #146: prepare safe production authentication |
| Sample data is optional | data | A new database starts empty | #140: configure persistent sample data |
| Manual release URL update | docs | Submission link is stale | #142: verify the v1.0 release |

## 5. Maintenance plan

| Area | What to do | Owner | How often |
|---|---|---|---|
| Bugs | Triage, reproduce, and release fixes | Team | Each report |
| Dependencies | Review Rust, Node, and action updates | SM | Monthly |
| Backups | Verify hosted PostgreSQL backups and restore point | PO | Weekly |
| Monitoring | Check logs, `/health`, error rate, and latency | SM | Each demo day |
| Documentation | Compare README and this runbook with the platform | PO | Every release |

## 6. Lessons learned

1. Localhost configuration did not work on a hosted service; environment-based
   database and port configuration fixes this, and we would deploy earlier.
2. A health screen protected by admin login was not an operational health
   check; a public dependency check is now separate, and we would define it
   during architecture work.
3. Release evidence is easy to lose; the checklist links the URL, CI run, and
   tag, and we would capture it during every sprint.

## 7. Final submission checklist

Before submission, verify that the first line of `README.md` is the live
HTTPS URL, the test accounts are documented without real passwords, and the
CI badge links to a green run on `main`. Confirm that branch protection
requires a pull request, approval, and green check. Confirm that tag `v1.0`
points to the commit running at the URL, then export the required
`Team08_M5.pdf` evidence.
