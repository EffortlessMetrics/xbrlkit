# Research: describe-profile --json Acceptance Slice

## Overview
Issue #3 requires creating a BDD scenario that validates the `describe-profile --json` CLI command works correctly.

## 1. Existing Feature Files for CLI/Profile Commands

**Location:** `specs/features/`

Existing feature files found:
- `sec/taxonomy_years.feature` - Uses profile pack but validates filings
- `sec/inline_restrictions.feature` - Uses profile pack for validation
- `foundation/filing_manifest.feature` - No profile validation
- `workflow/*.feature` - Workflow-focused tests

**Finding:** No existing feature file specifically tests the CLI `describe-profile` command.

## 2. describe-profile Command Implementation

**Location:** `crates/xbrlkit-cli/src/main.rs`

```rust
Command::DescribeProfile { profile, json } => {
    let profile = load_profile(&profile)?;
    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&profile).context("serializing profile json")?
        );
    } else {
        println!(
            "{}",
            serde_yaml::to_string(&profile).context("serializing profile yaml")?
        );
    }
    0
}
```

The command:
- Accepts `--profile <PROFILE_ID>` parameter
- Accepts `--json` flag to output JSON instead of YAML
- Loads profile via `load_profile_from_workspace()` from `sec-profile-types`
- Outputs serialized `ProfilePack` struct

## 3. ProfilePack JSON Structure

**Location:** `crates/sec-profile-types/src/lib.rs`

```rust
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct ProfilePack {
    pub id: String,
    pub label: String,
    #[serde(default)]
    pub forms: Vec<String>,
    #[serde(default)]
    pub enabled_rule_families: Vec<String>,
    #[serde(default)]
    pub inline_rules: InlineRules,
    #[serde(default)]
    pub accepted_taxonomies: AcceptedTaxonomies,
    #[serde(default)]
    pub standard_taxonomy_uris: Vec<String>,
    #[serde(default)]
    pub required_facts: Vec<String>,
}
```

Required fields for validation:
- `id` - Profile identifier (e.g., "sec/efm-77/opco")
- `label` - Human-readable label
- `forms` - List of supported forms (e.g., ["10-K", "10-Q", "8-K"])
- `enabled_rule_families` - List of rule families
- `standard_taxonomy_uris` - List of taxonomy URIs
- `required_facts` - List of required fact concepts

## 4. Existing CLI-Focused Scenarios

**Pattern from existing feature files:**

```gherkin
@REQ-XK-XXX
@layer.<layer>
@suite.<suite>
Feature: <Feature Name>

  @alpha-active
  @AC-XK-XXX-NNN
  @SCN-XK-XXX-NNN
  @speed.fast
  Scenario: <Scenario description>
    Given the profile pack "<profile_id>"
    And the fixture directory "<fixture_path>"
    When I <action>
    Then <assertion>
```

## 5. BDD Step Handlers

**Location:** `crates/xbrlkit-bdd-steps/src/lib.rs`

Current step handlers available:
- `Given the profile pack "..."` - Sets profile_id in World
- `Given the fixture directory "..."` - Sets fixture_dirs in World
- `When I validate the filing` - Runs validation scenario
- `When I resolve the DTS` - Runs DTS resolution
- `Then the validation report contains rule "..."` - Assertion
- Many more...

**Missing for CLI profile testing:**
- `When I run describe-profile --json` 
- `Then the output is valid JSON`
- `And the profile contains required fields`

## 6. Feature Grid and Meta Files

**Location:** `specs/features/<module>/<feature>.meta.yaml`

Meta files define:
- `feature_id` - Unique feature identifier
- `layer` - Architecture layer (sec, foundation, workflow, etc.)
- `module` - Module name
- `scenarios` - Map of scenario definitions with:
  - `ac_id` - Acceptance criterion ID
  - `req_id` - Requirement ID
  - `crates` - Affected crates
  - `fixtures` - Required fixtures (optional)
  - `profile_pack` - Profile pack to use (optional)
  - `receipts` - Expected receipts
  - `suite` - Test suite (synthetic, corpus, etc.)
  - `speed` - Test speed (fast, slow)

## 7. Available Profiles

**Location:** `profiles/`

Available profile:
- `sec/efm-77/opco` - SEC EFM 77 operating companies
  - Forms: [10-K, 10-Q, 8-K, 20-F, 40-F, 6-K]
  - Rule families: standard_locations, taxonomy_years, inline_restrictions, required_facts

---

## Implementation Complete

### Files Created/Modified

1. **Feature File:** `specs/features/cli/describe_profile.feature`
   - Created new CLI feature file with `@alpha-active` scenario
   - Tests the `describe-profile --json` command output

2. **Meta File:** `specs/features/cli/describe_profile.meta.yaml`
   - Defines scenario metadata for feature grid integration
   - Includes AC-ID, REQ-ID, affected crates, and allowed edit roots

3. **BDD Steps:** `crates/xbrlkit-bdd-steps/src/lib.rs`
   - Added `cli_output` and `cli_json_output` fields to `World` struct
   - Added Given step: `a SEC profile is configured`
   - Added When step: `I run describe-profile --json`
   - Added Then steps: `the output is valid JSON`, `the profile contains required fields`
   - Changed `handle_then` signature to `&mut World` for JSON parsing storage

4. **Dependencies:** `crates/xbrlkit-bdd-steps/Cargo.toml`
   - Added `sec-profile-types` dependency for profile loading

### Scenario Definition

```gherkin
@REQ-XK-CLI
@layer.cli
@suite.synthetic
Feature: CLI describe-profile command

  @alpha-active
  @AC-XK-CLI-001
  @SCN-XK-CLI-001
  @speed.fast
  Scenario: Output profile as JSON
    Given a SEC profile is configured
    When I run describe-profile --json
    Then the output is valid JSON
    And the profile contains required fields
```

### Step Implementation Details

**Given a SEC profile is configured:**
- Sets `world.profile_id = Some("sec/efm-77/opco".to_string())`
- Uses the standard SEC profile for operating companies

**When I run describe-profile --json:**
- Loads profile using `sec_profile_types::load_profile_from_workspace()`
- Serializes to JSON using `serde_json::to_string_pretty()`
- Stores output in `world.cli_output`

**Then the output is valid JSON:**
- Parses `world.cli_output` as `serde_json::Value`
- Stores parsed value in `world.cli_json_output`

**Then the profile contains required fields:**
- Validates presence of: `id`, `label`, `forms`, `enabled_rule_families`, `standard_taxonomy_uris`, `required_facts`
- Fails with descriptive message if any field is missing

## Acceptance Criteria Status

- [x] New scenario created with @alpha-active tag
- [x] Step handlers implemented in xbrlkit-bdd-steps
- [x] Feature file and meta file created
- [ ] New scenario passes with @alpha-active (requires cargo test)
- [ ] All quality gates pass (requires CI)
- [ ] PR merged (requires git workflow)
