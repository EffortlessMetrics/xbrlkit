# Plan: Spec Ledger Gap Remediation — Revised (Issue #243)

> **Revision note:** This is a revised plan. The original plan (v1) was reviewed and approved by `reviewer-plan` and `reviewer-deep-plan`, but `reviewer-repo-alignment` identified additional untagged features and meta.yaml inconsistencies. This revision addresses those gaps.

## Overview

The `specs/spec_ledger.yaml` is the canonical source of truth for xbrlkit's requirement traceability — it links user stories (US-*) to requirements (REQ-*) to acceptance criteria (AC-*) to feature-file scenarios (SCN-*). The Scout automation detected **6 explicit `@REQ-*` tags** in feature files with no corresponding ledger entries, plus **4 feature files** that lack REQ tags entirely (the original plan identified only 2 of these).

This plan addresses the traceability gap by:
1. Adding **6 missing REQ entries** to `specs/spec_ledger.yaml` (explicit tags)
2. Adding **4 new REQ entries** for untagged features, aligning feature tags with their `.meta.yaml` `req_id` fields
3. Fixing **2 `.meta.yaml` files** missing `req_id` fields
4. Adding missing **layer/suite tags** to `streaming_parser.feature`
5. Establishing a repeatable verification mechanism to prevent future drift

## Current State

### Ledger Entries Present (6)
| REQ | Title | File |
|-----|-------|------|
| REQ-XK-MANIFEST | Build a normalized filing manifest | foundation/filing_manifest.feature |
| REQ-XK-IXDS | Assemble inline documents into an IXDS | inline/ixds_assembly.feature |
| REQ-XK-TAXONOMY | Resolve SEC taxonomy locations deterministically | taxonomy/standard_locations.feature |
| REQ-XK-SEC-INLINE | Enforce SEC inline restrictions | sec/inline_restrictions.feature |
| REQ-XK-DUPLICATES | Apply duplicate fact policy | foundation/duplicate_facts.feature |
| REQ-XK-WORKFLOW | Package scenario-bounded work for agents | workflow/feature_grid.feature |

### Ledger Entries Missing (6 explicit REQ tags)
| REQ Tag | Feature File | Status | AC Tags | SCN Count |
|---------|-------------|--------|---------|-----------|
| REQ-XK-CLI | cli/describe_profile.feature | @alpha-active | AC-XK-CLI-001 | 1 |
| REQ-XK-CONTEXT | foundation/context_completeness.feature | @alpha-candidate | AC-XK-CONTEXT-001..004 | 4 |
| REQ-XK-SEC-DECIMAL | sec/decimal_precision.feature | @alpha-candidate | AC-XK-SEC-DECIMAL-001, 002 | 10 |
| REQ-XK-SEC-NEGATIVE | sec/negative_values.feature | @alpha-candidate | AC-XK-SEC-NEGATIVE-001..005 | 5 |
| REQ-XK-SEC-REQUIRED | sec/required_facts.feature | @alpha-active | AC-XK-SEC-REQUIRED-001, 002 | 2 |
| REQ-XK-TAXONOMY-LOADER | taxonomy/taxonomy_loader.feature | @alpha-active | AC-XK-TAX-LOAD-001..008 | 8 |

### Untagged Features (no `@REQ-*` tag at feature level) — 4 total
| File | Layer | Suite | Scenarios | meta.yaml req_id | Plan status |
|------|-------|-------|-----------|------------------|-------------|
| performance/streaming_parser.feature | (none) | (none) | SCN-XK-STREAM-001..004 | `REQ-XK-PERFORMANCE` | ✅ v1 identified |
| export/oim_json.feature | @layer.workflow | @suite.synthetic | SCN-XK-EXPORT-001 | **missing** | ✅ v1 identified |
| taxonomy/dimensions.feature | (none) | (none) | SCN-XK-DIM-001..017 | `REQ-XK-DIMENSIONS` | ❌ **v1 missed** |
| workflow/cockpit_pack.feature | @layer.workflow | @suite.synthetic | SCN-XK-WORKFLOW-003 | **missing** | ❌ **v1 missed** |

## Acceptance Criteria Breakdown

