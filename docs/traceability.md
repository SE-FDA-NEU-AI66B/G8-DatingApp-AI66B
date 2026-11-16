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

5 screens Table: Route / Purpose / Access (G, U, A) / Priority. Plus a flow diagram - every screen must appear and be reachable. Hand-drawn is fine.
all route are assign by hash

| name           | Purpose                          | Access | Priority |
| -------------- | -------------------------------- | ------ | -------- |
| /              | home                             | G      | P0       |
| /user_profile  | user profile for manaing account | U      | P0       |
| /user_home     | show status + chat iteract       | U      | P0       |
| /chat          | for chatting                     | U      | P0       |
| /admin_console | show server status               | A      | P0       |

                                            ┌─────────────┐
                                            │      /      │
                                            └──────┬──────┘
                                                   │ sign in
                                                   ▼
                 ┌─────────────────┐if admin┌─────────────────┐    ┌─────────────────┐    ┌────────────┐
                 │  /admin_console │────────│   /user_hom     │────│  /user_profile  │────│   /chat    │
                 └─────────────────┘        └─────────────────┘    └─────────────────┘    └────────────┘
