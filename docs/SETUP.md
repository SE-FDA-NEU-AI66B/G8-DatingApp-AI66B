# Setup

## Download-install

- Rust (rustup)
- Docker Compose

## Install

- `uv` : cargo install uv
- `cargo-leptos`: cargo install cargo-leptos
- `rustc-codegen-cranelift`: rustup component add rustc-codegen-cranelift

## setup

```sh
docker compose --env-file .env -f database/compose.yaml up -d db
uv sync
uv run marimo edit database/note.py
```

Click **Recreate application database** in the notebook. Then run the server:

```sh
cargo leptos watch
```

Open <http://localhost:3100>. Microsoft sign-in also requires the Entra values
in `.env`. Run tests with `uv run pytest -q` or open the test notebook with
`uv run marimo edit tests/test_suite.py`.
