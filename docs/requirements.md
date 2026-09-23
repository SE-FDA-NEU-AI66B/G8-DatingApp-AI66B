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

### Scenario 1 - Hoang Anh arranges a discreet weekend coffee

1. During his lunch break at the internship, Hoang Anh opens the web app in his laptop browser.
2. He applies a privacy filter to ensure his profile is completely hidden from anyone in his own cohort year.
3. He indicates his availability for a casual coffee date this upcoming Saturday afternoon.
4. He browses through a curated list of younger students who share his weekend availability and similar music tastes.
5. He finds a compatible profile and sends a direct invitation to grab coffee.
6. He immediately switches browser tabs to return to his spreadsheet work.
7. Later that evening, while commuting home, he opens the web app on his phone browser.
8. He sees the invitation was accepted and exchanges two quick messages to finalize the exact cafe location.

### Scenario 2 - Minh Chau safely connects with an upperclassman

1. While resting in the dormitory, Minh Chau opens the web app using her phone's browser.
2. She logs in using her official university student email, ensuring her profile receives a verified student badge.
3. She filters her view to only show older students who have successfully verified their university credentials.
4. She looks for individuals who have explicitly indicated they are open to sharing campus tips or casual food outings.
5. She matches with a sophomore who also enjoys street food near the university gates.
6. They exchange a few messages about their favorite local snack stalls to establish mutual trust.
7. She proposes a quick meeting time for the following afternoon right after her last class.
8. The other student confirms the time, giving Minh Chau peace of mind that she is meeting a legitimate, verified student.

### Scenario 3 - Tuan Kiet finds a last-minute study partner

1. At 8:00 PM, Tuan Kiet is preparing for a long night of writing an essay.
2. He opens the web app in a background tab on his laptop and sets his status to look for an immediate study session.
3. He specifies a 24-hour cafe near the university that he plans to walk to in twenty minutes.
4. He leaves the tab open and switches over to his word processor to start outlining his essay.
5. Ten minutes later, a browser notification appears on his screen alerting him of a potential study match.
6. He checks the notification and sees another student is also heading to the same cafe to study for an exam.
7. He sends a quick message asking if they want to share a corner table to work quietly.
8. The other student agrees instantly, and they coordinate exactly which floor they will meet on.

## 3. User Stories

| ID | Story | Priority | Points |
| :--- | :--- | :--- | :--- |
| US01 | As Minh Chau, I want to register using my @neu.edu.vn email so that I am verified as a real NEU student. | P0 | 5 |
| US02 | As Tuan Kiet, I want to create a "study date" request with a specific time slot so that others can join me. | P0 | 8 |
| US03 | As Minh Chau, I want to browse active study date requests within a 24-hour window so I can find a study partner today. | P0 | 5 |
| US04 | As Hoang Anh, I want to send a match request to a specific profile so that we can potentially connect. | P0 | 5 |
| US05 | As Hoang Anh, I want to accept or decline an incoming match request so that I can control who messages me. | P0 | 3 |
| US06 | As Tuan Kiet, I want to send text messages via the web interface so we can coordinate our meeting. | P1 | 8 |
| US07 | As Minh Chau, I want to filter matches by cohort (e.g., K65, K68) so that I can connect with specific academic years. | P1 | 3 |
| US08 | As Hoang Anh, I want to toggle a "Hide from my cohort" setting so that I avoid people I might already know. | P1 | 5 |
| US09 | As a user, I want to report a suspicious profile so that the platform remains safe. | P2 | 3 |
| US10 | As Tuan Kiet, I want to deactivate my account temporarily so that my profile is hidden during exam seasons. | P2 | 2 |

**US01 - Register with NEU email · P0 · 5 points · Screen: /register**
As Minh Chau, I want to register using my @neu.edu.vn email so that I am verified as a real NEU student.
* **Acceptance criteria**
  * Given I enter the email "minhchau@neu.edu.vn" and submit, then my account is created and I receive a 6-digit OTP code for verification.
  * Given I enter the email "minhchau@gmail.com", when I submit the form, then it is rejected with the exact message "Please use a valid @neu.edu.vn email address".
* **Tasks**
  * Registration UI and input validation - @quang
  * Backend authentication and email OTP routing - @hai
  * Automated tests for domain rejections - @quyen

**US02 - Create a study date request · P0 · 8 points · Screen: /study-date/new**
As Tuan Kiet, I want to create a "study date" request with a specific time slot so that others can join me for a deadline session.
* **Acceptance criteria**
  * Given I select a start time of 20:00 and duration of 3 hours, when I publish the request, then it appears on the live feed for 180 minutes.
  * Given I try to set a duration of 14 hours, when I submit, then it is rejected with the message "Maximum study date duration is 12 hours".
* **Tasks**
  * Time slot picker and submission form - @quang
  * Database schema for study sessions - @hai
  * Validation logic for maximum duration rules - @quyen

