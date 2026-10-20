# NEUDating — Milestone 2 Design

## 1. Architecture

The walking skeleton is the admin console at `/admin_console`.

```mermaid
flowchart LR
    Browser[Browser / Leptos WASM]
    Web[Actix Web server]
    DB[(PostgreSQL)]
    Browser -->|HTTP requests and HTML/WASM assets| Web
    Web -->|SQL queries and transactions| DB
```

The browser never connects directly to PostgreSQL. The server owns routing,
authentication, validation, authorization, and database access. The arrows
are labelled with the data that crosses each boundary.

## 2. Data model

```mermaid
erDiagram
    users ||--o{ auth_cookies : owns
    users ||--o{ study_dates : creates
    users ||--o{ user_reports : reports
    users ||--o{ match_requests : sends
    users ||--o{ match_requests : receives
    match_requests ||--o| conversations : opens
    users {
        uuid id PK
        text email UK
        boolean email_verified
        text role
    }
    auth_cookies {
        text token_hash PK
        uuid user_id FK
        timestamptz expires_at
    }
    study_dates {
        bigint id PK
        uuid creator_id FK
        timestamptz starts_at
        timestamptz ends_at
        text status
    }
    user_reports {
        bigint id PK
        uuid reporter_id FK
        uuid reported_user_id FK
        text status
    }
    match_requests {
        bigint id PK
        uuid sender_id FK
        uuid receiver_id FK
        text status
    }
    conversations {
        bigint id PK
        uuid user_low FK
        uuid user_high FK
        bigint match_request_id FK
    }
```

The schema uses consistent plural domain names. The application schema is
created and reset from the `## Application database` section in
`database/note.py`. The server connects to those tables directly.

| Table | Columns and constraints | Purpose and M1 rule |
|---|---|---|
| `users` | `id UUID` PK; unique lowercase NEU `email`; `email_verified`, constrained `role`, `created_at` | Account identity and authorization role. |
| `auth_cookies` | `token_hash TEXT` PK; `user_id UUID` FK to `users.id`; `expires_at`, `created_at` | Hashed server-side cookie sessions, deleted with their account. |
| `study_dates` | `id BIGSERIAL` PK; `creator_id UUID` FK; `starts_at`, `ends_at`, `duration_min`, constrained `status`, `created_at` | Scheduled study meetups with a checked time window and duration. |
| `user_reports` | `id BIGSERIAL` PK; `reporter_id`, `reported_user_id UUID` FKs; constrained `status`, `created_at` | Moderation reports; self reports are rejected. |
| `match_requests` | `id BIGSERIAL` PK; sender and receiver UUID FKs; constrained `status`; `created_at`, `responded_at`; unique sender/receiver pair | Tracks requests through pending, accepted, or declined states. |
| `conversations` | `id BIGSERIAL` PK; ordered participant UUIDs; unique `match_request_id`; `created_at` | One conversation per participant pair, linked to the accepted match request. |

## 3. API design

| Method | Path | Input | Success output | Error codes |
|---|---|---|---|---|
| `GET` | `/api/admin/statistics` | Admin session cookie | `200` with three counts | `401` unauthenticated; `403` non-admin; `500` database failure |
| `GET` | `/api/admin/health` | Admin session cookie | `200` with server/database status and timestamps | `401`; `403` |
| `GET` | `/api/auth/login` | — | Redirect to configured OAuth2 provider | `500` provider configuration failure |
| `GET` | `/api/auth/callback` | OAuth2 authorization response | `302` session cookie | `400`, `401`, `500` |
| `POST` | `/api/match-requests` | Auth session and target user id | `201` request confirmation | `401`; `404` target missing; `409` duplicate request |
| `GET` | `/api/profiles/{id}` | Auth session and profile id | `200` profile and relationship | `401`; `404` profile missing |

Statistics are queried on every request, so refresh reads current stored data.
Database failures return an explicit error and the UI does not convert them to
zero-valued counts.

## 4. Walking skeleton

**Route:** `GET /admin_console`.

**Database tables:** `users`, `auth_cookies`,
`study_dates`, `user_reports`, `match_requests`, and `conversations`.

The statistics query is:

```sql
SELECT
  (SELECT COUNT(*) FROM users WHERE email_verified = TRUE),
  (SELECT COUNT(*) FROM study_dates
   WHERE status = 'active'
     AND starts_at < now() + interval '24 hours'
     AND ends_at > now()),
  (SELECT COUNT(*) FROM user_reports WHERE status = 'pending');
```

The end-to-end execution is:

| Step | Component | Action |
|---:|---|---|
| 1 | Browser | Open `/admin_console`. |
| 2 | Browser | Send `GET /api/admin/statistics` with the session cookie. |
| 3 | Actix | Parse the cookie and hash the opaque session token. |
| 4 | PostgreSQL | Look up the session and verify its expiry. |
| 5 | Actix | Verify that the session user has the `admin` role. |
| 6 | PostgreSQL | Execute the three `COUNT(*)` subqueries. |
| 7 | Actix | Serialize the typed JSON response. |
| 8 | Browser | Render registered users, active requests, and pending reports. |
| 9 | Browser | Display server and database health status. |
| 10 | Browser | Repeat the request when Refresh is clicked. |
| 11 | Browser | Display an error and Retry action if the request fails. |

The database notebook creates the schema. The reproducible URL is
`http://localhost:3100/admin_console`. A screenshot of the running route with
the browser address bar visible is included in the final submitted PDF.

## 5. Design decisions (ADRs)

### ADR-1 — PostgreSQL instead of an in-memory store

**Options:** PostgreSQL, SQLite, or an in-memory vector. **Decision:** use
PostgreSQL because the app needs relational constraints, concurrent requests,
and production-like SQL. An in-memory vector would lose data and SQLite would
not match the deployment target. **Change trigger:** if the instructor's
fresh-machine test cannot complete in the time budget despite Docker and the
setup guide, evaluate a self-contained SQLite profile.

### ADR-2 — Server-side authorization instead of UI-only protection

**Options:** hide the route in the UI, enforce authorization in Actix, or use a
separate admin service. **Decision:** enforce it in Actix with an expiring
HttpOnly session and a role check. UI hiding is not a security boundary, while
a separate service is unnecessary for this slice. **Change trigger:** if
deployment requires independent scaling or a separate identity provider,
extract the admin API behind that provider.

## 6. What changed since M1

1. **The product gained a concrete admin dashboard slice.** Sprint 1 defined
   moderation and study-date concepts; Sprint 2 added measurable counts and
   health indicators so an administrator can observe platform usage.
2. **Authentication and authorization became explicit acceptance behavior.**
   The Sprint 1 review highlighted that an admin screen must not expose counts
   to anonymous or regular users, so the session, expiry, role, `401`, and
   `403` behaviors were added to the API and tested.

## Verification and submission metadata

| Field | Value |
|---|---|
| Team | QHQ |
| Topic | Walking skeleton: admin dashboard statistics and system health |
| Members | Nguyen Son Hai (`hai291`), Nguyen Minh Quang (`iambadwithname`), Pham Ha (`pham-ha-gif`) |
| Product Owner | `@hai291` |
| Scrum Master | `@pham-ha-gif` (Sprint 2; different from Sprint 1) |
| Repository | https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B |
| Project board | https://github.com/orgs/SE-FDA-NEU-AI66B/projects/20 |
| Setup guide | [docs/SETUP.md](./SETUP.md) |

Validation: `cargo check --features ssr`, `cargo test --lib --features ssr`,
and CI passed for the implementation PRs. The final PDF must include the
cover block, two project-board screenshots, and one browser screenshot with
the address bar visible.
