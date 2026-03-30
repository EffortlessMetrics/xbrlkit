# Maintainer Commands Guide

This guide documents the maintainer command surface for xbrlkit contributors. It covers the essential commands for development, quality gates, and release preparation.

## Quick Start

```bash
# Fast feedback during development
make quick

# Comprehensive validation before PR
make full
```

## The Two Gates

### `make quick` — Fast Feedback

**Purpose:** Run essential quality checks quickly during development iteration.

**What it runs:**
```bash
cargo fmt --check          # Verify code formatting
cargo clippy --workspace -- -D warnings  # Lint all crates
cargo test --workspace     # Run all unit tests
```

**When to use:**
- During active development for rapid feedback
- Before committing changes locally
- After rebasing or merging branches

**Expected runtime:** ~30-60 seconds (depending on hardware)

**Example workflow:**
```bash
# Edit some code
vim crates/my-crate/src/lib.rs

# Quick check before committing
make quick

# Fix any issues, then commit
git add .
git commit -m "feat: add new feature"
```

---

### `make full` — Complete Validation

**Purpose:** Run the full alpha gate validation before submitting a PR.

**What it runs:**
```bash
cargo xtask alpha-check
```

**When to use:**
- Before creating a pull request
- Before merging to main
- After significant architectural changes
- When preparing a release

**Expected runtime:** 2-5 minutes (depending on hardware)

**Example workflow:**
```bash
# Feature is complete, ready for PR
make full

# If passed, push and create PR
git push origin feature-branch
gh pr create --title "feat: new feature"
```

---

## xtask Commands

The `cargo xtask` subsystem provides repo-local developer automation. These are the primary commands maintainers use.

### `cargo xtask doctor`

**Purpose:** Verify the repository has all required directories.

**Checks:**
- `contracts/schemas` exists
- `specs/features` exists
- `profiles/sec/efm-77/opco` exists
- `fixtures/synthetic` exists

**Example:**
```bash
cargo xtask doctor
# Output: doctor: repo layout looks healthy
```

---

### `cargo xtask feature-grid`

**Purpose:** Compile the feature grid from Gherkin feature files and sidecar metadata.

**Output:** `artifacts/feature.grid.v1.json`

**When to use:**
- After adding or modifying feature files
- After updating sidecar metadata (`*.meta.yaml`)
- When the scenario grid needs refreshing

**Example:**
```bash
cargo xtask feature-grid
# Output: compiled 42 scenarios
```

---

### `cargo xtask schema-check`

**Purpose:** Validate that emitted JSON outputs conform to their schemas.

**When to use:**
- After modifying contract schemas
- After changing JSON serialization code
- During the full alpha gate

**Example:**
```bash
cargo xtask schema-check
```

---

### `cargo xtask test-ac <ac_id>`

**Purpose:** Run a specific acceptance criterion (AC) by ID.

**Active AC IDs:**
- `AC-XK-SEC-INLINE-001` - Inline XBRL validation
- `AC-XK-SEC-INLINE-002` - Inline XBRL edge cases
- `AC-XK-SEC-REQUIRED-001` - Required facts validation
- `AC-XK-SEC-REQUIRED-002` - Required facts edge cases
- `AC-XK-TAXONOMY-001` - Taxonomy resolution
- `AC-XK-TAXONOMY-002` - Taxonomy edge cases
- `AC-XK-DUPLICATES-001` - Duplicate detection
- `AC-XK-IXDS-001` - IXDS assembly (single file)
- `AC-XK-IXDS-002` - IXDS assembly (multi-file)
- `AC-XK-EXPORT-001` - Export functionality

**When to use:**
- Debugging a specific failing AC
- Developing a new feature related to one AC
- Running focused tests during iteration

**Example:**
```bash
# Run a specific AC
cargo xtask test-ac AC-XK-IXDS-002

# Multiple ACs (run separately)
cargo xtask test-ac AC-XK-TAXONOMY-001
cargo xtask test-ac AC-XK-TAXONOMY-002
```

---

### `cargo xtask bdd --tags <tags>`

**Purpose:** Run BDD scenarios filtered by tags.

**Common tags:**
- `@alpha-active` - Active alpha scenarios (default)

**When to use:**
- Running tagged scenario suites
- CI/CD pipeline execution
- Bulk scenario validation

**Example:**
```bash
# Run all active alpha scenarios
cargo xtask bdd --tags @alpha-active

# Output: bdd: selected 12 scenarios for @alpha-active
```

---

### `cargo xtask alpha-check`

**Purpose:** Run the complete alpha gate validation.

**Steps executed:**
1. `doctor` - Verify repo layout
2. `feature-grid` - Compile scenarios
3. `schema-check` - Validate JSON schemas
4. `test-ac` - All 10 active ACs
5. `bdd:@alpha-active` - Run tagged BDD scenarios
6. `golden:feature-grid` - Compare output to golden files
7. `golden:taxonomy-resolve` - Compare taxonomy output
8. `cli:describe-profile` - CLI smoke test
9. `cli:validate-fixture-success` - CLI success path
10. `cli:validate-fixture-failure` - CLI failure path

**Output:** `artifacts/alpha-check.summary.v1.json`

**When to use:**
- Before submitting PRs (use `make full` shortcut)
- Release validation
- CI/CD pipeline

**Example:**
```bash
cargo xtask alpha-check
# Output: alpha-check: active alpha gate passed
```

---

### `cargo xtask package-check`

**Purpose:** Verify that every publishable workspace crate packages cleanly for crates.io.

