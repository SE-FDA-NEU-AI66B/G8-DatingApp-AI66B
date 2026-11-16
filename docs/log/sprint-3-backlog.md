# Sprint 3 backlog refinement

This is the reviewed order for the Milestone 3 backlog. Estimates use points
(`S`, `M`, `L`) because the team has not adopted a numeric velocity scale for
individual tasks. Owners are the GitHub assignees; a shared owner means the
work requires both implementation and review.

| Order | Issue | Scope | Estimate | Owner(s) | Depends on | Done evidence |
|---:|---:|---|:---:|---|---|---|
| 1 | #87 | P0 story/status matrix | M | `pham-ha-gif` | — | Matrix links each P0 story to states and tests |
| 2 | #88 | Wireframes for P0 screens | M | `pham-ha-gif` | #87 | Wireframes cover every P0 screen |
| 3 | #89 | M3 error-message catalog | S | `pham-ha-gif` | #87 | Catalog is referenced by UI/error states |
| 4 | #90 | Assemble `docs/ui.md` | M | `pham-ha-gif` | #87, #88, #89 | UI document is committed and linked |
| 5 | #91 | Prepare M3 evidence links | S | `pham-ha-gif` | #90 | Evidence index links artifacts and validation |
| 6 | #92 | Dashboard loading/data/empty/refresh | M | `iambadwithname`, `hai291` | #87, #90 | Component and browser behavior verified |
| 7 | #93 | Dashboard error/unauthorized states | M | `iambadwithname`, `hai291` | #92 | 401/403/API failures have recovery states |
| 8 | #94 | Responsive and accessible dashboard UX | M | `iambadwithname`, `hai291` | #92, #93 | Mobile and keyboard/screen-reader checks pass |
| 9 | #95 | Complete P0 browser tests | M | `iambadwithname` | #92, #93, #94 | Playwright covers success and failures |
| 10 | #96 | Seeded data and test fixtures | M | `iambadwithname` | #95 | Fixtures are repeatable and isolated |
| 11 | #97 | Refine Sprint 3 backlog | S | `hai291` | #87–#96 | This table is reviewed and linked from the issue |
| 12 | #98 | Complete admin dashboard P0 flow | L | `hai291` | #92–#96 | Entry point, states, browser test, and screenshot |
| 13 | #99 | Update M3 project documents | M | `hai291` | #97, #98 | README/setup/design/log/traceability agree |
| 14 | #100 | Capture Sprint 3 review evidence | M | `hai291` | #98, #99 | Board, real-data, error, and validation evidence |
| 15 | #101 | Sprint 3 wrap-up | M | `hai291` | #99, #100 | Summary, links, contributions, and checklist complete |

## Board and dependency notes

- Keep the order above in the Sprint 3 board. Issues #87–#91 establish the
  product and evidence vocabulary; #92–#96 implement and verify the P0 slice;
  #97–#101 close the sprint and submission loop.
- #98 is the integration story, not a replacement for its implementation
  tasks. It should be closed only after #92–#96 have demonstrable evidence.
- #101 is intentionally last because its velocity and contribution summary
  must use the final state of the preceding work.

Reviewed against the Sprint 3 issue list on 2026-10-17.
