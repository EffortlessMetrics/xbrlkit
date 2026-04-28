## Summary
17 SEC validation scenarios are defined but not activated with `@alpha-active` tag.

## Affected Scenarios

### Decimal Precision (10 scenarios)
- SCN-XK-SEC-DECIMAL-001 through SCN-XK-SEC-DECIMAL-010
- File: `specs/features/sec/decimal_precision.feature`

### Negative Values (5 scenarios)
- SCN-XK-SEC-NEGATIVE-001 through SCN-XK-SEC-NEGATIVE-005
- File: `specs/features/sec/negative_values.feature`

### Required Facts (2 scenarios)
- SCN-XK-SEC-REQUIRED-001, SCN-XK-SEC-REQUIRED-002
- File: `specs/features/sec/required_facts.feature`

### Inline Restrictions (2 scenarios)
- SCN-XK-SEC-INLINE-001, SCN-XK-SEC-INLINE-002
- File: `specs/features/sec/inline_restrictions.feature`

## Blockers
Missing step handlers for:
- `When the document is validated`
- SEC-specific validation findings assertions

## Acceptance Criteria
- [ ] Add `@alpha-active` tag to each scenario
- [ ] Implement missing step handlers
- [ ] Verify all scenarios pass

---
*Discovered by Scout Agent on 2026-04-01*
