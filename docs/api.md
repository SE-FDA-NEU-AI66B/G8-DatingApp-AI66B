# API design

All protected endpoints require the `session` HttpOnly cookie. A successful
response is not assumed for every request; callers must handle the documented
error status.

| Method | Path | Input | Success | Errors |
|---|---|---|---|---|
| POST | `/api/auth/login` | `{email, otp}` | `200`, session cookie | `400`, `401`, `500` |
| POST | `/api/auth/logout` | session cookie | `204` | `401` |
| GET | `/api/admin/statistics` | session cookie | `200`, three counts | `401`, `403`, `500` |
| GET | `/api/admin/health` | session cookie | `200`, server/database status and timestamps | `401`, `403` |
| POST | `/api/auth/register` | `{email}` | `201` | `400`, `409`, `500` |

The admin endpoints intentionally return an error object with a `message`
instead of silently converting database failures to zero values.
