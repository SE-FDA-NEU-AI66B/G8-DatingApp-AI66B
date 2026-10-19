# End-to-end tests

The dashboard tests use Node.js 20+, npm 10+, and Playwright. The application
must be running before browser tests are executed because the server requires a
PostgreSQL connection.

From this directory:

```powershell
npm ci
npm run install:browsers
npm run test:list
```

Start the application from the repository root with `cargo leptos watch`, then
run the dashboard acceptance tests:

```powershell
$env:BASE_URL = "http://127.0.0.1:3100"
npm run test:admin-dashboard
```

`BASE_URL` may point to a deployed test instance. The test suite intercepts the
statistics API so the load, refresh, and error-state checks remain deterministic.
Never commit `node_modules`, Playwright browser binaries, or test credentials.
