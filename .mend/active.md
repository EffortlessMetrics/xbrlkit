# Active Work Tracking

**Purpose:** Tracks currently active work in progress for autonomous coordination.

## Current Status

| Item | Value |
|------|-------|
| **Phase** | Phase 4 Performance Optimization ✅ |
| **Status** | Wave 4 Streaming Parser complete; next focus is 19 `@alpha-candidate` scenarios |

## Phase 3: Feature Completeness — COMPLETE ✅

### Wave 1: Required Facts Validation (P0) ✅
- #9: Required facts validation ✅ Already implemented
- AC-XK-SEC-REQUIRED-001/002 passing

### Wave 2: Numeric Validation (P1) ✅
- #80: Negative value validation ✅ #86
- #81: Decimal precision validation (EFM 6.5.37) ✅ #93

### Wave 3: Context and Unit Validation (P1) ✅
- #82: Unit consistency validation ✅ #88
- #83: Context completeness validation ✅ #90

### Dimensional Validation (P1) ✅
- `dimensional-rules` crate created
- Taxonomy dimensions with domain hierarchies ✅ #23
- SCN-XK-DIM-001 to 004 active in the compiled feature grid
- SCN-XK-DIM-005 to 017 remain feature-tagged but are not sidecar-registered and are not selected by active BDD runs

## Phase 4: Performance Optimization — COMPLETE ✅

### Wave 4: Streaming Parser — COMPLETE ✅
- **Status:** Crate created, tests passing, integration complete
- **PR:** [#95](https://github.com/EffortlessMetrics/xbrlkit/pull/95) (merged 2026-03-27)
- **Components:**
  - `xbrl-stream` crate with SAX-style parsing
  - `validation-run` integration
  - 4 BDD scenarios (`@alpha-active`)

### Next Activation Wave

The next 19 `@alpha-candidate` scenarios are concentrated in three feature files:

- `context_completeness.feature`: 4 scenarios
- `decimal_precision.feature`: 10 scenarios
- `negative_values.feature`: 5 scenarios

## Final Metrics
- 33 @alpha-active scenarios passing
- 100 workspace tests listed by `cargo nextest`
- CI: Green
- Phase 4: 100% complete (streaming parser foundation shipped)

---
*Phase 4 complete. Streaming parser foundation shipped; the next activation wave is listed above.*
