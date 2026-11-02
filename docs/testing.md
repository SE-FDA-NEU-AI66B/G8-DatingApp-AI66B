# Test plan

This document turns the P0 stories and business rules in
[traceability.md](traceability.md) into concrete checks. The listed cases are
the tests to implement and run; they are not evidence that each case already
has an automated test. Tests should use isolated fixtures or mocked API
responses, never the development database. Do not use live Microsoft accounts
or real passwords in test data.

## 1. What we test, and why

| Layer | Test approach | Coverage and reason |
|---|---|---|
| Services | Unit tests for validation and business decisions; handler tests for status codes and response bodies. | Registration, study-date validation, match decisions, authorization, health, and dashboard aggregation. These rules can reject invalid or unauthorized actions and should be deterministic. |
| Repository | Tests against an isolated temporary PostgreSQL database or a fake repository. | Verify uniqueness, filtered/sorted feeds, pending/accepted/declined request state, statistics, and health query behavior. Never initialize or mutate the application database from tests. |
| Routes | Actix handler/integration tests with a test app and isolated dependencies. | Verify authentication and authorization, HTTP status/body, and that failures do not persist invalid state. |
| Interface | Manual checks for Microsoft OAuth and rendered page states. | Browser automation is currently not configured. OAuth requires Microsoft configuration and an authorized test account. |

Registration, authorization, and match decisions receive the strongest rule
coverage because mistakes can expose accounts or allow interactions users did
not request. Study-date limits and admin health/statistics also receive
boundary and failure checks because incorrect time filtering or misleading
health data can make the service appear usable when it is not.

## 2. Test cases for every P0 story

`Kind` identifies the preferred automated layer; manual checks are marked
explicitly. IDs are stable references for test names or comments.

