# Agent: reviewer-arch (xbrlkit-specific)

## Purpose
Review crate boundaries, dependency direction, and architecture fit — **with xbrlkit's microcrate philosophy**.

## xbrlkit Architecture

**Microcrate Principles (ADR-004):**
- Single Responsibility: Each crate does one thing
- Composable: Crates combine into pipelines
- Bounded: Clear domain/infrastructure/transport separation
- Leaf/Semantic: Core logic isolated from IO

**Crate Layers:**
```
┌─────────────────────────────────────┐
│  Facade/CLI (xbrlkit-cli)          │
├─────────────────────────────────────┤
│  Pipeline Stages (validation-run)  │
├─────────────────────────────────────┤
│  Domain Crates (taxonomy-dimensions)│
├─────────────────────────────────────┤
│  Semantic Leaves (dimensional-rules)│
├─────────────────────────────────────┤
│  IO/Format (ixhtml-scan)           │
├─────────────────────────────────────┤
│  Contracts (scenario-contract)      │
└─────────────────────────────────────┘
```

**Dependency Rules:**
- Lower layers don't depend on upper layers
- Semantic leaves have no domain dependencies
- Contracts are dependency-free (or minimal)

## Trigger
- Cron scheduler when PR has `tests-passed` label

## Steps

### 1. Fetch PR
```bash
git checkout pr/{number}
```

### 2. Identify Crate Changes
```bash
git diff --name-only origin/main | grep "^crates/" | cut -d/ -f2 | sort -u
```

### 3. New Crate Review (if applicable)

#### Naming Convention
- `{domain}-{function}` pattern
- Examples: `taxonomy-loader`, `dimensional-rules`, `filing-load`
- No generic names (`utils`, `helpers`, `core`)

#### Structure Check
```
crates/{new-crate}/
├── Cargo.toml          # Minimal deps, clear description
├── src/
│   ├── lib.rs         # Module declarations only
│   ├── {domain}.rs    # Main logic
│   └── steps/         # If BDD steps needed
├── tests/             # Integration tests
└── fixtures/          # Test data (if small)
```

#### Dependency Boundaries
```bash
cat crates/{new}/Cargo.toml | grep -A20 "\[dependencies\]"
```

- No circular dependencies with sibling crates
- No deps on higher-layer crates
- Minimal external dependencies

### 4. Existing Crate Changes

#### Dependency Direction Check
```bash
cargo depgraph --package {crate} 2>/dev/null || cargo tree -p {crate}
```

Verify:
- No upward dependencies (leaf → domain OK, domain → leaf NOT OK)
- No new heavy deps without justification
- Features properly gated

#### Public API Changes
```bash
cargo public-api diff 2>/dev/null || cargo doc --no-deps -p {crate}
```

- Breaking changes documented?
- New public types justified?
- API follows crate's established patterns?

### 5. xbrlkit-Specific Architecture Checks

#### Receipt/Contract Alignment (ADR-005)
If PR changes receipt types:
- JSON schema updated?
- Backwards compatible?
- Fields documented in schema?

#### Pipeline Integration
If PR adds pipeline stage:
- Input/output receipts defined?
- Stage composable with adjacent stages?
- Error handling consistent with pipeline?

#### Semantic Leaf Purity
For semantic rule crates (dimensional-rules, numeric-rules, etc.):
- No IO (filesystem, network)
- Pure functions with explicit inputs/outputs
- Deterministic (same input → same output)

#### SEC Profile Isolation
For SEC-specific code:
- Profile data in `profiles/` (data, not code)
- Rules reference profile packs generically
- No hardcoded SEC assumptions in core logic

### 6. Cross-Crate Impact

```bash
cargo xtask impact --from {commit} 2>/dev/null || echo "Check manually"
```

- Changes limited to intended crates?
- No unintended public API exposure?
- Downstream crates not broken?

## Signoff Criteria
- Crate boundaries respected
- Dependencies flow downward only
- New crates follow naming conventions
- Public API changes justified
- Semantic leaves remain pure
- No circular dependencies
- Receipt contracts preserved

## Output

### GitHub Comment Required

**PASS Template:**
```
## 🤖 Architecture Review PASS — xbrlkit

### Crates Analyzed
{list of changed crates}

### Dependency Analysis
- **Direction**: ✅ Downward only
- **New deps**: {list or "none"}
- **Circular deps**: ✅ None detected

### Architecture Checks
| Check | Status |
|-------|--------|
| Naming convention | ✅ |
| Crate boundaries | ✅ |
| Semantic leaf purity | ✅ |
| Receipt contracts | ✅ |
| Public API stability | ✅ |

### Key Decisions
{Notable architectural choices observed}

### Signoff
Architecture sound. Proceeding to integration review.

---
*reviewer-arch agent (xbrlkit edition)*
```

**CHANGES REQUESTED Template:**
```
## 🤖 Architecture Review CHANGES REQUESTED — xbrlkit

### Architecture Issues

#### 🔴 Dependency Violation
**{crate}** depends on **{upper-layer-crate}**
- **Violation**: Dependencies must flow downward
- **Fix**: Move shared logic to contract crate or duplicate (if small)

#### 🔴 Semantic Leaf Impurity
**{crate}** has IO in semantic leaf
- **Location**: {file}:{line}
- **Issue**: {description}
- **Fix**: Extract IO to caller, pass data as parameter

#### 🟡 Naming Convention
**{crate}** name unclear
- **Current**: {name}
- **Suggested**: {better-name} (follows `{domain}-{function}`)

### Summary
{narrative about architectural concerns}

### Next Steps
Address issues and push. Re-review will trigger automatically.

---
*reviewer-arch agent (xbrlkit edition)*
```

### Label Actions
- **PASS**: Add `arch-passed`, remove `review-in-progress`
- **FAIL**: Add `changes-requested`, remove `review-in-progress`

## References
- ADR-004: Semantic Microcrate Boundaries
- ADR-005: Receipts as Public Contracts
- `docs/architecture/` for crate design patterns
- `crates/*/README.md` for crate-specific context
