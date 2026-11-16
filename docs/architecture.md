# Architecture

The walking skeleton is a modular monolith:

```mermaid
flowchart LR
    Browser[Browser / Leptos WASM]
    Web[Actix Web server]
    DB[(PostgreSQL)]
    Mail[Development mail stub]

    Browser -->|HTTP JSON requests and HTML/WASM assets| Web
    Web -->|SQL queries and transactions| DB
    Web -->|OTP message in development| Mail
```

The browser never connects directly to PostgreSQL. The Actix server owns
authentication, request validation, authorization, and database access.
Dashboard statistics and health checks are read fresh from PostgreSQL for each
request.