### AC-1: Add REQ-XK-CLI to ledger
- **Parent story**: US-XK-001 (Validate a filing deterministically) — CLI is the entry point for validation
- **REQ title**: Provide CLI commands for profile and report introspection
- **Acceptance criteria to map**:
  - AC-XK-CLI-001: Output profile as JSON → `specs/features/cli/describe_profile.feature`
  - SCN-XK-CLI-001

### AC-2: Add REQ-XK-CONTEXT to ledger
- **Parent story**: US-XK-001 — context completeness is a foundational validation gate
- **REQ title**: Validate that all facts reference defined contexts
- **Acceptance criteria to map**:
  - AC-XK-CONTEXT-001: Fact references missing context → `specs/features/foundation/context_completeness.feature`
  - AC-XK-CONTEXT-002: All facts reference valid contexts
  - AC-XK-CONTEXT-003: Context ID matching is case-insensitive
  - AC-XK-CONTEXT-004: Multiple facts with missing contexts
  - SCN-XK-CONTEXT-001..004

### AC-3: Add REQ-XK-SEC-DECIMAL to ledger
- **Parent story**: US-XK-001 — SEC numeric validation rule (EFM 6.5.37)
- **REQ title**: Enforce decimal precision rules for numeric facts
- **Acceptance criteria to map**:
  - AC-XK-SEC-DECIMAL-001: Nonzero digits must not be truncated → `specs/features/sec/decimal_precision.feature`
  - AC-XK-SEC-DECIMAL-002: Valid rounding with appropriate decimals is allowed
  - SCN-XK-SEC-DECIMAL-001..010

### AC-4: Add REQ-XK-SEC-NEGATIVE to ledger
- **Parent story**: US-XK-001 — SEC semantic validation for non-negative concepts
- **REQ title**: Reject negative values for semantically non-negative concepts
- **Acceptance criteria to map**:
  - AC-XK-SEC-NEGATIVE-001: Negative share count detected as error → `specs/features/sec/negative_values.feature`
  - AC-XK-SEC-NEGATIVE-002: Valid non-negative share count passes
  - AC-XK-SEC-NEGATIVE-003: Negative employee count detected
  - AC-XK-SEC-NEGATIVE-004: Accounting notation (parentheses) detected
  - AC-XK-SEC-NEGATIVE-005: Financial loss values can be negative
  - SCN-XK-SEC-NEGATIVE-001..005

### AC-5: Add REQ-XK-SEC-REQUIRED to ledger
- **Parent story**: US-XK-001 — SEC DEI required-facts policy
- **REQ title**: Verify presence of SEC-required DEI facts
- **Acceptance criteria to map**:
  - AC-XK-SEC-REQUIRED-001: Missing required fact detected and reported → `specs/features/sec/required_facts.feature`
  - AC-XK-SEC-REQUIRED-002: All required facts present passes validation
  - SCN-XK-SEC-REQUIRED-001, 002

### AC-6: Add REQ-XK-TAXONOMY-LOADER to ledger
- **Parent story**: US-XK-001 — taxonomy resolution dependency
- **REQ title**: Load and cache taxonomy definitions for validation
- **Acceptance criteria to map**:
  - AC-XK-TAX-LOAD-001: Load dimension definitions from schema → `specs/features/taxonomy/taxonomy_loader.feature`
  - AC-XK-TAX-LOAD-002: Load domain hierarchies from definition linkbase
  - AC-XK-TAX-LOAD-003: Load typed dimension definitions
  - AC-XK-TAX-LOAD-004: Load hypercube definitions
  - AC-XK-TAX-LOAD-005: Cache taxonomy files locally
  - AC-XK-TAX-LOAD-006: Handle schema imports recursively
  - AC-XK-TAX-LOAD-007: Validate dimension-member against loaded taxonomy
  - AC-XK-TAX-LOAD-008: Reject invalid member against loaded taxonomy
  - SCN-XK-TAX-LOAD-001..008

