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
