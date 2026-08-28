# Process Dossier

## 1. Chosen process and its position on the spectrum

We use a lightweight, iterative process built around short-lived feature branches and pull requests, not a staged plan-driven process. A cycle starts when someone picks up an item from the team's informal backlog (currently tracked in [TODO.md](../TODO.md) rather than a formal board), branches off `main`, and works until the slice is demonstrable — for example the Cloudflare tunnel setup, the Playwright end-to-end scaffold, or a README/SETUP pass. The branch is opened as a pull request, at least one other member reviews it, and it is merged once the reviewer is satisfied. There is no separate design or requirements phase before coding starts.

Our position is close to 85% agile, 15% plan-driven. The evidence is in the repository itself: our git history (`git log --oneline`) already shows four merged pull requests (`#1`–`#4`) built from throwaway branch names — `feature/hai_dep_trai`, `mệt`, `MichaelJackson`, `who-ever-delete-or-merge-this-branch-is-gay` — alongside commits pushed straight to `main` with messages like `stuff`, `net`, and `no toby`. That mix tells us the team already leans informal and adaptive rather than following a documented plan. The fixed 15% is external: the semester milestone dates, the requirement that every merge to `main` go through a reviewed PR, and the core scope (a Rust/Leptos dating-app web project) are not up for renegotiation mid-semester.

## 2. The five diagnostic questions

**Requirements volatility.** Requirements are unstable. [TODO.md](../TODO.md) lists user authentication, a database layer, tests, and documentation as still-open, and the current codebase is a working Leptos/WASM shell (`src/`, `style/`, `assets/`) without any of those. Nothing about the data model or auth flow is locked down yet, so a plan-driven spec written today would be obsolete within a sprint.

**Criticality / safety impact.** Low. This is a course project with no physical or financial control surface. There is no CI pipeline in the repo (no `.github/workflows`) and the only automated check available is a Playwright scaffold under `end2end/` with a single placeholder spec (`tests/example.spec.ts`) — consistent with a project that does not yet need heavyweight verification. The one place criticality does matter is anywhere the app touches user credentials or profile data once authentication lands; those changes should get closer review than a README tweak.

**Team size and communication cost.** Small. Commit authorship (`git log --format='%an'`) shows a handful of active contributors (Sơn Hải/hai291, pham-ha-gif, iambadwithname, mq). At this size, PR comments and chat are enough to resolve most disagreements — we don't need handoff documents or a change-control board.

**Customer/stakeholder availability.** Intermittent, not continuous. The instructor evaluates at fixed checkpoints (milestones, final demo); there is no always-available product owner. Feedback has to be batched at each checkpoint and turned into concrete TODO/issue items afterward rather than incorporated continuously.

**Culture.** The team already defaults to "just push a branch and open a PR" — visible from the four real merged PRs in history — rather than following a rigid process document. That favors keeping the process agile: a heavier process would fight the team's existing habits instead of building on them.

## 3. Critical thinking: risks of the opposite choice

If we instead adopted a plan-driven process — full spec for auth, database schema, and UI before writing code — the dominant risk is wasted design work under changing requirements. The mechanism: with authentication and persistence still undecided (per TODO.md), a detailed upfront design would encode assumptions (session model, schema shape) that are likely to be wrong once real screens and flows get built in Leptos. The first visible symptom would be a milestone where `docs/` contains a thorough design but `src/` still doesn't compile against it, because implementation reality diverged from the plan before review caught it — exactly the failure mode a small, requirements-volatile, low-criticality team is least equipped to absorb.

## 4. Process rules our team commits to

- Every change to `main` goes through a pull request; direct pushes to `main` are no longer allowed even for small fixes, closing the gap visible in our current history.
- A PR needs at least one approval from someone other than the author, with a substantive comment (not just "LGTM"), before it can be merged. Self-approved or unreviewed merges are treated as process failures, not shortcuts.
- Branch names describe the work (e.g. `feature/auth-login`, `docs/process-dossier`) instead of the joke names in our current history, so reviewers and future readers can tell what a branch is for.
- Any PR touching authentication or stored user data must say in the description what data is affected and how it's protected, even before a formal security review process exists.
- TODO.md items move to "in progress" only once a branch exists for them, so the backlog reflects what's actually being worked on.
