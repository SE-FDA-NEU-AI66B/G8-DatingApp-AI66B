# NEUDating setup

## Prerequisites

- Git 2.40+
- Rust 1.85+ with `rustup`; the repository pins the nightly toolchain
- Docker Desktop 4.30+ and Docker Compose
- Node.js 20+ and npm 10+ for browser checks
- Windows 10/11, macOS 13+, or a current Linux distribution

## Clone and configure

```bash
git clone https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B.git
cd G8-DatingApp-AI66B
```

Windows PowerShell:

```powershell
Copy-Item .env.example .env
```

macOS/Linux:

```bash
cp .env.example .env
```

Set `DATABASE_URL` to the PostgreSQL database created by Compose and set
`ADMIN_EMAIL` to the email that will receive development admin access. Never
commit real passwords, OTPs, or keys.

## Start, migrate, and seed the database

```bash
docker compose -f database/compose.yaml up -d db
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
cargo leptos build
```

Run migrations by starting the server once, then seed ten users, two active
requests, and one pending report. Use two terminals because the development
server must remain running while `psql` connects:

Windows PowerShell:

```powershell
# Terminal 1
cargo leptos watch
# Terminal 2
psql $env:DATABASE_URL -f database/seed.sql
```

macOS/Linux:

```bash
# Terminal 1
cargo leptos watch
# Terminal 2
psql "$DATABASE_URL" -f database/seed.sql
```

Stop the watcher with `Ctrl+C` only after the seed command completes. The
seed file is idempotent for its named fixture rows.

## How to know it worked

Open <http://localhost:3100>, complete the development OTP flow for the
configured admin email, then open
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

PostgreSQL integration tests require an isolated `TEST_DATABASE_URL`.

## Troubleshooting

**`connection refused` or `database does not exist`:** confirm Docker Desktop
is running, run `docker compose -f database/compose.yaml up -d db`, and make
sure `.env` uses the PostgreSQL port `5432` and database `datingapp`.

**`psql is not recognized`:** install PostgreSQL client tools and add their
`bin` directory to `PATH`, or run `psql` using its full installation path.

**`cargo leptos` is not recognized:** run `cargo install cargo-leptos` and
restart the terminal so Cargo's bin directory is on `PATH`.

## Tested by

Before submission, a non-author teammate must follow this file on a fresh
machine. Record the tester handle, operating system, date, and elapsed time
here, for example: `@iambadwithname — Windows 11 — 2025-03-08 — 12 minutes`.

## Submission workflow

Create a documentation branch, commit `docs/design.md`, `docs/SETUP.md`, and
the ERD, then open a pull request into `main`. A different member must select
**Review changes**, leave a substantive comment, and approve. Merge only after
that approval. Export the merged `docs/design.md` to exactly
`Team<NN>_M2.pdf` and submit that one PDF to the LMS.
