# Maintainer Documentation Research Notes

## Research Date: 2025-03-24
## Issue: #2 - Document maintainer command surface

### Current State

#### Makefile Targets
- `make help` - Shows available commands
- `make quick` - Fast quality gates (fmt, clippy, test)
- `make full` - Full alpha gate validation

#### xtask Commands (from xtask/src/main.rs)
- `cargo xtask doctor` - Sanity-check required repo directories
- `cargo xtask feature-grid` - Compile the feature grid from sidecars
- `cargo xtask bundle <selector>` - Create bounded context bundle for selector
- `cargo xtask impact --changed <paths>` - Estimate impacted scenarios from changed paths
- `cargo xtask test-ac <ac_id>` - Focused AC helper (runs specific acceptance criteria)
- `cargo xtask schema-check` - Validate emitted JSON outputs against schemas
- `cargo xtask alpha-check` - Run the alpha upload gate (FULL GATE)
- `cargo xtask bdd --tags <tags>` - Run active BDD scenarios by tag
- `cargo xtask cockpit-pack` - Wrap validation receipt into sensor.report.v1

#### Alpha Check Details (from xtask/src/alpha_check.rs)
The `alpha-check` command runs:
1. doctor - Verify repo layout
2. feature-grid - Compile scenarios
3. schema-check - Validate JSON schemas
4. test-ac for ACTIVE_ALPHA_ACS:
   - AC-XK-SEC-INLINE-001
   - AC-XK-SEC-INLINE-002
   - AC-XK-SEC-REQUIRED-001
   - AC-XK-SEC-REQUIRED-002
   - AC-XK-TAXONOMY-001
   - AC-XK-TAXONOMY-002
   - AC-XK-DUPLICATES-001
   - AC-XK-IXDS-001
   - AC-XK-IXDS-002
   - AC-XK-EXPORT-001
5. bdd:@alpha-active - Run BDD scenarios
6. golden:feature-grid - Compare to golden
7. golden:taxonomy-resolve - Compare to golden
8. cli:describe-profile - CLI smoke test
9. cli:validate-fixture-success - CLI success path
10. cli:validate-fixture-failure - CLI failure path

### Existing Documentation Structure
- `README.md` - Main project overview
- `CONTRIBUTING.md` - Contribution guidelines with basic commands
- `docs/architecture/` - Architecture docs
- `docs/audit/` - Audit docs
- `docs/how-to/` - How-to guides (add-scenario.md exists)
- `docs/reference/` - Reference docs (repo-map.md exists)

### Documentation Gaps
- No comprehensive maintainer command reference
- No explanation of when to use `make quick` vs `make full`
- No detailed explanation of each xtask command
- No usage examples for common maintainer workflows

### Recommendation
Create `docs/how-to/maintainer-commands.md` as a comprehensive guide covering:
1. Quick start for maintainers
2. The two gate commands (quick vs full)
3. All xtask commands with examples
4. Common workflows
5. CI/CD integration notes
