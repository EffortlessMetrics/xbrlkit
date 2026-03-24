# Active Work Tracking

**Purpose:** Tracks currently active work in progress for autonomous coordination.

## Current Status

| Item | Value |
|------|-------|
| **Issue** | Bundle scenarios (SCN-XK-WORKFLOW-002, 004) |
| **AC** | AC-XK-WORKFLOW-002 |
| **Stream** | Workflow |
| **Stage** | 🔍 Research |
| **Started** | 2026-03-24 |

## Scope

Activate bundle.feature scenarios:
- SCN-XK-WORKFLOW-002: Bundle an AC into a bounded context packet
- SCN-XK-WORKFLOW-004: Reject a selector that matches no scenarios

## Research Findings

### Current State
- `scenario-contract` has `BundleManifest` struct defined
- `xbrlkit-feature-grid` compiles feature grid from sidecars
- Bundle step handlers NOT yet implemented in `xbrlkit-bdd-steps`
- No `@alpha-active` tag on bundle.feature

### Required Work
1. Add bundle step handlers to `xbrlkit-bdd-steps/src/lib.rs`:
   - `Given the feature grid is compiled`
   - `When I bundle the selector "{selector}"`
   - `Then the bundle manifest lists scenario "{scenario_id}"`
   - `Then bundling fails because no scenario matches`

2. Create bundle execution logic in `scenario-runner` or `xbrlkit-cli`

3. Add `@alpha-active` tag to scenarios

4. Add AC to alpha-check

---
*This file is maintained by autonomous agents. Last updated: 2026-03-24*
