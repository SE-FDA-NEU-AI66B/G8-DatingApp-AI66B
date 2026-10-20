# API design

All protected endpoints require the `datingapp_session` HttpOnly cookie. A
successful response is not assumed for every request; callers must handle the
documented error status.

| Method | Path | Input | Success | Errors |
|---|---|---|---|---|
| POST | `/api/register` | `{email}` | `201` | `400`, `409`, `500` |
| POST | `/api/auth/login` | `{email, otp}` | `200`, session cookie | `400`, `401`, `500` |
| POST | `/api/auth/logout` | session cookie | `204`, clears cookie | `500` |
| GET | `/api/study-dates` | session cookie | `200`, active study-date cards | `401`, `500` |
| POST | `/api/study-dates` | session cookie, `{starts_at, duration_min}` | `201`, created study date | `401`, `422`, `500` |
| GET | `/api/profiles/{id}` | session cookie | `200`, profile and relationship | `401`, `404`, `500` |
| POST | `/api/match-requests` | session cookie, `{to_user_id}` | `201`, request sent | `400`, `401`, `404`, `409`, `429`, `500` |
| GET | `/api/match-requests/incoming` | session cookie | `200`, incoming requests | `401`, `500` |
| GET | `/api/matches` | session cookie | `200`, matched users | `401`, `500` |
| POST | `/api/match-requests/{id}/accept` | session cookie | `200`, match and chat-room id | `401`, `403`, `404`, `500` |
| POST | `/api/match-requests/{id}/decline` | session cookie | `200`, decline confirmation | `401`, `403`, `404`, `500` |
| GET | `/api/admin/access` | admin session cookie | `204` | `401`, `403`, `500` |
| GET | `/api/admin/statistics` | admin session cookie | `200`, three counts | `401`, `403`, `500` |
| GET | `/api/admin/health` | admin session cookie | `200`, server/database status and timestamps | `401`, `403` |

The admin statistics endpoint intentionally returns an error object with a
`message` instead of silently converting database failures to zero values.
`422` (study-date input validation) and `429` (match-request daily limit) are
new since M2. The implementation has no `/api/auth/register`,
`/api/auth/request-otp`, or `/api/auth/verify-otp` route.
