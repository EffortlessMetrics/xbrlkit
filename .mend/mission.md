# xbrlkit Mission

**Goal:** Build a modern, well-architected Rust XBRL processor from a legacy foundation.

**Secondary Goal:** Use this process to refine autonomous maintainer capabilities — research-driven development, microcrate architecture, documentation-first thinking.

## Principles

1. **Research Before Code**
   - Strong definitions from XBRL specs
   - Documentation drives implementation
   - Types are contracts

2. **Microcrate Architecture**
   - Small, focused, single-responsibility crates
   - Composable pipelines
   - Clear boundaries (domain vs infrastructure vs transport)

3. **Documentation as Exhaust**
   - Every decision has an ADR
   - Every pattern is recorded
   - Friction is logged and addressed

4. **Quality Gates**
   - `cargo fmt` — Format
   - `cargo clippy` — Lint (zero warnings)
   - `cargo test` — Unit tests
   - `cargo xtask alpha-check` — Integration scenarios

## Current State

- Legacy foundation in place
- Dimensional validation infrastructure complete
- BDD scenario framework active
- 13 @alpha-active scenarios passing

## Target Architecture

```
xbrlkit/
├── crates/
│   ├── xbrl-contexts/        # Context parsing
│   ├── taxonomy-dimensions/  # Dimension taxonomy
│   ├── dimensional-rules/    # Validation rules
│   ├── validation-run/       # Validation pipeline
│   └── ...                   # More to come
```

## Success Metrics

- All SEC validation rules implemented
- Full XBRL Dimensions support
- CLI tools for inspection
- Clean, documented, tested codebase

## Maintainer Growth

Every PR should:
1. Solve a concrete problem
2. Improve the codebase
3. Teach something (documented)
4. Refine the process
