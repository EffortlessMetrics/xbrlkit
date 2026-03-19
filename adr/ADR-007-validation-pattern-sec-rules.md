# ADR-007: Validation Pattern for SEC Rules

## Decision

SEC validation rules follow a consistent pattern: **pure functions** that take context + profile, return structured findings. This enables composable, testable, and profile-aware validation.

## Pattern

```rust
#[must_use]
pub fn validate_<rule_name>(
    context: &RuleContext,      // Facts, HTML, entry points, etc.
    profile: &ProfilePack,      // Profile-specific configuration
) -> Vec<ValidationFinding>    // Structured findings with rule IDs
```

## Example: Required Facts Validation

```rust
pub fn validate_required_facts(
    facts: &[Fact],
    profile: &ProfilePack,
) -> Vec<ValidationFinding> {
    let mut findings = Vec::new();
    let present: HashSet<_> = facts.iter().map(|f| &f.concept).collect();
    
    for required in &profile.required_facts {
        if !present.contains(required) {
            findings.push(ValidationFinding {
                rule_id: format!("SEC.REQUIRED_FACT.{}", sanitize(required)),
                severity: "error".to_string(),
                message: format!("Required fact '{}' is missing", required),
                member: None,
                subject: Some(required.clone()),
            });
        }
    }
    findings
}
```

## Rule ID Convention

Format: `SEC.{CATEGORY}.{SPECIFIC}`

| Category | Example Rule IDs |
|----------|------------------|
| INLINE | SEC.INLINE.ELEMENT.BANNED, SEC.INLINE.ATTR.BANNED |
| TAXONOMY | SEC.TAXONOMY.SAME_YEAR |
| REQUIRED_FACT | SEC.REQUIRED_FACT.ENTITY_REGISTRANT_NAME |

## BDD Integration

Each validation has Gherkin scenarios in `specs/features/sec/`:

```gherkin
Feature: SEC Required Facts Validation
  @alpha-active
  Scenario: AC-XK-SEC-REQUIRED-001 - Missing required DEI facts
    Given an IXDS document missing required DEI facts
    When validation runs with SEC profile
    Then findings include SEC.REQUIRED_FACT.* errors
```

## Adding a New SEC Rule

1. **Implement** in `crates/efm-rules/src/lib.rs`
2. **Add BDD** scenario in `specs/features/sec/<rule>.feature`
3. **Wire up** in `xbrlkit-cli` or `scenario-runner`
4. **Test** with `cargo xtask test-ac <scenario-id>`
5. **Activate** by adding `@alpha-active` tag

## Rationale

- **Pure functions** enable parallel execution and caching
- **ProfilePack injection** allows rule behavior to vary by SEC filing type
- **Structured findings** support downstream reporting and aggregation
- **BDD-first** ensures rules are documented and testable

## Related

- ADR-001: Scenario-driven workspace
- ADR-003: Profile packs as data
