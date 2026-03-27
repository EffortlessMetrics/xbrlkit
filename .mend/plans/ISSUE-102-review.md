## 📝 Plan Review: CHANGES NEEDED

**Reviewer:** reviewer-plan agent  
**Date:** 2026-03-28  
**Result:** CHANGES NEEDED

---

### Critical Issues

#### 1. Implementation/Documentation Mismatch ⚠️
The plan claims the decision is to use **blocking reqwest**, but the actual implementation in `crates/taxonomy-loader/src/lib.rs` uses `tokio::task::block_in_place()` with an async client wrapped in blocking calls:

```rust
// Current implementation (NOT true blocking reqwest)
let content = tokio::task::block_in_place(|| {
    tokio::runtime::Handle::current().block_on(async {
        let response = client.get(url).send().await?;
        ...
    })
});
```

This contradicts the claimed decision of using blocking reqwest.

#### 2. Dependency Mismatch
`crates/taxonomy-loader/Cargo.toml` still lists `tokio` as a dependency:
```toml
tokio = { workspace = true }
```

This contradicts the rationale in the plan about "reduced dependency tree (no tokio runtime needed)".

#### 3. ADR Format Inconsistency
The proposed ADR structure includes sections like "Context", "Consequences table" that don't match the existing minimal ADR format in the repo (see ADR-001, ADR-004, ADR-006 for reference).

---

### Recommendations

**Option A - Update Implementation to Match ADR:**
1. Switch to true blocking reqwest: `reqwest::blocking::Client`
2. Remove `tokio` dependency from `taxonomy-loader/Cargo.toml`
3. Then document the ADR as planned

**Option B - Update ADR to Match Implementation:**
1. Revise the ADR to document the actual approach: "async reqwest with blocking wrapper"
2. Update rationale to explain why this hybrid approach was chosen
3. Remove claims about "no tokio runtime needed"

---

### Required Plan Updates

| Item | Action |
|------|--------|
| Decision clarity | Explicitly state which approach (A or B) will be taken |
| Dependencies | Update acceptance criteria to include Cargo.toml changes |
| ADR format | Align with existing ADR minimal format (see ADR-007 as reference) |
| Code references | Verify actual implementation matches documented decision |

---

### Positive Findings

✅ Plan correctly identifies related issues (#97)  
✅ Risk assessment is appropriate (Low risk)  
✅ Effort estimate is reasonable (~1.25 hours)  
✅ Plan correctly scopes this as documentation-only task

---

**Next Steps:**
Please update the plan to address the implementation/ADR mismatch, then re-request review.