### AC-7: Resolve streaming_parser.feature REQ tag (REVISED)
- **Decision**: Add `@REQ-XK-PERFORMANCE` to the feature file, **not** `@REQ-XK-STREAM`
- **Rationale**: The `.meta.yaml` already declares `req_id: REQ-XK-PERFORMANCE` for all 4 scenarios. The feature tag must match the meta.yaml or the Scout's forward-traceability check will fail.
- **Additional action**: Add `@layer.performance` and `@suite.synthetic` to the feature file to match the meta.yaml `layer: performance` and `suite: synthetic` values. Every other feature file declares both; this is the only exception.
- **Ledger entry**: REQ-XK-PERFORMANCE — "Stream-parse large XBRL filings with bounded memory"
- **ACs to map**: AC-XK-STREAM-001..004 (memory <50MB, fallback logic, context completeness, error handling)
- **SCNs**: SCN-XK-STREAM-001..004

### AC-8: Resolve oim_json.feature REQ tag (REVISED)
- **Decision**: Add `@REQ-XK-EXPORT` to the feature file
- **Rationale**: OIM JSON export is a user-facing output format with a clear acceptance criterion (canonical JSON export with provenance). The feature already has `@alpha-active`.
- **Additional action**: Add `req_id: REQ-XK-EXPORT` to `oim_json.meta.yaml` for `SCN-XK-EXPORT-001`. This meta.yaml is the only other one besides `cockpit_pack.meta.yaml` that lacks `req_id`.
- **Ledger entry**: REQ-XK-EXPORT — "Emit canonical OIM JSON export with provenance"
- **ACs to map**: AC-XK-EXPORT-001
- **SCNs**: SCN-XK-EXPORT-001

### AC-9: Add REQ-XK-DIMENSIONS to ledger (NEW)
- **Parent story**: US-XK-001 — dimensional validation is a core taxonomy capability
- **REQ title**: Validate dimension-member pairs against taxonomy definitions
- **Rationale**: `dimensions.feature` has 17 scenarios (SCN-XK-DIM-001..017) with corresponding AC tags (AC-XK-DIM-001..017). Its `.meta.yaml` already declares `req_id: REQ-XK-DIMENSIONS` per scenario. This is a fully-specified feature that was simply missed by the Scout's initial scan.
- **Feature file action**: Add `@REQ-XK-DIMENSIONS` at the feature level of `dimensions.feature`
- **ACs to map**: AC-XK-DIM-001..017
- **SCNs**: SCN-XK-DIM-001..017
- **Note**: The AC-to-SCN ratio is 1:1 for this REQ (17 ACs, 17 SCNs), which is the simplest mapping pattern in the ledger.

### AC-10: Resolve cockpit_pack.feature REQ tag (NEW)
- **Parent story**: US-XK-001 — cockpit packaging is a workflow-layer deliverable
- **REQ title**: Package validation receipts for cockpit sensor reports
- **Rationale**: `cockpit_pack.feature` has one scenario (`SCN-XK-WORKFLOW-003`) with `@layer.workflow @suite.synthetic`. Its `.meta.yaml` is missing `req_id`. Since the scenario ID (`SCN-XK-WORKFLOW-003`) aligns with the existing `REQ-XK-WORKFLOW` story family, map it there rather than creating a new REQ.
- **Feature file action**: Add `@REQ-XK-WORKFLOW` at the feature level
- **Meta.yaml action**: Add `req_id: REQ-XK-WORKFLOW` to `SCN-XK-WORKFLOW-003`
- **Ledger action**: Add `SCN-XK-WORKFLOW-003` as an additional test under the existing `AC-XK-WORKFLOW-003` entry ("Run the alpha readiness gate"). The `cockpit_pack.feature` tests receipt packaging, which is part of the alpha readiness workflow.
- **Note**: `alpha_check.feature` already maps to `AC-XK-WORKFLOW-003` via `workflow/alpha_check.feature`. Having two feature files for the same AC is acceptable if they test different aspects of the same criterion.

### AC-11: Ensure meta.yaml req_id consistency (NEW)
- **Scope**: All `.meta.yaml` files in `specs/features/`
- **Check**: Every scenario entry must have a `req_id` field
- **Files requiring fixes**:
  - `export/oim_json.meta.yaml` — add `req_id: REQ-XK-EXPORT` to `SCN-XK-EXPORT-001`
  - `workflow/cockpit_pack.meta.yaml` — add `req_id: REQ-XK-WORKFLOW` to `SCN-XK-WORKFLOW-003`
- **Verification**: After fixes, run: `find specs/features -name "*.meta.yaml" -exec sh -c 'grep -q "req_id:" "$1" || echo "MISSING: $1"' _ {} \;` — should return nothing.

