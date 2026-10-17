# Sprint 3 review evidence

This checklist records what is available and what still requires a manual
capture. It intentionally does not claim screenshots that were not produced.

## Implementation links

- [Issue #92](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/92)
  and branch `issue-92-dashboard-loading-refresh`
- [Issue #93](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/93)
  and branch `issue-93-dashboard-errors`
- [Issue #94](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/94)
  and branch `issue-94-dashboard-accessibility`
- [PR #104](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/pull/104),
  [PR #105](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/pull/105),
  and [PR #106](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/pull/106)

## Validation evidence

| Command | Result |
|---|---|
| `cargo check --lib --features hydrate` | Passed during dashboard implementation |
| `cargo test --lib --features ssr` | Passed: 6 tests |
| `cargo fmt -- --check` | Existing repository formatting differences remain outside the dashboard changes |
| Playwright dashboard suite | CI harness passed; local full run requires the application and PostgreSQL |

## Required manual captures

- [ ] Project board screenshot with Sprint 3 order and issue status visible
- [ ] Real-data `/admin_console` screenshot with browser address bar visible
- [ ] Clear API-error `/admin_console` screenshot with browser address bar visible
- [ ] Independent fresh-machine setup walkthrough recorded in `docs/SETUP.md`
- [ ] Final PDF/repository checklist reviewed

## Capture procedure

1. Start PostgreSQL using `docker compose -f database/compose.yaml up -d db`.
2. Follow `docs/SETUP.md` and start `cargo leptos watch`.
3. Authenticate an admin account using the documented development flow.
4. Capture the real-data and API-error screens with the browser address bar
   visible.
5. Save screenshots under the review artifact location and replace the
   unchecked items above only after inspecting each file.
