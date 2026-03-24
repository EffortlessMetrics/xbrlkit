# Use the Bundle Workflow

The bundle workflow creates a bounded context packet (bundle manifest) containing all scenarios that match a selector. This is useful for focused testing, CI/CD pipelines, and sharing test contexts.

## What is a Bundle?

A bundle is a JSON manifest (`artifacts/bundles/<selector>.json`) that contains:
- The selector used to create it
- All matching scenarios with their complete metadata
- Crate dependencies, fixtures, and allowed edit paths

## Create a Bundle

### Bundle by AC ID

Create a bundle for a specific Acceptance Criteria:

```bash
cargo xtask bundle AC-XK-SEC-INLINE-001
```

Output: `artifacts/bundles/AC-XK-SEC-INLINE-001.json`

### Bundle by Scenario ID

Create a bundle for a specific scenario:

```bash
cargo xtask bundle SCN-XK-WORKFLOW-002
```

Output: `artifacts/bundles/SCN-XK-WORKFLOW-002.json`

### Bundle by Requirement ID

Create a bundle for all scenarios under a requirement:

```bash
cargo xtask bundle REQ-XK-WORKFLOW
```

Output: `artifacts/bundles/REQ-XK-WORKFLOW.json`

### Bundle with Tag Syntax

Use `@` prefix for tag-style selectors:

```bash
cargo xtask bundle @AC-XK-SEC-INLINE-001
```

## Selector Matching Rules

The bundle command matches scenarios using these rules (in order):

| Selector Format | Matches |
|----------------|---------|
| `SCN-XXX-NNN` | Exact scenario_id |
| `AC-XXX-NNN` | Exact ac_id |
| `REQ-XXX-NAME` | Exact req_id |
| `@SCN-XXX-NNN` | Exact scenario_id (tag style) |
| `@AC-XXX-NNN` | Exact ac_id (tag style) |

## Bundle Output Structure

The generated bundle JSON has this structure:

```json
{
  "selector": "AC-XK-SEC-INLINE-001",
  "scenarios": [
    {
      "scenario_id": "SCN-XK-SEC-INLINE-001",
      "ac_id": "AC-XK-SEC-INLINE-001",
      "req_id": "REQ-XK-SEC-INLINE",
      "feature_file": "specs/features/sec/inline_restrictions.feature",
      "sidecar_file": "specs/features/sec/inline_restrictions.meta.yaml",
      "layer": "sec",
      "module": "FEAT-XK-SEC-INLINE:inline-restrictions",
      "crates": ["ixhtml-scan", "efm-rules", ...],
      "fixtures": ["synthetic/sec/inline/ix-fraction-01"],
      "profile_pack": "sec/efm-77/opco",
      "receipts": ["validation.report.v1", "scenario.run.v1"],
      "allowed_edit_roots": ["crates/ixhtml-scan", ...],
      "suite": "synthetic",
      "speed": "fast"
    }
  ]
}
```

See [Bundle Format Reference](../reference/bundle-format.md) for field details.

## Common Use Cases

### Pre-commit Bundle for Focused Testing

Before committing changes, create a bundle for the AC you're working on:

```bash
# Edit some code in crates/efm-rules/src/lib.rs
cargo xtask bundle AC-XK-SEC-INLINE-001
cargo xtask test-ac AC-XK-SEC-INLINE-001
```

### CI/CD Integration

Bundles help CI systems understand what to test:

```bash
# In CI: create bundle and run scenarios
cargo xtask bundle $AC_ID
# Pass bundle to downstream jobs for parallel execution
```

### Sharing Test Context

Bundles can be shared between team members to reproduce exact test contexts:

```bash
# Developer A creates bundle
cargo xtask bundle AC-XK-IXDS-002
# Share artifacts/bundles/AC-XK-IXDS-002.json

# Developer B uses it to understand the test context
cat artifacts/bundles/AC-XK-IXDS-002.json | jq '.scenarios[0].fixtures'
```

## Error Handling

If the selector matches no scenarios:

```bash
$ cargo xtask bundle AC-XK-DOES-NOT-EXIST
Error: bundle: selector matched no scenarios: AC-XK-DOES-NOT-EXIST
```

## Prerequisites

Before creating bundles, ensure the feature grid is compiled:

```bash
cargo xtask feature-grid
```

This generates `artifacts/feature.grid.v1.json` which serves as the source of truth for all bundle operations.
