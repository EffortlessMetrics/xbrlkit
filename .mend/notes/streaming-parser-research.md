# Streaming Parser Research — Phase 4 Wave 4

**Date:** 2026-03-26  
**Issue:** Large SEC filings (>100MB) cause memory pressure with current DOM parsing  
**Solution:** SAX-style streaming XML parser

---

## Current State

| Component | Parser | Approach | Issue |
|-----------|--------|----------|-------|
| `taxonomy-loader` | `roxmltree` | DOM | Loads entire taxonomy into memory |
| `xbrl-contexts` | `roxmltree` | DOM | Context parsing memory-heavy |
| `ixhtml-scan` | Custom string scan | Streaming-ish | OK for inline, not for full XBRL |

**Problem:** SEC filings can be 100MB-500MB. DOM parsing requires 3-10x memory multiplier.

---

## SAX vs DOM

| Aspect | DOM (current) | SAX (target) |
|--------|---------------|--------------|
| Memory | O(document size) | O(depth) |
| Speed | Parse once, query many | Single pass |
| Random access | Yes | No |
| Large files | ❌ Fails | ✅ Streams |

---

## Rust Options

| Library | Style | Pros | Cons |
|---------|-------|------|------|
| `quick-xml` | SAX + async | Fast, mature, async support | Manual event handling |
| `xml-rs` | SAX | Pure Rust | Slower than quick-xml |
| `xmlparser` | Low-level token | Minimal overhead | Too low-level |

**Choice:** `quick-xml` — industry standard, used by deno, serde integration

---

## Design: `xbrl-stream` Crate

### Core Types

```rust
/// Streaming XBRL fact parser
pub struct XbrlStreamReader<R: BufRead> {
    reader: Reader<R>,
    buffer: Vec<u8>,
    fact_handler: Box<dyn FactHandler>,
}

/// Callback for processed facts
pub trait FactHandler: Send + Sync {
    fn on_fact(&mut self, fact: StreamingFact) -> Result<(), Box<dyn Error>>;
    fn on_context(&mut self, context: StreamingContext) -> Result<(), Box<dyn Error>>;
    fn on_unit(&mut self, unit: StreamingUnit) -> Result<(), Box<dyn Error>>;
}

/// Minimal fact representation during streaming
pub struct StreamingFact {
    pub concept: String,
    pub context_ref: String,
    pub unit_ref: Option<String>,
    pub decimals: Option<String>,
    pub value: String,
    pub xml_line: u64,
}
```

### Architecture

```
Raw XML → quick-xml events → XBRL semantic layer → FactHandler
             ↓                      ↓
       (low-level)            (high-level)
       - Start/End tags       - Fact detected
       - Attributes           - Context built
       - Text content         - Unit resolved
```

### Memory Budget

| File Size | DOM Peak | SAX Peak | Savings |
|-----------|----------|----------|---------|
| 10MB | 50MB | 5MB | 10x |
| 100MB | 500MB | 5MB | 100x |
| 500MB | 2.5GB | 5MB | 500x |

---

## Implementation Plan

1. **Create crate:** `xbrl-stream` with `quick-xml` dependency
2. **Event parser:** Convert XML events to XBRL facts
3. **Handler traits:** Define callback interfaces
4. **Integration:** Use in `validation-run` for large files
5. **BDD scenarios:** Add large file handling tests

---

## Open Questions

1. Should we keep contexts/units in memory or spill to disk?
2. How to handle validation errors mid-stream?
3. Progress callbacks for UX?

---

**Status:** Ready for implementation
