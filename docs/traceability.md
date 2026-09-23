# Traceability

## Business rules

Numbered, so issues and tests can cite them.

| #   | Rule                           | example                                                     |
| --- | ------------------------------ | ----------------------------------------------------------- |
| BR1 | Minimum User Age 18            | Registration is rejected.                                   |
| BR2 | Profile Completion Requirement | The profile is incomplete and must not appear in discovery. |
| BR3 | Mutual Match Requirement       | Alex likes Jordan. Jordan does not like Alex. -> no match   |
| BR4 | One Decision Per Profile       | Alex selects Like for Jordan again -> ignored               |
| BR5 | Message Length Limit           | A message must contain between 1 and 500 characters.        |
| BR6 | Blocked Users Cannot Interact  |                                                             |

## Screens and flow

5 screens. Route / Purpose / Access (G = Guest, U = User, A = Admin) / Priority.
Plus a flow diagram — every screen must appear and be reachable.

| Route          | Purpose                                                     | Access | Priority |
| -------------- | ----------------------------------------------------------- | ------ | -------- |
| /              | Landing & Auth (login, register NEU email)                  | G      | P0       |
| /discover      | Feed: swipe, filter, find study dates, report profiles      | U      | P0       |
| /chat          | Inbox: view pending requests, active matches, and messaging | U      | P0       |
| /profile       | My Profile: settings, privacy toggles, create study date    | U      | P0       |
| /admin_console | Admin Dashboard: system status, review flagged users        | A      | P0       |

                                            ┌─────────────┐
                                            │      /      │
                                            └──────┬──────┘
                                                   │ sign in
                                                   ▼
                 ┌─────────────────┐if admin┌─────────────────┐
                 │ /admin_console  │────────│    /discover    │
                 └─────────────────┘        └──────┬───┬──────┘
                                      navigation   │   │ navigation
                                      tabs (bot)   │   │ tabs (bot)
                                                   ▼   ▼
                                    ┌────────────┐       ┌─────────────┐
                                    │   /chat    │       │  /profile   │
                                    └────────────┘       └─────────────┘
