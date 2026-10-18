# P0 admin dashboard flow

## Entry point and access

The documented entry point is `http://localhost:3100/admin_console`. An
authenticated admin session is required. The server, not the client route,
enforces the admin role for both `/api/admin/statistics` and
`/api/admin/health`.

## Observable states

| State | Expected evidence |
|---|---|
| Initial loading | `Loading dashboard statistics...`; the metrics are not shown as zero |
| Data | Three labelled metrics and separate server/database health statuses |
| Empty | A successful all-zero response shows a next-action message |
| Refresh | `Refresh` becomes `Refreshing...` and is disabled while requests run |
| API/network failure | An alert explains the failure; existing data is retained during a failed refresh |
| 401/403 | No admin metrics are exposed; the alert provides a sign-in or return action |

Status is communicated with text and accessible live regions; health is not
communicated by color alone.

## Automated browser coverage

The Playwright suite in
[`end2end/tests/admin-dashboard.spec.ts`](../end2end/tests/admin-dashboard.spec.ts)
covers:

- rendering seeded-shaped non-zero statistics;
- refreshing statistics and replacing the displayed values;
- rendering an API error instead of zero-valued statistics.

Run the suite from `end2end/` after starting the application:

```powershell
npm ci
npx playwright install chromium
npm test
```

## Manual evidence capture

For the review package, sign in using the development admin flow, open the
entry point above, and capture a screenshot with the browser address bar
visible. Repeat the capture for a clear API error state. The screenshot files
must be added to the review evidence package only after they are produced from
the running application; this document does not substitute fabricated images.
