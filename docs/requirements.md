# Requirement

## Product vision

for lonely old guys

## Personas

2 (teams of 5: 3) Built on conversations with at least 2 real people. Each: role, goal, what blocks them, one quoted sentence. Add an interview note - who you spoke to and when.

| who   | goal             | what block them | quoted |
| ----- | ---------------- | --------------- | ------ |
| Quyen | somebody to love | lonely          |        |
| Hai   | somebody to love | social anxiety  |        |

## Scenarios

Persona 1: Quyen — New User Looking for a Match

    Quyen creates an account using a valid email address and a password with at least 8 characters.
    Quyen confirms that they are at least 18 years old.
    Quyen adds their name, age, location, biography, interests, and at least one photo.
    Quyen selects their preferred age range and maximum distance.
    Quyen reviews suggested profiles that match their preferences.
    Quyen indicates interest in profiles they like and skips profiles they are not interested in.
    Quyen receives a mutual match when another user has also indicated interest in them.
    Quyen sends messages to the mutual match and continues the conversation.

Persona 2: Hai — User Managing Safety and Interactions

    Hai signs in to their account.
    Hai reviews new matches and unread messages.
    Hai reads a message from a mutual match.
    Hai decides that the conversation is unwanted.
    Hai prevents the other user from contacting or viewing their profile.
    Hai reports the user and selects a reason for the report.
    The system records the report and removes the blocked user from Hai’s matches and conversations.
    Hai continues using the dating app without seeing the blocked user again.

## User stories

Then for every story: ≥2 acceptance criteria in Given–When–Then, of which ≥1 contains a concrete number or exact expected value.

| ID     | Story               | Priority                                                                                   | Points |
| ------ | ------------------- | ------------------------------------------------------------------------------------------ | ------ |
| DA-001 | As a visitor        | I want to create an account so that I can use the dating app.                              | 3      |
| DA-002 | As a user           | I want to create a dating profile so that other users can learn about me.                  | 5      |
| DA-003 | As a user           | I want to discover potential matches so that I can find people who interest me.            | 5      |
| DA-004 | As a user           | I want to like or pass on profiles so that I can express my preferences.                   | 3      |
| DA-005 | As a user           | I want to message a mutual match so that we can communicate.                               | 5      |
| DA-006 | As a user           | I want to edit my profile so that my information remains accurate.                         | 3      |
| DA-007 | As a user           | I want to filter potential matches so that I can find compatible people.                   | 5      |
| DA-008 | As a user           | I want to block or report another user so that I can control my interactions and safety.   | 3      |
| DA-009 | As a user           | I want to receive notifications about matches and messages so that I do not miss activity. | 3      |
| DA-010 | As an administrator | I want to review reported profiles so that I can help keep the platform safe.              | 5      |