**What it does:**
1. Enumerates workspace crates
2. Skips workspace-only crates marked out of the publish surface
3. Runs `cargo package -p <crate> --allow-dirty --locked --list` for each remaining crate

`--list` is intentional here. It validates the package layout and manifest rewrite without requiring interdependent workspace crates to already exist on crates.io yet.

**When to use:**
- Before opening a release preparation PR
- After changing `Cargo.toml` files or crate boundaries
- Before publishing crates to crates.io

**Example:**
```bash
cargo xtask package-check
# Output: package-check: packaged 40 crate(s)
```

---

### `cargo xtask impact --changed <paths>`

**Purpose:** Determine which scenarios are impacted by changed file paths.

**Output:** `artifacts/impact/impact.report.v1.json`

**When to use:**
- Before running tests to know what's affected
- CI optimization (run only impacted scenarios)
- Understanding blast radius of changes

**Example:**
```bash
# Check impact of a specific file
cargo xtask impact --changed crates/ixds-assemble/src/lib.rs

# Check impact of multiple files
cargo xtask impact --changed crates/taxonomy-resolve/src/lib.rs --changed crates/ixds/src/lib.rs
```

---

### `cargo xtask bundle <selector>`

**Purpose:** Create a bounded context bundle for a scenario selector.

**Output:** `artifacts/bundles/<selector>.json`

**When to use:**
- Creating test bundles for specific scenarios
- Packaging scenarios for external testing

**Example:**
```bash
# Bundle a specific AC
cargo xtask bundle AC-XK-IXDS-002

# Bundle by scenario ID
cargo xtask bundle SCN-XK-WORKFLOW-002
```

---

### `cargo xtask cockpit-pack`

**Purpose:** Wrap a validation receipt into sensor.report.v1 format for cockpit integration.

**Output:** `artifacts/cockpit/sensor.report.v1.json`

**When to use:**
- Preparing reports for external monitoring
- CI/CD artifact generation

**Example:**
```bash
cargo xtask cockpit-pack
# Output: cockpit-pack: wrote artifacts/cockpit/sensor.report.v1.json
```

---

## Common Workflows

### Daily Development Loop

```bash
# 1. Make changes
vim crates/my-crate/src/lib.rs

# 2. Quick quality check
make quick

# 3. Iterate until clean

# 4. Check impact if working across crates
cargo xtask impact --changed crates/my-crate/src/lib.rs

# 5. Run specific AC if needed
cargo xtask test-ac AC-XK-TAXONOMY-001
```

### Pre-PR Checklist

```bash
# 1. Full validation
make full

# 2. If the change touches release metadata or crate boundaries
cargo xtask package-check

# 3. Verify clean git state
git status

# 4. Push and create PR
git push origin feature-branch
gh pr create --title "feat: description" --body "Details..."
```

### Debugging a Failing AC

```bash
# 1. Run the specific failing AC
cargo xtask test-ac AC-XK-IXDS-002

# 2. Check the scenario receipt
cat artifacts/runs/scenario.run.v1.json | jq

# 3. Check feature grid for context
cat artifacts/feature.grid.v1.json | jq '.scenarios[] | select(.ac_id == "AC-XK-IXDS-002")'

# 4. Run doctor to verify repo state
cargo xtask doctor
```

### Working with Fixtures

```bash
# Validate a specific fixture
cargo run -p xbrlkit-cli -- validate-fixture \
  --profile sec/efm-77/opco \
  --json \
  fixtures/synthetic/inline/ixds-single-file-01/member-a.html

# Check profile
cargo run -p xbrlkit-cli -- describe-profile --profile sec/efm-77/opco --json
```

---

## CI/CD Integration

The alpha gate is designed to be boring and stable for CI/CD:

```yaml
# Example CI workflow
- name: Quick Check
  run: make quick

- name: Full Alpha Gate
  run: make full
```

Or using xtask directly:

```yaml
# More granular CI steps
- name: Doctor
  run: cargo xtask doctor

- name: Feature Grid
  run: cargo xtask feature-grid

- name: Schema Check
  run: cargo xtask schema-check

- name: Alpha Check
  run: cargo xtask alpha-check
```

---

## Artifact Locations

Commands generate artifacts in the following locations:

| Command | Output Location |
|---------|-----------------|
| `feature-grid` | `artifacts/feature.grid.v1.json` |
| `schema-check` | Validated against `contracts/schemas/` |
| `test-ac` | `artifacts/runs/scenario.run.v1.json` |
| `alpha-check` | `artifacts/alpha-check.summary.v1.json` |
| `impact` | `artifacts/impact/impact.report.v1.json` |
| `bundle` | `artifacts/bundles/<selector>.json` |
| `cockpit-pack` | `artifacts/cockpit/sensor.report.v1.json` |
| `bdd` | `artifacts/runs/scenario.run.v1.json` |

---

## Troubleshooting

### Alpha Check Fails

1. Check which step failed in `artifacts/alpha-check.summary.v1.json`
2. Run the failing step individually for more details
3. Verify repo state with `cargo xtask doctor`
4. Check if golden files need updating (intentionally, with care)

### Feature Grid Compilation Fails

1. Verify sidecar YAML syntax (`*.meta.yaml` files)
2. Ensure Gherkin feature files are valid
3. Check that all referenced crates exist

### Schema Check Fails

1. Verify JSON output matches schema in `contracts/schemas/`
2. Check if schema version matches code output
3. Regenerate outputs if schema changes are intentional
