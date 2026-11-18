Running app: <https://g8-datingapp-ai66b-1.onrender.com>

# NEUDating

NEUDating is a student-focused app for meeting people with shared interests
and goals.

[![CI](https://github.com/pham-ha-gif/H-iDB/actions/workflows/ci.yml/badge.svg)](https://github.com/pham-ha-gif/H-iDB/actions/workflows/ci.yml)

## Features

- Register with a unique username, NEU email, and password.
- Verify an NEU Outlook address through Microsoft sign-in before password login.
- Create and browse study dates, send and respond to match requests, and use
  the community feed.
- Admin console for service health and application statistics.

## Demo data

The database notebook can add reusable demo users, study dates, reports,
matches, conversations, and posts. Seeded accounts use the password
`password`; the database stores only Argon2 password hashes. The administrator
role is assigned when a user's email matches the server's `ADMIN_EMAIL` value.
Demo records on the public host may differ from local seed data.

## Run locally

Install Rust with `rustup`, Docker Compose, `uv`, `cargo-leptos`, and the
`rustc-codegen-cranelift` Rust component. Copy `.env.example` to `.env` and
set the local PostgreSQL and Microsoft Entra values. See [local setup](docs/SETUP.md)
for the full environment and database instructions.

Start PostgreSQL and initialize the development database from the notebook:

```sh
docker compose --env-file .env -f database/compose.yaml up -d db
uv sync
uv run marimo edit database/note.py
```

In the notebook, use **Recreate application database** and then
**Initialize sample data**. This recreates the development schema and erases
its existing tables. Start the app with:

```sh
cargo leptos watch
```

Open <http://localhost:3100>. Microsoft sign-in requires the Entra settings
in `.env`.

## Tests and CI

The GitHub Actions workflow starts an isolated PostgreSQL database, creates
its schema and sample data, builds the app, starts it, and runs pytest. To run
tests locally, see [testing](docs/testing.md); database integration tests
require `TEST_DATABASE_URL` to point to a separate isolated database.

CI deploys to Render only for a tag whose commit is on `main`. Configure the
`RENDER_DEPLOY_HOOK_URL` GitHub secret for deployments. Production environment
values belong in Render, not in this repository. See the [release guide](docs/release.md)
and [operations guide](docs/operations.md).

## Project links

- [P0 traceability and implementation status](docs/traceability.md)
- [Test plan and coverage limits](docs/testing.md)
- [Sprint 3 review evidence](docs/log/sprint-3-review-evidence.md)
- [Sprint log](docs/sprint-log.md)
- [Project board](https://github.com/orgs/SE-FDA-NEU-AI66B/projects/20)

## Team QHQ

Nguyen Son Hai (11247282), Nguyen Minh Quang (11247344), Le Duy Quyen
(11247345), and Phạm Hà (`pham-ha-gif`).
