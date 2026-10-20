# API design

All protected endpoints require the `datingapp_session` HttpOnly cookie. A
successful response is not assumed for every request; callers must handle the
documented error status.

| Method | Path | Input | Success | Errors |
|---|---|---|---|---|
| POST | `/api/register` | `{ email, username, password }` | `201`, account created pending Microsoft verification | `400`, `409`, `500` |
| POST | `/api/auth/password` | `{ username_or_email, password }` (`email` is also accepted) | `200`, session cookie | `401`, `403` unverified email, `500` |
| GET | `/api/auth/session` | session cookie | `200`, current user's username, NEU email, and role | `401`, `500` |
| GET | `/api/auth/login` | — | Redirect to configured OAuth2 provider | `500` |
| GET | `/api/auth/callback` | OAuth2 authorization response | `302`, session cookie | `400`, `401`, `500` |
| POST | `/api/auth/logout` | session cookie | `204` | `401` |
| GET | `/api/profiles/{id}` | session cookie | `200`, profile details and relationship to current user | `401`, `404`, `500` |
| GET | `/api/admin/statistics` | session cookie | `200`, three counts | `401`, `403`, `500` |
| GET | `/api/admin/health` | session cookie | `200`, server/database status and timestamps | `401`, `403` |
| GET | `/api/user-profile` | session cookie | `200`, current user's profile (empty defaults until saved) | `401`, `404`, `500` |
| PUT | `/api/user-profile` | `{ display_name, bio, cohort, interests }` | `200`, saved profile | `400`, `401`, `500` |
| GET | `/api/conversations` | session cookie | `200`, current user's matched conversations and latest messages | `401`, `500` |
| GET | `/api/conversations/{id}/messages` | session cookie | `200`, up to 500 messages in chronological order | `401`, `404`, `500` |
| POST | `/api/conversations/{id}/messages` | `{ body }` (1–2000 characters) | `201`, created message | `400`, `401`, `404`, `500` |
| GET | `/api/threads` | session cookie | `200`, newest community threads with comment counts | `401`, `500` |
| POST | `/api/threads` | `{ title, body }` (title 1–120, body 1–3000 characters) | `201`, created thread | `400`, `401`, `500` |
| GET | `/api/threads/{id}/comments` | session cookie | `200`, comments in chronological order | `401`, `404`, `500` |
| POST | `/api/threads/{id}/comments` | `{ body }` (1–2000 characters) | `201`, created comment | `400`, `401`, `404`, `500` |

The admin endpoints intentionally return an error object with a `message`
instead of silently converting database failures to zero values.

Chat endpoints only expose conversations to their two matched participants.
Messages are stored in `chat_messages`; the application database notebook
creates this table when it recreates the development schema.

The signed-in `/userhome` page includes a community feed beneath the profile
and action cards. Threads and comments are stored in `threads` and
`thread_comments`; chat remains private one-to-one messaging between matched
participants.

User profile details are private to the signed-in account and are stored in
`user_profiles`. The `/user_profile` page lets an account edit its display
name, short bio, cohort label, and interests.