## Proposed Approach

### Ledger Structure
All new REQ entries follow the existing schema in `specs/spec_ledger.yaml`:

```yaml
stories:
  - id: US-XK-001
    title: Validate a filing deterministically
    requirements:
      # existing 6 entries...
      - id: REQ-XK-CLI
        title: Provide CLI commands for profile and report introspection
        acceptance_criteria:
          - id: AC-XK-CLI-001
            title: Output profile as JSON
            tests:
              - type: bdd
                tag: "@AC-XK-CLI-001"
                file: "specs/features/cli/describe_profile.feature"
      # ... etc
```

### Parent Story Assignment
All missing REQ entries logically belong under **US-XK-001** ("Validate a filing deterministically") because:
- CLI introspection (REQ-XK-CLI) is the primary user interface for running validation
- Context completeness (REQ-XK-CONTEXT) is a prerequisite validation gate
- SEC decimal/negative/required rules (REQ-XK-SEC-*) are validation subsystems
- Taxonomy loading (REQ-XK-TAXONOMY-LOADER) and dimensions (REQ-XK-DIMENSIONS) are taxonomy dependencies
- Streaming (REQ-XK-PERFORMANCE) and export (REQ-XK-EXPORT) are operational modes of the validation pipeline
- Workflow packaging (REQ-XK-WORKFLOW, already present) includes cockpit receipt packaging

If the project later splits US-XK-001 into narrower stories (e.g., US-XK-002 for SEC rules, US-XK-003 for CLI), the REQ entries can be re-parented without structural changes.

### REQ Naming Alignment
The following REQ IDs must be used to maintain consistency with existing `.meta.yaml` files:

| Feature File | meta.yaml req_id | Feature Tag to Add |
|-------------|------------------|-------------------|
| `streaming_parser.feature` | `REQ-XK-PERFORMANCE` | `@REQ-XK-PERFORMANCE` |
| `dimensions.feature` | `REQ-XK-DIMENSIONS` | `@REQ-XK-DIMENSIONS` |
| `oim_json.feature` | (missing) | `@REQ-XK-EXPORT` |
| `cockpit_pack.feature` | (missing) | `@REQ-XK-WORKFLOW` |

For `oim_json` and `cockpit_pack`, the `req_id` is added to the meta.yaml as part of this plan. The chosen IDs (`REQ-XK-EXPORT`, `REQ-XK-WORKFLOW`) align with the feature's purpose and existing story family.

## Files to Modify/Create

### Modified Files (7)
1. **`specs/spec_ledger.yaml`** — Add **10** new REQ entries (6 explicit + 4 newly tagged) with their acceptance criteria and test mappings. No breaking schema changes.
2. **`specs/features/performance/streaming_parser.feature`** — Add `@REQ-XK-PERFORMANCE` at feature level. Add `@layer.performance @suite.synthetic`.
3. **`specs/features/export/oim_json.feature`** — Add `@REQ-XK-EXPORT` at feature level.
4. **`specs/features/taxonomy/dimensions.feature`** — Add `@REQ-XK-DIMENSIONS` at feature level.
5. **`specs/features/workflow/cockpit_pack.feature`** — Add `@REQ-XK-WORKFLOW` at feature level.
6. **`specs/features/export/oim_json.meta.yaml`** — Add `req_id: REQ-XK-EXPORT` to `SCN-XK-EXPORT-001`.
7. **`specs/features/workflow/cockpit_pack.meta.yaml`** — Add `req_id: REQ-XK-WORKFLOW` to `SCN-XK-WORKFLOW-003`.

### New Files (0)
No new files required for this plan.

### No Code Changes
This is a documentation/traceability task; no Rust code changes are needed.

## Test Strategy

### Verification Steps
1. **Tag completeness (forward)**: Extract all `@REQ-*` tags from `.feature` files and assert each has a corresponding entry in `specs/spec_ledger.yaml`.
2. **Tag completeness (reverse)**: Extract all `REQ-XK-*` IDs from the ledger and assert each has at least one `@REQ-*` tag in a `.feature` file.
3. **AC coverage**: For each REQ in the ledger, verify every AC-* and SCN-* referenced in the feature file is listed under that REQ's `acceptance_criteria`.
4. **Meta.yaml consistency**: Verify every `.meta.yaml` scenario entry has a `req_id` field, and that it matches the corresponding `@REQ-*` tag in the feature file.
5. **Schema validation**: Confirm the updated `spec_ledger.yaml` parses as valid YAML and matches the expected structure (version, stories, requirements, acceptance_criteria, tests).

