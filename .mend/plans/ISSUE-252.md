# Plan: ISSUE-252 — Duplicate quoted-string parsing in xbrlkit-bdd-steps

## Problem
The same quoted-string parsing logic is duplicated 3 times in `crates/xbrlkit-bdd-steps/src/lib.rs`:
- Line ~413: Context completeness Given steps
- Line ~473: Fact specification parsing (facts referencing concepts)
- Line ~513: Decimal precision Given steps

All three use:
```rust
let quoted: Vec<String> = text
    .split('"')
    .enumerate()
    .filter(|(i, _)| i % 2 == 1)
    .map(|(_, s)| s.to_string())
    .collect();
```

## Solution
Extract a helper function `parse_quoted_strings(text: &str) -> Vec<String>` and replace all 3 sites.

## Steps
1. Add helper near existing `parse_count_suffix` at end of file
2. Replace site 1 (context completeness) with `parse_quoted_strings(&step.text)`
3. Replace site 2 (facts referencing concepts) with `parse_quoted_strings(&step.text)`
4. Replace site 3 (decimal precision) with `parse_quoted_strings(&step.text)`
5. Run `cargo build --workspace`, `cargo test --workspace`, `cargo clippy --workspace`
6. Commit, push, create PR

## Verification
- All tests pass
- Clippy clean
- No functional change — pure refactor
