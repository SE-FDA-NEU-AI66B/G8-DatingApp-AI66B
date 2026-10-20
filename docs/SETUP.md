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

Set `DATABASE_URL` to the PostgreSQL database created by Compose. Register a
Microsoft Entra app for NEU's directory. Under **Authentication**, add the Web
redirect URI `http://localhost:3100/api/auth/callback`. In **Token configuration**,
add the `email` optional claim to the ID token; the app also falls back to the
NEU email shaped `preferred_username` claim when `email` is absent. Set
`MICROSOFT_TENANT_ID` to NEU's Entra tenant GUID from the directory overview,
then set the application client ID, client secret, and exact callback URI in
`.env`. The tenant-specific endpoint restricts sign-in to NEU's directory.
For local development, `http://localhost` is supported; deployed callbacks
must use HTTPS and exactly match the registered URI.
Set `ADMIN_EMAIL` to the account that should receive development admin access.
Never commit real passwords, OAuth client secrets, or keys.

Users register with a unique username, `neu.edu.vn` email (including subdomains), and password at `/register`.
They must complete Microsoft sign-in with that Outlook address to verify the
account. Then they can use either Microsoft sign-in or username and password.
Passwords are stored as Argon2 hashes; password login is disabled until the
email has been verified.

## Create and start the database

```bash
docker compose -f database/compose.yaml up -d db
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
cargo leptos build
```

Open the database notebook and click **Recreate application database**. This
drops and rebuilds the `public` schema; the server expects the notebook-created
tables and does not create the schema itself. Then click **Initialize sample
data** to add demo users, study dates, a pending report, match requests, and a
conversation. This button is safe to click again and does not reset the schema.
The seeded users all use the username shown in the notebook data and the shared
demo password `password`. This intentionally weak password is for local demo
data only; never reuse it outside the disposable development database.

Windows PowerShell:

```powershell
# The notebook reads PGPASS from .env
uv run marimo edit database/note.py
# Click Recreate application database, then Initialize sample data
# Close the notebook when both actions finish
# Terminal 1
cargo leptos watch
```

macOS/Linux:

```bash
# The notebook reads PGPASS from .env
uv run marimo edit database/note.py
# Click Recreate application database, then Initialize sample data
# Close the notebook when both actions finish
cargo leptos watch
```

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
