# Scout Issue Finder Agent

## Purpose

Discover and file issues automatically for gaps in the xbrlkit codebase.

## Scout Responsibilities

| Discovery Target | Detection Method | Issue Type |
|------------------|------------------|------------|
| Unactivated scenarios | Feature files without @alpha-active tag | Plan (micro PR) |
| Placeholder crates | src/lib.rs < 20 lines | Research |
| Orphaned scenarios | Scenario IDs in features not in bdd-steps | Bug/Plan |
| Missing step handlers | Gherkin steps without handler impl | Plan |
| Schema drift | Receipt structs vs JSON schemas | Bug |
| Test coverage gaps | Uncovered modules in report | Research |

## Execution Procedure

1. **Scan for unactivated scenarios**
   - Find all @SCN-* tags in feature files
   - Filter out those with @alpha-active tag
   - Group by category (SEC, workflow, foundation, etc.)

2. **Scan for placeholder crates**
   - Check all crates/*/src/lib.rs file sizes
   - Flag those with < 20 lines

3. **Scan for orphaned scenarios**
   - Cross-reference scenario IDs with step handlers
   - Flag scenarios with no handler infrastructure

4. **Scan for missing handlers**
   - Parse feature files for Gherkin steps
   - Cross-reference with bdd-steps handlers
   - Flag missing implementations

5. **File GitHub issues**
   - Create issues for findings (max 3 per run)
   - Apply labels: scout-discovered, needs-triage
   - Deduplicate by checking issue titles

6. **Post summary**
   - Update issue #119 with findings summary
   - Save detailed report to .mend/notes/

## Configuration

- Schedule: Daily at 00:00 UTC (off-peak)
- Rate limit: Max 3 issues per run to avoid spam
- Report location: .mend/notes/xbrlkit-scout-report.md

## Acceptance Criteria

- [x] Scout agent configuration created
- [ ] GitHub API integration for issue creation
- [ ] Cron job configuration
- [x] Initial run discovers at least 5 legitimate issues
- [x] Documentation in .mend/agents/scout-issue-finder.md
