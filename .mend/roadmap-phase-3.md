# Phase 3 Roadmap: Feature Completeness — SEC Validation Rules

**Theme:** Implement real SEC EDGAR validation rules to move from synthetic tests to production-ready validation.

**Goal:** Achieve feature completeness for core SEC XBRL validation, enabling xbrlkit to validate actual SEC filings against EDGAR rules.

---

## Wave 1: Required Facts Validation (P0)

**Focus:** Implement SEC-required facts validation — the most critical compliance rule.

**Background:** SEC filings must include certain mandatory facts (entity identifier, document type, fiscal year, etc.). Missing required facts result in EDGAR rejection.

**Deliverables:**
1. Research SEC required facts specification (EFM 6.5, 6.6)
2. Implement `required_facts` rule crate
3. Add BDD scenarios for required facts validation
4. Wire into validation-run

**Acceptance Criteria:**
- AC-XK-SEC-REQUIRED-001: Detect missing required facts
- AC-XK-SEC-REQUIRED-002: Validate required fact values

**Est. Effort:** 3-4 days

---

## Wave 2: Numeric Validation (P1)

**Focus:** Implement numeric fact validation rules.

### 2.1 Negative Value Validation
**Rule:** Detect negative values where prohibited by taxonomy (e.g., shares outstanding, entity count).

**Deliverables:**
- Research taxonomy-based negative value constraints
- Implement `negative_values` rule
- BDD scenarios

**AC:** AC-XK-SEC-NEGATIVE-001

### 2.2 Decimal Precision Validation
**Rule:** Validate decimal attribute correctness for numeric facts.

**Deliverables:**
- Research XBRL decimal attribute rules
- Implement `decimal_precision` rule
- BDD scenarios

**AC:** AC-XK-SEC-DECIMAL-001

**Est. Effort:** 4-5 days

---

## Wave 3: Context and Unit Validation (P1)

**Focus:** Implement context and unit consistency rules.

### 3.1 Unit Consistency
**Rule:** Validate unit references match fact types (easures vs. ratios).

### 3.2 Period Validity
**Rule:** Validate period definitions (no instant vs duration mismatches).

### 3.3 Context Completeness
**Rule:** Validate all facts reference valid contexts.

**Est. Effort:** 4-5 days

---

## Wave 4: Performance and Scalability (P2)

**Focus:** Optimize for large SEC filings.

### 4.1 Streaming Parser
**Current:** DOM-based parsing loads entire filing into memory.
**Target:** SAX-style streaming for filings >100MB.

### 4.2 Parallel Validation
**Current:** Sequential rule execution.
**Target:** Parallel rule validation by fact group.

### 4.3 Taxonomy Caching
**Current:** Reload taxonomy for each validation.
**Target:** Persistent taxonomy cache across validations.

**Est. Effort:** 5-7 days

---

## Wave 5: Extended Taxonomy Support (P2)

**Focus:** Support non-US taxonomies.

### 5.1 IFRS Taxonomy Support
**Scope:** Full IFRS taxonomy loading and validation.

### 5.2 ESEF Taxonomy Support
**Scope:** European Single Electronic Format support.

### 5.3 Multi-Taxonomy Validation
**Scope:** Validate filings using multiple taxonomies.

**Est. Effort:** 5-7 days

---

## Current Status

| Wave | Status | Priority | Blockers |
|------|--------|----------|----------|
| Wave 1 | 📋 Ready | P0 | None |
| Wave 2 | 📋 Planned | P1 | Wave 1 |
| Wave 3 | 📋 Planned | P1 | Wave 1 |
| Wave 4 | 📋 Planned | P2 | Waves 1-3 |
| Wave 5 | 📋 Planned | P2 | Waves 1-3 |

---

## Next Action

**Start Wave 1:** Required Facts Validation

1. Create research spike issue
2. Review EFM 6.5, 6.6 for required facts specification
3. Identify required facts for common form types (10-K, 10-Q, 8-K)
4. Design rule architecture

**Issue:** Reopen #9 or create new issue for required facts validation.
