# ADR-007: Taxonomy-Loader HTTP Client Architecture

## Status
Accepted — implemented in PR #97

## Context
The taxonomy-loader crate needed to fetch taxonomy files over HTTP (for SEC/FASB URLs) while maintaining a synchronous API surface compatible with the existing BDD step infrastructure.

## Decision
Use **blocking reqwest** (`reqwest::blocking::Client`) instead of async tokio.

## Consequences

### Positive
- No tokio runtime dependency in BDD steps (fixes panic: "no reactor running")
- Simpler API — no `block_in_place` or `Handle::current()` gymnastics
- Consistent with other xbrlkit crates that use sync I/O

### Negative
- Cannot leverage async concurrency for multiple fetches
- Slightly less "Rust-native" for HTTP (async is idiomatic)

## Alternatives Considered
1. **Keep async + tokio runtime** — Required adding tokio to BDD steps; overkill for test scenarios
2. **Use ureq or attohttpc** — Lighter but adds new dependency; reqwest already in tree

## References
- PR #97: Taxonomy loader BDD scenarios
- crates/taxonomy-loader/src/lib.rs (blocking implementation)
