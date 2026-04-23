# Implementation Plan: ISSUE-177 — Add ValidationFinding Builder

## Overview

Multiple crates manually construct `ValidationFinding` structs with nearly identical boilerplate:
- `.to_string()` everywhere  
- `severity: "error".to_string()` repeated ~30 times  
- `member: Some(fact.member.clone()), subject: Some(fact.concept.clone())` as a common pairing  
- `sanitize_for_rule_id` duplicated across `numeric-rules` and `efm-rules`

This plan adds a builder + convenience constructors to `xbrl_report_types`, then migrates all call sites crate-by-crate.

---

## Acceptance Criteria

| # | Criterion |
|---|---|
| 1 | `ValidationFinding` gains builder-style constructors (`error()`, `info()`, `warning()`) and chained setters (`.with_member()`, `.with_subject()`, `.for_fact()`) |
| 2 | `sanitize_for_rule_id` utility moved from individual crates to `xbrl_report_types` |
| 3 | One crate migrated as proof-of-concept with CI green |
| 4 | All remaining crates migrated |
| 5 | Zero functional regressions — all existing tests pass |
| 6 | Documentation updated (crate-level docs, at least one doc-test example) |

---

## Proposed Approach

### Phase 1 — Builder API in `xbrl-report-types` (estimated: small)

Add to `crates/xbrl-report-types/src/lib.rs`:

```rust
impl ValidationFinding {
    /// Create an error-level finding.
    pub fn error(rule_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: "error".to_string(),
            message: message.into(),
            member: None,
            subject: None,
        }
    }

    /// Create an info-level finding.
    pub fn info(rule_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: "info".to_string(),
            message: message.into(),
            member: None,
            subject: None,
        }
    }

    /// Create a warning-level finding.
    pub fn warning(rule_id: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            rule_id: rule_id.into(),
            severity: "warning".to_string(),
            message: message.into(),
            member: None,
            subject: None,
        }
    }

    /// Set the member field.
    pub fn with_member(mut self, member: impl Into<String>) -> Self {
        self.member = Some(member.into());
        self
    }

    /// Set the subject field.
    pub fn with_subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    /// Derive member and subject from a Fact.
    pub fn for_fact(mut self, fact: &Fact) -> Self {
        self.member = Some(fact.member.clone());
        self.subject = Some(fact.concept.clone());
        self
    }
}
```

Also add:

```rust
/// Sanitize a concept/identifier for use in a rule ID.
pub fn sanitize_for_rule_id(value: &str) -> String {
    value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_uppercase()
            } else {
                '_'
            }
        })
        .collect()
}
```

**Rationale for `impl Into<String>`**: Most call sites pass `&str`, `String`, or `format!()` results. `Into<String>` eliminates the `.to_string()` call at every site without forcing callers to change.

### Phase 2 — Proof of Concept: `numeric-rules` (estimated: small)

Migrate `crates/numeric-rules/src/lib.rs` and `crates/numeric-rules/src/decimal_precision.rs`.

Before:
```rust
findings.push(ValidationFinding {
    rule_id: format!("SEC.NEGATIVE_VALUE.{}", sanitize_for_rule_id(&fact.concept)),
    severity: "error".to_string(),
    message: format!("Concept '{}' has negative value '{}' ...", fact.concept, fact.value),
    member: Some(fact.member.clone()),
    subject: Some(fact.concept.clone()),
});
```

After:
```rust
findings.push(
    ValidationFinding::error(
        format!("SEC.NEGATIVE_VALUE.{}", sanitize_for_rule_id(&fact.concept)),
        format!("Concept '{}' has negative value '{}' ...", fact.concept, fact.value),
    )
    .for_fact(fact),
);
```

Remove local `sanitize_for_rule_id`, import from `xbrl_report_types`.

### Phase 3 — Migrate Remaining Crates (estimated: small–medium)

