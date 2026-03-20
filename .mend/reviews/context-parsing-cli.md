

---

## Reviewer Correction (2026-03-20 18:52)

Upon re-verification, the `InspectContexts` CLI command **IS present** in the code:
- Defined at line 38 in the `Command` enum
- Implemented at line 94 in the main match statement

The command supports:
- Parsing XBRL instance documents
- Displaying contexts (ID, entity, period, dimensions)
- JSON output mode via `--json` flag

Alpha-check **passes** on this branch (9 scenarios, no failures).

**Revised Verdict: APPROVE** — The PR is complete and ready for merge.

Remaining non-blocking suggestions:
- Add tests for `Forever` period, scenario containers (nice to have)
- Add doc examples (nice to have)
- Address typed dimension TODO (future work)
