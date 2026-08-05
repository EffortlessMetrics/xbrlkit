# Review: ISSUE-248

## Status: NEEDS_WORK

## Issues Found

- [ ] **Missing plan file**: `.mend/plans/ISSUE-248.md` does not exist. No planning document has been drafted for this issue. A planner agent must create the plan before implementation can begin.

- [ ] **Stale issue description — line numbers incorrect**: Issue references `crates/xbrlkit-bdd-steps/src/lib.rs:1690` and `:1704`, but the file is only 1626 lines total. The referenced TODO comments for `#233` do not exist in the current codebase.

- [ ] **Missing step definitions**: The acceptance criteria mention updating `i_see_cache_hit_for_url` and `i_see_schema_imported` BDD steps, but neither step handler exists in `crates/xbrlkit-bdd-steps/src/lib.rs` (verified by `grep`).

- [ ] **Missing struct fields**: `TaxonomyLoaderContext` (at `crates/xbrlkit-bdd-steps/src/lib.rs:75`) currently contains only `loader`, `taxonomy`, `cache_dir`, `schema_path`, and `loaded`. The issue's acceptance criteria assume adding `cache_hit` and `loaded_schemas` fields, but no planning has been done for:
  - Where the `HashSet<Url>` type comes from (no `url::Url` import exists in the file)
  - How cache hits are detected during taxonomy loading
  - How schema imports are tracked during resolution

## Suggestions

1. **Create the plan first**: A planner should write `ISSUE-248.md` covering:
   - Exact file paths for `TaxonomyLoaderContext` modifications
   - Design for cache hit detection (instrumentation point in the loader)
   - Design for schema import tracking (hook in the resolver)
   - Where the new BDD step definitions should be added (since they don't exist yet)
   - Test strategy — how to verify both fields are populated correctly during a BDD run

2. **Update issue description**: Once the plan is drafted, consider updating the issue body to reference correct line numbers or remove line-specific references (they rot quickly).

3. **Verify scope**: Adding two fields plus instrumentation plus BDD steps may be two separate PRs. The plan should clarify whether this is a single unit of work.
