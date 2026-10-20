# Agent instructions

## Database ownership

- `database/note.py` is the single source of truth for creating, recreating, and initializing the application database.
- Put the database recreation control and its execution flow in the `## Application database` section of that notebook.
- The notebook may load schema or seed SQL from an external file, but it must own the action that loads and executes it. Keep such files under `database/` and make their purpose clear.
- Keep the notebook's embedded HTML export in `database/__marimo__/note.html` synchronized when changing notebook behavior or schema, if that export is maintained in the change.
- Do not add Rust startup schema creation, migrations, automatic initialization, or a second database setup path. The server should connect to the schema created by the notebook.
- Do not add database migration code or migration files. This project recreates its empty development database from the notebook.
- There is no existing application data to preserve. Do not create or run data migrations; change the schema in `database/note.py` and recreate the database instead.
- When changing tables or columns, update the notebook schema and the Rust queries/types that use them together. Check documentation for stale schema descriptions.
- SQL in tests may create isolated test fixtures only when required by the test; it must not become an alternative application database initializer.

## Authentication and accounts

- Accounts use a unique username, an NEU email, and a password hash when password registration is enabled.
- Microsoft Outlook sign-in verifies the NEU email address. Password login must require a verified email.
- Store password hashes only; never store or log plaintext passwords.
- Do not reintroduce OTP or email-verification-code flows.

## Project conventions

- Keep secrets in environment variables or local `.env` files; never commit credentials.
- Preserve the existing Leptos/Actix server and hydration feature boundaries.
- Update the relevant documentation when changing setup, API, schema, or authentication behavior.
