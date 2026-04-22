# Plan: Set up Autonomous Workflow Labels (Issue #98)

## Overview

This plan addresses Issue #98 which proposes establishing a consistent labeling system to support autonomous workflow management of issues and PRs in the xbrlkit repository. The labels enable clear visual categorization, filtered views for different work streams, and automated workflow triggers.

The issue specifies 5 new labels with distinct colors and purposes, designed to integrate with existing label patterns (like `status/*` and `swarm-*`) while introducing a new `agent/*` namespace for autonomous workflow markers.

## Acceptance Criteria Breakdown

### AC-1: Create Label `agent/autonomous`
- **Color**: Blue (1D76DB)
- **Purpose**: Marks issues/PRs created or managed by autonomous workflow
- **Usage**: Applied automatically when an agent creates an issue or PR

### AC-2: Create Label `agent/in-review`
- **Color**: Yellow (FBCA04)
- **Purpose**: Indicates items currently under active review
- **Usage**: Applied when an agent is actively reviewing an issue/PR

### AC-3: Create Label `agent/wip`
- **Color**: Orange (D93F0B)
- **Purpose**: Work in progress - not ready for review
- **Usage**: Applied when an agent is working on an issue but it's not yet ready

### AC-4: Create Label `agent/needs-human`
- **Color**: Red (D73A4A)
- **Purpose**: Requires human decision before proceeding
- **Usage**: Applied when autonomous workflow encounters a blocker requiring human judgment

### AC-5: Create Label `agent/tech-debt`
- **Color**: Purple (7057FF)
- **Purpose**: Technical debt, legacy code, or maintenance tasks
- **Usage**: Applied to items identified as technical debt during autonomous workflow

### AC-6: Document Label Usage
- Document each label's purpose and when to apply/remove
- Include in contributing guidelines or agent documentation

### AC-7: Verify Label Creation
- All 5 labels exist in the repository
- Colors match specifications
- Descriptions are clear and actionable

## Proposed Approach

### Label Naming Convention
Following the existing repository patterns:
- `status/*` for workflow state (already exists: `status/ready-to-merge`, `status/needs-review`, etc.)
- `swarm-*` for swarm-specific categorization (already exists: `swarm-core`, `swarm-improve-docs`, etc.)
- `agent/*` for autonomous workflow markers (NEW namespace)

The `agent/*` prefix clearly distinguishes autonomous workflow labels from human workflow labels (`status/*`).

### Color Scheme Mapping
| Emoji | Color | Hex Code | Label |
|-------|-------|----------|-------|
| 🔵 | Blue | 1D76DB | agent/autonomous |
| 🟡 | Yellow | FBCA04 | agent/in-review |
| 🟠 | Orange | D93F0B | agent/wip |
| 🔴 | Red | D73A4A | agent/needs-human |
| 🟣 | Purple | 7057FF | agent/tech-debt |

### Implementation Strategy

1. **Label Creation**: Use GitHub CLI (`gh label create`) to create all 5 labels with proper colors and descriptions
2. **Documentation**: Update agent documentation to reference these labels
3. **Automation Hooks**: Labels are designed to be applied/removed by autonomous workflows (future work)

## Files to Modify/Create

### New Files (1)
1. `.github/labels.yml` (optional) - YAML manifest for label definitions enabling GitHub Actions or CLI scripts to sync labels

### Modified Files (1)
1. `AGENTS.md` - Document the autonomous workflow labels and their usage

### No Code Changes Required
This is a repository configuration task; no Rust code changes are needed.

## Test Strategy

### Verification Steps
1. **Label Existence**: Run `gh label list` to verify all 5 labels exist
2. **Color Verification**: Visually inspect label colors in GitHub UI match specification
3. **Description Check**: Verify descriptions are clear and accurate
4. **Integration Test**: Apply labels to test issue and verify they display correctly

### Test Commands
```bash
# Verify all labels exist
gh label list --repo EffortlessMetrics/xbrlkit --json name | jq -r '.[].name' | grep "^agent/"

# Expected output:
# agent/autonomous
# agent/in-review
# agent/needs-human
# agent/tech-debt
# agent/wip
```

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| Color accessibility issues | Low | Low | Use GitHub's default accessible colors; verify contrast |
| Label name conflicts with existing automation | Low | Medium | Check existing workflows for label patterns; `agent/*` is new namespace |
| Human confusion about when to apply agent labels | Medium | Medium | Clear documentation in AGENTS.md; labels are primarily for agent use |
| Label bloat with too many labels | Low | Low | Only 5 new labels; fits within existing label structure |

## Estimated Effort

| Task | Estimate |
|------|----------|
| Create 5 labels via GitHub CLI | 30 min |
| Update AGENTS.md documentation | 1h |
| Verify labels and test | 30 min |
| **Total** | **~2 hours** |

## Implementation Commands

### Create Labels (gh CLI)
```bash
# Blue - Marks issues/PRs created by autonomous workflow
gh label create "agent/autonomous" --color "1D76DB" --description "Marks issues/PRs created or managed by autonomous workflow" --repo EffortlessMetrics/xbrlkit

# Yellow - Items under active review
gh label create "agent/in-review" --color "FBCA04" --description "Indicates items currently under active review by an agent" --repo EffortlessMetrics/xbrlkit

# Orange - Work in progress
gh label create "agent/wip" --color "D93F0B" --description "Work in progress - not ready for review" --repo EffortlessMetrics/xbrlkit

# Red - Requires human decision
gh label create "agent/needs-human" --color "D73A4A" --description "Requires human decision before proceeding" --repo EffortlessMetrics/xbrlkit

# Purple - Technical debt
gh label create "agent/tech-debt" --color "7057FF" --description "Technical debt, legacy code, or maintenance tasks" --repo EffortlessMetrics/xbrlkit
```

## Dependencies

- **GitHub CLI (`gh`)**: Required for label creation
- **Repository Permissions**: Write access to create labels

## Future Work

After label creation, autonomous workflows can be enhanced to:
1. Auto-apply `agent/autonomous` when creating issues/PRs
2. Transition labels as work progresses (`agent/wip` → `agent/in-review` → `status/ready-for-review`)
3. Auto-apply `agent/needs-human` when encountering decision points
4. Generate reports filtered by `agent/*` labels

---
*Plan created by planner-initial agent*
