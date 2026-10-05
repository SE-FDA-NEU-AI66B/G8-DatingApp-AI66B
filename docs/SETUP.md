# NEUDating setup

## Prerequisites

- Rust and `rustup`
- Rust nightly toolchain
- `cargo-leptos`
- `wasm32-unknown-unknown` target
- PostgreSQL 14 or newer
- Node.js 20+ and npm 10+ for Playwright checks

## Clone and configure

```bash
git clone https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B.git
cd G8-DatingApp-AI66B
copy .env.example .env
```

Set `DATABASE_URL` in `.env` to an isolated development database. The
development authentication flow also uses `ADMIN_EMAIL` to provision the
configured account as an admin. Never use this OTP stub or provisioning flow
as production identity management.

## Install Rust tools

```bash
rustup toolchain install nightly
rustup default nightly
rustup target add wasm32-unknown-unknown
cargo install cargo-leptos
```

## Start PostgreSQL

The repository includes a local database definition:

```bash
docker compose -f database/compose.yaml up -d
```

If Docker is unavailable, start PostgreSQL using the local installation and
ensure `DATABASE_URL` points to it.

## Run the application

```bash
cargo leptos watch
```

Open <http://localhost:3100>. Database migrations run when the server starts.
Register a `@neu.edu.vn` account, use the six-digit OTP printed by the
development mail stub, and set `ADMIN_EMAIL` before restarting the server to
open `/admin_console`.

## Run validation

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

## Submission workflow

Create a documentation branch, commit `docs/design.md` and `docs/SETUP.md`,
and open a pull request into `main`. A different member must select **Review
changes**, leave at least one substantive comment, and approve. Merge only
after that approval. Export the merged `docs/design.md` to
`Team<NN>_M2.pdf` and submit the single PDF to the LMS.
