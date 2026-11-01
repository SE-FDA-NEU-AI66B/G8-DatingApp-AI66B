# Setup

Install Rust (rustup), Docker Compose, `uv`, and `cargo-leptos`. Rustup reads
the pinned nightly and WASM target from `rust-toolchain.toml`.

Copy `.env.example` to `.env` and set the local PostgreSQL password and
`DATABASE_URL`. Start PostgreSQL and create the schema in the database notebook:

```sh
docker compose --env-file .env -f database/compose.yaml up -d db
uv sync
uv run marimo edit database/note.py
```

Click **Recreate application database** in the notebook. Then run the server:

```sh
cargo install cargo-leptos
cargo leptos watch
```

Open <http://localhost:3100>. Microsoft sign-in also requires the Entra values
in `.env`. Run tests with `uv run pytest -q` or open the test notebook with
`uv run marimo edit tests/test_suite.py`.
