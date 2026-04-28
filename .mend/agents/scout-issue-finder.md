# Scout Issue Finder

## Purpose

The Scout Issue Finder runs periodic scans to discover and file issues for:
- Unactivated scenarios (feature files with @alpha-candidate but not @alpha-active)
- Placeholder crates (src/lib.rs < 20 lines or minimal implementation)
- Orphaned scenarios (scenarios in features without corresponding metadata entries)
- Missing step handlers (Gherkin steps without handler implementations in xbrlkit-bdd-steps)
- Broken plans (plans in .mend/plans/ that reference completed or non-existent issues)
- Schema drift (receipt structs vs JSON schemas)

## Scout Procedure

### 1. Scan for Unactivated Scenarios

**Detection:** Find scenarios with @alpha-candidate or other pre-active tags that aren't @alpha-active

**Files to check:**
- `specs/features/**/*.feature`

**Command:**
```bash
grep -rn "@alpha-candidate" specs/features/
grep -rn "@alpha-future" specs/features/
grep -rn "@alpha-pending" specs/features/
```

### 2. Scan for Placeholder Crates

**Detection:** Crates with minimal lib.rs (< 20 lines) or placeholder implementations

**Files to check:**
- `crates/*/src/lib.rs`
- `crates/*/Cargo.toml`

**Command:**
```bash
for crate in crates/*/; do
  lines=$(wc -l < "$crate/src/lib.rs" 2>/dev/null || echo 0)
  if [ "$lines" -lt 20 ]; then
    echo "Placeholder: $crate ($lines lines)"
  fi
done
```

### 3. Scan for Orphaned Scenarios

**Detection:** Scenarios in feature files that don't have corresponding metadata entries in .meta.yaml files

**Files to check:**
- `specs/features/**/*.feature`
- `specs/features/**/*.meta.yaml`

### 4. Scan for Missing Step Handlers

**Detection:** Gherkin steps without corresponding handler implementations in xbrlkit-bdd-steps

**Files to check:**
- `specs/features/**/*.feature` (extract step patterns)
- `crates/xbrlkit-bdd-steps/src/` (check for handlers)

### 5. Scan for Broken Plans

**Detection:** Plans in .mend/plans/ that reference completed/non-existent issues or are stale

**Files to check:**
- `.mend/plans/*.md`

### 6. Check ACTIVE_ALPHA_ACS Alignment

**Detection:** Scenarios marked @alpha-active in features but not in ACTIVE_ALPHA_ACS list

**File to check:**
- `xtask/src/alpha_check.rs`

## Issue Templates

### Scenario Activation Issue

```markdown
## Scenario Activation: [SCENARIO-ID]

**Feature:** [feature-file.feature]
**AC ID:** [AC-ID]
**Current Status:** @alpha-candidate

### Pre-Activation Checklist
- [ ] @alpha-active tag added to feature file
- [ ] ac_id added to meta.yaml
- [ ] profile_pack added to meta.yaml (if needed)
- [ ] Step handlers implemented in xbrlkit-bdd-steps
- [ ] AC assertion added to scenario-runner
- [ ] AC added to ACTIVE_ALPHA_ACS in xtask/src/alpha_check.rs
- [ ] cargo test --workspace passes
- [ ] cargo xtask alpha-check passes locally

### Related
- Issue #[tracking-issue]
```

### Placeholder Crate Issue

```markdown
## Placeholder Crate: [crate-name]

**Location:** `crates/[crate-name]/`
**Current Lines:** [count]

### Description
This crate has minimal implementation and needs to be fleshed out.

### Suggested Implementation
- [ ] Core types and interfaces
- [ ] Error handling
- [ ] Unit tests
- [ ] Documentation
```

## Scout Run Output Format

Report findings in the following format:

```
## Scout Report: [Date]

### Unactivated Scenarios: [N]
| Scenario | Feature | AC | Status |
|----------|---------|-----|--------|
| SCN-XXX | xxx.feature | AC-XXX | @alpha-candidate |

### Placeholder Crates: [N]
| Crate | Lines | Notes |
|-------|-------|-------|
| xxx | 15 | Minimal lib.rs |

### Orphaned Scenarios: [N]
### Missing Handlers: [N]
### Broken Plans: [N]

### Actions Taken
- Filed issue #XXX for [description]
- Updated issue #XXX with findings
- Applied auto-fix: [description]
```

## Automation

To run the scout:

```bash
# From xbrlkit repo root
read .mend/agents/scout-issue-finder.md
# Execute the scout procedure
# File issues for findings
# Post summary to issue #119
```
