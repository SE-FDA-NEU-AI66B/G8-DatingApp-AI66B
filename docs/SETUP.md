# NEUDating setup

## Prerequisites

- Git 2.40+
- Rust 1.85+ with `rustup`; the repository pins the nightly toolchain
- Docker Desktop 4.30+ and Docker Compose
- `uv` for the database notebook
- Node.js 20+ and npm 10+ for browser checks
- Windows 10/11, macOS 13+, or a current Linux distribution

## Clone and configure

```bash
git clone https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B.git
cd G8-DatingApp-AI66B
```

## Install build tools

```bash
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
```

## Set up the database

Set `PGPASS` in `.env` to a local password. Keep the password in
`DATABASE_URL` the same; the database name is `userdb`.

```bash
docker compose --env-file .env -f database/compose.yaml up -d db
uv run marimo edit database/note.py
```

In the notebook, click **Recreate application database** once to create the
application tables, then click **Initialize sample data** if you want demo
accounts and records. Recreating drops and replaces every table in the
database's `public` schema, so only use it to reset this development database.
The server does not create the schema; the notebook owns that step.

Keep the notebook open only while setting up the database. Start the app in a
separate terminal after closing it:

The notebook and server both read their connection settings from `.env`.
Compose persists database files in a Docker volume, so stopping the container
does not reset the database.

The configured `ADMIN_EMAIL` is promoted to admin on server startup. Sign in
through Microsoft using that account to use the dashboard.

## How to know it worked

Open <http://localhost:3100>, sign in with the configured admin account, then open
<http://localhost:3100/admin_console>. The page shows non-zero values for
registered users, active study-date requests, and pending reports, plus
separate server and database health statuses.

## Validation

```bash
cargo fmt --all -- --check
cargo check --features ssr
cargo test --lib --features ssr
```

For browser checks:

```bash
cd end2end
npm ci
npx playwright install --with-deps chromium
npm test
```
