# Filing Manifest Research Notes

## Scenario Requirements (SCN-XK-MANIFEST-001)

### Feature File
```gherkin
@REQ-XK-MANIFEST
@layer.standard
@suite.synthetic
Feature: Filing manifest

  @AC-XK-MANIFEST-001
  @SCN-XK-MANIFEST-001
  @speed.fast
  Scenario: Build a manifest from a minimal filing container
    Given the fixture "synthetic/filing/minimal-container-01"
    When I build the filing manifest
    Then the filing manifest receipt is emitted
```

## Fixture Analysis: minimal-container-01

**Location:** `fixtures/synthetic/filing/minimal-container-01/`

**Contents:**
- `README.md`: "Minimal filing-container fixture."
- `submission.txt`:
```
<SEC-DOCUMENT>0000000000-25-000001
<SEC-HEADER>ACCESSION NUMBER: 0000000000-25-000001
</SEC-DOCUMENT>
```

This is a minimal SEC submission with an accession number that can be parsed.

## Crate Dependencies

### filing-load (already exists)
**Location:** `crates/filing-load/`

**Current API:**
```rust
pub fn load_from_submission(input: &str) -> (FilingManifest, Receipt)
```

**Dependencies:**
- `edgar-sgml` - parse_identity function
- `edgar-attachments` - FilingManifest, Attachment, build_manifest
- `receipt-types` - Receipt, RunResult

**Key Types:**
- `FilingManifest` { filing: FilingIdentity, attachments: Vec<Attachment> }
- `FilingIdentity` { accession: String, cik: String, form: String }
- `Attachment` { name: String, kind: String }
- `Receipt` { kind, version, subject, result, artifacts, notes }

### edgar-sgml
**parse_identity()**: Extracts accession number from "ACCESSION NUMBER: " prefix

### edgar-attachments
**build_manifest()**: Creates FilingManifest from FilingIdentity and attachments

## Step Handler Implementation Plan

### 1. Update xbrlkit-bdd-steps Cargo.toml
Add dependency:
```toml
filing-load = { path = "../filing-load" }
```

### 2. Add to World struct (lib.rs)
```rust
pub filing_manifest: Option<FilingManifest>,
pub filing_receipt: Option<receipt_types::Receipt>,
```

### 3. Implement Given step (already exists pattern)
```rust
// Pattern exists in handle_given:
// "the fixture \"...\"" - loads fixture into world.fixture_dirs
```

### 4. Implement When step
```rust
if step.text == "I build the filing manifest" {
    // Read submission.txt from first fixture_dir
    let fixture_dir = world.fixture_dirs.first().context("no fixture loaded")?;
    let submission_path = fixture_dir.join("submission.txt");
    let submission = std::fs::read_to_string(&submission_path)?;
    
    // Use filing-load crate
    let (manifest, receipt) = filing_load::load_from_submission(&submission);
    world.filing_manifest = Some(manifest);
    world.filing_receipt = Some(receipt);
    return Ok(true);
}
```

### 5. Implement Then step
```rust
if step.text == "the filing manifest receipt is emitted" {
    if world.filing_receipt.is_none() {
        anyhow::bail!("filing manifest receipt was not emitted");
    }
    // Verify it's a filing.manifest receipt
    let receipt = world.filing_receipt.as_ref().unwrap();
    if receipt.kind != "filing.manifest" {
        anyhow::bail!("expected receipt kind 'filing.manifest', got '{}'", receipt.kind);
    }
    return Ok(());
}
```

### 6. Add @alpha-active tag to feature file
Add `@alpha-active` tag after `@speed.fast` in the scenario.

## Quality Gates

- [x] xbrlkit-bdd-steps compiles with new dependency
- [x] SCN-XK-MANIFEST-001 passes with @alpha-active
- [x] No regressions in existing scenarios
- [x] All quality gates pass

## Implementation Complete

### Changes Made:

1. **crates/xbrlkit-bdd-steps/Cargo.toml**
   - Added `filing-load = { path = "../filing-load" }`
   - Added `edgar-attachments = { path = "../edgar-attachments" }`

2. **crates/xbrlkit-bdd-steps/src/lib.rs**
   - Added `filing_manifest: Option<edgar_attachments::FilingManifest>` to World struct
   - Added `filing_receipt: Option<receipt_types::Receipt>` to World struct
   - Initialized new fields in `World::new()`
   - Implemented "I build the filing manifest" When step handler
   - Implemented "the filing manifest receipt is emitted" Then step handler

3. **specs/features/foundation/filing_manifest.feature**
   - Added `@alpha-active` tag to scenario

4. **xtask/src/alpha_check.rs**
   - Added `"AC-XK-MANIFEST-001"` to ACTIVE_ALPHA_ACS

5. **crates/scenario-runner/src/lib.rs**
   - Added assertion case for AC-XK-MANIFEST-001 in assert_scenario_outcome

### Verification:
- `cargo check --workspace` - ✅ Pass
- `cargo test --workspace --locked` - ✅ Pass
