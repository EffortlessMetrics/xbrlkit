# Deep Review: ISSUE-252

## Status: PASSED

## Edge Cases
- **Empty input** (`""`): `parse_quoted_strings` returns `[]`; callers guard with `if quoted.len() >= 2` — safe.
- **No quotes in step text**: returns `[]` — same as before; callers skip the block safely.
- **Odd number of quotes** (unmatched): behaves identically to the old inline code — the trailing segment after the last `"` is treated as quoted content. Not a new regression.
- **Empty quotes** (`""`): returns `[""]` — callers treat this as a valid empty string. Same as before.
- **Consecutive quotes** (`""""`): returns `["", ""]`. Same as before.
- **Unicode content inside quotes**: `&str::split('"')` handles UTF-8 correctly. No issue.
- **Escaped quotes** (`\"`): NOT handled — the parser splits on every `"` regardless. But BDD step text is controlled input with no escaped quotes in practice. Same limitation as before.

## Integration Impact
- **xbrlkit-bdd-steps**: Direct change — 3 call sites replaced, one new private helper added. No public API change.
- **xbrlkit-bdd**: Depends on `xbrlkit-bdd-steps` via `run_scenario` only. No direct impact.
- **xbrlkit-feature-grid / scenario-runner / context-completeness / numeric-rules**: No impact — these consume the BDD runner's outputs, not its step-parsing internals.
- **No breaking changes** for downstream consumers.

## Performance Notes
- **No change**: The helper performs the exact same `split('"')`, `enumerate`, `filter`, `map(to_string)`, `collect` sequence as the 3 removed inline copies.
- **No extra allocations**: `.to_string()` calls were present in the original inline code.
- **Not a hot path**: Called once per matching BDD Given step during scenario setup. Negligible cost.

## Safety Concerns
- **None**: No unsafe blocks. No panics — the function is pure and total over `&str`.
- **Error handling preserved**: All 3 call sites keep their existing guards (`if quoted.len() >= 2`, `for ctx_id in contexts`). The last site still `bail!`s on invalid spec.

## Minor Observations (non-blocking)
1. **Inconsistency**: The singular step `a fact referencing concept "..." with context "..."` (line ~430) still uses manual `find('"')` parsing. It could also use `parse_quoted_strings` for uniformity, but it's a 2-quote fixed schema so manual parsing is arguably clearer.
2. **Test gap**: `xbrlkit-bdd-steps` has 0 unit tests. The refactor is low-risk (pure code movement), but there is no automated coverage for edge cases like empty input or unmatched quotes.

## Label Updates
- Remove: `planning-in-progress`
- Add: `deep-plan-reviewed`
