# Process Dossier

## 1. Chosen process and its position on the spectrum

**(a) The model.** We follow an **Incremental model**, with each increment run under agile practices rather than a plan-driven gate. One cycle: someone picks an open item from [TODO.md](../TODO.md), branches off `main`, implements it, and opens a pull request. At least one other member reviews and leaves a substantive comment; the author addresses it; the reviewer approves; the branch merges. The cycle ends with a working increment on `main` — e.g. the Cloudflare tunnel setup, the Playwright end-to-end scaffold, or a README/SETUP pass — not a design document.

**(b) The position.** We sit at roughly 80% agile, 20% plan-driven. Evidence for the agile lean: our git history already shows four merged PRs (`#1`–`#4`) from short, disposable branches, plus commits pushed straight to `main` with messages like `stuff` and `net` — a team that ships increments, not specs. **Frozen for the whole semester:** the four milestone dates and final demo, the rule that every merge to `main` goes through a reviewed PR, and the core scope (a Rust/Leptos dating-app web project). **Reopened every cycle:** backlog priority, UI details, and the design of authentication and the database — all still undecided per TODO.md.

## 2. The five diagnostic questions

**Requirements stability.** Volatile. TODO.md lists authentication, a database layer, tests, and documentation as open; the current `src/` tree is a Leptos/WASM shell with none of those wired in, so any spec written today would be guesswork.

**Safety and legal impact.** Safety impact is negligible — no physical or financial control surface, and no CI exists yet (no `.github/workflows`), consistent with low required rigor. Legal impact is not zero: this is a dating app, so once auth and a database land we will be storing profile data and need basic handling documentation. No PII is stored yet, so this doesn't demand change control today, but it will shape review rules once persistence exists (see Section 4).

**Team size and communication cost.** Small and co-located. Commit authorship shows a handful of active contributors (Sơn Hải/hai291, pham-ha-gif, iambadwithname, mq). PR comments and chat resolve most disagreements; we don't need handoff documents.

**Customer availability.** Fixed checkpoints only. The instructor evaluates at milestones and the final demo, not continuously, so feedback is batched per checkpoint and turned into TODO items afterward.

**Culture and contract constraints.** The course fixes the four milestones and demo date; internal process choice is otherwise ours. Before submission we will make the repository accessible to the instructor (public, or add as a collaborator), and this single `docs/process.md`, once merged, is the one process file for the whole team.

## 3. Critical thinking: risks of the opposite choice

If we instead went fully plan-driven — a complete spec for auth and the database schema before writing code — the biggest risk is wasted design under requirements we don't yet understand. Mechanism: with auth and persistence undecided (Section 2), a detailed upfront design would lock in assumptions likely to be wrong once real Leptos screens get built, forcing a rewrite of the spec itself. The first observable symptom would be a milestone where `docs/` holds a thorough design but `src/` doesn't compile against it, because implementation reality diverged from the plan before any review caught it.

## 4. Process rules our team commits to

- Every change to `main` goes through a pull request reviewed and approved by at least one other member; self-approved or unreviewed merges don't count as done.
- Sprint length is two weeks; TODO.md is re-prioritized at the start of each sprint.
- Any requirement change made after a sprint starts is recorded in `docs/changelog.md` before implementation begins.
- Branches are named for the work (`feature/auth-login`, `docs/process-dossier`), not jokes, so history stays traceable.
- Any PR touching authentication or stored user data must state in its description what data is affected and how it's protected.
