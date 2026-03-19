# Friction Log

**Log pattern:** Date | Friction | Impact | Potential Fix | Status

**Friction:** Context switching between repos is cognitively heavy — remembering branches, locations, what I was doing
**Impact:** Medium — interrupts flow, adds mental overhead
**Potential Fix:** `mctx` automation script for one-command context switching
**Status:** ✅ FIXED — built mctx, msync, mstart/mend automation

---

## 2026-03-19 (continued)

**Friction:** Tool call parameter syntax errors (`oldText` vs `old_string`, `file_path` vs `path`)
**Impact:** Low but annoying — breaks flow, causes retry
**Potential Fix:** Consistent wrapper functions or IDE-level validation
**Status:** Logged — low priority, may self-resolve with practice

---

**Friction:** Multiple PRs in flight across repos, hard to track state
**Impact:** Medium — context-switching overhead, risk of working on wrong branch
**Potential Fix:** `mstatus` command showing active branches/commits across all repos
**Status:** Logged — consider if it recurs 2+ more times

---

**Friction:** Conversation context compaction loses early context
**Impact:** Medium — have to re-read files to recover state
**Potential Fix:** Better session state tracking, proactive memory writes
**Status:** Logged — partially mitigated by session capture automation

---

**Friction:** Scenario activation requires touching 5+ files (feature, steps, runner, alpha_check, etc.)
**Impact:** Medium — slows down each activation, error-prone
**Potential Fix:** Codegen or macro to scaffold scenario activation
**Status:** Logged — consider if pattern repeats 3+ times

---

**Friction:** Golden files drift when scenario metadata changes
**Impact:** Low — caught by CI, but requires manual fix
**Potential Fix:** Auto-update golden files in CI or make tests more tolerant
**Status:** Logged — current fix (manual update) is acceptable for now

---

## How to Use This Log

When friction repeats:
1. Count occurrences (3+ = pattern)
2. Estimate time lost per occurrence
3. If fix effort < (time lost × remaining occurrences), prioritize fix
4. Create improvement issue in kimi-claw-workspace
