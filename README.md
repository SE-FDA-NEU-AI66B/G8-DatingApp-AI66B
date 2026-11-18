https://www.dt-fwb.ipv64.de

# NEUDating

NEUDating is a student-focused app for meeting people with shared interests and goals.

[![CI](https://github.com/pham-ha-gif/H-iDB/actions/workflows/ci.yml/badge.svg)](https://github.com/pham-ha-gif/H-iDB/actions/workflows/ci.yml)

## Test accounts

The database notebook can seed these demo usernames. It sets their password hashes to `NULL`, so **there is no demo password login**. Sign-in requires a verified NEU email through Microsoft; the seeded accounts are not guaranteed to be available on the public host. Configure an account's email as `ADMIN_EMAIL` on the server to grant it administrator access.

| Role | Seeded username | Password |
|---|---|---|
| Regular user | `alex_demo`, `linh_demo`, `minh_demo` | Not configured |
| Admin-capable seed | `admin` | Not configured; role is assigned by `ADMIN_EMAIL` |

## P0 stories and starting points

Routes below are the app's starting screens. Most signed-in stories require a verified account; the presence of a URL does not guarantee public-host access. Current implementation status and test gaps are in [P0 traceability](docs/traceability.md).

- **US01 Register:** [`/register`](https://www.dt-fwb.ipv64.de/register)
- **US02 Create a study date:** [`/userhome/study-date/new`](https://www.dt-fwb.ipv64.de/userhome/study-date/new) (after sign-in)
- **US03 Browse study dates:** [`/userhome/study-dates`](https://www.dt-fwb.ipv64.de/userhome/study-dates) (after sign-in)
- **US04 Send a match request:** start at [`/userhome/study-dates`](https://www.dt-fwb.ipv64.de/userhome/study-dates), then open a profile at `/profile/:id` (after sign-in)
- **US05 Accept or decline a request:** [`/userhome/requests`](https://www.dt-fwb.ipv64.de/userhome/requests) (after sign-in)
- **US12–US14 Admin access, health, and statistics:** [`/admin_console`](https://www.dt-fwb.ipv64.de/admin_console) (admin account required)
- **Service health:** [`/health`](https://www.dt-fwb.ipv64.de/health)

## CI

The CI workflow builds the Leptos application, starts it against an isolated PostgreSQL service, waits for `/health`, and runs pytest against the local server. It runs on pushes and pull requests targeting `main`. Successful CI runs on `main` trigger a Render deploy hook; configure the repository secret `RENDER_DEPLOY_HOOK_URL` with the service's Render deploy hook URL. Render automatic deploy is disabled in [`render.yaml`](render.yaml) so deployments wait for CI.

## Team QHQ

Nguyễn Sơn Hải (`hai291`, Product Owner) · Nguyễn Minh Quang (`iambadwithname`) · Phạm Hà (`pham-ha-gif`). See the [project board](https://github.com/orgs/SE-FDA-NEU-AI66B/projects/20).

## Hosting and setup

The public URL is `https://www.dt-fwb.ipv64.de`. The checked-in [`render.yaml`](render.yaml) describes a Render web service with automatic deploy enabled, but does not configure a CI deployment gate. Confirm deployment availability, seeded data, and sign-in on the host before relying on the public demo. Hosting and release details are in the [release guide](docs/release.md) and [operations guide](docs/operations.md).

For local setup, see [SETUP.md](docs/SETUP.md). The database notebook owns development schema recreation and sample data initialization. Keep `.env`, database credentials, and platform secrets out of Git.

## Testing and project records

- [Test plan and current coverage limits](docs/testing.md)
- [Sprint 3 review evidence index](docs/log/sprint-3-review-evidence.md) · [evidence screenshots](docs/evident/)
- [Sprint log](docs/sprint-log.md) · [repository](https://github.com/pham-ha-gif/H-iDB)
