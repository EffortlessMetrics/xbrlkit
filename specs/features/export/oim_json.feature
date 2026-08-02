@REQ-XK-EXPORT
@layer.workflow
@suite.synthetic
Feature: OIM JSON export

  @alpha-active
  @SCN-XK-EXPORT-001
  @speed.fast
  Scenario: Emit canonical JSON export with provenance
    Given the fixture "synthetic/inline/ixds-single-file-01"
    When I export the canonical report to JSON
    Then the export report receipt is emitted

  @alpha-candidate
  @AC-XK-EXPORT-002
  @SCN-XK-EXPORT-002
  @speed.fast
  Scenario: Surface canonical JSON export failures
    Given the fixture "synthetic/inline/ixds-single-file-01"
    When I export the canonical report with a failing writer
    Then the export serialization error is surfaced
