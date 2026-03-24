# Use the Impact Workflow

The impact workflow analyzes which scenarios are affected by changes to specific files. This helps optimize testing by running only the scenarios that could be impacted by your changes.

## What is Impact Analysis?

Impact analysis examines changed file paths and determines which scenarios need to be re-tested. It uses the feature grid metadata to map files to scenarios.

**Output:** `artifacts/impact/impact.report.v1.json`

## Run Impact Analysis

### Single File Change

Check impact of changing one file:

```bash
cargo xtask impact --changed crates/ixds-assemble/src/lib.rs
```

Output:
```
impact: wrote artifacts/impact/impact.report.v1.json
```

### Multiple File Changes

Check impact of multiple changes:

```bash
cargo xtask impact \
  --changed crates/ixds-assemble/src/lib.rs \
  --changed crates/ixhtml-scan/src/scanner.rs \
  --changed specs/features/sec/inline_restrictions.feature
```

### Changed Directories

Impact analysis works with directories too:

```bash
cargo xtask impact --changed crates/efm-rules/
```

## How Impact Detection Works

A scenario is marked as impacted if:

1. **Allowed Edit Root Match**: The changed path starts with one of the scenario's `allowed_edit_roots`
   - Example: Changing `crates/ixhtml-scan/src/lib.rs` impacts scenarios with `allowed_edit_roots: ["crates/ixhtml-scan"]`

2. **Crate Match**: The changed path contains one of the scenario's crate names
   - Example: Changing `crates/efm-rules/src/rules.rs` impacts any scenario listing `efm-rules` in its `crates` field

### Path Normalization

The impact command normalizes paths before matching:
- Converts `\` to `/` (Windows compatibility)
- Strips leading `./` prefixes

## Impact Report Structure

The generated impact report JSON has this structure:

```json
{
  "changed_paths": [
    "specs/features/workflow/bundle.feature"
  ],
  "impacted_scenarios": [
    "SCN-XK-WORKFLOW-001",
    "SCN-XK-WORKFLOW-002",
    "SCN-XK-WORKFLOW-003",
    "SCN-XK-WORKFLOW-004",
    "SCN-XK-WORKFLOW-005"
  ]
}
```

See [Impact Format Reference](../reference/impact-format.md) for field details.

## Common Use Cases

### Pre-commit Testing

Before committing, check what your changes might break:

```bash
# Make changes to some files
cargo xtask impact --changed crates/my-crate/src/lib.rs

# Review the impact report
cat artifacts/impact/impact.report.v1.json | jq '.impacted_scenarios'

# Run tests for impacted scenarios only
for scenario in $(cat artifacts/impact/impact.report.v1.json | jq -r '.impacted_scenarios[]'); do
    cargo xtask bundle $scenario
    # Run scenario tests...
done
```

### CI Optimization

Use impact analysis to run only affected tests in CI:

```bash
# Get list of changed files from git
CHANGED_FILES=$(git diff --name-only HEAD~1)

# Build impact arguments
IMPACT_ARGS=""
for file in $CHANGED_FILES; do
    IMPACT_ARGS="$IMPACT_ARGS --changed $file"
done

# Run impact analysis
cargo xtask impact $IMPACT_ARGS

# Run only impacted scenarios
# (CI logic to parse impact.report.v1.json and run tests)
```

### Feature Development Workflow

When working on a feature that spans multiple crates:

```bash
# Edit files in multiple crates
# Then check overall impact
cargo xtask impact \
  --changed crates/taxonomy-resolver/src/ \
  --changed crates/dimensional-rules/src/

# Results show which ACs need verification
```

## Combining with Bundle Workflow

Impact analysis and bundle workflow work together:

```bash
# Check what changed
cargo xtask impact --changed crates/efm-rules/src/lib.rs

# Get impacted ACs and create bundles for them
for ac in $(cat artifacts/impact/impact.report.v1.json | jq -r '.impacted_scenarios[]' | sed 's/SCN-/AC-/'); do
    cargo xtask bundle $ac 2>/dev/null || echo "No bundle for $ac"
done
```

## Limitations

Impact analysis is conservative - it may flag more scenarios than strictly necessary. This is intentional to ensure no regressions are missed. The current logic:

- Does not analyze function-level changes
- Does not track data dependencies across crates
- Uses simple path prefix matching

## Prerequisites

Before running impact analysis, ensure the feature grid is compiled:

```bash
cargo xtask feature-grid
```

This generates `artifacts/feature.grid.v1.json` which serves as the source of truth for all impact calculations.
