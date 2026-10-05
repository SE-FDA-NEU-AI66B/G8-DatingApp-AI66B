# Walking skeleton

The working slice is the admin dashboard at `/admin_console`.

- **Route:** `/admin_console`
- **Server endpoint:** `GET /api/admin/statistics`
- **Health endpoint:** `GET /api/admin/health`
- **Database tables:** `app_user`, `study_date_request`, `report`
- **Statistics query:** `src/ss/admin.rs` reads verified users, active
  study-date requests overlapping the next 24 hours, and pending reports.
- **Failure behavior:** PostgreSQL/API failures render an explicit error state;
  missing counts are never replaced with zero.

Run the database and application using the commands in [SETUP.md](../SETUP.md),
then authenticate as the configured development admin and open
`http://localhost:3100/admin_console`.

The repository includes Playwright coverage for dashboard load, refresh, and
API failure behavior under `end2end/tests/admin-dashboard.spec.ts`.
