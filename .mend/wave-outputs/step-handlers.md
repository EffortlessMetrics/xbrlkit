# Step Handler Analysis

## Source File
`/tmp/xbrlkit-wt/test-002/crates/xbrlkit-bdd-steps/src/lib.rs`

## Given Handlers (lines 82-108)
- `the profile pack "{id}"` - validates profile pack matches scenario metadata
- `the fixture directory "{path}"` / `the fixture "{path}"` - validates fixture path matches metadata

## When Handlers (lines 110-128)
- `I validate the filing` - executes validation scenario
- `I validate duplicate facts` - executes duplicate facts scenario  
- `I resolve the DTS` - executes DTS resolution scenario
- `I export the canonical report to JSON` - executes export scenario

## Then Handlers (lines 130-158)
- `the validation report has no error findings`
- `the taxonomy resolution succeeds`
- `the concept set is:` (table-based)
- `the export report receipt is emitted`

## Parameterized Assertions (lines 160-196)
- `the validation report contains rule "{rule_id}"`
- `the validation report does not contain rule "{rule_id}"`
- `the IXDS assembly receipt contains {n} member(s)`
- `the taxonomy resolution resolves at least {n} namespace(s)`
- `the report contains {n} fact(s)`

## Missing for SCN-XK-WORKFLOW-002
The following steps are NOT implemented:
- `Given the feature grid is compiled`
- `When I bundle the selector "{selector}"`
- `Then the bundle manifest lists scenario "{scenario_id}"`

---
task: step_handler_finder
status: completed
found_at: 2026-03-19T09:52:27+08:00