### Automated Guard (post-implementation)
Add a CI step or xtask command that runs on every PR touching `specs/`:
```bash
# Conceptual
./scripts/verify_spec_ledger.py \
  --ledger specs/spec_ledger.yaml \
  --features specs/features/ \
  --meta specs/features/
```
This prevents future spec drift by failing the build if:
- A feature file introduces a REQ tag not present in the ledger
- A ledger entry references a nonexistent AC/SCN tag
- A `.meta.yaml` scenario lacks a `req_id` field
- A `.meta.yaml` `req_id` disagrees with the feature file's `@REQ-*` tag

### Manual Verification Commands
```bash
# 1. List all REQ tags in feature files (feature level)
grep -rh "^@REQ-XK-" specs/features/ | sort -u

# 2. List all REQ IDs in ledger
yq '.stories[].requirements[].id' specs/spec_ledger.yaml | sort -u

# 3. Diff forward (feature → ledger)
diff <(grep -rh "^@REQ-XK-" specs/features/ | sed 's/@//' | sort -u) \
     <(yq '.stories[].requirements[].id' specs/spec_ledger.yaml | sort -u)

# 4. Check meta.yaml req_id completeness
find specs/features -name "*.meta.yaml" -exec sh -c 'grep -q "req_id:" "$1" || echo "MISSING req_id: $1"' _ {} \;

# 5. Reverse check: ledger entries with no feature-file tag
for req in $(yq '.stories[].requirements[].id' specs/spec_ledger.yaml | sort -u); do
  if ! grep -rq "@$req" specs/features/; then
    echo "ORPHAN: $req"
  fi
done
```

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|------------|--------|------------|
| **Spec drift recurrence** | High | Medium | Add automated CI check (see Test Strategy). File follow-up issue immediately after closing #243. |
| **US-XK-001 becomes overloaded** | Medium | Low | Acceptable for now; can split into US-XK-002+ later without breaking existing structure |
| **Untagged features excluded again** | Medium | Medium | Document tagging convention in `specs/README.md` or `AGENTS.md`; add Scout full-tree scan |
| **Schema change in future breaks ledger** | Low | High | Ledger has `version: 1` field; any schema evolution should bump version and migrate |
| **Inconsistent REQ title wording** | Medium | Low | Follow existing pattern: verb-led, concise, describes what the system must do |
| **Meta.yaml / feature-file tag mismatch** | Medium | Medium | AC-11 explicitly checks this; verification script catches disagreements |

## Estimated Effort

| Task | Estimate |
|------|----------|
| Add 6 explicit REQ entries to ledger with AC/SCN mappings | 1h |
| Add 4 new REQ entries for untagged features (performance, export, dimensions, cockpit) | 45 min |
| Add REQ tags to 4 feature files + layer/suite tags to streaming | 20 min |
| Fix 2 meta.yaml req_id gaps | 10 min |
| Verify with diff / grep / yq / meta checks | 30 min |
| Document tagging convention (prevent recurrence) | 30 min |
| **Total** | **~3–3.5 hours** |

## Dependencies

- None blocking. This plan is self-contained.
- **Recommended follow-up**: File a separate issue for the automated spec-ledger verification CI step / xtask script. Do this before closing #243 so the #1 risk (spec drift recurrence) has a tracked mitigation.

## Future Work

1. **Automated spec-ledger verification CI step** — Run on every PR to `specs/`. File as separate issue before closing #243.
2. **Story decomposition** — If US-XK-001 grows beyond ~14 requirements, split into narrower stories (e.g., US-XK-002: SEC Validation, US-XK-003: CLI & Export).
3. **Cross-reference with issues #170–#175** — Those issues previously tracked some of these gaps; after ledger update, verify and close them as resolved.
4. **Scout improvement** — Update the Scout to scan the full `specs/features/` tree (not stopping at the first N untagged features) and to validate `.meta.yaml` `req_id` completeness.

---
*Plan revised by planner-initial agent for Issue #243 (revision v2)*