| ID | Story | Input / setup | Expected result | Kind |
|---|---|---|---|---|
| US01-T01 | US01 Register with NEU email | Valid unique username, NEU email, and password of at least 12 characters. | Account is created; password hash is stored instead of plaintext; response says Microsoft verification is required before password sign-in. | Route + repository |
| US01-T02 | US01 Register with NEU email | Malformed or non-NEU email, invalid username, or password shorter than 12 characters. | Registration is rejected with a client error and no account is created. | Service + route |
| US01-T03 | US01 Register with NEU email | Reuse an existing username or email. | Registration is rejected as a conflict; the existing account remains unchanged. | Route + repository |
| US01-T04 | US01 Register with NEU email (BR1 boundary) | Attempt registration with age 17, then with age 18 (assuming age is collected by the registration flow). | Age 17 is refused; age 18 meets the minimum-age rule. If registration does not collect age, record this as a product/rule gap and test the age gate at the point where age is collected. | Service / manual flow |
| US01-T05 | US01 Register with NEU email | Try password login before Microsoft verification, then complete NEU Microsoft sign-in and log in. | Unverified account cannot password-login; verified account can. Microsoft identity with a non-NEU email is refused. | Route + manual OAuth |
| US02-T01 | US02 Create study date | Authenticated user submits a valid future local date/time and duration of 1 minute. | Request is created and appears in the creator's feed with the submitted time and duration. | Route + repository |
| US02-T02 | US02 Create study date (duration boundary) | Submit duration 0, 1, 720, and 721 minutes. | 0 and 721 are rejected; 1 and 720 are accepted. | Service + route |
| US02-T03 | US02 Create study date | Submit malformed/impossible date-time (for example `2026-02-30T10:00`) or a past start time. | Request is rejected; no row is created. | Service + route |
| US02-T04 | US02 Create study date | Submit valid request without an authenticated session. | Request is rejected as unauthorized; no row is created. | Route |
| US03-T01 | US03 Browse active study date requests | Feed contains active future requests by other users, an expired request, and the current user's own request. | Feed shows only eligible active requests, in ascending start-time order; own requests follow the specified visibility policy. | Repository + route |
| US03-T02 | US03 Browse active study date requests | No eligible requests exist. | Page shows a clear empty state rather than an error or stale cards. | Interface |
| US03-T03 | US03 Browse active study date requests | Feed API returns a server error. | Page shows an error/retry state and does not present the failure as an empty successful feed. | Interface |
| US04-T01 | US04 Send a match request (BR3) | A completes profile and likes B; B has not liked A. | One pending incoming request is created; no match or conversation is created yet. | Service + repository |
| US04-T02 | US04 Send a match request (BR3) | Send a request to a nonexistent profile or to oneself. | Request is rejected; no request or conversation is created. | Service + route |
| US04-T03 | US04 Send a match request (BR2) | Attempt to discover or request a user whose profile is incomplete. | Incomplete profile is absent from discovery and cannot receive a request. | Service + repository |
| US04-T04 | US04 Send a match request (BR4) | Send a decision to the same profile twice, including after a prior decline. | Only the first decision is recorded; the repeated decision is ignored or rejected without creating another row. | Service + repository |
| US04-T05 | US04 Send a match request (BR6) | A is blocked by B (and reverse direction in a separate fixture); attempt a request. | Blocked users cannot send requests or interact; no new request is stored. | Service + repository |
| US05-T01 | US05 Accept or decline a match request | Recipient accepts a pending incoming request. | Request becomes accepted and exactly one conversation is available to both matched users. | Route + repository |
| US05-T02 | US05 Accept or decline a match request | Recipient declines a pending incoming request. | Request becomes declined; no conversation is created and it is no longer pending. | Route + repository |
| US05-T03 | US05 Accept or decline a match request | A user tries to accept/decline a request addressed to another user, or repeats a response to a completed request. | Unauthorized/wrong-recipient response is refused; completed decision is not changed and no duplicate conversation is created. | Service + route |
| US12-T01 | US12 Restrict admin console to admins | Authenticated admin opens `/admin_console` and calls admin APIs. | Dashboard and admin API data load successfully. | Route + interface |
| US12-T02 | US12 Restrict admin console to admins | Anonymous visitor opens console or calls admin API. | User is redirected to sign-in or receives unauthorized status; no admin data is returned. | Route + interface |
| US12-T03 | US12 Restrict admin console to admins | Authenticated regular user opens console or calls admin API. | Access is denied; no admin data is returned. | Route + interface |
| US13-T01 | US13 View database and server health | Admin requests health while server and database are available. | Response independently reports server and database healthy. | Route + repository |
| US13-T02 | US13 View database and server health | Admin requests health while database query fails. | Response reports database unhealthy without claiming the database is healthy; endpoint responds with the documented health status. | Route + repository |
| US13-T03 | US13 View database and server health | Anonymous or non-admin requests health. | Access is denied and health details are not disclosed. | Route |
| US14-T01 | US14 View admin dashboard statistics | Seed deterministic values for verified/unverified users, active/inactive study dates, and pending reports; load dashboard. | Dashboard shows the expected counts; only verified users count as registered and only active requests count as active study dates. | Repository + interface |
| US14-T02 | US14 View admin dashboard statistics | Refresh after API values change. | Dashboard shows the latest values after refresh, without duplicating old values. | Interface |
| US14-T03 | US14 View admin dashboard statistics | Statistics API returns an error. | Dashboard shows an error state, not zero-valued statistics presented as real data. | Interface |
| US14-T04 | US14 View admin dashboard statistics | Statistics API returns all valid counts as zero. | Dashboard displays zero as valid data and does not show the API error state. | Repository + interface |

### Shared business-rule cases

These cases cover traceability rules that apply across the P0 flows and must
also be exercised when testing the relevant story.

| ID | Story | Input / setup | Expected result | Kind |
|---|---|---|---|---|
| BR5-T01 | Chat message length | Send a one-character message in an authorized conversation. | Message is saved and returned to the sender. | Service + route |
| BR5-T02 | Chat message length boundary | Send messages with 0, 1, 500, and 501 characters. | 0 and 501 are rejected; 1 and 500 are accepted, according to BR5. | Service + route |
| BR6-T01 | Blocked users cannot interact | A blocked participant attempts to send a chat message or access the other participant's conversation. | Interaction is denied and no message is saved. | Service + repository |

## 3. Automated tests

Implement each ID above as an automated test in the marimo pytest notebook,
with the ID in the pytest test name or a comment. Use fake API
responses or isolated test fixtures so tests need neither a running server nor
the development database. OAuth verification is a documented manual test
because it depends on Microsoft configuration and an authorized test account.

The desired one-command Python run is:

```bash
pytest -q
```

Add this command to `docs/SETUP.md` when the pytest suite is implemented. Do
not claim test IDs pass until their tests exist and have been run. Capture a
terminal screenshot of the final clean run for review evidence; no screenshot
is included yet because this file only defines the plan.

## 4. What the tests found

No new bug is claimed by this plan: the cases above have not all been
implemented or executed as a pytest suite. Once implemented, record one
verified finding here with the issue, code fix, and guarding test ID, or
deliberately break one rule, show the failing test output, restore the rule,
and show the passing test. The cases above have not all been implemented or
executed as a pytest suite.
