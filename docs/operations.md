# Operations

## 1. Release and rollback

**Public URL:** none is provisioned. The [Render candidate](https://neudating.onrender.com)
returned HTTP 404 on 2026-11-16; it is not production. Replace it after #138.
The [manifest](../render.yaml) sets `autoDeploy: true`, which does not wait for
CI. Until #144 is verified, set `autoDeploy: false`, sync the service, and
confirm automatic deploys are disabled. Deploy manually after the checks pass.

1. Open a PR to `main`; obtain a teammate review and green checks. Until the
   branch rule is evidenced, have a maintainer verify it. Run `cargo fmt
   --all -- --check`, `cargo check --features ssr`, and `cargo test --lib
   --features ssr`; current CI does not build Rust.
2. Merge after checks pass. Record the main SHA and its green
   [Actions run](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/actions/runs/41417457026).
3. In Render Dashboard, open `neudating` → **Manual Deploy** and deploy that
   exact SHA. Wait for completion.
4. Copy the confirmed host from Render. Open
   `https://<confirmed-host>/health` and require HTTP 200, then complete the
   P0 smoke flow in a private browser window. Record URL, SHA, and deploy event.

For rollback, open `neudating` in Render, select the last known-good SHA in
**Events**, and redeploy it. Recheck `/health` and the P0 flow, then record
failure and restored SHA. A code rollback does not restore database contents;
back up before schema/data changes and forward-fix if rollback would lose data.
Rehearse under #147.

## 2. CI and branch protection

The [CI workflow](../.github/workflows/ci.yml) runs on pushes and pull
requests. It checks stack detection, Python tests on Linux and Windows, a
credential/database-dump scan, and Playwright test discovery. Lint is
warning-only; Playwright does not run browser flows, and CI does not compile
or test Rust. Do not treat this as a release gate until #143 closes these gaps.

The latest successful `main` run found during this update is [run #222](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/actions/runs/41417457026),
completed 2026-11-02 on commit `48ff6fd20af4368402dbd25ca4fd24aedd47b4a2`.
It passed its five jobs in about 67 seconds; it predates the current deployment
work and does not validate unmerged changes.

The [main branch protection settings](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/settings/branches)
are authoritative (maintainer access required). The rule and screenshot are
not yet verified; see #144. Require a PR, one approval, and green CI before
enabling deploy-after-green.

## 3. Monitoring and secret handling

Render is configured to probe `GET /health`; the required contract is HTTP 200
when the service can query PostgreSQL and HTTP 503 otherwise. The route and
automated healthy/unhealthy checks must be verified before release under
issue #139. Until then, a missing or non-200 response blocks release.

In the Render service's **Metrics**, watch 5xx error rate (5xx responses /
total responses) and p95 response time. Use 5% errors or p95 above 2 seconds
for five minutes as initial investigation thresholds; confirm available
metrics and adjust them for the selected plan. Check `/health` after each
deploy and during demos. Render **Logs** captures application stdout/stderr;
filter by the failing time and operation, then compare with the deploy event.
There is no external alerting yet (#145); checks are manual.

Store `DATABASE_URL`, production authentication credentials, and other
secrets only in Render Environment settings or an ignored local `.env`.
`ADMIN_EMAIL` is configuration, not a password. Never paste secret values in
issues, screenshots, command output, or logs. If a value is exposed, rotate it
at the provider and update Render before restarting the service.

## 4. Five known risks

| Risk | Impact | Issue-based plan |
|---|---|---|
| No provisioned public service or verified URL | Users cannot reach or smoke-test production. | [#138 — deploy HTTPS service](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/138) |
| `/health` route/checks not verified on `main` | Render may report a false failure or miss a database outage. | [#139 — implement and test health](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/139) |
| Production database/sample records not confirmed | Data may be ephemeral or the demo may start empty. | [#140 — persistent data and fixtures](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/140) |
| CI and branch protection do not yet gate deploys | A broken or unreviewed commit could reach the service. | [#143 — CI](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/143) and [#144 — protected `main`](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/144) |
| No production alerting or agreed log retention | An outage may be noticed late or evidence may be unavailable. | [#145 — monitoring guidance](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/145) |

## 5. Five maintenance areas

| Area | Action | Owner | Cadence |
|---|---|---|---|
| Release and recovery | Review deploy SHA, verify smoke checks, rehearse rollback. | Release maintainer | Every release; rollback drill per [#147](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/147) |
| Database | Confirm backup/restore point and persistence; review fixture safety. | Database owner | Weekly and before schema changes |
| Monitoring | Check `/health`, 5xx rate, p95 response time, and Render logs. | Operations maintainer | After deploy and each demo day |
| Dependencies and CI | Review Rust/toolchain, Python, npm, and action updates; run full checks. | Development team | Monthly |
| Secrets and demo accounts | Audit access, rotate exposed values, verify non-production accounts. | Deployment owner | Every release; see [#146](https://github.com/SE-FDA-NEU-AI66B/G8-DatingApp-AI66B/issues/146) |

## 6. Three lessons learned

1. Localhost and hosted services differ in ports and database URLs; use
   platform environment configuration and test a clean deployment earlier.
2. An admin-only health card is not an operational probe; keep public
   dependency health separate and verify it independently.
3. Release evidence is easy to lose; record the URL, SHA, green run, branch
   rule, and rollback result as each release happens.

## 7. Final release checklist

- [ ] Replace the candidate with the live HTTPS URL; test desktop/mobile private windows (#138).
- [ ] Confirm `/health` and the P0 smoke flow on deployed SHA (#139, #141).
- [ ] Verify `main` protection and deploy-after-green; attach settings evidence (#144).
- [ ] Record green run, tag `v1.0` at deployed SHA, rehearse rollback (#142, #147).
- [ ] Verify demo accounts and sample data; include no credentials (#140, #146).
