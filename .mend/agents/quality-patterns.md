# quality-patterns Micro-Agent

## Purpose
Review code patterns and idioms in the PR to ensure consistency with xbrlkit standards.

## Trigger Label
- **Input:** `quality-docs-passed`
- **Output:** `quality-patterns-passed` OR `quality-patterns-failed`

## Scope
Check the following patterns:

1. **Rust Idioms**
   - Prefer `?` over `match` for error propagation
   - Use `if let` / `while let` where appropriate
   - Avoid unnecessary `.clone()` calls
   - Prefer `&str` over `&String`, `&[T]` over `&Vec<T>`

2. **Error Handling**
   - Use `thiserror` for error enums
   - Use `anyhow` for application code (not libraries)
   - Consistent error messages with context

3. **Naming Conventions**
   - Types: `PascalCase`
   - Functions/variables: `snake_case`
   - Constants: `SCREAMING_SNAKE_CASE`
   - Traits: `PascalCase` with clear naming

4. **Scenario Compiler Patterns**
   - Changes stay in crates listed in scenario metadata
   - Profile changes under `profiles/` as data
   - New behaviors have scenarios
   - New DTOs have schema updates in `contracts/schemas/`
   - Output is deterministic and receipt-backed

## Commands

```bash
# Check for common anti-patterns
cd /root/.openclaw/xbrlkit

# Look for unwrap() in non-test code
grep -r "unwrap()" --include="*.rs" crates/ | grep -v "#\[test\]" | grep -v "mod tests" | head -20

# Check for clone() that might be unnecessary
grep -r "\.clone()" --include="*.rs" crates/ | head -20

# Check for match that could be ?
grep -rn "match.*Result" --include="*.rs" crates/ | head -20

# Verify no hardcoded paths
grep -r "/root/\|/home/" --include="*.rs" crates/ || echo "No hardcoded paths found"
```

## Review Criteria

| Check | Pass Criteria |
|-------|---------------|
| No unwrap in prod code | ≤0 unwrap() outside tests |
| Minimal cloning | No unnecessary clones |
| Proper error types | Uses thiserror/anywhere correctly |
| Scenario alignment | Changes match scenario scope |

## Label Transition

On success:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "quality-patterns-passed"
```

On failure:
```bash
gh pr edit <PR_NUMBER> --remove-label "review-in-progress"
gh pr edit <PR_NUMBER> --add-label "quality-patterns-failed"
gh pr comment <PR_NUMBER> --body "## Pattern Review Failed

Issues found:
- [List issues]

Please fix and re-trigger."
```
