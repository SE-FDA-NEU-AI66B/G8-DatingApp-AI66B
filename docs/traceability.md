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

The route table below is the source of truth for the implemented screens.
Every route must be reachable from the application entry point or its
documented authenticated flow.

| name           | Purpose                          | Access | Priority |
| -------------- | -------------------------------- | ------ | -------- |
| /              | home                             | G      | P0       |
| /register      | register a campus account        | G      | P0       |
| /feed          | discover profiles                | U      | P0       |
| /study-date/new| create a study-date request     | U      | P0       |
| /profile/:id   | view a profile                   | U      | P0       |
| /requests      | manage incoming requests        | U      | P0       |
| /admin_console | show statistics and system health | A      | P0       |

                                            ┌─────────────┐
                                            │      /      │
                                            └──────┬──────┘
                                                   │ sign in
                                                   ▼
                 ┌─────────────────┐if admin┌─────────────────┐    ┌─────────────────┐    ┌────────────┐
                 │  /admin_console │
                 └─────────────────┘        └─────────────────┘    └─────────────────┘    └────────────┘
