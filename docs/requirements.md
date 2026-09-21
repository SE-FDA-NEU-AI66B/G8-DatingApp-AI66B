# Requirement

## Product vision

for lonely NEU guys


# User Personas: NEU Dating Webapp

Target Audience: National Economics University (NEU) students.

### Persona 1: The Privacy-Conscious Senior

* **Role:** Hoang Anh – Fourth-year student (Cohort K65), 21 years old, living off-campus. Balances an intensive schedule between an off-campus corporate internship and his graduation thesis.

* **Goal:** Find a meaningful relationship with someone from the same university who understands final-year pressure, while keeping his profile completely invisible to classmates, club peers, and mutual acquaintances.

* **Blocked by:** Fear of social exposure and gossip if peers screenshot his profile; zero patience for endless back-and-forth small talk during work hours.

* **In his words:** *"I just want to find someone compatible to grab coffee with on the weekends, but my biggest nightmare is an acquaintance screenshotting my profile and sharing it in our group chat."*

* **Technical skill:** Accesses the web app primarily via his laptop browser during lunch breaks or his phone's browser on commutes. Requires a discreet web interface and a quick way to switch tabs or hide the screen if a coworker or friend walks by.

* **Interview note:** Spoke with N.V. Hoang Anh (Senior, K65) on September 12.

### Persona 2: The Eager Freshman Explorer

* **Role:** Minh Chau – First-year student (Cohort K68), 18 years old, lives in the campus dormitory. Highly active in university clubs and eager to expand her campus network.

* **Goal:** Connect with upperclassmen and fellow students across campus to build a trusted social circle, find a hangout buddy, and exchange tips on university life and study materials.

* **Blocked by:** Difficulty verifying whether users are legitimate NEU students; mainstream dating platforms feel unsafe, overwhelming, and flooded with strangers with whom she shares zero common context.

* **In her words:** *"I want to date someone from NEU because we share the same campus vibe, but looking for people online feels creepy since you never know who is actually genuine."*

* **Technical skill:** Strictly uses Safari/Chrome on her phone. She prefers a web app so she doesn't have to download a dedicated dating app (which saves phone storage and keeps it hidden from nosy roommates). The web app must be highly responsive and feel native on a small screen.

* **Interview note:** Spoke with D.T. Minh Chau (Freshman, K68) on September 14.

### Persona 3: The Deadline-Driven "Study Date" Seeker

* **Role:** Tuan Kiet – Third-year student (Cohort K66), 20 years old, rents an apartment near campus. Constantly pulls late-night sessions working on group projects, case competitions, and exams.

* **Goal:** Find casual "study-date" partners to work alongside during evening study hours before committing to a formal relationship.

* **Blocked by:** Zero bandwidth for traditional dating routines (formal dinners, cinema outings); most platforms prioritize vanity and pickup lines over schedule matching and study habits.

* **In his words:** *"I don't have time to text for three weeks before meeting. If someone is down to plug in their laptop and grind next to me at a cafe from 9 PM onwards, we will click immediately."*

* **Technical skill:** Keeps the web app open in a background browser tab on his laptop while doing research or writing essays. Relies on browser notifications to see if someone messages him so he doesn't have to constantly check his phone.

* **Interview note:** Spoke with L. Tuan Kiet (Junior, K66) on September 15.

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

## Business rules

### BR1 — Minimum User Age

A user must be at least 18 years old to create or use a dating profile.
Worked example:
Current date: September 16, 2026
User date of birth: September 17, 2008
User age: 17
Result: Registration is rejected.
A user born on September 16, 2008, is exactly 18 and may register.

### BR2 — Profile Completion Requirement

A dating profile must contain all required information before it can appear in discovery.
Required information:
Display name
Age
Location
Biography
At least 1 profile photo
Worked example:
A user provides:
Display name: Alex
Age: 24
Location: Manchester
Biography: Completed
Profile photos: 0

Result: The profile is incomplete and must not appear in discovery.

### BR3 — Mutual Match Requirement

Users may send messages only when both users have liked each other.

Worked example:

    Alex likes Jordan.
    Jordan does not like Alex.
    Match status: No match.
    Result: Alex cannot send Jordan a message.

If Jordan later likes Alex:

    Alex likes Jordan: Yes
    Jordan likes Alex: Yes
    Match status: Mutual match
    Result: Messaging is allowed.

### BR4 — One Decision Per Profile

A user may make only one active decision—like or pass—for the same profile at a time.

Worked example:

    Alex selects Like for Jordan.
    Alex tries to select Like for Jordan again.
    Number of active decisions for Jordan: 2 attempted
    Allowed decisions: 1
    Result: The second like is rejected or ignored.

### BR5 — Message Length Limit

A message must contain between 1 and 500 characters.

Worked example:

    Message A: 120 characters
    Result: Accepted.
    Message B: 0 characters
    Result: Rejected because it is empty.
    Message C: 501 characters
    Result: Rejected because it exceeds the 500-character limit.

### BR6 — Blocked Users Cannot Interact

When a user blocks another user, the blocked user must not appear in discovery, matches, or messages.
Worked example:
Alex blocks Jordan.
Before blocking, Alex has 4 active conversations.
One conversation is with Jordan.
After blocking: 4−1=34−1=3 visible conversations.
Result: Jordan is removed from Alex’s conversations and cannot message Alex.

The block must apply in both directions: Alex must not see Jordan, and Jordan must not see Alex.

## Screens and flow

5 screens Table: Route / Purpose / Access (G, U, A) / Priority. Plus a flow diagram - every screen must appear and be reachable. Hand-drawn is fine.

| Route    | Purpose       | Access (G, U, A) | Priority |
| -------- | ------------- | ---------------- | -------- |
| hashbase | home          | guest /user      | high     |
| hashbase | login         | guest            | high     |
| hashbase | admin console | guest            | high     |
| hashbase | chat          | user             | low      |
