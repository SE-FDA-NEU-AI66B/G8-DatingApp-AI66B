https://www.dt-fwb.ipv64.de

# NEUDating — Milestone 5 submission

NEUDating is a student-focused dating app for meeting people with shared interests and goals.

[![Cargo Build & Test workflow](https://github.com/pham-ha-gif/H-iDB/actions/workflows/eltonjohn.yml/badge.svg)](https://github.com/pham-ha-gif/H-iDB/actions/workflows/eltonjohn.yml)

**CI status caveat:** the current workflow checks out the code, but its Cargo build and test commands are commented out. A green badge does not mean the Rust app was built or tested.

## Demo accounts

Local notebook seed accounts (created by the sample-data action in `database/note.py`):

| Role | Test account | Local demo password |
|---|---|---|
| Admin | `admin` | `password` |
| Student | `alex_demo`, `linh_demo`, `minh_demo` | `password` |

These are demo-only credentials for the local seeded development database, not real user credentials. Availability of these accounts on the public host is not guaranteed.

## P0 stories and starting points

- **US01 Register:** [`/register`](https://www.dt-fwb.ipv64.de/register)
- **US02 Create a study date:** [`/userhome/study-date/new`](https://www.dt-fwb.ipv64.de/userhome/study-date/new)
- **US03 Browse study dates:** [`/userhome/study-dates`](https://www.dt-fwb.ipv64.de/userhome/study-dates)
- **US04 Send a match request:** start at [`/userhome/study-dates`](https://www.dt-fwb.ipv64.de/userhome/study-dates), then open a profile at `/profile/:id`
- **US05 Accept or decline a request:** [`/userhome/requests`](https://www.dt-fwb.ipv64.de/userhome/requests)
- **US12–US14 Admin access, health, and statistics:** [`/admin_console`](https://www.dt-fwb.ipv64.de/admin_console)

Story status and evidence: [P0 traceability](docs/traceability.md).

## Team QHQ

Nguyễn Sơn Hải (`hai291`, Product Owner) · Nguyễn Minh Quang (`iambadwithname`) · Phạm Hà (`pham-ha-gif`). See the [project board](https://github.com/orgs/SE-FDA-NEU-AI66B/projects/20).

## Hosting and setup

The app and PostgreSQL are self-hosted. Cloudflare Tunnel routes the HTTPS URL to the app on the host at `localhost:3100`; PostgreSQL runs in Docker Compose and is bound to host loopback. This repository does not configure managed hosting, automated deployment, or a production database service. Keep `.env` and tunnel credentials on the host, outside Git. The checked-in database seed is demo data, and the development database notebook recreates the schema.

For local setup, see [setup](docs/SETUP.md). Production/release and monitoring details: [release guide](docs/release.md) · [operations guide](docs/operations.md).

## Testing and repository evidence

- [Test plan and current coverage limits](docs/testing.md)
- [Sprint 3 review evidence index](docs/log/sprint-3-review-evidence.md) · [evidence screenshots](docs/evident/)
- [Sprint log](docs/sprint-log.md) · [repository](https://github.com/pham-ha-gif/H-iDB)
