## Tested by

Automated validation currently covers the repository build and library tests:
`cargo fmt --all -- --check`, `cargo check --features ssr`, and
`cargo test --lib --features ssr`. An independent fresh-machine walkthrough is
still required before submission; record the tester handle, operating system,
date, and elapsed time here, for example:
`@iambadwithname — Windows 11 — 2025-03-08 — 12 minutes`.

## Submission workflow

Create a documentation branch, commit `docs/design.md`, `docs/SETUP.md`, and
the ERD, then open a pull request into `main`. A different member must select
**Review changes**, leave a substantive comment, and approve. Merge only after
that approval. Export the merged `docs/design.md` to exactly
`Team<NN>_M2.pdf` and submit that one PDF to the LMS.