| Crate | Files | Patterns | Complexity |
|---|---|---|---|
| `efm-rules` | `src/lib.rs` | 4 call sites, `sanitize_for_rule_id` duplicate | Low |
| `unit-rules` | `src/validator.rs` | 1 call site, uses `Fact` | Low |
| `context-completeness` | `src/lib.rs` | 1 call site, uses `Fact` | Low |
| `dimensional-rules` | `src/lib.rs` | ~12 call sites, heavy `Err(ValidationFinding {...})` usage | Medium |
| `validation-run` | `src/lib.rs` | 5 call sites, mixed severity levels | Low |

**Order**: `efm-rules` → `unit-rules` → `context-completeness` → `validation-run` → `dimensional-rules` (most complex last).

For `dimensional-rules` specifically, the `Result<(), ValidationFinding>` pattern means the change is:

```rust
// Before
return Err(ValidationFinding {
    rule_id: "XBRL.DIMENSION.UNKNOWN".to_string(),
    severity: "error".to_string(),
    message: format!("Unknown dimension: {}", dim_member.dimension),
    member: Some(dim_member.dimension.clone()),
    subject: Some(dim_member.member.clone()),
});

// After
return Err(
    ValidationFinding::error("XBRL.DIMENSION.UNKNOWN", format!("Unknown dimension: {}", dim_member.dimension))
        .with_member(&dim_member.dimension)
        .with_subject(&dim_member.member),
);
```

### Phase 4 — Tests & Documentation (estimated: small)

- Add unit tests for new constructors in `xbrl-report-types`
- Add at least one doc-test example showing builder chaining
- Verify all existing tests pass across workspace (`cargo test --workspace`)
- Update crate-level doc comments in `numeric-rules` and `efm-rules` to remove mentions of manual `ValidationFinding` construction

---

## Files to Modify / Create

### New
- None (all changes are modifications to existing files)

### Modified
| File | Change |
|---|---|
| `crates/xbrl-report-types/src/lib.rs` | Add builder impl + `sanitize_for_rule_id` + tests |
| `crates/numeric-rules/src/lib.rs` | Migrate to builder, remove local `sanitize_for_rule_id` |
| `crates/numeric-rules/src/decimal_precision.rs` | Migrate to builder |
| `crates/efm-rules/src/lib.rs` | Migrate to builder, remove local `sanitize_for_rule_id` |
| `crates/unit-rules/src/validator.rs` | Migrate to builder |
| `crates/context-completeness/src/lib.rs` | Migrate to builder |
| `crates/validation-run/src/lib.rs` | Migrate to builder |
| `crates/dimensional-rules/src/lib.rs` | Migrate to builder (largest delta) |

---

## Test Strategy

1. **Unit tests** for new constructors in `xbrl-report-types`
2. **Existing tests** must all pass unchanged — this is a pure refactor
3. **Doc tests** — at least one example of builder chaining
4. **CI green** — full `cargo test --workspace` before each crate migration

---

## Risk Assessment

| Risk | Likelihood | Mitigation |
|---|---|---|
| Missing a `to_string()` edge case (e.g., caller passes `&String`) | Low | `impl Into<String>` handles `&str`, `String`, `&String`, `Cow<str>` |
| `for_fact` doesn't match all manual `member`/`subject` patterns | Low | Audit every call site before migration; `dimensional-rules` uses `dim_member` not `fact` — those use `.with_member()` / `.with_subject()` explicitly |
| Build break from removing `sanitize_for_rule_id` in numeric-rules before efm-rules is updated | Low | Update both in same PR or use `pub(crate)` temporarily |

**Overall risk level**: **Low** — this is a pure internal refactor with no API surface changes outside the crate family.

---

## Estimated Effort

- Phase 1 (builder API): ~15 min
- Phase 2 (numeric-rules PoC): ~15 min
- Phase 3 (remaining crates): ~30–45 min
- Phase 4 (tests + docs + CI): ~15 min

**Total**: ~1.5 hours of focused work. Fits in a single PR.

---

## Rollout Order

1. Single PR: builder API + all migrations + tests
2. CI green → merge
3. Follow-up issue (optional): consider `&'static str` for `severity` to avoid allocations, or an enum `Severity { Error, Warning, Info }`