**US03 - Browse active study date requests · P0 · 5 points · Screen: /feed**
As Minh Chau, I want to browse active study date requests within a 24-hour window so I can find a study partner today.
* **Acceptance criteria**
  * Given there are 5 active requests starting within the next 24 hours, when I load the feed, then exactly 5 cards are displayed in chronological order.
  * Given there are 0 active requests, when the page loads, then it shows "0 study dates found for today" instead of a blank screen.
* **Tasks**
  * Feed interface and card components - @quang
  * API endpoint to fetch and sort chronological data - @hai
  * Integration tests for empty states - @quyen

**US04 - Send a match request · P0 · 5 points · Screen: /profile/:id**
As Hoang Anh, I want to send a match request to a specific profile so that we can potentially connect.
* **Acceptance criteria**
  * Given I view a profile I haven't matched with, when I click "Connect", then the button changes to "Pending" and I cannot send a second request.
  * Given I have already sent 20 requests today, when I try to send another, then it is blocked with the message "Daily limit of 20 requests reached".
* **Tasks**
  * Profile view and connection button state - @quang
  * Matchmaking logic and daily limit constraints - @hai
  * Test cases for button state changes - @quyen

**US05 - Accept or decline a match request · P0 · 3 points · Screen: /requests**
As Hoang Anh, I want to accept or decline an incoming match request so that I can control who messages me.
* **Acceptance criteria**
  * Given I have an incoming request from User A, when I click "Accept", then a chat room is created and User A moves to my "Matched" list.
  * Given I have 3 incoming requests, when I click "Decline" on one, then the list immediately updates to show exactly 2 pending requests.
* **Tasks**
  * Incoming request list UI - @quang
  * State update logic (Accept/Decline handling) - @hai
  * Tests for list rendering updates - @quyen

**US06 - Real-time chat messaging · P1 · 8 points · Screen: /chat/:id**
As Tuan Kiet, I want to send text messages via the web interface so we can coordinate our meeting.
* **Acceptance criteria**
  * Given I am in a matched chat room, when I send a message, then it appears in the chat log within 1 second.
  * Given I try to send a message exceeding 500 characters, when I hit enter, then the system truncates the input or displays "Message exceeds 500 characters".
* **Tasks**
  * Chat UI and scroll behavior - @quang
  * WebSocket/Backend messaging implementation - @hai
  * Character limit validation and tests - @quyen

**US07 - Filter matches by cohort · P1 · 3 points · Screen: /discover**
As Minh Chau, I want to filter matches by cohort so that I can connect with specific academic years.
* **Acceptance criteria**
  * Given I select the "K68" filter, when the list updates, then 100% of the displayed profiles have the cohort attribute "K68".
  * Given I select multiple cohorts, when I apply the filter, then the URL query updates to include `?cohorts=K66,K68`.
* **Tasks**
  * Filter dropdown and URL param syncing - @quang
  * Database query optimization for cohort tags - @hai
  * Testing multiple cohort selections - @quyen

**US08 - Hide from my cohort · P1 · 5 points · Screen: /settings**
As Hoang Anh, I want to toggle a "Hide from my cohort" setting so that I avoid people I might already know.
* **Acceptance criteria**
  * Given I am K65 and I toggle this setting ON, when another K65 user searches the feed, then my profile is returned 0 times in their results.
  * Given the setting is OFF by default, when I view my settings panel for the first time, then the toggle shows as disabled (grayed out).
* **Tasks**
  * Settings toggle UI - @quang
  * Query exclusion logic for matching cohorts - @hai
  * Privacy rule unit tests - @quyen

**US09 - Report a suspicious profile · P2 · 3 points · Screen: /profile/:id**
As a user, I want to report a suspicious profile so that the platform remains safe.
* **Acceptance criteria**
  * Given I submit a report, when it processes, then the reported user's profile is immediately hidden from my personal view.
  * Given a profile receives exactly 5 unique reports in 24 hours, then the system automatically flags it for admin review.
* **Tasks**
  * Reporting modal and reason selection - @quang
  * Flagging logic and report aggregation - @hai
  * End-to-end test for report submission - @quyen

**US10 - Deactivate account temporarily · P2 · 2 points · Screen: /settings**
As Tuan Kiet, I want to deactivate my account temporarily so that my profile is hidden during exam seasons.
* **Acceptance criteria**
  * Given I click "Deactivate", when I confirm, then my session ends and my profile is hidden for at least 7 days.
  * Given my account is deactivated, when I attempt to log back in before 7 days, then I see a prompt "Account deactivated. Reactivate now?".
* **Tasks**
  * Deactivation confirmation dialog - @quang
  * Account status toggle in database - @hai
  * Login flow restrictions - @quyen

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

| Route          | Purpose                          | Access (G, U, A) | Priority |
| -------------- | -------------------------------- | ---------------- | -------- |
| /              | home                             | G                | P0       |
| /user_profile  | user profile for managing account| U                | P0       |
| /user_home     | show status + chat interact      | U                | P0       |
| /chat          | for chatting                     | U                | P0       |
| /admin_console | show server status               | A                | P0       |
