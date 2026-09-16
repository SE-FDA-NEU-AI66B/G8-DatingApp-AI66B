# <Project name> — INT2208 Software Engineering

> Replace this heading with your product name, then delete this quote block.

**Team:** <name> · **Members:** @user1 @user2 @user3 @user4
**Product Owner (fixed all term):** @
**Scrum Master (rotates every sprint):** @

---

## The product in one sentence

<Who it is for, what it does, and why they would use it instead of the obvious alternative.>

## Running it

```bash
git clone <this repo>
cd <repo>
# setup commands here — a person with none of your local state must be able to follow these
```

Then open <http://localhost:3000>.

> Keep these steps working. The Definition of Done requires the project to run
> from a clean clone, and the marker will test exactly this.

---

## How we work

We run Scrum on this repository. Five two-week sprints, plus a setup week.

| Sprint   | Weeks | Opens with              | Milestone               |
| -------- | ----- | ----------------------- | ----------------------- |
| Sprint 0 | 4     | Scrum & XP              | —                       |
| Sprint 1 | 5–6   | Requirements            | **M1** Sun before wk 7  |
| Sprint 2 | 7–8   | Architecture & REST API | **M2** Sun before wk 9  |
| Sprint 3 | 9–10  | UI design               | **M3** Sun before wk 11 |
| Sprint 4 | 11–12 | Testing & TDD           | **M4** Sun before wk 13 |
| Sprint 5 | 13–14 | DevOps & CI/CD          | **M5** week 15 demo     |

**Board:** <link to the team's view of the course Project>
**Definition of Done:** [`docs/definition-of-done.md`](docs/definition-of-done.md)
**Sprint log:** [`docs/sprint-log.md`](docs/sprint-log.md)
**Traceability:** [`docs/traceability.md`](docs/traceability.md)

### The rules that are actually enforced

1. **No work without an issue.** If it is not on the board, it does not exist.
   CI rejects a pull request that links to no issue.
2. **No commits to `main`.** Branch, open a PR, get a teammate to approve it.
3. **One branch per issue**, named `<issue-number>-short-description`
   (for example `12-address-book-endpoint`).
4. **Commit messages** follow `<type>: <what changed>` where type is one of
   `feat` `fix` `test` `docs` `refactor` `chore`.
   Then a blank line, then `Refs #12`.
5. **Review someone else's code every sprint.** Reviewing is graded work,
   not a favour you do for your teammates.

### Why the rules exist

Your individual mark is derived from what this repository records: issues you
owned and closed, pull requests you authored, and reviews you gave. Work done
outside this system is invisible to the grader, no matter how much of it there
was. A teammate committing on your behalf transfers your marks to them.

Commit steadily. Twenty commits spread across ten weeks is a healthy project;
twenty commits in the last three days is a red flag that will be raised at the
demo.

---

## Branching

```
main        ← protected. only merged PRs. always demoable.
  └── develop           (optional — use it if you prefer, agree as a team)
        └── 12-address-book-endpoint
        └── 15-fix-login-redirect
```

## Team agreement

Filled in at Sprint 0 and not changed afterwards without a retro decision.

- **Stand-up:** <day and time, 15 minutes, in person or on Discord>
- **Core hours:** <when teammates can expect a reply>
- **Definition of "responsive":** <e.g. reply within 24h on weekdays>
- **If someone stops responding:** <what the team does, before it becomes a crisis>
